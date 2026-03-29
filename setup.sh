#!/bin/bash
set -ex
export DEBIAN_FRONTEND=noninteractive

# 1. Add Third-Party Repos for the LATEST versions
# NodeSource for latest Node.js (Version 24/25+ as of 2026)
curl -fsSL https://deb.nodesource.com/setup_current.x | sudo -E bash -

# 2. Update and Upgrade existing system packages
sudo apt-get update
sudo apt-get upgrade -yq

# 3. Install/Upgrade everything via apt-get
# This will upgrade rustc, cargo, and nodejs if newer versions are in the repos
sudo apt-get install -yq \
    build-essential pkg-config curl wget file \
    libxdo-dev libssl-dev libayatana-appindicator3-dev \
    librsvg2-dev libwebkit2gtk-4.1-dev \
    nodejs \
    rustc cargo

# 4. Handle pnpm (NodeSource doesn't bundle pnpm, so we use npm to globalize it)
# This is the 'apt-friendly' way to keep it updated
sudo npm install -g pnpm@latest

# Verify
node -v
rustc --version
cargo --version
pnpm -v
