pub fn bubble_sort(nums: &mut [i32]) {
    for i in 0..nums.len() {
        for j in (i + 1)..nums.len() {
            if nums[i] > nums[j] {
                nums.swap(i, j);
            }
        }
    }
}

pub fn quick_sort(nums: &mut [i32]) {
    let len = nums.len();
    if len == 2 {
        if nums[0] > nums[1] {
            nums.swap(0, 1);
        }
        return;
    }
    if len <= 1 {
        return;
    }

    let p = nums[0];
    let mut left = 1;
    let mut right = len - 1;

    while left < right {
        while left < len && nums[left] <= p {
            left += 1;
        }
        while right >= 0 && nums[right] > p {
            right -= 1;
        }
        if left < right {
            nums.swap(left, right);
        }
    }

    nums.swap(0, right);

    quick_sort(&mut nums[..left - 1]);
    quick_sort(&mut nums[left..]);
}
