pub fn bubble_sort(nums: &mut [i32]) -> &[i32] {
    for i in 0..nums.len() {
        for j in (i + 1)..nums.len() {
            if nums[i] > nums[j] {
                nums.swap(i, j);
            }
        }
    }
    return nums;
}
