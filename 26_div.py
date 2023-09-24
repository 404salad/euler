def divide(number,divisor):
	quotient = ""
	once = 1
	if number<divisor:
		if once == 1:
			quotient += "."
		once += 1
		number*=10
	else:
		quotient+=str(number//divisor)
		remainder = number % divisor	
		number = remainder
		return (quotient,remainder)
print(divide(1,7))
