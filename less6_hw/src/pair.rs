pub type Pair = (i32, i32);

pub fn default_pair() -> less6_hw::Pair {
    (0, 0)
}

pub fn pair_scalar_sum(a: less6_hw::Pair, b: less6_hw::Pair) -> i32 {
    a.0 + a.1 + b.0 + b.1
}

pub fn pair_vector_sum(a: Pair, b: Pair) -> Pair {
    (a.0 + b.0, a.1 + b.1)
}
