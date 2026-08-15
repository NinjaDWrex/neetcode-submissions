import operator
class Solution:
    def evalRPN(self, tokens: List[str]) -> int:
        stack = []
        ops = {"+" : operator.add, "-": operator.sub, "*" : operator.mul, "/" : lambda a,b : int(a/b)} 
        for i in tokens:
            # add each value to the stack, but when you get to an op, do the op and push that to the stack instead
            if i not in ops:
                stack.append(int(i))
                continue
            # now we can do op because it is an operator
            b = stack.pop()
            a = stack.pop()
            # dont forget div by 0
            if i == "/" and b == 0:
                raise ValueError("Division by 0 error")

                # not quite correct should instead, in rust would use Option<int> return and then instead return None, or coule use Result<int> and return Err("dont divide by 0")
            stack.append(ops[i](a,b))
        return stack[-1]