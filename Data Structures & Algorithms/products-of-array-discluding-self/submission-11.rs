impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        // left products put it into the result
        let mut result : Vec<i32> = vec![1; nums.len()];
        let mut mul : i32 = 1;
        for (i,val) in nums.iter().enumerate() {
            result[i] = mul;
            mul *= val;
        }
        // now go from right
        mul = 1;
        for (i,val) in nums.iter().rev().enumerate() {
            result[(nums.len() as usize) - 1 - i] *= mul;
            mul *= val; 
        }
        result
    }
}
