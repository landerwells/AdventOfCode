# part one 
expr $(wc -m < ../../input/2015/day8.txt) - $(sed 's/\\\\/A/g; s/\\x../A/g; s/\\"/A/g; s/"//g' ../../input/2015/day8.txt | wc -m)

# part two
expr $(sed 's/\\\\/AAAA/g; s/\\x../AAAAA/g; s/\\\"/AAAA/g; s/"/AAA/g' ../../input/2015/day8.txt | wc -c) - $(wc -c < ../../input/2015/day8.txt)
