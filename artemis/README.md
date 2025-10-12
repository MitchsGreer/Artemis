# Artemis
Artemis is a requirements management tool.

## Setup
First set a password for the DB in a local `.env` file:
```bash
echo "POSTGRES_PASSWORD='password'" > .env
```

## Running the Application
```bash
# build the entire application
cargo build

# run the server crate specifically
cargo run -p server
```

