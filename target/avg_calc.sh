#!/bin/bash

# Array to store the times in milliseconds
files=("example.mtx" "bio-diseasome.mtx" "soc-wiki-vote.mtx" "aves-wildbird.mtx" "bio-CE-GN.mtx" "bio-HS-CX.mtx")

for f in files; do
  times=()
  
  # Run the program 5 times
  for i in {1..5}; do
    # Get the time from the program
    time_output=$(./main --f ../../files/$f)
    echo "Run $i: $time_output"
    
    times+=($time_output)
  done

  # Calculate the average time in milliseconds
  total=0
  for t in "${times[@]}"; do
    total=$((total + t))
  done
  average=$((total / 5))

  # Output the average time in milliseconds
  echo $f "Average time: $average ms"
done