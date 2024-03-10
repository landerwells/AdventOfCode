#!/bin/sh

cat day1.txt | tr ',' '\n' | tr -d ' ' | awk '
BEGIN { direction = 0; x = 0; y = 0; }  # Initialize direction (0=North) and coordinates
{
    if ($0 ~ /^R/) {  # Right turn
        direction = (direction + 1) % 4;
        steps = substr($0, 2);
    } else if ($0 ~ /^L/) {  # Left turn
        direction = (direction + 3) % 4;  # Equivalent to subtracting 1, modulo 4
        steps = substr($0, 2);
    }
    # Move in the current direction
    if (direction == 0)      # North
        y += steps;
    else if (direction == 1) # East
        x += steps;
    else if (direction == 2) # South
        y -= steps;
    else if (direction == 3) # West
        x -= steps;
}
END { print x, y; print "Distance:", abs(x) + abs(y); }  # Output final coordinates and Manhattan distance

function abs(v) { return v < 0 ? -v : v; }  # Helper function for absolute value
'

