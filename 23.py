import math
#23
#Find the sum of all the positive integers which cannot be written as the sum of two abundant numbers.

"""
Every multiple of a perfect number (except the perfect number itself) is abundant.[2] For example, every multiple of 6 greater than 6 is abundant because 1 + n 2 + n 3 + n 6 = n + 1. {\displaystyle 1+{\tfrac {n}{2}}+{\tfrac {n}{3}}+{\tfrac {n}{6}}=n+1.}
Every multiple of an abundant number is abundant.[2] For example, every multiple of 20 (including 20 itself) is abundant becaus
"""

def factorise(n):
    factors = []
    for potentialfactor in range(1,(n//2)+1):
        if n % potentialfactor == 0:
            factors.append(potentialfactor)
    return factors

def abundant(n):
    return sum(factorise(n))>n

# totallist = [x for x in range(28123)]
main_number = 28123
is_abun = [False for x in range(main_number+1)]
ans = 0
abun = []

for x in range(1,main_number+1):
    if abundant(x):
        abun.append(x)
        is_abun[x] = True
    lo = 0
    hi = len(abun)-1
    add = True
    while lo<=hi:
        if abun[lo]+abun[hi]==x:
            add = False
            break
        elif abun[lo]+abun[hi]>x:
            hi-=1
        else:
            lo+=1
    if add:
        ans += x
    else:
        print(x)


# 20 and
print(ans)

