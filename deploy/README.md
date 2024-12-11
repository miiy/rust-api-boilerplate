# Deploy

## Build

```bash
docker build -f deploy/Dockerfile -t rust-api .
```

## Run

```bash
docker run --name rust-api --restart always \
    -p 8080:8080 \
    -v /app/config:/app/config \
    rust-api
```
