filename = '../../inputs/2016/3'

with open(filename, 'r') as file:
    contents = file.read()

count = 0

lines = contents.splitlines()
for line in lines:
    nums = line.split()
    nums = [int(x) for x in nums]
    nums = sorted(nums)
    print(nums)
    if (nums[0] + nums[1] > nums[2]):
        count += 1

print(count)


