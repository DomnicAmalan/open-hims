#!/bin/bash

# Open HIMS - Asset Generation Script
# Uses native macOS tools (sips) - no ImageMagick or Node.js required
# Usage: ./scripts/generate-assets.sh

set -e

echo "🎨 Open HIMS - Generating assets for all platforms..."
echo ""

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Get script directory
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

# Source images (you should place these in assets-source/)
SOURCE_DIR="$PROJECT_ROOT/assets-source"
LOGO_FULL="$SOURCE_DIR/logo-full.png"       # logo-1.png or logo-3.png
LOGO_ICON="$SOURCE_DIR/logo-icon.png"       # logo-2.png or logo-4.png
LOGO_CIRCLE="$SOURCE_DIR/logo-circle.png"   # logo-5.png

# Check if source images exist
if [ ! -d "$SOURCE_DIR" ]; then
    echo "${YELLOW}⚠️  Creating assets-source directory...${NC}"
    mkdir -p "$SOURCE_DIR"
    echo ""
    echo "📋 Please place your logo files in: $SOURCE_DIR/"
    echo "   - logo-full.png   (Full logo with text, ~600x300px)"
    echo "   - logo-icon.png   (Square icon, 1024x1024px)"
    echo "   - logo-circle.png (Circular icon, 512x512px)"
    echo ""
    echo "Then run this script again."
    exit 1
fi

if [ ! -f "$LOGO_ICON" ]; then
    echo "${YELLOW}⚠️  logo-icon.png not found in $SOURCE_DIR${NC}"
    echo "Please add at least logo-icon.png (1024x1024px) to continue."
    exit 1
fi

# Function to resize image using sips (native macOS tool)
resize_image() {
    local input="$1"
    local output="$2"
    local size="$3"
    
    if [ -f "$input" ]; then
        sips -z "$size" "$size" "$input" --out "$output" > /dev/null 2>&1
        echo "${GREEN}✓${NC} Generated: $output (${size}x${size})"
    else
        echo "${YELLOW}⚠${NC} Skipped: $input not found"
    fi
}

# Function to create square canvas and center image
resize_with_padding() {
    local input="$1"
    local output="$2"
    local size="$3"
    
    if [ -f "$input" ]; then
        # First resize to fit within size with aspect ratio
        sips -Z "$size" "$input" --out "$output" > /dev/null 2>&1
        # Then pad to square (this is a limitation - sips can't add padding)
        # For proper padding, you'd need ImageMagick or manual editing
        echo "${GREEN}✓${NC} Generated: $output (~${size}x${size})"
    else
        echo "${YELLOW}⚠${NC} Skipped: $input not found"
    fi
}

echo "${BLUE}📱 Generating Mobile App Assets...${NC}"
echo "Location: apps/mobile/assets/"
mkdir -p "$PROJECT_ROOT/apps/mobile/assets"

# Mobile icon (1024x1024)
if [ -f "$LOGO_ICON" ]; then
    cp "$LOGO_ICON" "$PROJECT_ROOT/apps/mobile/assets/icon.png"
    echo "${GREEN}✓${NC} Generated: apps/mobile/assets/icon.png"
fi

# Adaptive icon (1024x1024 with padding)
if [ -f "$LOGO_ICON" ]; then
    cp "$LOGO_ICON" "$PROJECT_ROOT/apps/mobile/assets/adaptive-icon.png"
    echo "${GREEN}✓${NC} Generated: apps/mobile/assets/adaptive-icon.png"
    echo "${YELLOW}  Note: Add 20% padding manually for true adaptive icon${NC}"
fi

# Splash screen
if [ -f "$LOGO_FULL" ]; then
    # Create a white/gradient background splash (manual edit recommended)
    cp "$LOGO_FULL" "$PROJECT_ROOT/apps/mobile/assets/splash.png"
    echo "${GREEN}✓${NC} Generated: apps/mobile/assets/splash.png"
    echo "${YELLOW}  Note: Resize to 1284x2778 and add gradient background manually${NC}"
fi

# Favicon
resize_image "$LOGO_ICON" "$PROJECT_ROOT/apps/mobile/assets/favicon.png" 48

echo ""
echo "${BLUE}🌐 Generating Web App Assets...${NC}"
echo "Location: apps/web/public/"
mkdir -p "$PROJECT_ROOT/apps/web/public"

# Favicons (multiple sizes)
resize_image "$LOGO_ICON" "$PROJECT_ROOT/apps/web/public/favicon-32x32.png" 32
resize_image "$LOGO_ICON" "$PROJECT_ROOT/apps/web/public/favicon-16x16.png" 16

# Apple Touch Icon
resize_image "$LOGO_ICON" "$PROJECT_ROOT/apps/web/public/apple-touch-icon.png" 180

# PWA Icons
resize_image "$LOGO_ICON" "$PROJECT_ROOT/apps/web/public/logo192.png" 192
resize_image "$LOGO_ICON" "$PROJECT_ROOT/apps/web/public/logo512.png" 512

# OG Image (social media sharing)
if [ -f "$LOGO_FULL" ]; then
    sips -Z 1200 "$LOGO_FULL" --out "$PROJECT_ROOT/apps/web/public/og-image.png" > /dev/null 2>&1
    echo "${GREEN}✓${NC} Generated: apps/web/public/og-image.png"
    echo "${YELLOW}  Note: Manually resize to 1200x630 for proper OG image${NC}"
fi

# Create favicon.ico (requires iconutil or manual conversion)
echo "${YELLOW}  Note: For favicon.ico, use online converter or iconutil${NC}"

echo ""
echo "${BLUE}🖥️  Generating Desktop App Assets...${NC}"
echo "Location: apps/desktop/src-tauri/icons/"
mkdir -p "$PROJECT_ROOT/apps/desktop/src-tauri/icons"

# Copy source icon
cp "$LOGO_ICON" "$PROJECT_ROOT/apps/desktop/src-tauri/icons/icon.png"
echo "${GREEN}✓${NC} Generated: apps/desktop/src-tauri/icons/icon.png"

# Generate multiple sizes for desktop
resize_image "$LOGO_ICON" "$PROJECT_ROOT/apps/desktop/src-tauri/icons/32x32.png" 32
resize_image "$LOGO_ICON" "$PROJECT_ROOT/apps/desktop/src-tauri/icons/128x128.png" 128
resize_image "$LOGO_ICON" "$PROJECT_ROOT/apps/desktop/src-tauri/icons/128x128@2x.png" 256
resize_image "$LOGO_ICON" "$PROJECT_ROOT/apps/desktop/src-tauri/icons/icon@2x.png" 512

echo ""
echo "${BLUE}🔧 Platform-Specific Icon Generation...${NC}"
echo ""
echo "For proper .icns (macOS) and .ico (Windows) files, run:"
echo "  ${GREEN}cd apps/desktop && pnpm tauri icon ../src-tauri/icons/icon.png${NC}"
echo ""
echo "This requires Tauri CLI to be installed."

echo ""
echo "${GREEN}✅ Asset generation complete!${NC}"
echo ""
echo "📋 Next Steps:"
echo "  1. Review generated assets in apps/*/assets folders"
echo "  2. Manually adjust splash screens to proper dimensions"
echo "  3. Run Tauri icon generator for .icns/.ico files"
echo "  4. Add gradient backgrounds where needed"
echo "  5. Test on all platforms"
echo ""
echo "📚 See docs/ASSETS_GUIDE.md for detailed specifications"
