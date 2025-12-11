#!/Users/johnschoeman/.asdf/shims/lua

-- P :: set of primes, 1 to P
-- N :: set of powers, 0 to N
-- total size : |N|*|P|
--
-- Prime Notation : 
-- [7's place].[5's place].[3's place].[2's place]
-- 1.0.1.3 :: 7^1 * 5^0 * 3^1 * 2^3 == 168

-- p : { 2 } ; n : { 0 }
-- 0
-- 1
--
-- p : { 2 } ; n : { 0, 1 }
-- 0.1
-- 1.1
--
-- p : { 2, 3 } ; n : { 0, 1 }
-- 0.0
-- 1.0
-- 0.1
-- 1.1
--
-- p : { 2, 3 } ; n : { 0, 1, 2 }  
--
-- 0.0
-- 1.0
-- 2.0
-- 0.1
-- 1.1
-- 2.1
-- 0.2
-- 1.2
-- 2.2
--
-- 0.0
-- 0.1
-- 0.2
-- 1.0
-- 1.1
-- 1.2
-- 2.0
-- 2.1
-- 2.2
--
-- p : { 2, 3, 5 } ; n : { 0, 1, 2 }
-- 0.0.0 0.0.1 0.0.2
-- 1.0.0 1.0.1 1.0.2
-- 2.0.0 2.0.1 2.0.2
-- 0.1.0 0.1.1 0.1.2
-- 1.1.0 1.1.1 1.1.2
-- 2.1.0 2.1.1 2.1.2
-- 0.2.0 0.2.1 0.2.2
-- 1.2.0 1.2.1 1.2.2
-- 3.2.0 3.2.1 3.2.2


function build_powers_of_two(n)
  next = {}
  for i=0,n do
    key = tostring(i)
    next[key] = 2^i
  end
  return next
end

function build_composites_from(p_idx, n)
  if (p_idx <= 1) then
    return build_powers_of_two(n)
  end

  print(p_idx)
  prev_set = build_composites_from(p_idx - 1, n)
  next_set = {}
  prime = nth_prime(p_idx)
  for i=0,n do
    for k in pairs(prev_set) do
      next_key = tostring(i) .. "." .. k
      next_value = prev_set[k] * (prime^i)
      next_set[next_key] =  next_value
    end
  end
  return next_set
end

primes = {2,3,5,7,11,13,17,19}

function nth_prime(p_idx)
  return primes[p_idx]
end

function build_prime_set(p)
  if (p == 2) then
    return { 2 }
  elseif (p == 3) then
    return { 2, 3 }
  end
end

function print_numbers(number_table)
  for k in pairs(number_table) do
    print(k .. " : " .. number_table[k])
  end
end

function table.copy(t)
  local t2 = {}
  for k,v in pairs(t) do
    t2[k] = v
  end
  return t2
end

-- comps = build_composites_from(4,4)
-- print_numbers(comps)

-- order number of primes in number 1.4 -> 2; 3.3.3 -> 3
function suc(pn)
  order = len(pn)
  max_power = max(pn)

  if (order > max_power) then
    return increment_power(pn, order - 1)
  else
    return increment_prime(pn, order)
  end
end

-- 1.1.0.2 -> 1.1.0.3
-- 1.3.3.3 -> 3.0.0.0
function increment_power(pn, max_power)
  first_non_max_power = find_last_non_max_power(pn, max_power)
  if first_non_max_power == nil then
    next = {}
    for i=0, max_power + 1 do
      if i == max_power + 1 then
        next[i] = max_power + 1
      else
        next[i] = 0
      end
    end
    return next
  end

  next = table.copy(pn)
  for k,v in pairs(pn) do
    if k == first_non_max_power then
      next[k] = next[k] + 1
    elseif k > first_non_max_power then
      next[k] = 0
    end
  end
  return next
end

-- 1.2.2.3 -> 3
-- 3.3.3.3 -> nil
function find_last_non_max_power(pn, max_power)
  t = {}
  idx = 0
  for k, v in pairs(pn) do
    if v ~= max_power then
      idx = k
      t[k] = v
    end
  end
  if len(t) > 0 then
    return idx
  else
    return nil
  end
end

-- 0.4.1.4 -> 0.4.2.4
-- 0.1.4.4 -> 0.2.0.4
function increment_prime(pn, power)
  decimal = to_decimal(pn)
  next = decimal + 1
  while true do
    if has_power(next, power) and has_no_power_greater(next, power) then
      return from_decimal(next, power)
    end
    next = next + 1
  end
end

function has_power(dec, power)
  str = tostring(dec)
  r = string.find(str, tostring(power))
  return r
end

function has_no_power_greater(dec, power)
  str = tostring(dec)
  for c in str:gmatch"." do
    if tonumber(c) > power then
      return false
    end
  end
  return true
end

