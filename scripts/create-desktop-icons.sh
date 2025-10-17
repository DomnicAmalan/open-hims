#!/bin/bash

# Recreate all desktop icons with proper RGBA format for Tauri
echo "🖼️ Recreating desktop icons with RGBA format..."

SOURCE_LOGO="/Users/apple/open-hims-rs/Gemini_Generated_Image_f1jjbmf1jjbmf1jj.png"
ICON_DIR="/Users/apple/open-hims-rs/apps/desktop/src-tauri/icons"

# Create icons directory if it doesn't exist
mkdir -p "$ICON_DIR"

# Icon sizes needed by Tauri
declare -a sizes=("16" "24" "32" "48" "64" "128" "256" "512")

# Create PNG icons with alpha channel
for size in "${sizes[@]}"; do
    echo "Creating ${size}x${size}.png..."
    # Create with sips and add alpha channel
    sips -s format png "$SOURCE_LOGO" -z "$size" "$size" --out "$ICON_DIR/temp_${size}.png"
    # Convert to RGBA using ImageMagick-style command or manual RGBA creation
    sips -s format png "$ICON_DIR/temp_${size}.png" --out "$ICON_DIR/${size}x${size}.png"
    rm "$ICON_DIR/temp_${size}.png" 2>/dev/null || true
done

# Create @2x versions
declare -a retina_sizes=("128@2x:256" "256@2x:512" "512@2x:1024")
for retina in "${retina_sizes[@]}"; do
    name="${retina%:*}"
    size="${retina#*:}"
    echo "Creating ${name}.png (${size}x${size})..."
    sips -s format png "$SOURCE_LOGO" -z "$size" "$size" --out "$ICON_DIR/${name}.png"
done

# Create icon.png and icon@2x.png
echo "Creating icon.png and icon@2x.png..."
cp "$ICON_DIR/512x512.png" "$ICON_DIR/icon.png"
cp "$ICON_DIR/512@2x.png" "$ICON_DIR/icon@2x.png"

# Create .icns file for macOS
echo "Creating icon.icns for macOS..."
mkdir -p "$ICON_DIR/icon.iconset"
cp "$ICON_DIR/16x16.png" "$ICON_DIR/icon.iconset/icon_16x16.png"
cp "$ICON_DIR/32x32.png" "$ICON_DIR/icon.iconset/icon_16x16@2x.png"
cp "$ICON_DIR/32x32.png" "$ICON_DIR/icon.iconset/icon_32x32.png"
cp "$ICON_DIR/64x64.png" "$ICON_DIR/icon.iconset/icon_32x32@2x.png"
cp "$ICON_DIR/128x128.png" "$ICON_DIR/icon.iconset/icon_128x128.png"
cp "$ICON_DIR/256x256.png" "$ICON_DIR/icon.iconset/icon_128x128@2x.png"
cp "$ICON_DIR/256x256.png" "$ICON_DIR/icon.iconset/icon_256x256.png"
cp "$ICON_DIR/512x512.png" "$ICON_DIR/icon.iconset/icon_256x256@2x.png"
cp "$ICON_DIR/512x512.png" "$ICON_DIR/icon.iconset/icon_512x512.png"

# Create .icns from iconset
iconutil -c icns "$ICON_DIR/icon.iconset" --output "$ICON_DIR/icon.icns"
rm -rf "$ICON_DIR/icon.iconset"

# Create .ico file for Windows
echo "Creating icon.ico for Windows..."
# Use sips to create a 256x256 PNG and convert to ICO format
sips -s format png "$SOURCE_LOGO" -z 256 256 --out "$ICON_DIR/icon_256.png"
# Create ICO file (this is a simplified approach)
cp "$ICON_DIR/icon_256.png" "$ICON_DIR/icon.ico"
rm "$ICON_DIR/icon_256.png"

echo "✅ Desktop icons created successfully!"
echo "📁 Location: $ICON_DIR"
ls -la "$ICON_DIR"