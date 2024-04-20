use sortings::{quick_sort, selection_sort, insertion_sort, merge_sort};

fn main() {
    let mut numbers = vec![4, 2, 7, 5, 1, 3, 6];

    quick_sort(&mut numbers, |a, b| a.cmp(b));
    println!("Quick Sorted: {:?}", numbers);

    selection_sort(&mut numbers, |a, b| a.cmp(b));
    println!("Selection Sorted: {:?}", numbers);

    insertion_sort(&mut numbers, |a, b| a.cmp(b));
    println!("Insertion Sorted: {:?}", numbers);

    merge_sort(&mut numbers, |a, b| a.cmp(b));
    println!("Merge Sorted: {:?}", numbers);
}
