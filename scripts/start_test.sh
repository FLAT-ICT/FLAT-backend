#!/usr/bin/env bash


echo $('ls')
echo $('pwd')
# $('pwd')/scripts/wait-for-it.sh --timeout=90 --strict mysql:3306 -- mysql -udocker -p9Xu84SA1 -h mysql diesel_demo
$('pwd')/scripts/wait-for-it.sh --timeout=90 --strict mysql:3306 -- diesel setup
# diesel setup
diesel migration run
cargo test