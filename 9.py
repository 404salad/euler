#There exists exactly one Pythagorean triplet for which a + b + c = 1000.
#Find the product abc.
c = 0
while c<999:
    for a in range(998):
        for b in range(988):
            if (a**2 + b**2)**0.5 == (1000 - a - b):
                print(a*b*(1000-a-b))
            