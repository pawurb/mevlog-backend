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
    @echo "Waiting 20 seconds for server to start..."
    sleep 20
    @echo "Warming up homepage..."
    curl -s -o /dev/null -w "%{http_code}" https://mevlog.rs/
    @echo ""
    @echo "Warming up /explore..."
    curl -s -o /dev/null -w "%{http_code}" https://mevlog.rs/explore
    @echo ""
    @echo "Warming up /search..."
    curl -s -o /dev/null -w "%{http_code}" https://mevlog.rs/search
    @echo ""
    @echo "Warming up search with params..."
    curl -s -o /dev/null -w "%{http_code}" "https://mevlog.rs/search?blocks=100%3Alatest&from=jaredfromsubway.eth&chain_id=1"
    @echo ""
    @echo "Warmup complete."
