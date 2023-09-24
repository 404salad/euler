#23
#Find the sum of all the positive integers which cannot be written as the sum of two abundant numbers.

#CODE IS CORRECT NEED TO REDUCE COMPUTATION TIME
# one method is that all products of abundant numbers are abundant
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
    if sum(factorise(n))>n:
        returner = True
    else:
        returner = False
    return returner

totallist = [x for x in range(28123)]
mainnumber = 28123
abundantlist = []

for x in range(1,mainnumber+1):
    if abundant(x):
        abundantlist.append(x)
   

sumofabundant = []
# PRODUCT OF ALL ABUNDANT NUMBERS ARE ABUNDANT
for x in abundantlist:
    for y in abundantlist[0:(len(abundantlist)//2)]:
        try:
            totallist.remove(x+y)
        except Exception:
            pass
print(sum(totallist))
        
"""
def summable(n):
    for i in abundantlist:
        for j in abundantlist:
            if i+j == n:
                return True
    return False
    

finalsum = 0

for x in range(mainnumber):
    if not summable(x):
        print(x)
        finalsum += x
print(finalsum)
"""
