"""


The Fibonacci sequence is defined by the recurrence relation:

    Fn = Fn−1 + Fn−2, where F1 = 1 and F2 = 1.

Hence the first 12 terms will be:

    F1 = 1
    F2 = 1
    F3 = 2
    F4 = 3
    F5 = 5
    F6 = 8
    F7 = 13
    F8 = 21
    F9 = 34
    F10 = 55
    F11 = 89
    F12 = 144

The 12th term, F12, is the first term to contain three digits.

What is the index of the first term in the Fibonacci sequence to contain 1000 digits?


"""
from decimal import Decimal
root5 = (Decimal("2.23606797749978969640917366873127623544061835961152572427089"))
phi = (1 + root5)/2
psi = (1 - root5)/2
n = 10000
a,b = 0,1
for x in range(n):
	a1 = a
	a,b = b,b+a
	generatedNum = (phi**x - psi**x)/root5	
	if len(str(int(generatedNum))) == 1000:
		print('GENERATED',x)
	if len(str(a1)) == 1000:
		print('non functional',x)
