#What is the largest prime factor of the number 600851475143 ? closest prime factor to half
num = 600851475143
#find a factor and divide it as long as it is dividable
#how to find factors
factors = []
for x in range(2,5000):
    if num%x == 0:
        factors.append(x)
for x in factors:
    num = num//x
print("here")
for x in range(num//2,2,-1):
    if num%x == 0:
        num = num//x
    print(num)
