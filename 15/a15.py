
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
