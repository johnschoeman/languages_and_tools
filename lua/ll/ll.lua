#!/Users/johnschoeman/.asdf/shims/lua

function scandir(directory)
    local i, t, popen = 0, {}, io.popen
    local pfile = popen('ls -la "'..directory..'"')
    for filename in pfile:lines() do
        i = i + 1
        t[i] = filename
    end
    pfile:close()
    return t
end


function makeRed(str)
  red = "\27[31m"
  return str .. red
end

function makeBlue(str)
  blue = "\27[34m"
  return str .. purple
end

function makePurple(str)
  purple = "\27[35m"
  return str .. purple
end

-- d: directory
-- c: character device
-- b: block device
-- s: socket
-- p: pipe
-- D: door
-- l: symbolic link
-- -: file

function fileType(line)
  char = string.sub(line, 1, 1)
  return char
end

do
  local t = {
    d = makeBlue,
    l = makePurple,
  }
  function colorByType(fileType, line)
    return t[fileType](line) or line
  end
end

files = scandir("./")

for i,file in ipairs(files) do
  fType = fileType(line)
  print(fType)
  coloredLine = colorByType(fType, file)
  print(coloredLine)
end

