# Open HIMS Development Makefile
# Manages local development environment with Rust backend, web, desktop, and mobile apps

# Get current directory
PROJECT_DIR := $(shell pwd)

.PHONY: help setup dev dev-all start-rust start-web start-desktop start-mobile start-caddy stop clean check-hosts setup-hosts

# Default target
help:
	@echo "🚀 Open HIMS Development Commands"
	@echo ""
	@echo "Setup:"
	@echo "  make setup      - Setup development environment (hosts, dependencies)"
	@echo "  make setup-hosts - Configure /etc/hosts for local development"
	@echo ""
	@echo "Development:"
	@echo "  make dev        - Start ALL services in separate tabs (recommended)"
	@echo "  make dev-all    - Same as 'make dev'"
	@echo "  make dev-parallel - Start ALL services in same terminal (parallel)"
	@echo ""
	@echo "Individual Services:"
	@echo "  make start-rust    - Start Rust backend server"
	@echo "  make start-web     - Start React web app"
	@echo "  make start-desktop - Start Tauri desktop app"
	@echo "  make start-mobile  - Start React Native mobile app"
	@echo "  make start-caddy   - Start Caddy reverse proxy"
	@echo ""
	@echo "Cleanup:"
	@echo "  make stop       - Stop all development services"
	@echo "  make clean      - Clean build artifacts"

# Setup development environment
setup: check-hosts
	@echo "📦 Installing dependencies..."
	pnpm install
	@echo "🦀 Building Rust backend..."
	cargo build
	@echo "✨ Setup complete!"

# Check if hosts are configured, setup if needed
check-hosts:
	@if ! grep -q "dev.openhims.health" /etc/hosts; then \
		echo "🔧 Setting up local hosts..."; \
		./scripts/setup-hosts.sh; \
	else \
		echo "✅ Hosts already configured"; \
	fi

# Setup hosts manually
setup-hosts:
	@echo "🔧 Configuring /etc/hosts for local development..."
	./scripts/setup-hosts.sh

# Start all development services in separate terminals
dev: dev-all

dev-all: check-hosts
	@echo "🚀 Starting all development services in separate tabs..."
	@echo "This will open tabs for:"
	@echo "  - Tab 1: Rust backend (cargo run)"
	@echo "  - Tab 2: Web app (Vite dev server)"
	@echo "  - Tab 3: Desktop app (Tauri)"
	@echo "  - Tab 4: Mobile app (Expo)"
	@echo "  - Tab 5: Caddy reverse proxy"
	@echo ""
	@echo "Services will be available at:"
	@echo "  - Web: https://dev.openhims.health"
	@echo "  - API: https://api-dev.openhims.health"
	@echo "  - Desktop: Native app window"
	@echo "  - Mobile: Expo dev tools"
	@echo ""
	@echo "Close tabs to stop individual services"
	@echo ""
	./scripts/start-dev-tabs.sh

# Start all services in parallel in same terminal (for CI/containers)
dev-parallel: check-hosts
	@echo "🚀 Starting all development services in parallel..."
	@echo "Press Ctrl+C to stop all services"
	$(MAKE) -j5 start-rust start-web start-desktop start-mobile start-caddy

# Individual service targets
start-rust:
	@echo "🦀 Building Rust SDK..."
	cargo build
	@echo "✅ Rust SDK built successfully!"
	@echo "� SDK ready as library for frontend apps"

start-web:
	@echo "⚛️ Starting React web app..."
	pnpm dev:web

start-desktop:
	@echo "🖥️ Starting Tauri desktop app..."
	pnpm dev:desktop

start-mobile:
	@echo "📱 Starting React Native mobile app..."
	pnpm dev:mobile

start-caddy:
	@echo "🌐 Starting Caddy reverse proxy..."
	@echo "Note: Caddy requires sudo for certificate management and port binding"
	sudo caddy run --config Caddyfile.dev

# Stop all services
stop:
	@echo "🛑 Stopping all development services..."
	-pkill -f "cargo run"
	-pkill -f "vite"
	-pkill -f "tauri"
	-pkill -f "expo"
	-sudo caddy stop
	@echo "✅ All services stopped"

# Clean build artifacts
clean:
	@echo "🧹 Cleaning build artifacts..."
	cargo clean
	pnpm -r clean
	rm -rf target/
	rm -rf apps/*/dist/
	rm -rf apps/*/build/
	@echo "✅ Clean complete"

# Development shortcuts
rust: start-rust
web: start-web
desktop: start-desktop
mobile: start-mobile
caddy: start-caddy