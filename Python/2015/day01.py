filename = '../../inputs/2015/1'

with open(filename, 'r') as file:
    contents = file.read()

floor = 0
for char in contents:
    if char == '(':
        floor += 1
    else:
        floor -= 1

print(floor)
