astart=2 
aend=100
bstart=2
bend=100
sequence = []
sequence1 = []

def fastpower(a,b): #square and multiply algorithm
    value = 1 
    bbin = decimalToBinary(b)
    for x in bbin:
        if x == '0':
            value=value**2
        else:
            value=(value**2)*a
    return value

def decimalToBinary(n):
    return "{0:b}".format(int(n))   

for a in range(astart,aend+1):
    for b in range(bstart,bend+1):
        #element = fastpower(a,b)
        element = a**b
        if element not in sequence:
            sequence.append(element)

sequence = sorted(sequence)
print(len(sequence))
