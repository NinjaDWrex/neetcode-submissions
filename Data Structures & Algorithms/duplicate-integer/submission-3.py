class Solution:
    def hasDuplicate(self, nums: List[int]) -> bool:
        seen = {}
        for n in nums:
            if(seen.get(n) == True):
                return True
            seen[n] = True
        return False