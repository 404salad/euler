#22
"""
Using names.txt (right click and 'Save Link/Target As...'), a 46K text file containing over five-thousand first names, begin by sorting it into alphabetical order. Then working out the alphabetical value for each name, multiply this value by its alphabetical position in the list to obtain a name score.
For example, when the list is sorted into alphabetical order, COLIN, which is worth 3 + 15 + 12 + 9 + 14 = 53, is the 938th name in the list. So, COLIN would obtain a score of 938 × 53 = 49714.
What is the total of all the name scores in the file?
"""

# SORTING NAMES
file = open('p022_names.txt')
names1 = file.read()
file.close()
names1 = names1.split(',')
names = []

for name in names1:
    names.append(name.strip("\""))

names = sorted(names)

def alphaval(name):
    sum = 0
    for x in name:
        sum += (ord(x)-64)
    return sum

i = 0
finalsum = 0
for name in names:
    i+=1
    finalsum += i * alphaval(name)
print(finalsum)
"""
  
i = 0 
for x in range(200): 
    try:
        if shouldexchange(names[i],names[i+1]):
            names[i],names[i+1] = names[i+1],names[i]
    except Exception:
        pass
    i+=1
print(names)
"""