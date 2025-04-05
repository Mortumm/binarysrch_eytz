### Binary search testbench

Binary search is a common test case algo (Time complexity of O(log n)) for CS students and for anyone getting in to programming.

I happened to read a very interesting article written by Denis Bazhenov in 2023 about Eytzinger Binary Search adjustments to account for architectural predictability. 
This is my implementation of the algo, with the ability to input custom params for benchmarking:

cargo run -- --int-size 5000000 --string-size 200000

Defaults when ran without arguments are there too.
