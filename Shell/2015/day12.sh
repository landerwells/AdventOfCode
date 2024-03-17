grep -E -o '[-]?[0-9]*' ../../input/2015/12 | awk '{sum += $1} END {print sum}'
