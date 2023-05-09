# Use Node.js 16 based on Debian Bullseye
FROM node:16-bullseye

# Install required dependencies and tools
RUN apt-get update && \
    apt-get install -y wget git build-essential clang pkg-config libssl-dev python3 curl

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Set the PATH environment variable for Rust
ENV PATH="/root/.cargo/bin:${PATH}"

# Install Foundry
RUN curl -L https://foundry.paradigm.xyz | bash

# Install Node.js dependencies
RUN yarn global add node-gyp

# Clone the parachains-integration-tests repository
RUN git clone https://github.com/paritytech/parachains-integration-tests.git && \
    cd parachains-integration-tests && \
    git checkout frank/additional-keypair-types && \
    yarn install --force && \
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
# todo: download binary from github

# Launch network
RUN chmod +x scripts/launch.sh
CMD ["/bin/bash", "scripts/launch.sh"]

# Deploy Contracts & Initialize State
# CMD ["/bin/bash", "scripts/deploy.sh"]

# Run integration tests
# todo:

# Build image command:
# docker build -t your-image-name .
# or if on M1 Mac:
# docker build --platform linux/arm64/v8 -t test-demo-1 .
# img name: test-demo-1
# Run container command:
# docker run -it --rm --name your-container-name -p 9900:9900 -p 9910:9910 -p 9920:9920 -p 9930:9930 -p 9921:9921 your-image-name
# container name: test-demo-1-container
