#!/bin/bash

# Download the binary
curl -o kittenhttps://kitten.tronlang.org/v/latest

# Move the binary to /usr/local/bin
sudo mv kitten /usr/local/bin/

# Make the binary executable
sudo chmod +x /usr/local/bin/kitten

echo "Installation completed successfully."