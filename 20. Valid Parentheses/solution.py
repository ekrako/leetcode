# %%
from functools import reduce


# is_valid  with reduce O(n)
def isValid(s):
    stack = []
    bracket_map = {"(": ")", "[": "]", "{": "}"}

    for char in s:
        if char in bracket_map:
            stack.append(char)
        elif stack and char == bracket_map[stack[-1]]:
            stack.pop()
        else:
            return False
    return not stack


# is_valid  with reduce O(n)
def isValid(s: str) -> bool:
    def check(so_far, new):
        if so_far == False:
            return False
        if new in "({[":
            return new + so_far
        if new in ")}]":
            if len(so_far) == 0:
                return False
            if new == ")" and so_far[0] == "(":
                return so_far[1:]
            if new == "}" and so_far[0] == "{":
                return so_far[1:]
            return so_far[1:] if new == "]" and so_far[0] == "[" else False

    return reduce(check, s) == ""


def isValid(s: str) -> bool:
    bracket_map = {")": "(", "}": "{", "]": "["}

    def check(so_far, new):
        if (
            so_far == False
            or new in ")}]"
            and (not so_far or bracket_map[new] != so_far[0])
        ):
            return False
        return so_far + new if new in "({[" else so_far[:-1]

    return reduce(check, s, "") == ""


def isValid(s: str) -> bool:
    if not s:
        return True

    parent = ["{}", "()", "[]"]

    return next((isValid(s.replace(el, "")) for el in parent if el in s), False)

def isValid(s: str) -> bool:
    while '()' in s or '[]'in s or '{}' in s:
        s = s.replace('()','').replace('[]','').replace('{}','')
    return not s

