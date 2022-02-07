#/usr/bin/env bash

if [ $# -ne 2 ]; then
    echo '引数が2つ必要です。target_id target_name'
    exit 1
fi

curl -i -X 'POST' \
  'http://localhost:3000/v1/user/name' \
  -H 'accept: application/json' \
  -H 'Content-Type: application/json' \
  -d "{
  \"my_id\": $1,
  \"target_name\": \"$2\"
}"