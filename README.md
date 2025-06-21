# reservation 开始写一个预约系统

一个预约系统
### pre-commit 挻多好处，检查提交前的问题，代码格式
```
pre-commit install
```

```bash
cargo install typos-cli

```

## How to use it

```bash
$ cargo generate --git https://github.com/tyrchen/rust-lib-template
```
### 生成项目的abi grpc ,生成的单独的代码，放到一个crate 里面,生成各种各样的trait
```bash
cargo new abi --lib
cargo new reservation --lib
cargo new service
```
### 添加 prost
```bash
cargo add prost -p abi
cargo add prost-types -p abi
cargo add tonic -p abi
cargo add tonic-build --build -p abi

touch abi/build.rs # 添加编译脚本

mkdir abi/src/pb # 创建编译输出文件夹
```
### 注意上面的版本信息
```yaml
[dependencies]
tonic = "0.13"
prost = "0.13"
prost-types = "0.13"
[build-dependencies]
tonic-build = {version = "0.13", features = ["prost"]}
```
### 启动 postgres 数据库
```bash
sh ./start_postgres.sh
```

### 引用  sqlx
```bash
cargo add dotenvy -p reservation
cargo add sqlx --features postgres -p reservation
```
### 添加 sqlx 的client
```bash
cargo install sqlx-cli
```
### 创建 .env 进行database 配置
```bash
DATABASE_URL=postgres://postgres:postgres@localhost:5432/reservation
```
### 初始化一个migrate 数据库
```bash
sqlx migrate add init -r
sqlx migrate add reservation -r
sqlx migrate add reservation_trigger -r
sqlx migrate add reservation_func -r
```
### 始始化一个migrate 迁移文件
```bash
sqlx migrate run
```

### 查看数据库
```bash
pgcli -U root -d reservation
```


### 进行 reservation 模块的写法
```bash
# 引用 abi 到 reservation 模块
cargo add abi -p reservation
# 添加 thiserr 通过 deriver 生成
cargo add thiserror -p reservation
```

### 添加 trait 的async 支持
```bash
cargo add async-trait -p reservation
```
### 添加 chrono 的 时间转换库
```bash
cargo add chrono --features serde -p reservation
cargo add chrono --features serde -p abi
```

### 为了跳过sql::query!
