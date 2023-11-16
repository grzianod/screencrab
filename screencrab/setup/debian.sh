#!/bin/bash

# Update the package repository
sudo apt update

# Install required dependencies for Rust and Node.js
sudo apt install libwebkit2gtk-4.0-dev build-essential curl wget libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev

# Install Rust using curl
curl -sSf https://rust-lang.org/install.sh | sh -s -- --rust-version=latest

# Install Node.js v18 from NodeSource
curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash -
sudo apt-get install -y nodejs

# Check the installed versions of Rust and Node.js
rustc --version
node -v