#!/usr/bin/env bash
git submodule update -i
cp ./sparse-checkout ./.git/modules/espresso-beacons/info/
cd espresso-beacons
git config core.sparsecheckout true
git read-tree -mu HEAD
cd ..