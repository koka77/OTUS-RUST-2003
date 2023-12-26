fn main() {
    println!("Hello, world!");

    println!("{}", double_int32(2, 4199999999));
    println!("{}", double_int32(2, 2));
    println!("{}", double_int64(3));
    println!("{}", double_float32(4.));
    println!("{}", double_float64(5));
    println!("{}", int_plus_float_to_float(6, 7.));
    println!("{}", int_plus_float_to_int(7, 8.));
    println!("{}", tuple_sum((8, 9)));
    println!("{}", array_sum([1, 2, 3]));
}

fn double_int32(x: u32, y: u32) -> u32 {
    x * 2
}

fn double_int64(x: u32) -> u64 {
    x as u64 * 2
    // let res = x.checked_mul(2);
}

fn double_float32(x: f32) -> f32 {
    x * 2.0
}

fn double_float64(x: u32) -> f64 {
    x as f64 * 2f64
}

fn int_plus_float_to_float(x: u32, y: f32) -> f64 {
    x as f64 + y as f64
}

fn int_plus_float_to_int(x: u32, y: f32) -> u64 {
    (x as f64 + y as f64) as u64
}

fn tuple_sum((x, y): (u32, u32)) -> u64 {
    x as u64 + y as u64
}

fn array_sum(arr: [u32; 3]) -> u64 {
    // arr[0] as u64 + arr[1] as u64 + arr[2] as u64
    arr.map(|v| { v as u64 }).iter().sum()
}