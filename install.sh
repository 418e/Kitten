#!/bin/bash

# Download the binary
curl -o kitten https://tronlang.org/kitten

# Move the binary to /usr/local/bin
sudo mv kitten /usr/local/bin/

# Make the binary executable
sudo chmod +x /usr/local/bin/kitten

echo "Installation completed successfully."