# Use the latest foundry image so Rust and Foundry are already installed
FROM ghcr.io/foundry-rs/foundry

# Install required dependencies and tools
RUN apk add --no-cache wget git build-base clang pkgconf openssl-dev yarn

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
RUN mkdir -p polkadot/target/release && \
    wget -q -O polkadot/target/release/polkadot "https://github.com/paritytech/polkadot/releases/download/v0.9.41/polkadot" && \
    chmod +x polkadot/target/release/polkadot

# Download the Cumulus binary
RUN mkdir -p cumulus/target/release && \
    wget -q -O cumulus/target/release/polkadot-parachain "https://github.com/paritytech/cumulus/releases/download/v0.9.400/polkadot-parachain" && \
    chmod +x cumulus/target/release/polkadot-parachain

# Download the Moonbeam binary
RUN mkdir -p moonbeam/target/release && \
    wget -q -O moonbeam/target/release/moonbeam "https://github.com/PureStake/moonbeam/releases/download/v0.31.1/moonbeam" && \
    chmod +x moonbeam/target/release/moonbeam

# Download the oracle consumer parachain binary (substrate-parachain-node)
# note: using locally-built release binary, since non availble on github
# todo: implement this

# Launch network
CMD ["parachains-integration-tests", "-m", "zombienet", "-c", "network-config.toml"]

# Deploy Contracts & Initialize State


# Run integration tests
# todo: implement this

# Build image command:
# docker build -t your-image-name .
# Run container command:
# docker run -it --rm --name your-container-name -p 9900:9900 -p 9910:9910 -p 9920:9920 -p 9930:9930 -p 9921:9921 your-image-name
