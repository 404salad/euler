#Find the largest palindrome made from the product of two 3-digit numbers
cand = []
def palindrome(a):
    a = str(a)
    if a == a[::-1]:
        return True
    else:
        return False


for x in range(999,700,-1):
    for y in range(999,700,-1):
        if palindrome(x*y)"
            print(x*y)
        