# %%
from collections import defaultdict
from itertools import combinations, dropwhile

# imperative solution with nested for loops O(n^2)
def twoSum(nums, target):
    for i in range(len(nums)):
        for j in range(i+1, len(nums)):
            if nums[i]+nums[j] == target:
                return [i, j]

# imperative solution with nested for loops using enumerate O(n^2)
def twoSum(nums, target):
    for i,x in enumerate(nums):
        for j,y in enumerate(nums[i+1:]):
            if x+y == target:
                return [i, j+i+1]

# imperative solution with nested for loops and dictionary O(n)            
def twoSum(nums, target):
    res_dict = defaultdict(list)
    for i,x in enumerate(nums):
        res_dict[x].append(i)
    for x,i in res_dict.items():
        if (target-x) == x:
                return res_dict[x][:2]
        if (target-x) in res_dict:
            return [i[0], res_dict[target-x][0]]

# imperative solution with nested for loops and dictionary O(n)
def twoSum(nums, target):
    res_dict = defaultdict(list)
    for i,x in enumerate(nums):
        res_dict[x].append(i)
    for x,i in res_dict.items():
        if (target-x) in res_dict:
            return [i[0], res_dict[target-x][0]]
        
# declarative solution using combinations O(n^2)
def twoSum(nums, target):
    return dict(
        zip(map(sum, combinations(nums, 2)), combinations(range(len(nums)), 2))
    )[target]


if __name__ == "__main__":
    nums = [11, 15, 2, 7]
    target = 9
    print(twoSum(nums, target))
