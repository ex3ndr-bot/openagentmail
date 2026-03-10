// Example: Setting up webhooks

import { OpenAgentMail } from 'openagentmail';

async function main() {
  const client = new OpenAgentMail({
    apiKey: process.env.OAM_API_KEY!,
  });

  // Create a webhook for new messages
  const webhook = await client.webhooks.create({
    url: 'https://your-server.com/webhooks/email',
    eventTypes: ['message.received', 'message.sent'],
    // Optionally filter to specific inboxes
    // inboxIds: ['inbox_abc123'],
  });

  console.log('Created webhook:', webhook.webhookId);
  console.log('Webhook secret:', webhook.secret);
  console.log('Events:', webhook.eventTypes.join(', '));

  // Update webhook to disable it temporarily
  const updated = await client.webhooks.update(webhook.webhookId, {
    enabled: false,
  });
  console.log('\nWebhook enabled:', updated.enabled);

  // Rotate the webhook secret
  const rotated = await client.webhooks.rotateSecret(webhook.webhookId);
  console.log('\nNew secret:', rotated.secret);

  // List all webhooks
  console.log('\nAll webhooks:');
  for await (const wh of client.webhooks.list()) {
    console.log(`  - ${wh.url} (${wh.enabled ? 'enabled' : 'disabled'})`);
  }

  // Clean up
  await client.webhooks.delete(webhook.webhookId);
  console.log('\nDeleted webhook');
}

main().catch(console.error);
