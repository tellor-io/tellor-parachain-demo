# Use the latest foundry image so Rust and Foundry are already installed
FROM ghcr.io/foundry-rs/foundry

# Install required dependencies and tools
RUN apt-get update && \
    apt-get install -y curl git build-essential clang libclang-dev pkg-config libssl-dev && \
    apt-get install -y yarn

# Clone and setup parachains-integration-tests
RUN git clone https://github.com/paritytech/parachains-integration-tests.git && \
    cd parachains-integration-tests && \
    git checkout frank/additional-keypair-types && \
    yarn install && \
    yarn global add file:$PWD

# Expose WebSocket ports
# Relay chain
EXPOSE 9900
# Statemine
EXPOSE 9910
# Moonbeam
EXPOSE 9920
# Oracle consumer parachain
EXPOSE 9930

# Expose Moonbeam RPC port
EXPOSE 9921

# Copy the repository files into the image
COPY . .

# Initialize submodules if needed
RUN git submodule update --init --recursive

# Download the Polkadot binary
RUN curl -L -o polkadot/polkadot "https://github.com/paritytech/polkadot/releases/download/v0.9.41/polkadot" && \
    chmod +x polkadot/polkadot

# Download the Cumulus binary
RUN curl -L -o cumulus/polkadot-parachain "https://github.com/paritytech/cumulus/releases/download/v0.9.400/polkadot-parachain" && \
    chmod +x cumulus/polkadot-parachain

# Download the Moonbeam binary
RUN curl -L -o moonbeam/moonbeam "https://github.com/PureStake/moonbeam/releases/download/v0.31.1/moonbeam" && \
    chmod +x moonbeam/moonbeam

# Download the oracle consumer parachain binary (substrate-parachain-node)
# todo: implement this

# Launch network
CMD ["parachains-integration-tests", "-m", "zombienet", "-c", "network-config.toml"]

# Deploy Contracts & Initialize State


# Run integration tests
# todo: implement this
