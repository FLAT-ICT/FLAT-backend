# fun-location-backend

初回ダウンロード時やってほしいこと
```
rm -rf espresso-beacons
./scripts/import_csv.sh
```

API

サーバー起動

`-d` をつけるとバックグラウンドで動く
```
docker compose up --build flat-backend
docker compose up --build -d flat-backend
```

テストをしたいとき

```
docker compose up --build flat-backend-test
```

ボリューム削除(DB 消す)

```
docker compose down -v
```

### API

こちらを参照してください
[FastAPI - Swagger UI](http://35.239.225.65:8080/docs#/)


### API のテスト
1. `test-sh` の中に移動してください
2. コマンドを実行します
    ```
    ./test0101.sh
    ```

### 以下関係なし

`diesel`  
WSL Ubuntu 20.04

```bash
sudo apt install libmysqlclient-dev
cargo install diesel_cli --no-default-features --features mysql
```

```
sudo mysql -u root -p
```

```
GRANT ALL ON hoge.* TO 'user'@'localhost';
```
