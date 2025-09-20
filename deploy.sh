#!/bin/bash
set -euo pipefail 

bash timestamp_assets.sh

rsync -azr --delete src/ $TARGET_NODE:/root/mevlog-backend/src
rsync -azr --delete bin/ $TARGET_NODE:/root/mevlog-backend/bin
rsync -azr --delete templates/ $TARGET_NODE:/root/mevlog-backend/templates
rsync -azr --delete assets/ $TARGET_NODE:/root/mevlog-backend/assets
rsync -azr --delete javascripts/ $TARGET_NODE:/root/mevlog-backend/javascripts
rsync -azr --delete styles/ $TARGET_NODE:/root/mevlog-backend/styles
rsync -azr --delete media/ $TARGET_NODE:/root/mevlog-backend/media
rsync -av Cargo.* $TARGET_NODE:/root/mevlog-backend/
rsync -av .env-remote $TARGET_NODE:/root/mevlog-backend/.env
ssh $TARGET_NODE 'source "$HOME/.cargo/env"; cd /root/mevlog-backend; OPENSSL_DIR=/usr/ OPENSSL_LIB_DIR=/usr/lib/x86_64-linux-gnu/ cargo build --release'
