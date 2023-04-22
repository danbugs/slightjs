#!/bin/bash

INSTALL_DIR="/usr/local"
OWNER_AND_REPO="${OWNER_AND_REPO:-danbugs/slightjs}"
BINARY_NAME="slightjs"

LATEST_RELEASE="$(curl -s https://api.github.com/repos/$OWNER_AND_REPO/releases | grep tag_name | awk 'NR == 1' | cut -d : -f 2 | cut -d \" -f 2)"
echo ">>> LATEST RELEASE: $LATEST_RELEASE..."

OS="$(uname)"
ARCH="$(uname -m)"
if [[ "${OS}" == "Linux" ]]
then
    TAR="slightjs-linux-x86_64.tar.gz"
elif [[ "${OS}" == "Darwin" ]]
then
    TAR="slightjs-macos.tar.gz"
else
  echo ">>> THIS INSTALLATION METHOD ONLY WORKS FOR MACOS AND LINUX."
  exit 1
fi

URL="https://github.com/$OWNER_AND_REPO/releases/download/$LATEST_RELEASE/$TAR"
echo ">>> DONLOADING FROM: $URL..."

curl -L -s $URL --output $TAR
echo ">>> DOWNLOADED BINARY TAR."

tar -xf $TAR
echo ">>> EXTRACTED BINARY TAR."

sudo install ./release/$BINARY_NAME $INSTALL_DIR/bin
echo ">>> INSTALLED BINARY."

rm $TAR
sudo rm -rf ./release
echo ">>> CLEANED UP."