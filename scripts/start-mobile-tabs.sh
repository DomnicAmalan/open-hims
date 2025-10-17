#!/bin/bash

# Mobile development launcher with backend and Caddy
# This script opens mobile development services in separate Terminal tabs

PROJECT_DIR="/Users/apple/open-hims-rs"

echo "📱 Starting Open HIMS Mobile development environment..."

# Function to run command in new tab
run_in_new_tab() {
    local title="$1"
    local command="$2"
    
    osascript <<EOF
tell application "Terminal"
    activate
    tell application "System Events" to keystroke "t" using command down
    delay 0.5
    do script "cd \"$PROJECT_DIR\" && echo \"$title\" && $command" in front window
end tell
EOF
}

# Start first service in current tab/window
osascript -e "tell app \"Terminal\" to do script \"cd \\\"$PROJECT_DIR\\\" && echo \\\"🦀 Starting Rust Backend...\\\" && make start-rust\""

sleep 2

# Start other services in new tabs
run_in_new_tab "📱 Starting Mobile App..." "make start-mobile"
sleep 1

run_in_new_tab "🌐 Starting Caddy Proxy..." "make start-caddy"

echo "✨ Mobile development environment started in separate tabs!"
echo "Services available at:"
echo "  - API: http://localhost:8080"
echo "  - Mobile: Expo dev tools and app"
echo "  - Proxied: https://api-dev.openhims.health (via Caddy)"
echo ""
echo "Close tabs individually to stop services"