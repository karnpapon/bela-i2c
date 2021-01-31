#!/usr/bin/env bash

mkdir -p ~/Bela/projects/bela-i2c
cp -r libbela/render.cpp ~/Bela/projects/bela-i2c
echo ' cp to project done'
cd ~/Bela/scripts
./build_project.sh ../projects/bela-i2c --force
