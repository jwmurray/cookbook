/// Borrowed from https://stackoverflow.com/a/45283083/4983398

fn main() {
    let v2 = 1..10;

    let powers: Vec<u32> = (0u32..9).filter(|x| x % 2 == 0).map(|x| x.pow(2)).collect();
    let evens: Vec<u32> = v2.filter(|x| x % 2 == 0).collect();

    println!("{:?}", powers);
    println!("{:?}", evens);
}
