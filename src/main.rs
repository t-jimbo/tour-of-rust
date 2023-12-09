/*
 * main function
 */
mod binary_search;
mod fizz_buzz;
mod sort;

fn main() {
    fizz_buzz::fizz_buzz(15);

    let found = binary_search::binary_search(3, &[1, 2, 3, 4, 5]);
    println!("{}", if found { "Yes" } else { "No" });

    let found = binary_search::binary_search(7, &[0, 2, 3, 5, 6, 8, 9, 10]);
    println!("{}", if found { "Yes" } else { "No" });

    let arr = &mut [5, 3, 2, 4, 1, 6, 9, 8, 7, 10];
    sort::bubble_sort(arr);
    println!("{:?}", arr);
}
