# ************************************
# See ./docs/docker-setup.md for usage
# ************************************

# Use same image as ubuntu-latest for GitHub Actions
FROM ubuntu:22.04

# Install wget and other dependencies
RUN apt-get update && \
    apt-get install -y wget git build-essential clang pkg-config libssl-dev python3

# Install Node.js
RUN wget -qO- https://deb.nodesource.com/setup_16.x | bash -
RUN apt-get install -y nodejs

# Install Yarn
RUN npm install -g yarn

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

# Download the oracle consumer parachain binary
RUN mkdir -p substrate-parachain-node/target/release && \
    wget -q -O substrate-parachain-node/target/release/parachain-template-node "https://github.com/tellor-io/substrate-parachain-node/releases/download/release-de93110b29a3d73235d3c720c1ce6a705ccd787d/parachain-template-node" && \
    chmod +x substrate-parachain-node/target/release/parachain-template-node

# Copy needed files to the image
COPY ./scripts/launch.sh ./scripts/launch.sh
COPY ./scripts/deploy.sh ./scripts/deploy.sh
COPY ./network-config.toml ./network-config.toml
COPY ./tellor-contracts ./tellor-contracts
