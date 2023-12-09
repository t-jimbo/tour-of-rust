pub fn binary_search(target: i32, nums: &[i32]) -> bool {
    if nums.len() == 0 {
        return false;
    }
    let mid = nums[nums.len() / 2];
    return if target == mid {
        true
    } else if target < mid {
        binary_search(target, &nums[..nums.len() / 2])
    } else {
        binary_search(target, &nums[nums.len() / 2 + 1..])
    };
}
