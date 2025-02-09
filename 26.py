import math

def is_prime(n):
    if n<2:
        return False
    for i in range(2,math.ceil(pow(n,0.5))):
        if n%i == 0:
            return False
    return True

def repeat_remainder(number,divisor):
    """ return whenever the remainder repeats (we get a cycle)"""
    seen = {}
    times = 0
    quotient = ""
    for _ in range(divisor*4): 
        if number in seen:
            return times
        if number<divisor:
            number*=10
        else:
            seen[number] = True
            times += 1
            quotient+=str(number//divisor)
            remainder = number % divisor
            number = remainder
    return -1

#if denominator in terms of 2^n * 5*m
def get_reduced(d):
    while d%2 == 0:
        d = d/2
    while d%5 == 0:
        d = d/5
    return math.floor(d)

ans = 1
cache = {}
for d in range(1,1000):
    # jab same remainder aata hai tab gone
    d = get_reduced(d)
    # if its not prime when reduced then we wont really get max
    if is_prime(d):
        if d in cache:
            continue
        cache[d] = repeat_remainder(1,d)
        if cache[d] == -1:
            print("NOOOOOO")
        ans = max(ans,cache[d])
for k,v in cache.items():
    if v == ans:
        print(k)
