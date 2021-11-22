#/usr/bin/env bash

if [ $# -ne 2 ]; then
    echo '引数が2つ必要です。my_id target_id'
    exit 1
fi

curl -X 'POST' \
  'http://127.0.0.1:3000/v1/friends/add' \
  -H 'accept: application/json' \
  -H 'Content-Type: application/json' \
  -d "{
  \"my_id\": $1,
  \"target_id\": $2
}"