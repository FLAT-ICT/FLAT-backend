#/usr/bin/env bash

if [ $# -ne 2 ]; then
    echo '引数が2つ必要です。target_name password'
    exit 1
fi

curl -X 'POST' \
  'http://127.0.0.1:3000/v1/login' \
  -H 'accept: application/json' \
  -H 'Content-Type: application/json' \
  -d "{
  \"name\": \"$1\",
  \"password\": \"$2\"
}"
