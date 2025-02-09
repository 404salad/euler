ans = 0
mod = 10_000_000_000
for i in range(1,1000+1):
	ans += pow(i,i,mod) # this does modular exponentiation
	ans = ans%mod
print(ans)
