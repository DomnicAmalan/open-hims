#!/bin/bash
# Setup script for Open HIMS local development with Caddy

echo "🔧 Setting up Open HIMS local development environment..."

# Check if Caddy is installed
if ! command -v caddy &> /dev/null; then
    echo "❌ Caddy is not installed. Please install it first:"
    echo "   macOS: brew install caddy"
    echo "   Linux: https://caddyserver.com/docs/install"
    exit 1
fi

# Add local domains to /etc/hosts if they don't exist
HOSTS_FILE="/etc/hosts"
DOMAINS=("local.openhims.health" "api.local.openhims.health")

echo "🔍 Checking /etc/hosts file..."

for domain in "${DOMAINS[@]}"; do
    if ! grep -q "$domain" "$HOSTS_FILE"; then
        echo "➕ Adding $domain to /etc/hosts..."
        echo "127.0.0.1 $domain" | sudo tee -a "$HOSTS_FILE" > /dev/null
    else
        echo "✅ $domain already exists in /etc/hosts"
    fi
done

# Create log directory for Caddy
echo "📁 Creating log directories..."
mkdir -p /tmp/caddy-logs
sudo mkdir -p /var/log/caddy 2>/dev/null || true

echo ""
echo "🎉 Setup complete!"
echo ""
echo "📋 Available commands:"
echo "   pnpm caddy:local    - Start local HTTPS server"
echo "   pnpm dev:web        - Start web development server"
echo "   pnpm dev:mobile     - Start mobile development server"  
echo "   pnpm dev:desktop    - Start desktop development server"
echo ""
echo "🌐 Local URLs:"
echo "   Web App: https://local.openhims.health"
echo "   API:     https://api.local.openhims.health"
echo ""
echo "⚠️  Note: You may see certificate warnings in browser for local development."
echo "   This is normal for self-signed certificates. Click 'Advanced' -> 'Proceed'"
echo ""