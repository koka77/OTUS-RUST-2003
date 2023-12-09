fn main() {
    println!("Hello, world!");
}

fn double_int32(x: i32) -> i32 {
    x * 2
}

fn double_int64(x: i32) -> i64 {
    (x * 2) as i64
}

fn double_float32(x: f32) -> f32 {
    x * 2.0
}

fn double_float64(x: i32) -> f64 {
    (x * 2) as f64
}

fn int_plus_float_to_float(x: i32, y: f32) -> f64 {
    ((x as f32) + y) as f64
}

fn int_plus_float_to_int(x: i32, y: f32) -> i64 {
    (x as f32 + y) as i64
}
fn tuple_sum((x, y): (i32, i32)) -> i64 {
    (x + y) as i64
}
fn array_sum(arr: [i32; 3]) -> i32 {
    arr[0] + arr[1] + arr[2]
}