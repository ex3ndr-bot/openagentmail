// Example: Sending messages

import { OpenAgentMail } from 'openagentmail';

async function main() {
  const client = new OpenAgentMail({
    apiKey: process.env.OAM_API_KEY!,
  });

  const inboxId = process.env.INBOX_ID!;

  // Send a plain text email
  const message1 = await client.messages.send(inboxId, {
    to: ['user@example.com'],
    subject: 'Hello from OpenAgentMail',
    text: 'This is a test email sent via the TypeScript SDK.',
  });
  console.log('Sent message:', message1.messageId);

  // Send an HTML email
  const message2 = await client.messages.send(inboxId, {
    to: ['user@example.com'],
    cc: ['cc@example.com'],
    subject: 'HTML Email Example',
    text: 'Plain text fallback',
    html: `
      <html>
        <body>
          <h1>Hello!</h1>
          <p>This is an <strong>HTML email</strong> sent via OpenAgentMail.</p>
        </body>
      </html>
    `,
  });
  console.log('Sent HTML message:', message2.messageId);

  // Reply to a thread
  const reply = await client.messages.send(inboxId, {
    to: ['user@example.com'],
    subject: 'Re: Hello from OpenAgentMail',
    text: 'This is a reply to the previous email.',
    threadId: message1.threadId,
  });
  console.log('Sent reply:', reply.messageId);

  // List messages
  console.log('\nRecent messages:');
  const messages = await client.messages.list(inboxId).take(5);
  for (const msg of messages) {
    console.log(`  - ${msg.subject} (from: ${msg.from})`);
  }
}

main().catch(console.error);