-- "4231" -> {4,2,3,1}
function from_decimal(dec, order)
  str = string.format("%0" .. order .. "d", tostring(dec))
  t = {}
  for c in str:gmatch"." do
    table.insert(t, tonumber(c))
  end
  return t
end

-- {4,2,3,1} -> 4321
function to_decimal(pn)
  r = ""
  for k, v in pairs(pn) do
    r = r .. v
  end
  return tonumber(r)
end

function len(pn)
  i = 0
  for k in pairs(pn) do
    i = i + 1
  end
  return i
end

function max(pn)
  m = 0
  for k, v in pairs(pn) do
    if v > m then
      m = v
    end
  end
  return m
end

function show(pn)
  result = ""
  for k, v in ipairs(pn) do
    result = result .. "." .. v
  end
  return string.sub(result, 2)
end


arr = {0,0,3}
s = suc(arr)
print(show(s))
-- print(show(s))

function foo(pn)
  for k,v in ipairs(pn) do
    print(k, v)
  end
end

n = {0}

t = {}

for i=0, 10 do
  foo = table.copy(n)
  n = suc(foo)
  print(show(foo))
end

for k,v in pairs(t) do
  print(k, v)
end

-- iterations --
-- i : p,n : size
-- 0 : 1,0 : 1
-- 1 : 1,1 : 2
-- 2 : 2,1 : 4
-- 3 : 2,2 : 9
-- 4 : 3,2 : 
-- 5 : 3,3
-- 6 : 4,3
-- 7 : 4,4
-- 8 : 5,4
-- 9 : 5,5
--
-- n odd,  (n+1)/2,(n+1)/2
-- n even, (n+2)/2,n/2

