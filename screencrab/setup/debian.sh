#!/bin/bash

# Update the package repository
sudo apt update
sudo apt-get install curl
# Install required dependencies for Rust and Node.js
sudo apt install libwebkit2gtk-4.0-dev build-essential curl wget libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev

# Install Rust using curl
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
export PATH=$PATH:$HOME/.cargo/bin
source "$HOME/.cargo/env"

# Install Node.js v18 from NodeSource
curl -s https://deb.nodesource.com/setup_18.x | sudo bash
sudo apt-get install -y nodejs

# Check the installed versions of Rust and Node.js
rustc --version
node -v
sudo snap install clion --classic
