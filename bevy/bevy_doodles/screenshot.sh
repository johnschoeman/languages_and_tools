#!/usr/bin/env bash
# Script to build, run, take screenshot, and exit automatically

set -e

echo "Building and running Bevy app with auto-screenshot..."
AUTO_SCREENSHOT=1 AUTO_DEBUG=1 nix develop -c cargo run

if [ -f "./tmp/bevy_screenshot_auto.png" ]; then
    echo "Screenshot saved to ./tmp/bevy_screenshot_auto.png"
    ls -lh ./tmp/bevy_screenshot_auto.png
else
    echo "Warning: Screenshot file not found!"
    exit 1
fi
