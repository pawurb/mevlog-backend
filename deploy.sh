#!/bin/bash
set -euo pipefail

bash timestamp_assets.sh

cross build --release --target x86_64-unknown-linux-musl 

rsync -avz target/x86_64-unknown-linux-musl/release/server $TARGET_NODE:/root/mevlog-backend/server
rsync -avz target/x86_64-unknown-linux-musl/release/scheduler $TARGET_NODE:/root/mevlog-backend/scheduler

rsync -azr --delete templates/ $TARGET_NODE:/root/mevlog-backend/templates
rsync -azr --delete assets/ $TARGET_NODE:/root/mevlog-backend/assets
rsync -azr --delete media/ $TARGET_NODE:/root/mevlog-backend/media
rsync -av .env-remote $TARGET_NODE:/root/mevlog-backend/.env
