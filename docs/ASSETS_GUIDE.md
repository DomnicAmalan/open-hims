# Open HIMS - Assets & Branding Guide

## 🎨 Logo Overview

**Brand Name:** Open HIMS  
**Tagline:** Healthcare, Simplified.  
**Symbol:** Wolf/Fox with Medical Cross (represents agility, intelligence, and healthcare)

## 📐 Asset Dimensions & Usage

### Logo Variants Provided

1. **Full Logo with Text** (`logo-1.png`, `logo-3.png`)
   - Dimensions: ~600x300px (horizontal layout)
   - Use for: Headers, splash screens, marketing materials

2. **App Icon / Favicon** (`logo-2.png`, `logo-4.png`)
   - Dimensions: ~1024x1024px (square)
   - Use for: App icons, favicons, social media avatars

3. **Circular Icon** (`logo-5.png`)
   - Dimensions: ~512x512px (circular)
   - Use for: Compact displays, notifications

---

## 📱 Mobile App (React Native / Expo)

### Required Assets

```
apps/mobile/assets/
├── icon.png              # 1024x1024 (logo-2.png or logo-4.png)
├── adaptive-icon.png     # 1024x1024 (logo-2.png with padding)
├── splash.png            # 1284x2778 (logo-1.png centered)
└── favicon.png           # 48x48 (logo-2.png resized)
```

