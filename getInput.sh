#!/bin/bash

YEAR=$1
DAY=$2
SESSION_TOKEN="53616c7465645f5f109f192fda00e4ac10bfc8d0fc9e8f645d2ba93e7e5650974660b1e4d5414ff38fb4f60b23b63f1721af665b5fe5e5db2ccd8e4175de0752"  # Replace this with your session token

if [ -z "$YEAR" ] || [ -z "$DAY" ]; then
    echo "Usage: $0 <year> <day>"
    exit 1
fi

# Ensure the input directory exists
mkdir -p inputs/${YEAR}

# Check if the file already exists
FILE="inputs/${YEAR}/${DAY}"
if [ -f "$FILE" ]; then
    echo "File $FILE already exists."
    exit 0
fi

# Download input file
curl -s "https://adventofcode.com/${YEAR}/day/${DAY}/input" -H "Cookie: session=${SESSION_TOKEN}" -o "$FILE"
