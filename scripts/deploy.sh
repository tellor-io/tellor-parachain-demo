#!/bin/bash
echo Deploying initial network state...
cd init || exit
cargo run --release