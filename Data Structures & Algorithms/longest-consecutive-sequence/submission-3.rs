impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        // use hashset of all values keep track of the highest count values from the lowest each start of subset ie if val - 1 not in the hashset, then count all values that are in the hashset then go from there. do for every number

        let set : HashSet<i32> = nums.into_iter().collect();
        let mut longest = 0;
        for val in set.iter() {
            if !set.contains(&(val-1)){
                //this is a starting value then
                let mut len = 1;
                //start with a count of 1
                while set.contains(&(val + len)){
                    len += 1;
                    //add to the length
                }
                longest = std::cmp::max(longest,len);
            }
        }
        longest
    }
}
