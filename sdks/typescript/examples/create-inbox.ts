// Example: Creating an inbox

import { OpenAgentMail } from 'openagentmail';

async function main() {
  const client = new OpenAgentMail({
    apiKey: process.env.OAM_API_KEY!,
  });

  // Create a new inbox
  const inbox = await client.inboxes.create({
    username: 'support',
    domain: 'example.com',
    displayName: 'Support Team',
    // Use clientId for idempotency
    clientId: 'support-inbox-v1',
  });

  console.log('Created inbox:', inbox.email);
  console.log('Inbox ID:', inbox.inboxId);

  // List all inboxes
  console.log('\nAll inboxes:');
  for await (const i of client.inboxes.list()) {
    console.log(`  - ${i.email} (${i.inboxId})`);
  }
}

main().catch(console.error);
