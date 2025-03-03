# Deploy

## Build

```bash
docker build -f deploy/Dockerfile -t rust-api .
```

## Run

```bash
docker run --name rust-api --restart always \
    --log-opt max-size=1g \
    -v /data/rust-api/config:/app/config \
    --net backend \
    rust-api
```
