#!/bin/bash

# Create macOS application bundle
echo "Creating macOS application bundle..."

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"
cd "$SCRIPT_DIR" || exit 1

APP_NAME="RestReminder"
APP_DIR="target/release/${APP_NAME}.app"
EXECUTABLE="target/release/rest-reminder"
ICO_PATH="assets/icon.ico"

# Check if executable exists
if [ ! -f "$EXECUTABLE" ]; then
    echo "Error: Executable not found. Please run 'cargo build --release' first"
    exit 1
fi

# Check for original icon file
if [ ! -f "$ICO_PATH" ]; then
    echo "Warning: Icon file not found at $ICO_PATH"
    exit 1
fi

# Create application bundle structure
mkdir -p "${APP_DIR}/Contents/MacOS"
mkdir -p "${APP_DIR}/Contents/Resources"

# Copy executable file
echo "Copying executable file..."
cp "$EXECUTABLE" "${APP_DIR}/Contents/MacOS/${APP_NAME}"
chmod +x "${APP_DIR}/Contents/MacOS/${APP_NAME}"

# Create real macOS icon file
echo "Creating macOS icon..."
TEMP_PNG="target/temp_macos_icon.png"
ICONSET_DIR="target/icon.iconset"
FINAL_ICNS="${APP_DIR}/Contents/Resources/icon.icns"

# Clean up old files
rm -rf "$ICONSET_DIR" "$TEMP_PNG" 2>/dev/null

# Step 1: Convert ICO to PNG
if sips -s format png "$ICO_PATH" --out "$TEMP_PNG" >/dev/null 2>&1; then
    echo "ICO to PNG conversion successful"
    
    # Step 2: Create iconset directory and multi-size icons
    mkdir -p "$ICONSET_DIR"
    
    # Generate different sized icons (only basic sizes)
    for size in 16 32 64 128 256; do
        case $size in
            16) filename="icon_16x16.png" ;;
            32) filename="icon_32x32.png" ;;
            64) filename="icon_32x32@2x.png" ;;
            128) filename="icon_128x128.png" ;;
            256) filename="icon_256x256.png" ;;
        esac
        
        if sips -z $size $size "$TEMP_PNG" --out "$ICONSET_DIR/$filename" >/dev/null 2>&1; then
            echo "Created ${size}x${size} icon"
        else
            echo "Warning: ${size}x${size} icon creation failed, skipping"
        fi
    done
    
    # Add @2x versions
    if [ -f "$ICONSET_DIR/icon_32x32.png" ]; then
        cp "$ICONSET_DIR/icon_32x32.png" "$ICONSET_DIR/icon_16x16@2x.png"
        echo "Created 16@2x icon"
    fi
    
    if [ -f "$ICONSET_DIR/icon_256x256.png" ]; then
        cp "$ICONSET_DIR/icon_256x256.png" "$ICONSET_DIR/icon_128x128@2x.png"
        echo "Created 128@2x icon"
    fi
    
    # Step 3: Use iconutil to create .icns file
    if iconutil -c icns "$ICONSET_DIR" -o "$FINAL_ICNS" >/dev/null 2>&1; then
        echo "Successfully created .icns icon file"
        # Clean up temporary files
        rm -rf "$ICONSET_DIR" "$TEMP_PNG"
    else
        echo "Warning: iconutil failed, using PNG as icon..."
        # Fallback: use PNG directly
        cp "$TEMP_PNG" "$FINAL_ICNS"
        rm -f "$TEMP_PNG"
    fi
else
    echo "Error: Icon conversion failed, using original ICO file"
    cp "$ICO_PATH" "$FINAL_ICNS"
fi

# Create Info.plist file
cat > "${APP_DIR}/Contents/Info.plist" << EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleDevelopmentRegion</key>
    <string>en</string>
    <key>CFBundleExecutable</key>
    <string>${APP_NAME}</string>
    <key>CFBundleIconFile</key>
    <string>icon</string>
    <key>CFBundleIdentifier</key>
    <string>com.example.rest-reminder</string>
    <key>CFBundleInfoDictionaryVersion</key>
    <string>6.0</string>
    <key>CFBundleName</key>
    <string>Rest Reminder</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleShortVersionString</key>
    <string>1.4.0</string>
    <key>CFBundleVersion</key>
    <string>1.4.0</string>
    <key>LSMinimumSystemVersion</key>
    <string>10.12</string>
    <key>NSHighResolutionCapable</key>
    <true/>
</dict>
</plist>
EOF

echo "Application bundle created: ${APP_DIR}"
echo "You can now see the application with icon in Finder!"
echo ""
echo "Usage:"
echo "1. Double-click ${APP_DIR} to run the application"
echo "2. Or run from terminal: open '${APP_DIR}'"
echo "3. Command line usage: '${APP_DIR}/Contents/MacOS/${APP_NAME}' rest --help"