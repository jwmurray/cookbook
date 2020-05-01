//
// Maintain list of largest N items

use fixed_vec_deque::FixedVecDeque;

fn main() {
    use_fixed_vec_deque();
}

fn use_fixed_vec_deque() {
    let mut d = FixedVecDeque::<[u32; 2]>::new();

    assert_eq!(d.front_mut(), None);

    *d.push_back() = 1;
    *d.push_back() = 2;

    match d.front_mut() {
        Some(x) => *x = 9,
        None => (),
    }

    assert_eq!(d.front(), Some(&9));
    assert_eq!(d.back(), Some(&2));

    // push_back() returns a reference to a mutable item
    // To set the item, we us '*' to dereference the reference
    *d.push_back() = 3;
    *(d.push_back()) = 35;
    assert_eq!(d.front(), Some(&3));
    assert_eq!(d.back(), Some(&35));
    println!("{:?}", d);
}
