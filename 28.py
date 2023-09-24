n = 500 
answer = 8*n*(n+1)*(2*n+1)*(1/3)+2*n*(n+1)+4*n+1
print(answer)
# found sum of each level of diagonal and used sigma n formula
'''
FOR THE THING
from pandas import *
#pandas for pretty printing
matrix5 = [
        [0,0,0,0,0],
        [0,0,0,0,0],
        [0,0,0,0,0],
        [0,0,0,0,0],
        [0,0,0,0,0]
        ]; 
length = 5 
matrix = []
for _z in range(length):
    matrixrow = []
    for _zz in range(length):
        matrixrow.append(0)
    matrix.append(matrixrow)
def display(matrix):
    print(DataFrame(matrix))
    //print("\n")
    
def patternorder(matrix):
    length = len(matrix)
    x = length//2 
    y = length//2
    matrix[y][x]=1
    display(matrix)
    upcounter =2 
    for count in range(length-1):
        count+=1
        print(count)

        if count % 2 == 0:
            operation = 'm'
        else:
            operation = 'p'

        for innerx in range(count):
            if operation == 'p':
                x = x+1
                matrix[y][x]=upcounter
                upcounter += 1
                display(matrix)
            else:
                x = x-1
                matrix[y][x]=upcounter
                upcounter += 1
                display(matrix)
        for innery in range(count):
            if operation == 'p':
                y = y+1
                matrix[y][x]=upcounter
                upcounter += 1
                display(matrix)
            else:
                y = y-1
                matrix[y][x]=upcounter
                upcounter += 1
                display(matrix)
    for x in range(length-1):
        matrix[0][x+1] = upcounter 
        upcounter+=1
        display(matrix)

patternorder(matrix)
center 
one right x+1 x+1 o
one down  y+1 y+1 o

two left  x-2 x-1 o
              x-1 o
two up    y-2 
              y-1 o 
              y-1 o

three right x+3
              x+1
              x+1
              x+1
three down y+3
four right
four down
five left
five up
recur(matrix,0,length)

in the thing

for 9
top right is n^2 
bottom right is n^2 - 6 * (n-1) 
bottom left n^2 - 4*(n-1)
top left n^2 - 2*(n-1)


total diagonal 4*n^2 -12*(n-1)
sigma 1 se n//2 
4*(n^2 -3*n - 3)
4*(n(n+1)/2 - 3 * (
'''
