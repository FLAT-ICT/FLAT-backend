# fun-location-backend
API

起動(どちらかを選ぶ)
```
docker compsoe up --build -d flat-backend
docker compsoe up --build -d flat-backend-test
```

ボリューム削除(DB消す)
```
docker compse down -v
```


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
