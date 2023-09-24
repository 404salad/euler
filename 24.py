#24 lex goo 
"""
A permutation is an ordered arrangement of objects. For example, 3124 is one possible permutation of the digits 1, 2, 3 and 4. If all of the permutations are listed numerically or alphabetically, we call it lexicographic order. The lexicographic permutations of 0, 1 and 2 are:

012   021   102   120   201   210

What is the millionth lexicographic permutation of the digits 0, 1, 2, 3, 4, 5, 6, 7, 8 and 9?

"""

# LMAOOON this ALL WRONG as repeats are there, just make a function to check if repeats are there if not then 
# basically just iterate using the number 001 002 003 like that and see if there are repeats, if there are ignore them
# no need of nesting loop
"""
import re
i = 1234567890
count = 1
while count!=1000000:
    tocomp = str(i)
    if not re.search(r"(.).*\1+",tocomp):
        count+=1
        print(i)
    i+=1
print("aa")
"""
#24 lex goo 
"""
A permutation is an ordered arrangement of objects. For example, 3124 is one possible permutation of the digits 1, 2, 3 and 4. If all of the permutations are listed numerically or alphabetically, we call it lexicographic order. The lexicographic permutations of 0, 1 and 2 are:

012   021   102   120   201   210

What is the millionth lexicographic permutation of the digits 0, 1, 2, 3, 4, 5, 6, 7, 8 and 9?

"""

# LMAOOON this ALL WRONG as repeats are there, just make a function to check if repeats are there if not then 
# basically just iterate using the number 001 002 003 like that and see if there are repeats, if there are ignore them
# no need of nesting loop
'''
import re
i = 2013456789
count = 0
while count !=  274240:
    tocomp = str(i)
    if not re.search(r"(.).*\1+",tocomp):
        count+=1
        print(count,i)
    i+=1
print("aa")
'''
import re
i = 2013456789
count = 0
while count !=  274240:
    tocomp = str(i)
    if tocomp[2:-1]=="98765430":
        i+=12345670
    elif tocomp[3:-1]=="9876540"
        i+=123460
    elif tocomp[4:-1]=="98765
    if not re.search(r"(.).*\1+",tocomp):
        count+=1
        print(count,i)
    i+=1
print("aa")
##3895047621
##3895061247
#3895061274
#3895061427
##3895047621
##3895061247
#3895061274
#3895061427

#274240