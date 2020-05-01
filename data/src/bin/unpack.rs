// Unpacking a sequence into Separate Variables
//
// Problem:  You have an n-element tuple and want to assign its values to Variables

fn main() {
    let tuple = (0, 1);
    let (x, y) = tuple;
    println!("tuple: {:?}, x: {}, y: {}", tuple, x, y);

    // Same example through a function
    let mut w: i32 = 0;
    let mut z: i32 = 0;

    assign_tuple_to_vars(tuple, &mut w, &mut z);
    println!("tuple: {:?}, w: {}, z:{}", tuple, w, z);

    let foo = transfer_list_to_tuple().expect("failed");
    println!("{:?})", foo);
}

// Assign tuple to variables
fn assign_tuple_to_vars(tuple: (i32, i32), x: &mut i32, y: &mut i32) {
    let (a, b) = tuple;
    *x = a;
    *y = b;
}

// Taken from https://stackoverflow.com/a/43658017/4983398
// How to move a list into a tuple
// You can then save it as a struct if you like.

#[derive(Debug)]
struct Foo {
    foo1: String,
    foo2: String,
    foo3: String,
    foo4: String,
    // ...
}
use itertools::Itertools; // 0.8.2

fn transfer_list_to_tuple() -> Result<Foo, ()> {
    let vector = vec![
        "a".to_string(),
        "b".to_string(),
        "c".to_string(),
        "d".to_string(),
    ];

    // The into_inter() call takes ownership of vector.
    // tupes() then creates a tuple of the elements of the vector
    // .next() iterates through the tuple to assign the elements to foo1, foo2, foo3, foo4
    // .drain() could have been used in place of '.into_iter()', but that would modify the vector
    //  e.g. if let Some((foo1, foo2, foo3, foo4)) = x.drain(..4).tuples().next() {
    // With drain(), the the first 4 items are taken out the vector, and the function can continue
    // to use the vector.
    if let Some((foo1, foo2, foo3, foo4)) = vector.into_iter().tuples().next() {
        let foo = Foo {
            foo1,
            foo2,
            foo3,
            foo4,
        };
        return Ok(foo);
    }
    Err(())
}
