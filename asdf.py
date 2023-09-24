#use a grid bitch
# x and y axis are exchanged
import random

def make_matrix(n):
    
    lose = []
    lost = []
    for x in range(n):
        for y in range(n):
            lost.append(0)
        lose.append(lost)
        lost = []
    lose[0][0] = 1 
    return lose

gridcollection = []
index = 21
grid3 = make_matrix(index) 

def side(x,y):
    possible=[]
    possiblelist = [(x+1,y),(x,y+1)]
    for a in possiblelist:
        if index not in a:
            possible.append(a)
    if possible!= []:
        return random.choice(possible)
score = 0
count = 0 
for _ in range(10000):
    while True:
        x,y = 0,0
        for z in range(10000):
            a = side(x,y)
            print(x,y)
            if x == index-1 and y == index-1:
                print("-"*15)  
                break
            x,y = a[0],a[1]
            if grid3[x][y]!=1:
                grid3[x][y]=1
                for w in grid3:
                    print(w)
                print("")
                if x == index and y == index:
                    print("-"*15)  
                    break
        break 
    if grid3 not in gridcollection:
        count+=1        
        gridcollection.append(grid3)

    grid3 = make_matrix(index) 
print(count)    
