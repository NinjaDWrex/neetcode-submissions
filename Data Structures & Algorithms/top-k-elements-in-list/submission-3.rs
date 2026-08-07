use std::collections::HashMap;
impl Solution{
fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let n = nums.len();

    // Step 1: count frequencies
    let mut freq_map: HashMap<i32, usize> = HashMap::new();
    for num in &nums {
        *freq_map.entry(*num).or_insert(0) += 1;
    }

    // Step 2: build buckets, index = frequency
    let mut buckets: Vec<Vec<i32>> = vec![Vec::new(); n + 1];
    for (num, count) in freq_map {
        buckets[count].push(num);
    }

    // Step 3: walk backwards, flatten, take k
    buckets.into_iter().rev().flatten().take(k as usize).collect()
}
}