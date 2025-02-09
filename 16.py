#16
#from plyer import notification
def sumlistor(num):
    sum = 0
    for x in str(num):
        sum += int(x)
    return sum
a=1
for x in range(1000):
    sum = 0 
    a = 2 * a    
    print(sumlistor(a))
"""
notification.notify(
title = "you live i",
message = "adf",
timeout = 10)
"""
