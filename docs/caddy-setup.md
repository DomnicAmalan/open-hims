# Web Server Setup with Caddy

This project uses Caddy as a reverse proxy and web server for both development and production environments.

## Quick Start

1. **Setup local hosts** (required for development):
   ```bash
   ./scripts/setup-hosts.sh
   ```

2. **Start development environment**:
   ```bash
   # Terminal 1: Start all development servers
   pnpm dev
   
   # Terminal 2: Start Caddy reverse proxy
   caddy run --config Caddyfile.dev
   ```

3. **Access the application**:
   - Web App: https://dev.openhims.health
   - API: https://api-dev.openhims.health

## Available Commands

### System Setup
- `./scripts/setup-hosts.sh` - Configure /etc/hosts for local development domains

### Caddy Commands (run directly)
#### Development
- `caddy run --config Caddyfile.dev` - Start Caddy with development configuration
- `caddy reload --config Caddyfile.dev` - Reload development configuration
- `caddy stop` - Stop Caddy server

#### Production
- `caddy run --config Caddyfile.prod` - Start Caddy with production configuration
- `caddy reload --config Caddyfile.prod` - Reload production configuration

## Configuration Files

### Caddyfile.dev
- **Domain**: dev.openhims.health
- **Features**: 
  - Proxies to Vite dev server (localhost:5173)
  - WebSocket support for Hot Module Replacement
  - CORS enabled for development
  - Separate API domain: api-dev.openhims.health

### Caddyfile.prod
- **Domain**: prod.openhims.health  
- **Features**:
  - Serves static files from `/var/www/openhims/web`
  - SPA routing support
  - Security headers (HSTS, CSP, etc.)
  - Gzip compression
  - Separate API domain: api.openhims.health
  - Rate limiting on API

## SSL Certificates

- **Development**: Uses self-signed certificates (`tls internal`) for local domains
- **Production**: Automatically obtains Let's Encrypt certificates for public domains

**Important**: You will need to accept the self-signed certificate warnings in your browser for local development domains (dev.openhims.health, api-dev.openhims.health).

## Logs

- Development: `/var/log/caddy/dev.openhims.health.log`
- Production: `/var/log/caddy/prod.openhims.health.log`
- API logs: Separate log files for API domains

## Troubleshooting

### Port Conflicts
If you get port binding errors:
```bash
# Check what's using port 80/443
sudo lsof -i :80
sudo lsof -i :443

# Stop Caddy if running
caddy stop
```

### Hosts File Issues
If domains don't resolve:
```bash
# Verify hosts file entries
cat /etc/hosts | grep openhims

# Re-run hosts setup
./scripts/setup-hosts.sh
```

### Certificate Issues
For development SSL certificate warnings:
1. Accept the certificate in your browser
2. Or add Caddy's root CA to your system trust store

## Backend Requirements

Make sure your Rust backend API is running on `localhost:8000` for both development and production configurations.