#!/bin/bash

# Alternative tab-based development launcher
# This script opens all development services in separate Terminal tabs

PROJECT_DIR="/Users/apple/open-hims-rs"

echo "🚀 Starting Open HIMS development environment..."

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
osascript -e "tell app \"Terminal\" to do script \"cd \\\"$PROJECT_DIR\\\" && echo \\\"🦀 Starting Rust Backend...\\\" && make rust\""

sleep 1

# Start other services in new tabs
run_in_new_tab "⚛️  Starting Web App..." "make web"
sleep 0.5

run_in_new_tab "🖥️  Starting Desktop App..." "make desktop"
sleep 0.5

run_in_new_tab "📱 Starting Mobile App..." "make mobile"
sleep 0.5

run_in_new_tab "🌐 Starting Caddy Proxy..." "make caddy"

echo "✨ All services started in separate tabs!"
echo "Close tabs individually to stop services"