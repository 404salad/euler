#By considering the terms in the Fibonacci sequence whose values do not exceed four million, find the sum of the even-valued terms.0112358
a = [0,1]
sum = 0
x = 0 
t = 0
while t<4000000:
	t = a[x] + a[x+1]
	a.append(t)
	x+=1	
for y in a:
	if y%2==0:
		sum+=y
	
print(sum)