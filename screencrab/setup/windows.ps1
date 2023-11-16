Install-PackageManager -Name Chocolatey
choco install libwebkit2gtk-4.0-dev build-essential curl wget libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev
curl -sSf https://rust-lang.org/install.sh | sh -s -- --rust-version=latest
choco install nodejs --version=18
rustc --version
node -v