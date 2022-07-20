#!/usr/bin/env bash

# echo "$(ls scripts)"
$('pwd')/scripts/import_csv.sh
$('pwd')/scripts/wait-for-it.sh --timeout=90 --strict postgres:5432 -- diesel setup
# diesel setup
diesel migration run
# cargo run 
flat_backend