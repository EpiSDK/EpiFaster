#!/bin/env sh

mkdir -p /opt/epiclang/lib/
mkdir -p $HOME/.local/bin/

cp -r ./plugins /opt/epiclang/lib/

cargo build -r
cp target/release/epiclang $HOME/.local/bin/epiclang

echo "Installation complete. Plugins have been copied to /opt/epiclang/lib/plugins/"