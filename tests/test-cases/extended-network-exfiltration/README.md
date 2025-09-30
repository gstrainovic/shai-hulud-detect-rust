# Extended Network Exfiltration Test

This test case contains comprehensive network exfiltration patterns including 15+ suspicious domains and private IP detection.

## Test Purpose
- Tests detection of 15+ suspicious domains (pastebin, hastebin, discord webhooks, etc.)
- Tests hardcoded private IP detection (10.0., 192.168., 172.16-31.)
- Tests C2 IP:Port patterns for Command & Control detection
- Tests PARANOID MODE network exfiltration detection

## Exfiltration Patterns
- **Pastebin family**: pastebin.com, hastebin.com, ix.io
- **Discord/Telegram**: discord.com/api/webhooks, telegram.org
- **Tunnel services**: ngrok.io, localtunnel.me
- **File sharing**: transfer.sh, file.io, 0x0.st
- **Webhook services**: requestbin.com, webhook.site, beeceptor.com
- **Private IPs**: 10.0.1.100:8080, 192.168.2.50:9999, 172.20.0.10:4444

## Expected Detection
- Should be classified as HIGH RISK
- Should trigger comprehensive network exfiltration warnings