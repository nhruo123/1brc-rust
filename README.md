https://godbolt.org/

Base line result
===============
real    2m35.141s
user    0m0.000s
sys     0m0.015s

My first try (memory mapped file)
============
real    1m7.125s
user    0m0.000s
sys     0m0.000s

Second try (unsafe utf8 read + compile time flags)
============
real    0m40.704s
user    0m0.000s
sys     0m0.000s
Third try (rolling my own basic parser)
===========
real    0m36.119s
user    0m0.000s
sys     0m0.000s
branchless parser not sure if faster but w/e
=========
real    0m35.752s
user    0m0.000s
sys     0m0.000s
better branchless parser
=====
real    0m34.433s
user    0m0.000s
sys     0m0.000s