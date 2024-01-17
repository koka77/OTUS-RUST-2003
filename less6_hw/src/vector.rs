pub const VEC3_LEN: usize = 3;

pub type Vec3 = [i32; VEC3_LEN];

pub fn default_vec3() -> Vec3 {
    [0; 3]
}

pub fn vec3_vector_sum(a: Vec3, b: Vec3) -> Vec3 {
    let mut c = default_vec3();
    for i in 0..3 {
        c[i] = a[i] + b[i];
    }
    c
}

pub fn vec3_scalar_sum(a: Vec3, b: Vec3) -> i32 {
    let mut c = 0;
    for i in 0..VEC3_LEN {
        c += a[i] + b[i];
    }
    c
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_vec3() {
        let arr = [0; VEC3_LEN];
        assert_eq!(default_vec3(), arr)
    }

    #[test]
    fn test_vec3_vector_sum() {
        let arr1 = [1, 2, 3];
        let arr2 = [2, 3, 4];
        assert_eq!(vec3_vector_sum(arr1, arr2), [3, 5, 7])
    }

    #[test]
    fn test_vec3_scalar_sum() {
        let arr1 = [1, 2, 3];
        let arr2 = [2, 3, 4];
        assert_eq!(vec3_scalar_sum(arr1, arr2), 15)
    }
}
