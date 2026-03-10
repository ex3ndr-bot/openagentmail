import nodemailer, { Transporter } from 'nodemailer';
import { config } from '../config.js';
import { v4 as uuidv4 } from 'uuid';

interface SendEmailOptions {
  from: string;
  to: string[];
  cc?: string[];
  bcc?: string[];
  subject: string;
  text?: string;
  html?: string;
  headers?: Record<string, string>;
  replyTo?: string;
}

interface SendEmailResult {
  messageId: string;
  accepted: string[];
  rejected: string[];
}

class EmailService {
  private transporter: Transporter | null = null;

  private getTransporter(): Transporter {
    if (!this.transporter) {
      this.transporter = nodemailer.createTransport({
        host: config.smtp.host,
        port: config.smtp.port,
        secure: config.smtp.secure,
        auth: config.smtp.user ? {
          user: config.smtp.user,
          pass: config.smtp.pass,
        } : undefined,
        // For development without auth
        ignoreTLS: config.nodeEnv === 'development',
      });
    }
    return this.transporter;
  }

  async sendEmail(options: SendEmailOptions): Promise<SendEmailResult> {
    const transporter = this.getTransporter();

    const mailOptions = {
      from: options.from,
      to: options.to.join(', '),
      cc: options.cc?.join(', '),
      bcc: options.bcc?.join(', '),
      subject: options.subject,
      text: options.text,
      html: options.html,
      replyTo: options.replyTo,
      headers: options.headers,
      messageId: `<${uuidv4()}@${config.defaultDomain}>`,
    };

    try {
      const info = await transporter.sendMail(mailOptions);
      return {
        messageId: info.messageId,
        accepted: Array.isArray(info.accepted) ? info.accepted.map(String) : [],
        rejected: Array.isArray(info.rejected) ? info.rejected.map(String) : [],
      };
    } catch (error) {
      console.error('Email send error:', error);
      throw error;
    }
  }

  async verifyConnection(): Promise<boolean> {
    try {
      const transporter = this.getTransporter();
      await transporter.verify();
      return true;
    } catch (error) {
      console.error('SMTP verification failed:', error);
      return false;
    }
  }
}

export const emailService = new EmailService();
