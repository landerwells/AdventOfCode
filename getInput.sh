#!/bin/bash

YEAR=$1
DAY=$2
SESSION_TOKEN="53616c7465645f5f10694aff43a1ba9dd3b58e9625bd63dfbd5a3b9440344d75a741ad757011c9bfdea4a254d345af978090e73164b8b648737c49204de724d6"  # Replace this with your session token

if [ -z "$YEAR" ] || [ -z "$DAY" ]; then
    echo "Usage: $0 <year> <day>"
    exit 1
fi

# Ensure the input directory exists
mkdir -p input/${YEAR}

# Check if the file already exists
FILE="input/${YEAR}/${DAY}"
if [ -f "$FILE" ]; then
    echo "File $FILE already exists."
    exit 0
fi

# Download input file
curl -s "https://adventofcode.com/${YEAR}/day/${DAY}/input" -H "Cookie: session=${SESSION_TOKEN}" -o "$FILE"
