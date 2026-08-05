class Solution:
    def longestConsecutive(self, nums: List[int]) -> int:
        # convert to a set
        numsSet = set(nums)
        longest = 0
        for n in numsSet:
            # check if this is the first position
            if (n - 1) not in numsSet:
                # start counting the number of consecutives
                leng = 1
                while (n+leng) in numsSet:
                    leng+=1
                longest = max(leng,longest)
        return longest


        