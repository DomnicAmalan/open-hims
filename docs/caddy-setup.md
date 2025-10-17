# Development Environment Setup

This project uses Make commands to orchestrate the complete development environment including Rust backend, React web app, Tauri desktop app, React Native mobile app, and Caddy reverse proxy.

## Quick Start

**One command to start everything:**
```bash
make dev
```

This will:
- Check and setup /etc/hosts if needed
- Start Rust backend server
- Start React web app (Vite)
- Start Tauri desktop app
- Start React Native mobile app (Expo)
- Start Caddy reverse proxy

**Access the applications:**
- Web App: https://dev.openhims.health
- API: https://api-dev.openhims.health
- Desktop: Native application window
- Mobile: Expo development tools

## Available Make Commands

### Development (Recommended)
- `make dev` or `make dev-all` - Start ALL development services at once
- `make setup` - Setup development environment (dependencies + hosts)
- `make setup-hosts` - Configure /etc/hosts only

### Individual Services
- `make start-rust` or `make rust` - Start Rust backend only
- `make start-web` or `make web` - Start React web app only  
- `make start-desktop` or `make desktop` - Start Tauri desktop app only
- `make start-mobile` or `make mobile` - Start React Native mobile app only
- `make start-caddy` or `make caddy` - Start Caddy reverse proxy only

### Cleanup
- `make stop` - Stop all development services
- `make clean` - Clean build artifacts

### Help
- `make help` - Show all available commands

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
# Check what's using ports
sudo lsof -i :80
sudo lsof -i :443
sudo lsof -i :5173
sudo lsof -i :8000
sudo lsof -i :19000

# Stop all development services
make stop
```

### Hosts File Issues
If domains don't resolve:
```bash
# Verify hosts file entries
cat /etc/hosts | grep openhims

# Re-run hosts setup
make setup-hosts
```

### Certificate Issues
For development SSL certificate warnings:
1. Accept the certificate in your browser
2. Or add Caddy's root CA to your system trust store

## Backend Requirements

Make sure your Rust backend API is running on `localhost:8000` for both development and production configurations.