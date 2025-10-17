#!/bin/bash

# Create RGBA icons for Tauri desktop app
cd /Users/apple/open-hims-rs/apps/desktop/src-tauri/icons

# Remove old icons
rm -f *.png *.ico *.icns

# Create base healthcare icon with RGBA transparency
create_icon() {
    local size=$1
    local border_radius=$((size / 5))
    local center=$((size / 2))
    local cross_size=$((size / 3))
    
    magick -size ${size}x${size} xc:transparent \
        -alpha transparent \
        -fill "#0066cc" \
        -draw "roundrectangle 2,2 $((size-2)),$((size-2)) ${border_radius},${border_radius}" \
        -stroke "#ffffff" \
        -strokewidth 2 \
        -draw "line $((center-cross_size/2)),${center} $((center+cross_size/2)),${center}" \
        -draw "line ${center},$((center-cross_size/2)) ${center},$((center+cross_size/2))" \
        ${size}x${size}.png
        
    echo "Created ${size}x${size}.png (RGBA)"
}

# Create all required icon sizes
sizes=(16 32 128 256 512)
for size in "${sizes[@]}"; do
    create_icon $size
done

# Verify RGBA format
echo -e "\nVerifying RGBA format:"
for size in "${sizes[@]}"; do
    format=$(file ${size}x${size}.png | grep -o "RGBA\|RGB")
    echo "${size}x${size}.png: $format"
done

# Create Windows ICO (contains multiple sizes)
magick 16x16.png 32x32.png 128x128.png 256x256.png icon.ico
echo "Created icon.ico"

# Create macOS ICNS
magick 16x16.png 32x32.png 128x128.png 256x256.png 512x512.png icon.icns
echo "Created icon.icns"

echo -e "\nAll icons created with RGBA format!"