#Find the sum of the digits in the number 100!
prod = 1
sum = 0
for x in range(1,100):
    prod = x * prod
for y in str(prod):
    sum += int(y)
print(sum)
