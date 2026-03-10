import { PrismaClient } from '@prisma/client';
import { config } from '../config.js';
import { v4 as uuidv4 } from 'uuid';
import crypto from 'crypto';

const prisma = new PrismaClient();

interface WebhookPayload {
  id: string;
  type: string;
  created_at: string;
  data: Record<string, unknown>;
}

class WebhookService {
  // Sign webhook payload
  signPayload(payload: string, secret: string): string {
    return crypto.createHmac('sha256', secret).update(payload).digest('hex');
  }

  // Deliver webhook with retries
  async deliver(webhookId: string, payload: WebhookPayload): Promise<void> {
    const webhook = await prisma.webhook.findUnique({
      where: { id: webhookId },
    });

    if (!webhook || !webhook.enabled) {
      return;
    }

    const payloadString = JSON.stringify(payload);
    const signature = this.signPayload(payloadString, webhook.secret);

    // Create delivery record
    const delivery = await prisma.webhookDelivery.create({
      data: {
        webhookId,
        eventType: payload.type,
        payload,
        status: 'pending',
      },
    });

    // Attempt delivery
    await this.attemptDelivery(delivery.id, webhook.url, payloadString, signature);
  }

  private async attemptDelivery(
    deliveryId: string,
    url: string,
    payload: string,
    signature: string,
    attempt: number = 1
  ): Promise<void> {
    try {
      const controller = new AbortController();
      const timeoutId = setTimeout(() => controller.abort(), config.webhook.timeoutMs);

      const response = await fetch(url, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'X-Webhook-Signature': signature,
          'X-Webhook-Delivery-Id': deliveryId,
        },
        body: payload,
        signal: controller.signal,
      });

      clearTimeout(timeoutId);

      const responseBody = await response.text().catch(() => '');

      await prisma.webhookDelivery.update({
        where: { id: deliveryId },
        data: {
          status: response.ok ? 'success' : 'failed',
          attempts: attempt,
          lastAttempt: new Date(),
          responseCode: response.status,
          responseBody: responseBody.slice(0, 1000),
        },
      });

      // Retry on failure
      if (!response.ok && attempt < config.webhook.maxRetries) {
        const delay = config.webhook.retryDelayMs * Math.pow(2, attempt - 1);
        setTimeout(() => {
          this.attemptDelivery(deliveryId, url, payload, signature, attempt + 1);
        }, delay);
      }
    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : 'Unknown error';
      
      await prisma.webhookDelivery.update({
        where: { id: deliveryId },
        data: {
          status: 'failed',
          attempts: attempt,
          lastAttempt: new Date(),
          responseBody: errorMessage,
        },
      });

      // Retry on error
      if (attempt < config.webhook.maxRetries) {
        const delay = config.webhook.retryDelayMs * Math.pow(2, attempt - 1);
        setTimeout(() => {
          this.attemptDelivery(deliveryId, url, payload, signature, attempt + 1);
        }, delay);
      }
    }
  }

  // Dispatch event to all matching webhooks
  async dispatchEvent(
    organizationId: string,
    eventType: string,
    data: Record<string, unknown>,
    inboxId?: string,
    podId?: string
  ): Promise<void> {
    // Find matching webhooks
    const webhooks = await prisma.webhook.findMany({
      where: {
        organizationId,
        enabled: true,
        eventTypes: { has: eventType },
        OR: [
          // Match by inbox
          { inboxIds: inboxId ? { has: inboxId } : undefined },
          // Match by pod
          { podIds: podId ? { has: podId } : undefined },
          // Match all (empty arrays)
          { AND: [{ inboxIds: { isEmpty: true } }, { podIds: { isEmpty: true } }] },
        ],
      },
    });

    const payload: WebhookPayload = {
      id: uuidv4(),
      type: eventType,
      created_at: new Date().toISOString(),
      data,
    };

    // Deliver to all matching webhooks
    await Promise.all(webhooks.map(wh => this.deliver(wh.id, payload)));
  }

  // Generate a random secret
  generateSecret(): string {
    return crypto.randomBytes(32).toString('hex');
  }
}

export const webhookService = new WebhookService();
