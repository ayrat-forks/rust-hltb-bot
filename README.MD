# How Long To Beat Telegram Bot

Telegram bot (hosted by AWS Lambda) that respond with [How Long To Beat](https://howlongtobeat.com/) information by scrapping site itself.

## Build

### Using docker

1. Prerequisite: Docker
2. Run docker compose
```
   docker compose up --build build
```

### Locally
1. Prerequisite: Linux
2. Install Rust https://www.rust-lang.org/tools/install
3. Install musl toolchain
   ```
   rustup target add x86_64-unknown-linux-musl
   ```
4. Install musl-tools
   ```
   sudo apt install musl-tools 
   ``` 
5. Install zip
   ```
   apt-get install zip
   ```
6. Install build essential
   ```
   apt-get install -y build-essential
   ```
7. Build
   ```
   sh build.sh
   ```
   
## License

MIT