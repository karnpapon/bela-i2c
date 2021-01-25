#!/bin/sh -xe

# OSX and Linux only for now

BELA=bela.local
DIRECTORY="arm-bela-linux-gnueabihf"

# initial setup: download and extract an arm toolchain locally, and pull some
# files from Bela
if [ ! -d $DIRECTORY ]
then
  host=`uname`
  if [ "$host" = "Darwin" ]; then
    PACKAGE="arm-bela-linux-gnueabihf.zip"
    curl -o $PACKAGE http://files.bela.io/gcc/$PACKAGE
    unzip $PACKAGE
    rm $PACKAGE
    echo $'[target.armv7-unknown-linux-gnueabihf]\nlinker =  "arm-bela-linux-gnueabihf-gcc"\n' > .cargo/config
  elif [ "$host" = "Linux" ]; then
    PACKAGE="gcc-linaro-6.3.1-2017.02-x86_64_arm-linux-gnueabihf.tar.xz"
    wget https://releases.linaro.org/components/toolchain/binaries/6.3-2017.02/arm-linux-gnueabihf/$PACKAGE
    tar xf $PACKAGE
    mv gcc-linaro-6.3.1-2017.02-x86_64_arm-linux-gnueabihf $DIRECTORY
    rm -r $PACKAGE
    echo $'[target.armv7-unknown-linux-gnueabihf]\nlinker =  "arm-linux-gnueabihf-gcc"\n' > .cargo/config
  fi

  # not sure why export PATH is not working?
  export PATH=$PATH:`pwd`/$DIRECTORY/bin
fi