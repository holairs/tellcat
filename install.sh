#!/bin/bash
cd tellcat
cd "$(dirname "$0")"

cargo build --release

if [ $? -eq 0 ]; then
    echo "Build successful. Proceeding to installation..."
    
    INSTALL_PATH="/usr/local/bin/tellcat"
    
    sudo cp ./target/release/tellcat $INSTALL_PATH
    
    sudo chmod +x $INSTALL_PATH
    
    if [ $? -eq 0 ]; then
        echo "Installation complete. Now running tellcat..."
        
        tellcat "Hello World"
    else
        echo "Failed to install tellcat."
        exit 1
    fi
else
    echo "Build failed. Please check for errors."
    exit 1
fi
