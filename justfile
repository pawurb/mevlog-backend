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
deploy_restart:
    ./deploy.sh && ./remote/restart.sh
