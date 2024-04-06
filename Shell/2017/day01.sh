#!/usr/bin/env bash

cat ../../inputs/2017/1 | awk '{print $0 substr($0, 1, 1)}' | fold -w1 | uniq -c | awk 'BEGIN {sum = 0 } { if ($1 > 1) sum += ($1 -1) * $2 } END { print sum }'
