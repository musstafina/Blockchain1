use sortings::{quick_sort, selection_sort, insertion_sort, merge_sort};

// fn main() {
//     let mut numbers = vec![4, 2, 7, 5, 1, 3, 6];

//     quick_sort(&mut numbers, |a, b| a.cmp(b));
//     println!("Quick Sorted: {:?}", numbers);

//     selection_sort(&mut numbers, |a, b| a.cmp(b));
//     println!("Selection Sorted: {:?}", numbers);

//     insertion_sort(&mut numbers, |a, b| a.cmp(b));
//     println!("Insertion Sorted: {:?}", numbers);

//     merge_sort(&mut numbers, |a, b| a.cmp(b));
//     println!("Merge Sorted: {:?}", numbers);
// }

use std::cmp::Ordering;

#[derive(Debug, Clone)]
struct Product {
    name: String,
    price: f64,
}

fn compare_by_price(product1: &Product, product2: &Product) -> Ordering {
    product1.price.partial_cmp(&product2.price).unwrap_or(Ordering::Equal)
}

fn main() {
    let mut products = vec![
        Product { name: String::from("Laptop"), price: 1200.0 },
        Product { name: String::from("Smartphone"), price: 800.0 },
        Product { name: String::from("Headphones"), price: 100.0 },
        Product { name: String::from("Tablet"), price: 500.0 },
    ];

    quick_sort(&mut products, compare_by_price);
    println!("After quick sort by price:");
    for product in &products {
        println!("{:?}", product);
    }
    println!();

    selection_sort(&mut products, compare_by_price);
    println!("After selection sort by price:");
    for product in &products {
        println!("{:?}", product);
    }
    println!();

    insertion_sort(&mut products, compare_by_price);
    println!("After insertion sort by price:");
    for product in &products {
        println!("{:?}", product);
    }
    println!();

    merge_sort(&mut products, compare_by_price);
    println!("After merge sort by price:");
    for product in &products {
        println!("{:?}", product);
    }
}
