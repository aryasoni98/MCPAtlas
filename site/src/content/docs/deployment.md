# Deployment

## Docker

```bash
docker run -p 3000:3000 ghcr.io/mcp-atlas/server:latest
```

Use a volume for cache persistence:

```bash
docker run -p 3000:3000 -v mcp-atlas-cache:/app/cache ghcr.io/mcp-atlas/server:latest
```

## Docker Compose

See `deploy/docker/` in the repo for a Compose file with health checks and cache volume.

## Kubernetes / Helm

```bash
helm install mcp-atlas deploy/helm/mcp-atlas
```

Chart includes Deployment, Service, optional Ingress, HPA, and PDB. See `deploy/helm/README.md` for values and security context.

## Health

- **HTTP:** `GET /health` returns 200 and project count when ready.
- **Readiness:** Server is ready when landscape and search index are loaded.
