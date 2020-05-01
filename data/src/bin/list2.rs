/// Inspired by https://stackoverflow.com/a/53689316/4983398
/// Use map/filter/collect to create a python-like dictionary?
use std::collections::HashMap;

fn main() {
    println!(
        "{:?}",
        (1..5).map(|i| (i + i, i * i)).collect::<HashMap<_, _>>()
    );
    let dict = (1..5).map(|i| (i + i, i * i)).collect::<HashMap<_, _>>();

    println!("dict[6]: {}", dict[&6]);
}
