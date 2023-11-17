#!/bin/bash

arr1=($(seq 1 1 5))
arr2=($(seq 1 1 6))

N=12

# prints 2 0
product_index $N "${#arr1[@]}" "${#arr2[@]}" 

# stores i1=2 i2=0
read -r i1 i2 <<< $(product_index $N "${#arr1[@]}" "${#arr2[@]}")

# echo "arr1 = ${arr1[@]}"
# echo "arr2 = ${arr2[@]}"
# Prints
echo "x,y = ${arr1[$i1]}, ${arr2[$i2]}"
