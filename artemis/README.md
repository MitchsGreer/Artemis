# Artemis
Artemis is a requirements management tool.

## Setup

1. Copy the example `.env` file to a local one, and modify any values inside desired:
```bash
cp .env.example .env
```
2. Run the following command to start the database
```bash
docker-compose up     # run in current terminal
docker-compose up -d  # run in detached/headless mode


# or for newer versions of docker
docker compose up
```
3. Start the server application
```bash
# build the entire application
cargo build

# run the server crate specifically
cargo run -p server
```

