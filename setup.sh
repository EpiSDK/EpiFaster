mkdir -p $HOME/.local/lib/epiclang/
mkdir -p $HOME/.local/bin/
cp -r ./plugins $HOME/.local/lib/epiclang/
cargo build -r
cp target/release/epiclang $HOME/.local/bin/epiclang
echo "Installation complete. Plugins have been copied to $HOME/.local/lib/epiclang/plugins/"