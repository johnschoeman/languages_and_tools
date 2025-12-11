#!/Users/johnschoeman/.asdf/shims/lua

function fact (n)
  if n == 0 then
    return 1
  else
    return n * fact(n-1)
  end
end

-- print("enter a number:")
-- a = io.read("*number")
-- print(fact(a))

function norm (x, y)
  local n2 = x^2 + y^2
  return math.sqrt(n2)
end

function twice (x)
  return 2*x
end


print(arg[1])
print(arg[2])

-- read 10 lines storing them in a table
    a = {}
    for i=1,10 do
      a[i] = io.read()
    end
-- print the lines
    for i,line in ipairs(a) do
      print(line)
    end
