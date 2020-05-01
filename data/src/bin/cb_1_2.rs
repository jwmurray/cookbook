// Unpack N elements from an itereable, but the iterable may be longer than 10 elements

fn main() {
    let grades: Vec<_> = vec![11.0, 12.0, 31.0, 89.0, 45.4f32];
    let grades = drop_first_last(grades);
    println!("{:?}", grades);
    let grade = drop_first_last_and_average(grades);
    println!("{:?}", grade);
}

fn drop_first_last_and_average(grades: Vec<f32>) -> f32 {
    let length = grades.len();
    let sum: f32 = grades.iter().skip(1).take(length - 2).sum();
    sum / grades.len() as f32
}

fn drop_first_last(grades: Vec<f32>) -> Vec<f32> {
    let length = grades.len();
    let middle: Vec<f32> = grades.into_iter().skip(1).take(length - 2).collect();
    middle
}
