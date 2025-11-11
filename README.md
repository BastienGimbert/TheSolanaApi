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

# Available Nodes: November 11, 2025

> **Be careful:** Some nodes may be behind the current Solana network state. Use [getLatestBlockhash](#4-get-latest-blockhash-random-routing) to verify the "node's freshness".
> Check the `slot` field in responses to ensure you're interacting with an up-to-date validator.

## 🌏 Asia
### 🇭🇰 Hong Kong
- `quarry-bay-1`

### 🇮🇱 Israel
- `tel-aviv-1`

### 🇯🇵 Japan
- `tokyo-1`

### 🇸🇬 Singapore
- `singapore-1`
- `singapore-2`
- `singapore-3`
- `singapore-4`


## 🌍 Europe
### 🇫🇮 Finland
- `espoo-1`
- `helsinki-1`
- `helsinki-2`

### 🇫🇷 France
- `lauterbourg-1`
- `strasbourg-1`

### 🇩🇪 Germany
- `fechenheim-1`
- `fechenheim-2`
- `fechenheim-3`
- `fechenheim-4`
- `fechenheim-5`
- `frankfurt-1`
- `frankfurt-2`
- `frankfurt-3`
- `frankfurt-4`
- `frankfurt-am-main-1`
- `frankfurt-am-main-2`
- `frankfurt-am-main-3`
- `frankfurt-am-main-4`
- `frankfurt-am-main-5`
- `frankfurt-am-main-6`
- `frankfurt-am-main-7`
- `hattersheim-1`
- `nuremberg-1`

### 🇱🇹 Lithuania
- `siauliai-1`

### 🇳🇱 Netherlands
- `amsterdam-1`
- `amsterdam-2`
- `amsterdam-3`
- `amsterdam-4`
- `amsterdam-5`

### 🇵🇱 Poland
- `warsaw-1`
- `warsaw-2`
- `warsaw-3`

### 🇪🇸 Spain
- `madrid-1`

### 🇸🇪 Sweden
- `stockholm-1`
- `stockholm-2`

### 🇳🇱 The Netherlands
- `amsterdam-6`
- `amsterdam-7`
- `amsterdam-8`
- `amsterdam-9`
- `amsterdam-10`


## 🌎 North America
### 🇺🇸 United States
- `atlanta-1`
- `bluffdale-1`
- `chicago-1`
- `dallas-1`
- `elk-grove-village-1`
- `los-angeles-1`
- `louisville-1`
- `new-york-1`
- `new-york-2`
- `newark-1`
- `newark-2`
- `newark-3`
- `newark-4`
- `phoenix-1`
- `pittsburgh-1`
- `secaucus-1`
- `st-louis-1`




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


