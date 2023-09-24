#If all the numbers from 1 to 1000 (one thousand) inclusive were written out in words, how many letters would be used? 
"""
1   - 1
10  - 1
13  - 1
100 - 2
101 - 4
121 - 5
141 - 5
420 - 4
432 - 5
999 - 5
1000- 2
"""

mapU = {1:"one", 2:"two", 3:"three", 4:"four", 5:"five", 6:"six", 7:"seven", 8:"eight",9:"nine",10:"ten",0:"zero"}
mapE = {10:"ten",11:"eleven",12:"twelve",13:"thirteen",14:"fourteen",15:"fifteen",16:"sixteen",17:"seventeen",18:"eighteen",19:"nineteen"}
mapT = {1:"onety", 2:"twenty", 3:"thirty", 4:"forty", 5:"fifty", 6:"sixty", 7:"seventy", 8:"eighty",9:"ninety",0:"zero"}
mapO = {100:"hundred",1000:"thousand",100000:"lakh",100000000:"crore"}

for x in range(1,1001):
    if x in range(1,20):
        if x-10 <= 0:
            print(mapU[x],end = "\n")
        else:
            print(mapE[x],end = "\n")
     
    elif x in range(20,100):
        if x%10 == 0:
            print(mapT[x//10],end = "\n")
        else:
            print(mapT[x//10],end = "")
            print(mapU[x%10],end = "\n")
 
    elif x in range(100,1000):
        if x%100 == 0:
            print(mapU[x//100],end = "")
            print(mapO[100],end = "\n")
        else:
            x = str(x)
            a = int(x[0])
            print(mapU[a],end = "")
            print(mapO[100],end = "")
            print("and",end = "")
            
            a = int(x[1])
            
            if a == 1 and int(x[0]) != 0:
                d = int(x[1::])
                print(mapE[d],end = "\n")
                continue
            elif a != 0:
                print(mapT[a],end = "")
                
            else:
                print("",end = "")
            
            a = int(x[2])
            if a != 0:
                print(mapU[a],end = "\n")
            else:
                print("",end = "\n")
    elif x == 1000:
        print("one",mapO[x],end = "")