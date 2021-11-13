#!/usr/bin/env bash

git submodule update -i
cp ./sparse-checkout ./.git/modules/espresso-beacons/info/
cd espresso-beacons
git config core.sparsecheckout true
git read-tree -mu HEAD
git update-index --assume-unchanged src/spots.csv
sed -i -e '1 s/:/_/g' src/spots.csv
cd ..