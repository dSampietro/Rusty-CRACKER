#!/bin/bash

# Initialize variables
total=0
iterations=5

cd target
cd release

# Loop to run the program 5 times
for ((i=1; i<=iterations; i++))
do
    # Run your program and capture the output
    result=$(./main.exe -f ../../files/bio-HS-CX.mtx)
    
    # Add the result to the total
    total=$((total + result))
done

# Calculate the average
average=$((total / iterations))

# Output the average
echo "The average is: $average