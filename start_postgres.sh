#!/bin/bash

# 配置参数
CONTAINER_NAME=local-postgres
POSTGRES_USER=root
POSTGRES_PASSWORD=admin
POSTGRES_DB=reservation
DATA_DIR=$PWD/data/postgres  # 数据保存的本地目录

# 创建本地数据目录（如果不存在）
mkdir -p "$DATA_DIR"

# 停止并删除已有容器（如果存在）
docker stop $CONTAINER_NAME 2>/dev/null
docker rm $CONTAINER_NAME 2>/dev/null

# 启动 Postgres 容器
docker run -d \
  --name $CONTAINER_NAME \
  -e POSTGRES_USER=$POSTGRES_USER \
  -e POSTGRES_PASSWORD=$POSTGRES_PASSWORD \
  -e POSTGRES_DB=$POSTGRES_DB \
  -v "$DATA_DIR":/var/lib/postgresql/data \
  -p 5432:5432 \
  --restart unless-stopped \
  postgres:15

echo "✅ PostgreSQL 容器已启动："
echo "  用户名: $POSTGRES_USER"
echo "  密码: $POSTGRES_PASSWORD"
echo "  数据库: $POSTGRES_DB"
echo "  本地目录: $DATA_DIR"
