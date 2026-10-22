# ServerWatch frontend

Use Node 24 LTS and npm 11. From this directory:

```sh
npm ci
npm run serve  # Local development only
npm run lint
npm test
npm run build  # Type-checks and writes ../dist
```

Vite builds the existing Vue UI, and vite-plugin-pwa generates the offline
shell worker at `service-worker.js`. The API is never cached. See the root
[README](../readme.md) for agent/frontend coordinated upgrades, authentication,
allowed origins, private-network deployment, and Cloudflare Access topology.
