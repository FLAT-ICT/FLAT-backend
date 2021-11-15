# fun-location-backend

API

サーバー起動

```
docker compose up --build -d flat-backend
```

テストをしたいとき

```
docker compose up --build -d flat-backend-test
```

ボリューム削除(DB 消す)

```
docker compose down -v
```

### API

こちらを参照してください
[FastAPI - Swagger UI](http://34.68.157.198:8080/docs#/)

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
