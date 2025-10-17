#!/bin/bash

# Setup script for local development hosts
# This script adds the necessary entries to /etc/hosts for local development

set -e

echo "🚀 Setting up local development hosts for Open HIMS..."

# Define the hosts to add
HOSTS_ENTRIES=(
    "127.0.0.1 dev.openhims.health"
    "127.0.0.1 api-dev.openhims.health"
    "127.0.0.1 prod.openhims.health"
    "127.0.0.1 api.openhims.health"
)

# Check if running as root
if [[ $EUID -eq 0 ]]; then
    HOSTS_FILE="/etc/hosts"
else
    echo "⚠️  This script needs to modify /etc/hosts and requires sudo privileges."
    echo "You may be prompted for your password."
    HOSTS_FILE="/etc/hosts"
    SUDO="sudo"
fi

# Backup hosts file
echo "📄 Creating backup of hosts file..."
$SUDO cp /etc/hosts /etc/hosts.backup.$(date +%Y%m%d_%H%M%S)

# Check if entries already exist and add them if they don't
for entry in "${HOSTS_ENTRIES[@]}"; do
    if grep -q "${entry#* }" "$HOSTS_FILE"; then
        echo "✅ Entry already exists: $entry"
    else
        echo "➕ Adding entry: $entry"
        echo "$entry" | $SUDO tee -a "$HOSTS_FILE" > /dev/null
    fi
done

echo ""
echo "✨ Setup complete! Your hosts file now includes:"
echo "   • dev.openhims.health (development web app)"
echo "   • api-dev.openhims.health (development API)"
echo "   • prod.openhims.health (production web app)"
echo "   • api.openhims.health (production API)"
echo ""
echo "🔧 To start development:"
echo "   1. Run: pnpm dev"
echo "   2. Run Caddy: pnpm caddy:dev"
echo "   3. Visit: https://dev.openhims.health"
echo ""
echo "📝 To remove these entries later, edit /etc/hosts or restore from backup:"
echo "   sudo cp /etc/hosts.backup.* /etc/hosts"