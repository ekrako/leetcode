from itertools import accumulate


def maxDepth(s: str) -> int:
    return max(accumulate(map(lambda x: 1 if x == "(" else -1 if x == ")" else 0, s)))


def maxDepth(s: str) -> int:
    max_depth = 0
    depth = 0
    for x in s:
        if x == "(":
            depth += 1
            if depth > max_depth:
                max_depth = depth
        elif x == ")":
            depth -= 1
    return max_depth


def maxDepth(s: str) -> int:
    ints = [-1 if x == ")" else 1 if x == "(" else 0 for x in s]
    s = 0
    depths = [s := s + x for x in ints]
    return max(depths)


s = "(1+(2*3)+((8)/4))+1"
a = accumulate(map(lambda x: 1 if x == "(" else -1 if x == ")" else 0, s))
print(a)
print(maxDepth(s))
