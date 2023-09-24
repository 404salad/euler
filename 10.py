#Find the sum of all the primes below two million.
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
n=1     
while 6*n+1 < 2000001:
    if prime(6*n-1) and n!=0:    
        list.append(6*n-1)
    if prime(6*n+1):
        list.append(6*n+1)
    n+=1
print(sum(list))