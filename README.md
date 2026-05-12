# TheSolanaAPI - Solana Validator API Proxy — Public API

Provides a single, stable access point to a fleet of Solana validators. The API accepts standard Solana JSON-RPC requests and routes them to an available validator based on your selection criteria.

**Base URL**: http://thesolanaapi.com

## Why Not Use Official Public Endpoints?

- **Severe Rate Limiting**: Public endpoints impose strict request limits that quickly trigger 429 "Too Many Requests" errors and throttle applications under load. For example:
  - Maximum requests per 10 seconds per IP: 100
  - Maximum requests per 10 seconds per IP for a single RPC: 40
  - Maximum concurrent connections per IP: 40
  - Maximum connection rate per 10 seconds per IP: 40
  - Maximum data transfer per 30 seconds: 100 MB
- **Congestion & Latency**: During high traffic periods, responses can become slow or erratic, impacting transaction submission and state reading. 
- **Zero Customization**: Impossible to adjust optimization strategies or routing based on region or method.


## Endpoints

- `GET /health` — Simple health check. Returns `{ "status": "ok" }`.
- `GET /validators` — Discover exposed validators (aliases and locations).
- `POST /?server=<alias>` — Forward JSON-RPC request to the named validator.
- `POST /?location=<label>` — Select a validator from the requested location.
- `POST /` — If no selector is provided, a validator is chosen randomly.

**Notes:**
- JSON-RPC request bodies and responses are transmitted as-is.
- Upstream errors (unavailable validator, misconfiguration) are propagated with their status codes.

## Request Examples

**Base URL**: http://thesolanaapi.com

**Common headers for all POST requests:**
- Method: POST
- Header: Content-Type: application/json

### 1) Health Check
```bash
curl -X GET "http://thesolanaapi.com/health"
```

**Expected response:**
```json
{
  "status": "ok"
}
```

### 2) List Validators
```bash
curl -X GET "http://thesolanaapi.com/validators"
```

**Response example:**
```json
{
  "validators": [
    { "name": "frankfurt-1", "location": "Frankfurt", "protocol": "http" },
    { "name": "paris-1", "location": "Paris", "protocol": "http" },
    { "name": "tokyo-1", "location": "Tokyo", "protocol": "http" }
  ]
}
```


### 3) Get Balance by Location
```bash
curl -X POST "http://thesolanaapi.com/?location=Paris" \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "getBalance",
    "params": ["<public-key>"]
  }'
```

### 4) Get Latest Blockhash (Random Routing)
```bash
curl -X POST "http://thesolanaapi.com/" \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "getLatestBlockhash",
    "params": []
  }'
```

### 5) Send Transaction to Specific Server
```bash
curl -X POST "http://thesolanaapi.com/?server=paris-1" \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "sendTransaction",
    "params": ["<base64-encoded-transaction>"]
  }'
```

### 6) Get Account Transaction History
```bash
curl -X POST "http://thesolanaapi.com/?server=frankfurt-1" \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "getSignaturesForAddress",
    "params": [
      "9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM",
      { "limit": 1000 }
    ]
  }'
```

### 7) Get Block with Commitment Level
```bash
curl -X POST "http://thesolanaapi.com/?location=Frankfurt" \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "getBlock",
    "params": [
      "latest",
      {
        "commitment": "finalized",
        "maxSupportedTransactionVersion": 0
      }
    ]
  }'
```

### 8) Simulate Transaction
```bash
curl -X POST "http://thesolanaapi.com/" \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "simulateTransaction",
    "params": [
      "<base64-encoded-transaction>",
      {
        "sigVerify": false,
        "replaceRecentBlockhash": true,
        "commitment": "processed"
      }
    ]
  }'
```

