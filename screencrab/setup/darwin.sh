#!/bin/bash

# Install Homebrew
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# Install required dependencies for Rust and Node.js
brew install libwebkit2gtk-4.0-dev build-essential curl wget libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev

# Install Rust using curl
curl -sSf https://rust-lang.org/install.sh | sh -s -- --rust-version=latest

# Install Node.js v18 from NodeSource
curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash -
sudo apt-get install -y nodejs

# Check the installed versions of Rust and Node.js
rustc --version
node -v