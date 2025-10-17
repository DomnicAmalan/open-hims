#!/bin/bash

# Create proper 8-bit RGBA icons for Tauri desktop app
cd /Users/apple/open-hims-rs/apps/desktop/src-tauri/icons

# Remove old icons
rm -f *.png *.ico *.icns

# Create base healthcare icon with proper 8-bit RGBA
create_icon() {
    local size=$1
    local border_radius=$((size / 5))
    local center=$((size / 2))
    local cross_half=$((size / 4))
    
    magick -size ${size}x${size} xc:transparent \
        -depth 8 \
        -type TrueColorAlpha \
        -fill "#0066cc" \
        -draw "roundrectangle 2,2 $((size-2)),$((size-2)) ${border_radius},${border_radius}" \
        -stroke "#ffffff" \
        -strokewidth $((size / 16 + 1)) \
        -draw "line $((center-cross_half)),${center} $((center+cross_half)),${center}" \
        -draw "line ${center},$((center-cross_half)) ${center},$((center+cross_half))" \
        ${size}x${size}.png
        
    echo "Created ${size}x${size}.png (8-bit RGBA)"
}

# Create all required icon sizes with proper format
sizes=(16 24 32 48 64 128 256 512)
for size in "${sizes[@]}"; do
    create_icon $size
done

# Create high-res variants
magick 128x128.png -resize 256x256 128x128@2x.png
magick 256x256.png -resize 512x512 256x256@2x.png  
magick 512x512.png -resize 1024x1024 512x512@2x.png

# Verify 8-bit RGBA format
echo -e "\nVerifying 8-bit RGBA format:"
for size in "${sizes[@]}"; do
    format=$(identify -format "%[bit-depth]bit %[colorspace] %[channels]" ${size}x${size}.png)
    echo "${size}x${size}.png: $format"
done

# Create Windows ICO (contains multiple sizes)
magick 16x16.png 32x32.png 48x48.png 128x128.png 256x256.png icon.ico
echo "Created icon.ico"

# Create macOS ICNS
magick 16x16.png 32x32.png 128x128.png 256x256.png 512x512.png icon.icns
echo "Created icon.icns"

echo -e "\nAll icons created with proper 8-bit RGBA format!"