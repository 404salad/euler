#What is the 10001st prime number?
list = [2,3]
def prime(a):
    i = 0
    for x in range(2,int(a**0.5)+1):
        if a % x == 0:
            i+=1
    if i == 0:
        return True
    else:
        return False

for n in range(1,30001):
    if prime(6*n-1):
        list.append(6*n-1)
    if prime(6*n+1):
        list.append(6*n+1)
print(list[10000])
