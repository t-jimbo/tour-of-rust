fn main() {
    let arr = &mut [5, 3, 2, 4, 1, 6, 9, 8, 7, 10];
    bubble_sort(arr);
    println!("{:?}", arr);

    let arr = &mut [25, 13, 20, 4, 1, 16, 29, 89, 10, 70, 10];
    quick_sort(arr);
    println!("{:?}", arr);
}

fn bubble_sort(nums: &mut [i32]) {
    for i in 0..nums.len() {
        for j in (i + 1)..nums.len() {
            if nums[i] > nums[j] {
                nums.swap(i, j);
            }
        }
    }
}

fn quick_sort(nums: &mut [i32]) {
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
        while nums[right] > p {
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
