# Just configuration for mevlog-backend

# Default recipe
default:
    @just --list

# Start the server with asset timestamping and environment setup
server:
    ./timestamp_assets.sh && source .envrc && cargo run --bin server

# Deploy using the deployment script
deploy:
    ./deploy.sh 

# Deploy, restart, and warmup
release: && warmup
    ./deploy.sh && ./remote/restart.sh

# Warmup the server after deployment
warmup:
    ./warmup.sh
