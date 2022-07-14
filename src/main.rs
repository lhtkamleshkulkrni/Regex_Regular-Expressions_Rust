fn main() {
    // let v1 = vec![1, 2, 3];
    // let v1_iter = v1.iter();

    // for value in v1_iter {
    //     println!("Got: {}", value);
    // }
}

// The Iterator Trait and the next Method ->
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // methods with default implementations elided
}

#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.into_iter();
    assert_eq!(v1_iter.next(), Some(1));
    assert_eq!(v1_iter.next(), Some(2));
    assert_eq!(v1_iter.next(), Some(3));
    assert_eq!(v1_iter.next(), None);
}

// cargo test

#[test]

// Methods that Consume the Iterator (show the sum of all numbers) ->
fn iterator_sum() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();
    assert_eq!(total, 6);
}

// Methods that Produce Other Iterators ->
