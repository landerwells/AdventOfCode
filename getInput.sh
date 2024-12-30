#!/bin/bash

YEAR=$1
DAY=$2
SESSION_TOKEN="53616c7465645f5fe3e6a120a6261ae43e1a8337131aa149199d36444158e57bb76fcb6bd11f2785679f0f0d57bc61fb0457a31484dd2bc18af2b8cb5732959a"  # Replace this with your session token

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