For more methods and parameters, refer to the [Solana JSON-RPC API documentation](http://docs.solana.com/developing/clients/jsonrpc-api).

# Available Nodes: May 12, 2026

> **Be careful:** Some nodes may be behind the current Solana network state. Use [getLatestBlockhash](#4-get-latest-blockhash-random-routing) to verify the "node's freshness".
> Check the `slot` field in responses to ensure you're interacting with an up-to-date validator.

## 🌏 Asia
### 🇯🇵 Japan
- `shibuya-city-1`
- `shinagawa-1`
- `shinagawa-2`
- `shinagawa-3`
- `shinagawa-4`
- `shinagawa-5`
- `tokyo-1`
- `tokyo-2`
- `tokyo-3`
- `tokyo-4`

### 🇸🇬 Singapore
- `singapore-1`
- `singapore-2`
- `singapore-3`
- `singapore-4`
- `singapore-5`
- `singapore-6`
- `singapore-7`
- `singapore-8`

### 🇹🇭 Thailand
- `bangkok-1`


## 🌍 Europe
### 🇫🇮 Finland
- `helsinki-1`
- `helsinki-2`

### 🇫🇷 France
- `strasbourg-1`

### 🇩🇪 Germany
- `falkenstein-1`
- `falkenstein-2`
- `falkenstein-3`
- `falkenstein-4`
- `fechenheim-1`
- `fechenheim-2`
- `fechenheim-3`
- `fechenheim-4`
- `fechenheim-5`
- `fechenheim-6`
- `fechenheim-7`
- `fechenheim-8`
- `frankfurt-1`
- `frankfurt-2`
- `frankfurt-3`
- `frankfurt-4`
- `frankfurt-5`
- `frankfurt-6`
- `frankfurt-7`
- `frankfurt-8`
- `frankfurt-9`
- `frankfurt-10`
- `frankfurt-oder-1`
- `frankfurt-oder-2`
- `frankfurt-oder-3`
- `frankfurt-oder-4`
- `frankfurt-oder-5`
- `frankfurt-am-main-1`
- `frankfurt-am-main-2`
- `frankfurt-am-main-3`
- `frankfurt-am-main-4`
- `frankfurt-am-main-5`
- `frankfurt-am-main-6`
- `frankfurt-am-main-7`
- `frankfurt-am-main-8`
- `frankfurt-am-main-9`
- `frankfurt-am-main-10`
- `frankfurt-am-main-11`
- `frankfurt-am-main-12`
- `frankfurt-am-main-13`
- `frankfurt-am-main-14`
- `frankfurt-am-main-15`
- `frankfurt-am-main-16`
- `frankfurt-am-main-17`
- `frankfurt-am-main-18`
- `frankfurt-am-main-19`
- `frankfurt-am-main-20`
- `frankfurt-am-main-21`
- `frankfurt-am-main-22`
- `frankfurt-am-main-23`
- `frankfurt-am-main-24`
- `frankfurt-am-main-25`
- `frankfurt-am-main-26`
- `gelnhausen-1`
- `rodelheim-1`

### 🇱🇻 Latvia
- `riga-1`

### 🇱🇹 Lithuania
- `siauliai-1`
- `siauliai-2`

### 🇳🇱 Netherlands
- `amsterdam-1`
- `amsterdam-2`
- `amsterdam-3`
- `amsterdam-4`
- `amsterdam-5`
- `amsterdam-6`
- `amsterdam-7`
- `amsterdam-8`
- `amsterdam-9`
- `amsterdam-10`
- `amsterdam-11`
- `amsterdam-12`
- `amsterdam-13`
- `amsterdam-14`
- `amsterdam-15`
- `amsterdam-16`
- `amsterdam-17`
- `amsterdam-18`
- `amsterdam-19`

### 🇳🇴 Norway
- `oslo-1`

### 🇵🇱 Poland
- `warsaw-1`
- `warsaw-2`

### 🇨🇭 Switzerland
- `baden-1`
- `carouge-1`

### 🇳🇱 The Netherlands
- `amsterdam-20`
- `amsterdam-21`
- `amsterdam-22`
- `amsterdam-23`
- `diemen-1`

### 🇬🇧 United Kingdom
- `city-of-london-1`
- `city-of-london-2`
- `city-of-london-3`
- `city-of-london-4`
- `city-of-london-5`
- `city-of-london-6`
- `london-1`


## 🌎 North America
### 🇨🇦 Canada
- `montreal-1`
- `toronto-1`
- `toronto-2`

### 🇺🇸 United States
- `ashburn-1`
- `ashburn-2`
- `ashburn-3`
- `ashburn-4`
- `ashburn-5`
- `ashburn-6`
- `ashburn-7`
- `ashburn-8`
- `ashburn-9`
- `ashburn-10`
- `ashburn-11`
- `chicago-1`
- `draper-1`
- `dublin-1`
- `miami-1`
- `new-york-1`
- `new-york-2`
- `new-york-3`
- `new-york-4`
- `new-york-5`
- `new-york-6`
- `new-york-7`
- `new-york-8`
- `new-york-9`
- `new-york-10`
- `new-york-11`
- `new-york-12`
- `new-york-13`
- `newark-1`
- `newark-2`
- `newark-3`
- `newark-4`
- `newark-5`
- `ogden-1`
- `piscataway-1`
- `quincy-1`
- `reston-1`
- `san-jose-1`
- `sterling-1`
- `sterling-2`
- `sterling-3`
- `sterling-4`
- `wilmington-1`





---

## Roadmap 

- [ ] **Implement CDN Integration**: Deploy the API behind a Content Delivery Network (CDN) to improve response times and reduce latency for users across different regions. This will ensure faster routing and caching of static responses where applicable.
- [ ] **Expand Geographical Coverage**: Add more nodes in different regions to provide better access and lower latency for users worldwide.
- [ ] **Implement WSS Support**: Add WebSocket support for real-time updates and subscriptions, enhancing the API's capabilities for applications that require live data.
- [ ] **Implement YellowStone GRPC Support**: Integrate YellowStone GRPC to provide an alternative communication protocol.
- [ ] **Implement ShredStream Support**: Add support for ShredStream to enhance data streaming capabilities.


---

## License

Shield: [![CC BY-NC-SA 4.0][cc-by-nc-sa-shield]][cc-by-nc-sa]

This work is licensed under a
[Creative Commons Attribution-NonCommercial-ShareAlike 4.0 International License][cc-by-nc-sa].

[![CC BY-NC-SA 4.0][cc-by-nc-sa-image]][cc-by-nc-sa]

[cc-by-nc-sa]: http://creativecommons.org/licenses/by-nc-sa/4.0/
[cc-by-nc-sa-image]: http://licensebuttons.net/l/by-nc-sa/4.0/88x31.png
[cc-by-nc-sa-shield]: http://img.shields.io/badge/License-CC%20BY--NC--SA%204.0-lightgrey.svg


