filename = "0089_roman.txt";
with open(filename) as file:
    print("[",end ="")
    for x in range(1000):
        print('"',end = "")
        print(file.readline().rstrip(),end = '",')
    print("]", end = "")
