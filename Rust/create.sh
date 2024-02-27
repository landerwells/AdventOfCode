#!/bin/sh

# The script expects two arguments: the year and the day number
year=$1
day_number=$2

# Use printf to format the day number with leading zeros
formatted_day=$(printf "day%02d" "$day_number")

# Define the file path for the destination file
destination_file="src/aoc$1/$formatted_day.rs"

# Check if the file already exists
if [ -f "$destination_file" ]; then
    echo "File '$destination_file' already exists."
else
    # If the file doesn't exist, copy the template file into the correct year and name it the correct day,
    # and modify the contents to match

    mkdir -p "src/aoc$year"
    cp src/template.rs "$destination_file"

    # Modify the contents of the copied file
    sed -i '' "s/get_daily_input()/get_daily_input($day_number, $year)/" "$destination_file"

    # Append the new module to the mod.rs file
    mod_file="src/aoc$year/mod.rs"

    # Add new mod declaration
    echo "pub mod $formatted_day;" >> "$mod_file"

    # Add the day's run function call in the run_all function.
    # We can use sed to do this.
    sed -i '' "/run_all()/ a\\
    $formatted_day::run();\\
    " "$mod_file"
fi
