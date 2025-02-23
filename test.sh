#!/bin/bash

# `cargo run` でサーバーをバックグラウンドで起動
cargo run &
SERVER_PID=$!
echo "==================================================================="
# サーバーが起動するのを待つ (3秒)
sleep 3

# `curl` でリクエストを送信
echo "Sending request to http://127.0.0.1:3000/"
curl -i http://127.0.0.1:3000/

echo "==================================================================="
# 実行後の処理
echo "capnp_client executed"
./capnp_client

echo "==============================="
# 結果待ち
sleep 3

# サーバープロセスを停止
kill $SERVER_PID