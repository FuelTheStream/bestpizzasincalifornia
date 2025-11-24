#!/bin/bash
# Build script that ensures assets are copied

echo "Building the project..."
dx build --release

echo "Copying assets..."
cp -r public/assets/*.css target/dx/app/release/web/public/assets/
cp -r public/assets/pitfire target/dx/app/release/web/public/assets/

echo "Build complete!"
