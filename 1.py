#If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.
#Find the sum of all the multiples of 3 or 5 below 1000.
sum = 0
x=1
while x<1000:
	if x%15==0:
		sum+=x
	elif x%5==0:
		sum+=x
	elif x%3==0:
		sum+=x
	else:
		pass
	x+=1
print(sum) 

