# prefix_storage
Simple prefix-storage based on [tst library](https://github.com/billyevans/tst) as container and [hyper http-server](https://github.com/hyperium/hyper).
It's kind of example of usage tst-container.


### To launch:
```
cargo build
./target/debug/prefix_storage 1337
```
### To add some data:
```
curl localhost:1337/key -d "data"
curl localhost:1337/key2 -d "data"
```
### To get data by key
```
curl localhost:1337/key
```
### To get data by prefix
```
curl localhost:1337/key?query=prefix
```
### To remove data by key
```
curl localhost:1337/key -X DELETE
```

### To fill with some data from dict:
```
cat dict | fill.py 1337
```
