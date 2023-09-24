#What is the value of the first triangle number to have over five hundred divisors?
import matplotlib.pyplot as plt
n = 1
list1 = []
list2 = []
number = 1
def numberOfDivisors(number):
    amount = 2
    for x in range(2,(number//2)+1):
        if number%x == 0:
            amount +=1
    return amount

while numberOfDivisors(number)<=100:
    list1.append(number)
    list2.append(numberOfDivisors(number))
    number = n * (n+1) //2
    n+=1
plt.xlabel("number")
plt.ylabel("number of divisors")
plt.plot(list1,list2)
plt.show()
