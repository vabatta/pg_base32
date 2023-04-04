FROM mcr.microsoft.com/devcontainers/rust:0-1-bullseye

# Include lld linker to improve build times either by using environment variable in .cargo/config.toml
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
	&& apt-get -y install clang lld python3-pip \
	&& apt-get autoremove -y && apt-get clean -y

# Install Commitizen for commit utilities
RUN pip3 install -U Commitizen

# Non-root user (required for PGX to spin up PG servers)
USER vscode

# Install PGX
RUN cargo install --locked cargo-pgx

# Init PGX
RUN cargo pgx init