-- pp sets
-- 1,0 : 0.0.0 : 1
--
-- 1,1 : 0.0.1 : 2
--
-- 2,1 : 0.1.0 : 3
-- 2,1 : 0.1.1 : 6
-- 2/6
--
-- 2,2 : 0.0.2 : 4
-- 2,2 : 0.1.2 : 12
-- 2,2 : 0.2.0 : 9
-- 2,2 : 0.2.1 : 18
-- 2,2 : 0.2.2 : 36
-- 27/36
--
-- 3,2 : 1.0.0 : 5
-- 3,2 : 1.0.1 : 10
-- 3,2 : 1.0.2 : 20
-- 3,2 : 1.1.0 : 15
-- 3,2 : 1.1.1 : 30
-- 3,2 : 1.1.2 : 60
-- 3,2 : 1.2.0 : 45
-- 3,2 : 1.2.1 : 90
-- 3,2 : 1.2.2 : 180
-- 3,2 : 2.0.0 : 25
-- 3,2 : 2.0.1 : 50
-- 3,2 : 2.0.2 : 100
-- 3,2 : 2.1.0 : 75
-- 3,2 : 2.1.1 : 150
-- 3,2 : 2.1.2 : 300
-- 3,2 : 2.2.0 : 225
-- 3,2 : 2.2.1 : 450
-- 3,2 : 2.2.2 : 900
-- |p|^n/(p^n)
--
-- 3,3 : 0.0.3 : 8
-- 3,3 : 0.1.3 : 24
-- 3,3 : 0.2.3 : 72
-- 3,3 : 0.3.0 : 27
-- 3,3 : 0.3.1 : 54
-- 3,3 : 0.3.2 : 108
-- 3,3 : 0.3.3 : 216
-- 3,3 : 1.0.3 : 40
-- 3,3 : 1.1.3 : 120
-- 3,3 : 1.2.3 : 360
-- 3,3 : 1.3.0 : 135
-- 3,3 : 1.3.1 : 270
-- 3,3 : 1.3.2 : 540
-- 3,3 : 1.3.3 : 1080
-- 3,3 : 2.0.3 : 200
-- 3,3 : 2.1.3 : 600
-- 3,3 : 2.2.3 : 1800
-- 3,3 : 2.3.0 : 675
-- 3,3 : 2.3.1 : 1350
-- 3,3 : 2.3.2 : 2700
-- 3,3 : 2.3.3 : 5400
-- 3,3 : 3.0.0 : 125
-- 3,3 : 3.0.1 : 250
-- 3,3 : 3.0.2 : 500
-- 3,3 : 3.0.3 : 1000
-- 3,3 : 3.1.0 : 375
-- 3,3 : 3.1.1 : 750
-- 3,3 : 3.1.2 : 1500
-- 3,3 : 3.1.3 : 3000
-- 3,3 : 3.2.0 : 1125
-- 3,3 : 3.2.1 : 2250
-- 3,3 : 3.2.2 : 4500
-- 3,3 : 3.2.3 : 9000
-- 3,3 : 3.3.0 : 3375
-- 3,3 : 3.3.1 : 6750
-- 3,3 : 3.3.2 : 13500
-- 3,3 : 3.3.3 : 27000
--
-- 4,3 : 1.0.0.0 : 7
-- 4,3 : 1.0.0.1 : 14
-- 4,3 : 1.0.0.2 : 28
-- 4,3 : 1.0.0.3 : 46
-- 4,3 : 1.0.1.0 : 21
-- 4,3 : 1.0.1.1 : xx
-- 4,3 : 1.0.1.2 : xx
-- 4,3 : 1.0.1.3 : xx
-- 4,3 : 1.0.2.0 : xx
-- 4,3 : 1.0.2.1 : xx
-- 4,3 : 1.0.2.2 : xx
-- 4,3 : 1.0.2.3 : xx
-- 4,3 : 1.0.3.0 : xx
-- 4,3 : 1.0.3.1 : xx
-- 4,3 : 1.0.3.2 : xx
-- 4,3 : 1.0.3.3 : xx
-- 4,3 : 1.1.0.0 : 7
-- 4,3 : 1.1.0.1 : 14
-- 4,3 : 1.1.0.2 : 28
-- 4,3 : 1.1.0.3 : 46
-- 4,3 : 1.1.1.0 : 21
-- 4,3 : 1.1.1.1 : xx
-- 4,3 : 1.1.1.2 : xx
-- 4,3 : 1.1.1.3 : xx
-- 4,3 : 1.1.2.0 : xx
-- 4,3 : 1.1.2.1 : xx
-- 4,3 : 1.1.2.2 : xx
-- 4,3 : 1.1.2.3 : xx
-- 4,3 : 1.1.3.0 : xx
-- 4,3 : 1.1.3.1 : xx
-- 4,3 : 1.1.3.2 : xx
-- 4,3 : 1.1.3.3 : xx
-- 4,3 : 1.2.0.0 : 7
-- 4,3 : 1.2.0.1 : 14
-- 4,3 : 1.2.0.2 : 28
-- 4,3 : 1.2.0.3 : 46
-- 4,3 : 1.2.1.0 : 21
-- 4,3 : 1.2.1.1 : xx
-- 4,3 : 1.2.1.2 : xx
-- 4,3 : 1.2.1.3 : xx
-- 4,3 : 1.2.2.0 : xx
-- 4,3 : 1.2.2.1 : xx
-- 4,3 : 1.2.2.2 : xx
-- 4,3 : 1.2.2.3 : xx
-- 4,3 : 1.2.3.0 : xx
-- 4,3 : 1.2.3.1 : xx
-- 4,3 : 1.2.3.2 : xx
-- 4,3 : 1.2.3.3 : xx
-- 4,3 : 1.3.0.0 : 7
-- 4,3 : 1.3.0.1 : 14
-- 4,3 : 1.3.0.2 : 28
-- 4,3 : 1.3.0.3 : 46
-- 4,3 : 1.3.1.0 : 21
-- 4,3 : 1.3.1.1 : xx
-- 4,3 : 1.3.1.2 : xx
-- 4,3 : 1.3.1.3 : xx
-- 4,3 : 1.3.2.0 : xx
-- 4,3 : 1.3.2.1 : xx
-- 4,3 : 1.3.2.2 : xx
-- 4,3 : 1.3.2.3 : xx
-- 4,3 : 1.3.3.0 : xx
-- 4,3 : 1.3.3.1 : xx
-- 4,3 : 1.3.3.2 : xx
-- 4,3 : 1.3.3.3 : xx
-- 4,3 : 2.0.0.0 : 49
-- 4,3 : 3.3.3.3 : 9261000
--
-- 4,4 : 0.0.0.4 : 16
-- 4,4 : 0.0.1.4 : xx
-- 4,4 : 0.0.2.4 : xx
-- 4,4 : 0.0.3.4 : xx
-- 4,4 : 0.0.4.0 : xx
-- 4,4 : 0.0.4.1 : xx
-- 4,4 : 0.0.4.2 : xx
-- 4,4 : 0.0.4.3 : xx
-- 4,4 : 0.0.4.4 : xx
-- 4,4 : 0.1.0.4 : xx
-- 4,4 : 0.1.1.4 : xx
-- 4,4 : 0.1.2.4 : xx
-- 4,4 : 0.1.3.4 : xx
-- 4,4 : 0.1.4.0 : xx
-- 4,4 : 0.1.4.1 : xx
-- 4,4 : 0.1.4.2 : xx
-- 4,4 : 0.1.4.3 : xx
-- 4,4 : 0.1.4.4 : xx
-- 4,4 : 0.2.0.4 : xx
--
-- 4,4 : 4.4.4.4 : 1944810000
--
--


function fromString(str)
  local t = {}
  local i1 = 0
  local j1 = 0
  local i2 = 0
  local j2 = 0
  while true do
    i2 = string.find(str, "\\.", i1 + 1)
    if i2 == nil then
      next = string.sub(str,i1+1)
      table.insert(t, next)
      break
    end
    next = string.sub(str,i1+1,i2-1)
    table.insert(t, next)
    i1 = i2
  end
  return t
end
