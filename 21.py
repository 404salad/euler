#Evaluate the sum of all the amicable numbers under 10000.

def sumOfDivisors(num):
    sum = 0
    for x in range(1,(num//2)+1):
        if num % x == 0:
            sum+=x
    return sum
    
for x in range(10001):
    if x == sumOfDivisors(sumOfDivisors(x)):
        print(x,sumOfDivisors(x),sumOfDivisors(sumOfDivisors(x)))
    