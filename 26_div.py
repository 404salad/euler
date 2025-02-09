def divide(number,divisor):
    quotient = ""
    once = 1
    for _ in range(divisor*6): 
        # we divide divisor by 6 because 
		# we need min 3 to detect cycles as we dont know where cycle will start
		# it might start max after divisor elements, and will run for max divisor element
		# one more *2 because of the wasted work in else
        if number<divisor:
            if once == 1:
                quotient += "."
            once += 1
            number*=10
        else:
            quotient+=str(number//divisor)
            remainder = number % divisor
            number = remainder
    return quotient
print(divide(1,7))
print(divide(1,8))
print(divide(1,9))
