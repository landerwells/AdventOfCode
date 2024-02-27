#! /bin/bash

in="1113122113"
for i in {1..40} 
do
  in=$(echo "$in" | gfold -w1 | guniq -c | gtr '\n' ' ' | gtr -d ' ');
  echo $in | gtr -d '\n' | gwc -c;
done

