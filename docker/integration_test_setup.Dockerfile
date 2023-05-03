# Use an appropriate base image, e.g., Ubuntu
FROM ubuntu:20.04

# Install necessary dependencies
RUN apt-get update && \
    apt-get install -y curl

# Download the Polkadot binary
RUN curl -L -o polkadot/polkadot "https://github.com/paritytech/polkadot/releases/download/v0.9.41/polkadot" && \
    chmod +x polkadot/polkadot

# Download the Cumulus binary
RUN curl -L -o cumulus/polkadot-parachain "https://github.com/paritytech/cumulus/releases/download/v0.9.400/polkadot-parachain" && \
    chmod +x cumulus/polkadot-parachain

# Download the Moonbeam binary
RUN curl -L -o moonbeam/moonbeam "https://github.com/PureStake/moonbeam/releases/download/v0.31.1/moonbeam" && \
    chmod +x moonbeam/moonbeam

# Copy the repository files into the image
COPY . .