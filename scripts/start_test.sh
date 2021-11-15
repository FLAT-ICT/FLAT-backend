#!/usr/bin/env bash

$('pwd')/scripts/import_csv.sh
if [ $? -eq 1 ]; then
    echo 'csvを取得できません'
    exit 1
fi
$('pwd')/scripts/wait-for-it.sh --timeout=90 --strict mysql:3306 -- diesel setup
diesel migration run
flat_backend &
cargo test
if [ $? -eq 1 ]; then
    echo 'testが落ちました'
    exit 1
fi