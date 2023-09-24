d = 1
n = 1
for x in range(1,21):
    d *= x
for y in range(21,41):
    n *= y
print(n/d)

This is what I find 2 months ago when I solved it:

Each movement in the horizontal is a zero.
Each movement in the vertical is a one.

1st binary# in this series:
0000000000000000000011111111111111111111
last:
1111111111111111111100000000000000000000
For the numbers in between, the amount of
 zeros should be the same as ones. In other
 words, the ones and zeros have to be rearranged.

The total is: 40!/(20!)(20!)
Just use the MS calculator.
137846528820
Best,
Rudy.
