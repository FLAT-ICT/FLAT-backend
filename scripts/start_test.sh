#!/usr/bin/env bash

$('pwd')/scripts/wait-for-it.sh --timeout=90 --strict mysql:3306 -- diesel setup
diesel migration run
cargo test
# flat_backend