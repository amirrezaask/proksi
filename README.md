# Proksi
Simple HTTP proxy server that has both a bridge mode and upstream mode, like v2ray but simpler.

# Sample usage
after running server using `cargo run`, you can test it with something like [[httpbin](http://httpbin.org/post)]:

```bash
http_proxy='http://localhost:8080' curl -X POST "http://httpbin.org/post" -H  "accept: application/json"
```

## TODO

- CLI ux
- configuration file
- encrypting on bridges and decrypting on upstream