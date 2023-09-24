#Which starting number, under one million, produces the longest chain?
large = 0
def chainlen(n):
    seq = [n]    
    while n!=1:
        if n%2==0:
            n = n//2
            seq.append(n) 
        else:
            n = 3*n + 1
            seq.append(n)
    return len(seq)
for x in range(837799,200000,-1):
    if chainlen(x)>large:
        print(x,chainlen(x))
        large = chainlen(x) 