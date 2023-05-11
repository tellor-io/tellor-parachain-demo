# Use Node.js 16 based on Debian Bullseye
FROM node:16-bullseye

# Install required dependencies and tools
RUN apt-get update && \
    apt-get install -y wget git build-essential clang pkg-config libssl-dev python3 curl

# Install Node.js dependencies
RUN yarn global add node-gyp

# Clone the parachains-integration-tests repository
RUN git clone https://github.com/paritytech/parachains-integration-tests.git && \
    cd parachains-integration-tests && \
    git checkout frank/additional-keypair-types && \
    yarn install && \
    yarn global add file:$PWD

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

# Download the oracle consumer parachain binary (./substrate-parachain-node/target/release/parachain-template-node)
# todo: download binary from github

# Copy needed files to the image
COPY ./scripts/launch.sh ./scripts/launch.sh
COPY ./scripts/deploy.sh ./scripts/deploy.sh
COPY ./network-config.toml ./network-config.toml

# Launch network
CMD ["/bin/bash", "scripts/launch.sh"]

# Deploy Contracts & Initialize State
# CMD ["/bin/bash", "scripts/deploy.sh"]

# Run integration tests
# todo:
