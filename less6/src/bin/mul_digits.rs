#![allow(dead_code)]
/*
    Написать функцию, которая будет вычислять произведение цифр числа,
    при это цифра 0 игнорируется. Затем повторить операцию с результатом
    произведения, пока не получится число, состоящее из одной цифры.
*/

//
pub fn digit_product(n: u32) -> u8 {
    if n == 0 {
        return 0;
    }
    let radix: u32 = 10;
    let res: u32 = n
        .to_string()
        .chars()
        .map(|c| c.to_digit(radix).unwrap())
        .filter(|i| i > &0)
        .reduce(|a, b| a * b)
        .unwrap();
    if res.to_string().len() > 1 {
        digit_product(res)
    } else {
        res.try_into().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(digit_product(0), 0);
        assert_eq!(digit_product(9), 9);
        assert_eq!(digit_product(10), 1);
        assert_eq!(digit_product(987), 2);
        assert_eq!(digit_product(123456), 4);
        assert_eq!(digit_product(123454321), 6);
    }
}

fn main() {}
