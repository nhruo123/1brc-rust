1BRC Rust
====
This is my ~~nice and safe~~ rust implementation of the java [1brc](https://github.com/gunnarmorling/1brc)


After all of my changes I think that the biggest way to speed this up is using a custom made hashmap, a linear probe with a very simple hash like FNV, pluse computing the hash as we iterate over the text instead of calling the hashmap `entry` function.

##### runner specs:
- 3.6 GHz with 16 cores
- 16 GB of ram
- slow AF hard drive ( but the test was done with a hot page without it we need like 5 min to read the file of the disk )



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
