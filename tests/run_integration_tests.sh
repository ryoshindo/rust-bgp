#!/bin/bash

docker compose -f ./tests/docker-compose.yml build --no-cache
docker compose -f ./tests/docker-compose.yml up -d

HOST_2_LOOPBACK_IP=10.100.220.3

docker compose -f ./tests/docker-compose.yml exec -T host1 ping -c 5 $HOST_2_LOOPBACK_IP

# docker compose exec の終了コードは実行したコマンドであり、ここでは `ping -c 5 $HOST_2_LOOPBACK_IP` のもの
# そのため、BGP でルートを交換し ping が通れば 0, それ以外の場合は 1 となる

TEST_RESULT=$?
if [ $TEST_RESULT -eq 0 ]; then
    printf "\e[32m%s\e[m\n" "結合テストが成功しました。"
else
    printf "\e[31m%s\e[m\n" "結合テストが失敗しました。"
fi

exit $TEST_RESULT
