# Sorting Library
This library provides various sorting algorithms including quick sort, select sort, insert sort, merge sort.

## Usage

1. Clone the repository:

```bash
git clone https://github.com/musstafina/Blockchain1.git
``` 
2. Navigate to the project directory:
```bash
cd sortings
```
3. Test the code:
```bash
cargo test
```
### Tests
```rust
let mut numbers = vec![4, 2, 7, 5, 1, 3, 6];
quick_sort(&mut numbers, |a, b| a.cmp(b));
assert_eq!(numbers, vec![1, 2, 3, 4, 5, 6, 7]);
```

```rust
let mut numbers = vec![4, 2, 7, 5, 1, 3, 6];
selection_sort(&mut numbers, |a, b| a.cmp(b));
assert_eq!(numbers, vec![1, 2, 3, 4, 5, 6, 7]);
```

```rust
let mut numbers = vec![4, 2, 7, 5, 1, 3, 6];
insertion_sort(&mut numbers, |a, b| a.cmp(b));
assert_eq!(numbers, vec![1, 2, 3, 4, 5, 6, 7]);
```

```rust
let mut numbers = vec![4, 2, 5, 1, 3, 6];
merge_sort(&mut numbers, |a, b| a.cmp(b));
assert_eq!(numbers, vec![1, 2, 3, 4, 5, 6]);
```

### Outputs
![alt text](/image/test.png)


## Example
Navigate to the project directory:
```bash
cd ..
cd example
```
Run the code:
```bash
cargo run
```
### Example #1
![Alt Text](/image/example.png)

### Example #2
The second example also used all types of sorting, but here only the quick sort part is shown.
![Alt Text](/image/example2.png)





