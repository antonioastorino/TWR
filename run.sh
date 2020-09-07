#!/bin/bash
CARGO_RELEASE_DIR="`pwd`/target/release"
echo "Building..."
cargo b --release
ERR=$?

[ $ERR -ne 0 ] && exit
echo "Build successfully"

sudo setcap cap_net_admin=eip "$CARGO_RELEASE_DIR/trust"
echo "Running"
sudo $CARGO_RELEASE_DIR/trust