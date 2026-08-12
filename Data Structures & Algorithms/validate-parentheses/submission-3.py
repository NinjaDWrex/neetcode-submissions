class Solution:
    def isValid(self, s: str) -> bool:
        stack = []
        for (i, val) in enumerate(s):
            if val == "[" or val == "(" or val == "{":
                stack.append(val)
            else:
                if len(stack) != 0 and stack[-1] == "[" and val == "]":
                    stack.pop()
                elif len(stack) != 0 and stack[-1] == "{" and val == "}":
                    stack.pop()
                elif len(stack) != 0 and stack[-1] == "(" and val == ")":
                    stack.pop()
                else:
                    return False

        return len(stack) == 0