### Splash Screen Specs
- **iOS:** 1284x2778 (iPhone 14 Pro Max)
- **Android:** 1080x1920 (common resolution)
- **Background:** Light blue gradient (#E8F4F8 to #FFFFFF)
- **Logo:** Centered, ~40% screen width

### App Icon Specs
- **iOS:** 1024x1024 (no transparency, square)
- **Android (Adaptive):** 1024x1024 (with safe area padding)
- **Format:** PNG

### Update `app.json`:
```json
{
  "expo": {
    "icon": "./assets/icon.png",
    "splash": {
      "image": "./assets/splash.png",
      "resizeMode": "contain",
      "backgroundColor": "#E8F4F8"
    },
    "android": {
      "adaptiveIcon": {
        "foregroundImage": "./assets/adaptive-icon.png",
        "backgroundColor": "#FFFFFF"
      }
    }
  }
}
```

---

## 🌐 Web App (Vite + React)

### Required Assets

```
apps/web/public/
├── favicon.ico           # 32x32, 16x16 (logo-2.png)
├── favicon.svg           # Vector version (optional)
├── apple-touch-icon.png  # 180x180 (logo-2.png)
├── logo192.png           # 192x192 (logo-2.png)
├── logo512.png           # 512x512 (logo-2.png)
├── og-image.png          # 1200x630 (logo-1.png + background)
└── manifest.json         # PWA manifest
```

### HTML `<head>` Tags:
```html
<!-- apps/web/index.html -->
<head>
  <link rel="icon" type="image/png" sizes="32x32" href="/favicon.ico">
  <link rel="apple-touch-icon" sizes="180x180" href="/apple-touch-icon.png">
  <link rel="manifest" href="/manifest.json">
  
  <!-- Open Graph / Social Media -->
  <meta property="og:image" content="/og-image.png">
  <meta property="og:title" content="Open HIMS - Healthcare, Simplified">
  <meta name="twitter:card" content="summary_large_image">
</head>
```

### Loading Screen
```tsx
// apps/web/src/components/LoadingScreen.tsx
export function LoadingScreen() {
  return (
    <div style={{
      display: 'flex',
      alignItems: 'center',
      justifyContent: 'center',
      height: '100vh',
      background: 'linear-gradient(135deg, #E8F4F8 0%, #FFFFFF 100%)'
    }}>
      <img 
        src="/logo192.png" 
        alt="Open HIMS" 
        style={{ width: 120, animation: 'pulse 2s infinite' }}
      />
    </div>
  );
}
```

---

## 🖥️ Desktop App (Tauri)

### Required Assets

```
apps/desktop/src-tauri/icons/
├── 32x32.png           # Windows taskbar
├── 128x128.png         # macOS Dock
├── 128x128@2x.png      # macOS Retina
├── icon.icns           # macOS bundle icon
├── icon.ico            # Windows executable icon
└── icon.png            # Source (1024x1024)
```

### Generate Icons with Tauri CLI:
```bash
cd apps/desktop
pnpm tauri icon path/to/logo-2.png
```

This auto-generates:
- `.icns` for macOS
- `.ico` for Windows
- Multiple PNG sizes for Linux

### Splash Screen (Optional)
```
apps/desktop/src-tauri/icons/splash.png  # 800x600
```

### Update `tauri.conf.json`:
```json
{
  "tauri": {
    "bundle": {
      "icon": [
        "icons/32x32.png",
        "icons/128x128.png",
        "icons/128x128@2x.png",
        "icons/icon.icns",
        "icons/icon.ico"
      ]
    },
    "windows": [
      {
        "title": "Open HIMS",
        "width": 1200,
        "height": 800,
        "decorations": true,
        "transparent": false
      }
    ]
  }
}
```

---

## 🎨 Asset Preparation Checklist

### From Provided Logos:

#### **logo-1.png** (Full Logo - Horizontal)
- [x] Use for: Web header, email signatures
- [ ] Resize to 600x300 for web header
- [ ] Create splash screen variant (1284x2778 mobile)
- [ ] Create OG image (1200x630)

#### **logo-2.png** (Square Icon - White Background)
- [x] Use for: App icons, favicons
- [ ] Generate favicon.ico (32x32, 16x16)
- [ ] Generate apple-touch-icon (180x180)
- [ ] Generate PWA icons (192x192, 512x512)
- [ ] Use as Tauri icon source (1024x1024)
- [ ] Generate mobile icon (1024x1024)

#### **logo-3.png** (Full Logo - Gradient Background)
- [x] Use for: Marketing materials, splash screens
- [ ] Mobile splash screen (center on gradient)
- [ ] Desktop splash screen (if used)

#### **logo-4.png** (Square Icon - Gradient Background)
- [x] Use for: Alternative app icon
- [ ] Social media avatars (512x512)
- [ ] Notification icons

#### **logo-5.png** (Circular Icon)
- [x] Use for: Profile pictures, compact displays
- [ ] Chat avatars
- [ ] Small widgets

---

## 🔧 Image Optimization Commands

### Install Image Processing Tools
```bash
# macOS
brew install imagemagick

# Generate favicons
convert logo-2.png -resize 32x32 favicon-32x32.png
convert logo-2.png -resize 16x16 favicon-16x16.png
convert favicon-32x32.png favicon-16x16.png favicon.ico

# Generate Apple Touch Icon
convert logo-2.png -resize 180x180 apple-touch-icon.png

# Generate PWA Icons
convert logo-2.png -resize 192x192 logo192.png
convert logo-2.png -resize 512x512 logo512.png

# Generate mobile splash (centered on gradient background)
convert -size 1284x2778 gradient:#E8F4F8-#FFFFFF \
  logo-1.png -gravity center -composite splash.png
```

### Using Sharp (Node.js)
```javascript
// scripts/generate-assets.js
const sharp = require('sharp');

async function generateAssets() {
  const icon = 'logo-2.png';
  
  // Favicon
  await sharp(icon).resize(32, 32).toFile('favicon-32x32.png');
  await sharp(icon).resize(16, 16).toFile('favicon-16x16.png');
  
  // Apple Touch Icon
  await sharp(icon).resize(180, 180).toFile('apple-touch-icon.png');
  
  // PWA Icons
  await sharp(icon).resize(192, 192).toFile('logo192.png');
  await sharp(icon).resize(512, 512).toFile('logo512.png');
  
  // Mobile Icon
  await sharp(icon).resize(1024, 1024).toFile('mobile-icon.png');
}

generateAssets();
```

---

## 📋 Platform-Specific Guidelines

### iOS
- **App Icon:** 1024x1024, no transparency, square corners
- **Splash:** 1284x2778 (iPhone 14 Pro Max)
- **Colors:** Use brand blue (#3B82C4)

### Android
- **Adaptive Icon:** 1024x1024 with 20% padding
- **Splash:** 1080x1920, centered logo
- **Notification Icon:** Monochrome, transparent background

### macOS
- **App Icon:** .icns bundle (16-1024px sizes)
- **Dock Icon:** 128x128@2x (256px actual)
- **Menu Bar:** 16x16@2x (32px actual)

### Windows
- **Executable Icon:** .ico (16, 32, 48, 256px)
- **Taskbar:** 32x32
- **Start Menu:** 256x256

### Web
- **Favicon:** 32x32 .ico or .png
- **PWA Icons:** 192x192, 512x512
- **OG Image:** 1200x630 for social sharing

---

## 🎨 Brand Colors

### Primary Colors
- **Deep Blue:** `#2E6DA4` (main brand color)
- **Light Blue:** `#85C1E9` (secondary)
- **Sky Blue:** `#E8F4F8` (backgrounds)

### Accent Colors
- **Medical Cross:** `#FFFFFF` (white)
- **Gradient Start:** `#3B82C4`
- **Gradient End:** `#2E6DA4`

### Text Colors
- **Primary Text:** `#1A1A1A`
- **Secondary Text:** `#6B7280`
- **On Blue:** `#FFFFFF`

---

## 🚀 Quick Setup Scripts

### Generate All Assets
```bash
#!/bin/bash
# scripts/generate-all-assets.sh

echo "🎨 Generating Open HIMS assets..."

# Web assets
mkdir -p apps/web/public
convert logo-2.png -resize 32x32 apps/web/public/favicon-32.png
convert logo-2.png -resize 16x16 apps/web/public/favicon-16.png
convert apps/web/public/favicon-32.png apps/web/public/favicon-16.png apps/web/public/favicon.ico
convert logo-2.png -resize 180x180 apps/web/public/apple-touch-icon.png
convert logo-2.png -resize 192x192 apps/web/public/logo192.png
convert logo-2.png -resize 512x512 apps/web/public/logo512.png

# Mobile assets
mkdir -p apps/mobile/assets
cp logo-2.png apps/mobile/assets/icon.png
convert logo-2.png -resize 1024x1024 -background white -gravity center -extent 1024x1024 apps/mobile/assets/adaptive-icon.png
convert -size 1284x2778 gradient:#E8F4F8-#FFFFFF logo-1.png -gravity center -resize 600x300 -composite apps/mobile/assets/splash.png

# Desktop assets
mkdir -p apps/desktop/src-tauri/icons
cd apps/desktop && pnpm tauri icon ../../logo-2.png

echo "✅ Assets generated successfully!"
```

---

## 📱 Testing Checklist

### Mobile
- [ ] Icon displays correctly on home screen
- [ ] Splash screen shows on app launch
- [ ] Adaptive icon works on Android
- [ ] No white borders on iOS

### Web
- [ ] Favicon appears in browser tab
- [ ] PWA installs with correct icon
- [ ] OG image shows in link previews
- [ ] Loading screen displays smoothly

### Desktop
- [ ] Icon shows in dock/taskbar
- [ ] Window icon displays correctly
- [ ] About dialog shows logo
- [ ] Installer uses correct icon

---

## 🔗 Resources

- **Logo Files:** `/path/to/logos/`
- **Figma Design:** [Link to design file]
- **Brand Guidelines:** [Full brand book]
- **Icon Generator:** https://icon.kitchen (for quick testing)

---

## 📞 Support

For asset-related questions:
- Check this guide first
- Review platform documentation
- Ask in #design channel

---

**Generated:** October 17, 2025  
**Version:** 1.0  
**Maintainer:** Design Team
