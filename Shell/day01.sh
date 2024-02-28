# Part one
cat input01.txt | fold -w1 | sort | uniq -c | awk '{print $1}'
# need to subract the two numbers given for the answer




