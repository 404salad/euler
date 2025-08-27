import math

def is_square(i: int) -> bool:
    return i == math.isqrt(i) ** 2

def fn(b,r):
	return b/(b+r) * (b-1)/(b+r-1)

for b in range(int(1e12),int(1e13)):
	# binary search
	for r in range(100000000,b):
		print(b,r,fn(b,r))
"""
for n in range(int(1e12),int(1e13)):
	ting = 1 + 2*pow(n,2) - 2*n
	if is_square(ting):
		print((1-pow(ting,0.5))/2)
		print((1+pow(ting,0.5))/2)
		break
"""
