#!/usr/bin/env bash

$('pwd')/scripts/import_csv.sh
if [ $? -ne 0 ]; then
    echo 'csvを取得できません'
    # exit 1
fi
$('pwd')/scripts/wait-for-it.sh --timeout=90 --strict postgres:5432 -- diesel setup
diesel migration run
flat_backend &
cargo test -- --test-threads=1
if [ $? -ne 0 ]; then
    echo 'testが落ちました'
    exit 1
fi