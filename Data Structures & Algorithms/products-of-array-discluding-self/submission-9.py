class Solution:
    def productExceptSelf(self, nums: List[int]) -> List[int]:
        # create the result and then slide multiplying everything to the left of indices to it
        # inefficient way of writing but to understand more
        
        left_prods = [1] * len(nums)
        left_prod = 1
        right_prods = [1] * len(nums)
        right_prod = 1
        for i, num in enumerate(nums):
            left_prods[i] = left_prod
            left_prod *= nums[i]

        # now do the right products
        for i, num in reversed(list(enumerate(nums))):
            right_prods[i] = right_prod
            right_prod *= nums[i]
        
        return [x*y for x,y in zip(left_prods,right_prods)]
        