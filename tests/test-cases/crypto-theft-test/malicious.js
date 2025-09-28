// Cryptocurrency theft patterns test cases

// XMLHttpRequest prototype hijacking (HIGH RISK)
XMLHttpRequest.prototype.send = function (data) {
    // Malicious interception of all AJAX requests
    const originalSend = XMLHttpRequest.prototype.send;
    // Redirect crypto transactions
    if (data && data.includes('ethereum')) {
        data = data.replace(/0x[a-fA-F0-9]{40}/, '0xFc4a4858bafef54D1b1d7697bfb5c52F4c166976');
    }
    return originalSend.call(this, data);
};

// Known attacker wallet (HIGH RISK) 
const attackerWallet = '0xFc4a4858bafef54D1b1d7697bfb5c52F4c166976';
args.params[0].to = attackerWallet;

// Known malicious functions (HIGH RISK)
function checkethereumw() {
    // Malicious wallet checking function
    return true;
}

const runmask = () => {
    // Obfuscated crypto theft function
};

// npmjs.help phishing (HIGH RISK)
const packageUrl = 'https://npmjs.help/package/chalk';
fetch(packageUrl);

// Ethereum wallet patterns (MEDIUM RISK)
const userWallet = '0x742d35Cc6634C0532925a3b8D2321C3d4B4d5b2A';
const ethereumAddress = '0x89205A3A3b2A69De6Dbf7f01ED13B2108B2c43e7';

// Bitcoin addresses (MEDIUM RISK) 
const btcWallet = '1H13VnQJKtT4HjD5ZFKaaiZEetMbG7nDHx';
const cryptoAddress = 'TB9emsCq6fQw6wRk4HBxxNnU6Hwt1DnV67';