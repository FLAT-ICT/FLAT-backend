#!/usr/bin/env bash

# echo "$(ls scripts)"
$('pwd')/scripts/import_csv.sh
$('pwd')/scripts/wait-for-it.sh --timeout=90 --strict mysql:3306 -- diesel setup
# diesel setup
diesel migration run
# cargo run 
flat_backend