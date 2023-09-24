#What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?
box = [f for f in range(2,21)]
e = []
for x in box:
    for y in [2,3,5,7,11,13,17,19]:
        if x % y == 0:
            while x % y == 0:
                e.append(y)
                x = x//y
    e.append("|")
print(e)
print(2**4 * 3**2 * 5 * 7 * 11 *13*17*19)
#just make list of list and then count individual factors mutliply the max ones