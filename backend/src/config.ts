import 'dotenv/config';

export const config = {
  // Server
  port: parseInt(process.env.PORT || '3000', 10),
  host: process.env.HOST || '0.0.0.0',
  nodeEnv: process.env.NODE_ENV || 'development',

  // Database
  databaseUrl: process.env.DATABASE_URL || 'postgresql://postgres:postgres@localhost:5432/openagentmail',

  // SMTP - Email sending
  smtp: {
    host: process.env.SMTP_HOST || 'localhost',
    port: parseInt(process.env.SMTP_PORT || '587', 10),
    secure: process.env.SMTP_SECURE === 'true',
    user: process.env.SMTP_USER,
    pass: process.env.SMTP_PASS,
  },

  // Default domain for inboxes
  defaultDomain: process.env.DEFAULT_DOMAIN || 'mail.openagentmail.com',

  // Rate limiting
  rateLimit: {
    free: {
      max: 60,
      timeWindow: '1 minute',
    },
    pro: {
      max: 600,
      timeWindow: '1 minute',
    },
    enterprise: {
      max: 6000,
      timeWindow: '1 minute',
    },
  },

  // Webhook
  webhook: {
    maxRetries: 3,
    retryDelayMs: 1000,
    timeoutMs: 30000,
  },

  // Pagination
  pagination: {
    defaultLimit: 20,
    maxLimit: 100,
  },
} as const;

export type Config = typeof config;
