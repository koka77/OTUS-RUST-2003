mod counter;
mod pair;
mod vector;

fn main() {
    let a = counter::next_signed(1);
    println!("{:?}", a);
    let a = counter::next_unsigned(1);
    println!("{:?}", a);
    let a = counter::default_signed_counter();
    println!("{:?}", a);
    let a = counter::default_unsigned_counter();
    println!("{:?}", a);
    let a = counter::prev_signed(1);
    println!("{:?}", a);

    let a = pair::pair_vector_sum((1,2), (3,4));
    println!("{:?}", a);
    let a = pair::pair_scalar_sum((1,2), (3,4));
    println!("{:?}", a);
    let a = pair::default_pair();
    println!("{:?}", a);

    let a = vector::vec3_scalar_sum(vector::default_vec3(), [1,2,3]);
    println!("{:?}", a);
    let a = vector::vec3_vector_sum(vector::default_vec3(), [1,2,3]);
    println!("{:?}", a);



}
