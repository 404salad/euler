cache = {}

def countRoutes(m,n):
    if n == 0 or m == 0:
        return 1
    try:
        if cache[(m,n)]:
            return cache[(m,n)]
    except Exception:
        pass
    cache[(m,n)] = countRoutes(m-1,n) + countRoutes(m,n-1) 
    return cache[(m,n)]
print(countRoutes(20,20))


