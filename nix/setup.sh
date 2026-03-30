sudo mkdir -p $HOME/.local/lib/epiclang/
sudo mkdir -p $HOME/.local/bin/
sudo cp -r ./plugins $HOME/.local/lib/epiclang/
make install
chmod +x epiclang
sudo cp epiclang $HOME/.local/bin/epiclang
    echo "Installation complete. Plugins have been copied to $HOME/.local/lib/epiclang/plugins/"