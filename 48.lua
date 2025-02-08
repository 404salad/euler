local sum = 0
local m = 10 ^ 12
print(m)
for i = 1, 1000 do
	local ii = 1
	for _ = 1, i do
		ii = (ii * i) % m
		print(i, ii)
	end
	sum = (sum + ii) % m
end
--[[
local ting = 3
local thing = 3
for i = 1, 1000 do
	ting = ting * 3
	thing = (thing * 3) % m
	print(i, ting, thing)
end
--]]

print(sum)
