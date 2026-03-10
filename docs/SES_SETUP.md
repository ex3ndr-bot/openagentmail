# Amazon SES Setup Guide for OpenAgentMail

This guide walks you through setting up Amazon Simple Email Service (SES) for use with OpenAgentMail. SES provides the email infrastructure for both sending and receiving emails.

---

## Table of Contents

1. [Prerequisites](#prerequisites)
2. [Getting Out of SES Sandbox](#getting-out-of-ses-sandbox)
3. [Domain Verification](#domain-verification)
4. [Inbound Email Setup](#inbound-email-setup)
5. [Outbound Email Setup](#outbound-email-setup)
6. [IAM Permissions](#iam-permissions)
7. [Cost Breakdown](#cost-breakdown)
8. [AWS CLI Commands Reference](#aws-cli-commands-reference)
9. [Troubleshooting](#troubleshooting)

---

## Prerequisites

Before starting, ensure you have:

- [ ] An **AWS account** with billing enabled
- [ ] A **domain you own** (e.g., `yourdomain.com`)
- [ ] Access to your domain's **DNS settings** (Route 53, Cloudflare, etc.)
- [ ] **AWS CLI** installed and configured with appropriate credentials
- [ ] Basic familiarity with AWS Console

### Choosing a Region

SES is available in several regions. Choose based on latency and compliance requirements:

| Region | Code | Inbound MX |
|--------|------|-----------|
| US East (N. Virginia) | us-east-1 | `inbound-smtp.us-east-1.amazonaws.com` |
| US West (Oregon) | us-west-2 | `inbound-smtp.us-west-2.amazonaws.com` |
| EU (Ireland) | eu-west-1 | `inbound-smtp.eu-west-1.amazonaws.com` |
| EU (Frankfurt) | eu-central-1 | `inbound-smtp.eu-central-1.amazonaws.com` |
| Asia Pacific (Mumbai) | ap-south-1 | `inbound-smtp.ap-south-1.amazonaws.com` |
| Asia Pacific (Sydney) | ap-southeast-2 | `inbound-smtp.ap-southeast-2.amazonaws.com` |

**Recommendation**: Use `us-east-1` for most deployments—it has the most features and best documentation.

---

## Getting Out of SES Sandbox

By default, new SES accounts are in **sandbox mode** with severe restrictions:

| Restriction | Sandbox | Production |
|-------------|---------|------------|
| Daily send limit | 200 emails | 50,000+ |
| Send rate | 1 email/second | 14+/second |
| Recipients | Verified addresses only | Anyone |
| Inbound email | Works | Works |

### Request Production Access

1. Go to **AWS Console → SES → Account dashboard**
2. Click **"Request production access"**
3. Fill out the form:
   - **Mail type**: Transactional (for agent emails)
   - **Website URL**: Your application URL
   - **Use case description**: Be specific and professional

**Example description:**
```
We are building OpenAgentMail, an email API for AI agents. Our platform
allows AI applications to create inboxes and send/receive transactional
emails. All emails are initiated by end users through our API. We will
implement bounce and complaint handling via SNS notifications, maintain
a suppression list, and follow AWS email best practices. Expected volume:
10,000 emails/day initially, scaling to 100,000/day within 6 months.
```

4. Submit and wait for approval (typically 24-48 hours)

### While in Sandbox

You can still test everything—just verify each email address you want to send to:

```bash
aws ses verify-email-identity --email-address test@example.com --region us-east-1
```

---

## Domain Verification

Domain verification proves you own the domain and enables DKIM signing.

### Step 1: Add Domain to SES

```bash
aws ses verify-domain-identity --domain yourdomain.com --region us-east-1
```

Response includes a verification token:
```json
{
    "VerificationToken": "ABCDEF1234567890..."
}
```

### Step 2: Add DNS Records

#### TXT Record (Domain Verification)

```
Name:  _amazonses.yourdomain.com
Type:  TXT
Value: ABCDEF1234567890...  (the token from Step 1)
```

#### DKIM Records

Enable DKIM (required for good deliverability):

```bash
aws ses verify-domain-dkim --domain yourdomain.com --region us-east-1
```

Response:
```json
{
    "DkimTokens": [
        "abcdef1234567890",
        "ghijkl0987654321",
        "mnopqr5678901234"
    ]
}
```

Add three CNAME records:

```
Name:  abcdef1234567890._domainkey.yourdomain.com
Type:  CNAME
Value: abcdef1234567890.dkim.amazonses.com

Name:  ghijkl0987654321._domainkey.yourdomain.com
Type:  CNAME
Value: ghijkl0987654321.dkim.amazonses.com

Name:  mnopqr5678901234._domainkey.yourdomain.com
Type:  CNAME
Value: mnopqr5678901234.dkim.amazonses.com
```

#### SPF Record

Authorize SES to send on behalf of your domain:

```
Name:  yourdomain.com (or @)
Type:  TXT
Value: "v=spf1 include:amazonses.com -all"
```

**Note**: If you already have an SPF record, add `include:amazonses.com` to it:
```
"v=spf1 include:_spf.google.com include:amazonses.com -all"
```

#### DMARC Record (Recommended)

```
Name:  _dmarc.yourdomain.com
Type:  TXT
Value: "v=DMARC1; p=quarantine; rua=mailto:dmarc-reports@yourdomain.com"
```

DMARC policies:
- `p=none` — Monitor only (start here)
- `p=quarantine` — Send failing emails to spam
- `p=reject` — Reject failing emails entirely

### Step 3: Verify DNS Propagation

Check verification status:

```bash
aws ses get-identity-verification-attributes \
    --identities yourdomain.com \
    --region us-east-1
```

Expected output when verified:
```json
{
    "VerificationAttributes": {
        "yourdomain.com": {
            "VerificationStatus": "Success"
        }
    }
}
```

Check DKIM status:

```bash
aws ses get-identity-dkim-attributes \
    --identities yourdomain.com \
    --region us-east-1
```

---

## Inbound Email Setup

Inbound email flows: **External sender → SES MX → Receipt Rule → SNS → Your Backend**

### Step 1: Set MX Records

Point your domain's MX record to SES:

```
Name:  yourdomain.com (or @)
Type:  MX
Priority: 10
Value: inbound-smtp.us-east-1.amazonaws.com
```

**Important**: Remove any existing MX records, or SES may not receive emails.

### Step 2: Create SNS Topic

Create a topic to receive email notifications:

```bash
aws sns create-topic --name ses-inbound-emails --region us-east-1
```

Response:
```json
{
    "TopicArn": "arn:aws:sns:us-east-1:123456789012:ses-inbound-emails"
}
```

### Step 3: Subscribe Your Endpoint

Subscribe your backend webhook to the SNS topic:

```bash
aws sns subscribe \
    --topic-arn arn:aws:sns:us-east-1:123456789012:ses-inbound-emails \
    --protocol https \
    --notification-endpoint https://api.yourdomain.com/webhooks/ses-inbound \
    --region us-east-1
```

**SNS will send a confirmation request** to your endpoint. Your endpoint must:
1. Accept POST requests with `Content-Type: text/plain` (SNS sends JSON as text)
2. On `SubscriptionConfirmation` type, visit the `SubscribeURL` to confirm
3. Return HTTP 200

Example confirmation handler (Node.js):
```javascript
app.post('/webhooks/ses-inbound', async (req, res) => {
  const message = JSON.parse(req.body);
  
  if (message.Type === 'SubscriptionConfirmation') {
    // Confirm subscription
    await fetch(message.SubscribeURL);
    return res.sendStatus(200);
  }
  
  if (message.Type === 'Notification') {
    const sesNotification = JSON.parse(message.Message);
    // Process email...
  }
  
  res.sendStatus(200);
});
```

### Step 4: Create Receipt Rule Set

```bash
# Create a rule set
aws ses create-receipt-rule-set \
    --rule-set-name openagentmail-rules \
    --region us-east-1

# Set as active
aws ses set-active-receipt-rule-set \
    --rule-set-name openagentmail-rules \
    --region us-east-1
```

### Step 5: Create Receipt Rule

```bash
aws ses create-receipt-rule \
    --rule-set-name openagentmail-rules \
    --rule '{
        "Name": "forward-to-sns",
        "Enabled": true,
        "ScanEnabled": true,
        "Recipients": ["yourdomain.com"],
        "Actions": [
            {
                "SNSAction": {
                    "TopicArn": "arn:aws:sns:us-east-1:123456789012:ses-inbound-emails",
                    "Encoding": "UTF-8"
                }
            }
        ]
    }' \
    --region us-east-1
```

**Rule options:**
- `Recipients`: Domain or specific addresses (e.g., `["support@yourdomain.com"]`)
- `ScanEnabled`: Enable spam/virus scanning
- `Encoding`: `UTF-8` or `Base64`

### Optional: Store Raw Email in S3

For large emails or attachment handling, store in S3 first:

```bash
# Create S3 bucket
aws s3 mb s3://openagentmail-raw-emails --region us-east-1

# Add bucket policy for SES
aws s3api put-bucket-policy --bucket openagentmail-raw-emails --policy '{
    "Version": "2012-10-17",
    "Statement": [
        {
            "Sid": "AllowSES",
            "Effect": "Allow",
            "Principal": {"Service": "ses.amazonaws.com"},
            "Action": "s3:PutObject",
            "Resource": "arn:aws:s3:::openagentmail-raw-emails/*",
            "Condition": {
                "StringEquals": {"AWS:SourceAccount": "123456789012"}
            }
        }
    ]
}'

# Update receipt rule to save to S3 then notify SNS
aws ses create-receipt-rule \
    --rule-set-name openagentmail-rules \
    --rule '{
        "Name": "store-and-notify",
        "Enabled": true,
        "ScanEnabled": true,
        "Recipients": ["yourdomain.com"],
        "Actions": [
            {
                "S3Action": {
                    "BucketName": "openagentmail-raw-emails",
                    "ObjectKeyPrefix": "emails/"
                }
            },
            {
                "SNSAction": {
                    "TopicArn": "arn:aws:sns:us-east-1:123456789012:ses-inbound-emails",
                    "Encoding": "UTF-8"
                }
            }
        ]
    }' \
    --region us-east-1
```

### Optional: Lambda Processing

For complex processing, invoke Lambda directly:

```bash
aws ses create-receipt-rule \
    --rule-set-name openagentmail-rules \
    --rule '{
        "Name": "lambda-processor",
        "Enabled": true,
        "ScanEnabled": true,
        "Recipients": ["yourdomain.com"],
        "Actions": [
            {
                "S3Action": {
                    "BucketName": "openagentmail-raw-emails",
                    "ObjectKeyPrefix": "emails/"
                }
            },
            {
                "LambdaAction": {
                    "FunctionArn": "arn:aws:lambda:us-east-1:123456789012:function:process-inbound-email",
                    "InvocationType": "Event"
                }
            }
        ]
    }' \
    --region us-east-1
```

---

## Outbound Email Setup

### Step 1: Generate SMTP Credentials

SES uses IAM credentials converted to SMTP format:

```bash
aws ses create-smtp-password \
    --iam-user-name ses-smtp-user \
    --region us-east-1
```

**Important**: The output shows the SMTP password ONCE. Save it securely.

Or create via IAM user in Console:
1. Create IAM user with `ses:SendRawEmail` permission
2. Go to **SES → SMTP Settings** → **Create SMTP credentials**
3. Download credentials CSV

### SMTP Configuration

| Setting | Value |
|---------|-------|
| Server | `email-smtp.us-east-1.amazonaws.com` |
| Port | `587` (STARTTLS) or `465` (TLS Wrapper) |
| Username | IAM Access Key ID |
| Password | Generated SMTP password |
| Authentication | PLAIN or LOGIN |

### Step 2: Send via SMTP (Node.js Example)

```javascript
const nodemailer = require('nodemailer');

const transporter = nodemailer.createTransport({
  host: 'email-smtp.us-east-1.amazonaws.com',
  port: 587,
  secure: false,
  auth: {
    user: process.env.SES_SMTP_USER,
    pass: process.env.SES_SMTP_PASS,
  },
});

await transporter.sendMail({
  from: 'sender@yourdomain.com',
  to: 'recipient@example.com',
  subject: 'Hello from OpenAgentMail',
  text: 'This is a test email.',
});
```

### Step 3: Send via AWS SDK (Alternative)

```javascript
const { SESClient, SendEmailCommand } = require('@aws-sdk/client-ses');

const ses = new SESClient({ region: 'us-east-1' });

await ses.send(new SendEmailCommand({
  Source: 'sender@yourdomain.com',
  Destination: {
    ToAddresses: ['recipient@example.com'],
  },
  Message: {
    Subject: { Data: 'Hello from OpenAgentMail' },
    Body: {
      Text: { Data: 'This is a test email.' },
    },
  },
}));
```

### Step 4: Handle Bounces and Complaints

Set up SNS notifications for delivery feedback:

```bash
# Create topics for each notification type
aws sns create-topic --name ses-bounces --region us-east-1
aws sns create-topic --name ses-complaints --region us-east-1
aws sns create-topic --name ses-deliveries --region us-east-1

# Configure SES to use these topics
aws ses set-identity-notification-topic \
    --identity yourdomain.com \
    --notification-type Bounce \
    --sns-topic arn:aws:sns:us-east-1:123456789012:ses-bounces \
    --region us-east-1

aws ses set-identity-notification-topic \
    --identity yourdomain.com \
    --notification-type Complaint \
    --sns-topic arn:aws:sns:us-east-1:123456789012:ses-complaints \
    --region us-east-1

aws ses set-identity-notification-topic \
    --identity yourdomain.com \
    --notification-type Delivery \
    --sns-topic arn:aws:sns:us-east-1:123456789012:ses-deliveries \
    --region us-east-1
```

**Critical**: Handle bounces and complaints to maintain sender reputation:
- **Hard bounces**: Remove email from your list immediately
- **Complaints**: Unsubscribe user immediately
- **Soft bounces**: Retry with exponential backoff, then remove after N failures

### Dedicated IPs (Optional)

For high-volume sending, consider dedicated IPs:

```bash
# Request dedicated IPs (costs ~$25/month per IP)
# Done via AWS Console: SES → Dedicated IPs → Request

# Assign to configuration set
aws ses put-configuration-set-dedicated-ip-pool \
    --configuration-set-name production \
    --dedicated-ip-pool default \
    --region us-east-1
```

Benefits of dedicated IPs:
- Full control over sender reputation
- IP warming (gradual volume increase)
- Better for high volume (>100K/day)

---

## IAM Permissions

### Minimum Required Policy

```json
{
    "Version": "2012-10-17",
    "Statement": [
        {
            "Sid": "SESSend",
            "Effect": "Allow",
            "Action": [
                "ses:SendEmail",
                "ses:SendRawEmail"
            ],
            "Resource": "*"
        },
        {
            "Sid": "SESInbound",
            "Effect": "Allow",
            "Action": [
                "ses:CreateReceiptRule",
                "ses:CreateReceiptRuleSet",
                "ses:DescribeReceiptRule",
                "ses:DescribeReceiptRuleSet",
                "ses:UpdateReceiptRule"
            ],
            "Resource": "*"
        },
        {
            "Sid": "SNSPublish",
            "Effect": "Allow",
            "Action": [
                "sns:Publish",
                "sns:Subscribe",
                "sns:CreateTopic"
            ],
            "Resource": "arn:aws:sns:us-east-1:123456789012:ses-*"
        },
        {
            "Sid": "S3EmailStorage",
            "Effect": "Allow",
            "Action": [
                "s3:GetObject",
                "s3:PutObject"
            ],
            "Resource": "arn:aws:s3:::openagentmail-raw-emails/*"
        }
    ]
}
```

### SES Service Role for S3/Lambda

SES needs permission to write to S3 and invoke Lambda:

```json
{
    "Version": "2012-10-17",
    "Statement": [
        {
            "Sid": "AllowSESToS3",
            "Effect": "Allow",
            "Principal": {"Service": "ses.amazonaws.com"},
            "Action": "s3:PutObject",
            "Resource": "arn:aws:s3:::openagentmail-raw-emails/*",
            "Condition": {
                "StringEquals": {"AWS:SourceAccount": "123456789012"}
            }
        }
    ]
}
```

For Lambda invocation, add resource-based policy to your function:

```bash
aws lambda add-permission \
    --function-name process-inbound-email \
    --statement-id AllowSES \
    --action lambda:InvokeFunction \
    --principal ses.amazonaws.com \
    --source-account 123456789012 \
    --region us-east-1
```

---

## Cost Breakdown

SES pricing is simple and cheap:

### Sending

| Volume | Price |
|--------|-------|
| First 62,000 emails/month (from EC2) | **Free** |
| Additional emails | **$0.10 per 1,000** |
| Attachments | **$0.12 per GB** |

### Receiving

| Volume | Price |
|--------|-------|
| First 1,000 emails/month | **Free** |
| Additional emails | **$0.10 per 1,000** |
| Incoming email chunks (256KB each) | **$0.09 per 1,000 chunks** |

### Related Services

| Service | Price |
|---------|-------|
| SNS notifications | First 1M free, then $0.50/million |
| S3 storage | ~$0.023/GB/month |
| Lambda invocations | First 1M free, then $0.20/million |
| Dedicated IP | ~$25/IP/month |

### Example: 100,000 Inboxes

Assuming 10 emails/day per inbox = 1M emails/day = 30M emails/month:

| Item | Monthly Cost |
|------|-------------|
| Outbound emails (30M) | $3,000 |
| Inbound emails (30M) | $3,000 |
| SNS notifications | $15 |
| S3 storage (1TB) | $23 |
| **Total** | **~$6,000** |

Compare to **Google Workspace** at $6/user: 100K users = **$600,000/month**

---

## AWS CLI Commands Reference

### Domain Management

```bash
# Verify domain
aws ses verify-domain-identity --domain yourdomain.com

# Enable DKIM
aws ses verify-domain-dkim --domain yourdomain.com

# Check verification status
aws ses get-identity-verification-attributes --identities yourdomain.com

# Check DKIM status
aws ses get-identity-dkim-attributes --identities yourdomain.com

# List verified identities
aws ses list-identities --identity-type Domain
```

### Receipt Rules

```bash
# Create rule set
aws ses create-receipt-rule-set --rule-set-name my-rules

# Set active rule set
aws ses set-active-receipt-rule-set --rule-set-name my-rules

# List rule sets
aws ses list-receipt-rule-sets

# Describe rule set
aws ses describe-receipt-rule-set --rule-set-name my-rules

# Delete rule
aws ses delete-receipt-rule --rule-set-name my-rules --rule-name my-rule

# Delete rule set (must be empty)
aws ses delete-receipt-rule-set --rule-set-name my-rules
```

### Sending

```bash
# Send test email (verified addresses only in sandbox)
aws ses send-email \
    --from sender@yourdomain.com \
    --to recipient@example.com \
    --subject "Test" \
    --text "Hello World"

# Check send quota
aws ses get-send-quota

# Check send statistics
aws ses get-send-statistics
```

### Notifications

```bash
# Set bounce notification topic
aws ses set-identity-notification-topic \
    --identity yourdomain.com \
    --notification-type Bounce \
    --sns-topic arn:aws:sns:us-east-1:123456789012:bounces

# Include headers in notifications
aws ses set-identity-headers-in-notifications-enabled \
    --identity yourdomain.com \
    --notification-type Bounce \
    --enabled
```

---

## Troubleshooting

### Email Not Being Received

1. **Check MX records**: `dig MX yourdomain.com` should show SES endpoint
2. **Check receipt rule set is active**: Only one can be active
3. **Check receipt rule recipients**: Must match the receiving address/domain
4. **Check SNS subscription confirmed**: Subscription must be in "Confirmed" state
5. **Check CloudWatch logs**: SES logs delivery attempts

### Email Going to Spam

1. **Verify DKIM is passing**: Check email headers for `dkim=pass`
2. **Verify SPF is passing**: Check for `spf=pass`
3. **Check DMARC alignment**: Both SPF and DKIM should align with From domain
4. **Check reputation**: SES dashboard shows account reputation
5. **Warm up new domain/IP**: Start with low volume, gradually increase

### SNS Not Delivering

1. **Check subscription status**: Must be "Confirmed"
2. **Check endpoint accessibility**: Must be publicly reachable over HTTPS
3. **Check endpoint response**: Must return HTTP 200 within 15 seconds
4. **Check dead-letter queue**: Failed messages go to DLQ if configured

### Common Errors

| Error | Cause | Fix |
|-------|-------|-----|
| `MessageRejected` | Sending from unverified address | Verify domain or email |
| `Throttling` | Exceeded send rate | Request quota increase |
| `InvalidParameterValue` | Malformed email | Check email format, headers |
| `AccessDenied` | IAM permissions | Check IAM policy |

---

## Next Steps

1. [Set up monitoring in CloudWatch](./MONITORING.md)
2. [Configure your backend webhook](./WEBHOOKS.md)
3. [Deploy with CDK/Terraform](./DEPLOYMENT.md)

---

*Last updated: 2026-03*
