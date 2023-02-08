

from functools import reduce
from operator import iconcat
from typing import List

def shuffle(nums: List[int], n: int) -> List[int]:
        return  reduce(iconcat,zip(nums[:len(nums)//2], nums[len(nums)//2:]),[])
    
print(shuffle([2,5,1,3,4,7],3))