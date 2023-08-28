#!/usr/bin/env bash

cat $1 | xargs -P32 -I{} bash -c 'curl -X OPTIONS -iLk {} | tee ./sites/headers/{}-headers.txt'