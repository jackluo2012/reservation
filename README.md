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
cargo generate --git https://github.com/tyrchen/rust-lib-template
```

### 生成项目的 abi grpc ,生成的单独的代码，放到一个 crate 里面,生成各种各样的 trait

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

### 引用 sqlx

```bash
cargo add dotenvy -p reservation
cargo add sqlx --features postgres -p reservation
```

### 添加 sqlx 的 client

```bash
cargo install sqlx-cli
```

### 创建 .env 进行 database 配置

```bash
DATABASE_URL=postgres://postgres:postgres@localhost:5432/reservation
```

### 初始化一个 migrate 数据库

```bash
sqlx migrate add init -r
sqlx migrate add reservation -r
sqlx migrate add reservation_trigger -r
sqlx migrate add reservation_func -r
```

### 始始化一个 migrate 迁移文件

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

### 添加 trait 的 async 支持

```bash
cargo add async-trait -p reservation
```

### 添加 chrono 的 时间转换库

```bash
cargo add chrono --features serde -p reservation
cargo add chrono --features serde -p abi
```

### 为了跳过 sql::query!

因为我用的不是 public schema 这个用宏定义，还没有找到原因，先不用宏，感觉应该是 schema_dsl 宏定义的问题，所以要带上 schema_dsl,rsvp.reservation 这样的数据库表名。

````bash

``

### 添加 sqlx 测试，提供 tokio 运行环境

```bash
cargo add tokio --features full -p reservation --dev
````

### 的 abi 下面创建 type 目录

```bash
mkdir abi/src/types
```

### 进行 测试

```bash
cargo nextest run
cargo nextest run --nocapture
```

### 添加 builder 构建器模式，crate

```bash
cargo add derive_builder -p abi
```

### 引用 prost-types

```bash
cargo add prost-types -p reservation --dev
```

### 给 reservation-service 包 添加 tonic 服务

```bash
cargo add tonic -p reservation-service
```
