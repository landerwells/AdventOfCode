in="1113122113"
for i in {1..50}
do
  in=$(echo "$in" | fold -w1 | uniq -c | tr '\n' ' ' | tr -d ' ');
done

echo ${#in}
