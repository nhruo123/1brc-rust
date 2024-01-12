1BRC Rust
====
This is my ~~nice and safe~~ rust implementation of the java [1brc](https://github.com/gunnarmorling/1brc)


After all of my changes I think that the biggest way to speed this up is using a custom made hashmap, a linear probe with a very simple hash like FNV, and while iterating over the string compute the hash at the parsing loop instead of the hashmap `entry` function.


### Base line result
real    2m35.141s
user    0m0.000s
sys     0m0.015s

### My first try (memory mapped file)
real    1m7.125s
user    0m0.000s
sys     0m0.000s

### Second try (unsafe utf8 read + compile time flags)
real    0m40.704s
user    0m0.000s
sys     0m0.000s
### Third try (rolling my own basic parser)
real    0m36.119s
user    0m0.000s
sys     0m0.000s
### branchless parser not sure if faster but w/e
real    0m35.752s
user    0m0.000s
sys     0m0.000s
### better branchless parser
real    0m34.433s
user    0m0.000s
sys     0m0.000s
### removed some bounds check
real    0m33.155s
user    0m0.000s
sys     0m0.000s
### added basic multi-threading
real    0m4.718s
user    0m0.000s
sys     0m0.000s
### reserve memory in hashmap
real    0m4.531s
user    0m0.000s
sys     0m0.000s