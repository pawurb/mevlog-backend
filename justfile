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

# Deploy and restart
release:
    ./deploy.sh && ./remote/restart.sh
