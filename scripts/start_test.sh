#!/usr/bin/env bash

# yes yes | ssh -T git@github.com -v
# ssh -oStrictHostKeyChecking=no -T git@github.com
# if [ $? -ne 0 ]; then
#     echo 'ssh接続失敗'
#     exit 1
# fi
# $('pwd')/scripts/import_csv.sh
# if [ $? -ne 0 ]; then
#     echo 'csvを取得できません'
#     exit 1
# fi
$('pwd')/scripts/wait-for-it.sh --timeout=90 --strict mysql:3306 -- diesel setup
diesel migration run
flat_backend &
cargo test
if [ $? -ne 0 ]; then
    echo 'testが落ちました'
    exit 1
fi