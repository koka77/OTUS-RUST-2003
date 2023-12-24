#![allow(dead_code)]
/*
    Написать функцию, которая превращает число в строку по следующим правилам:
    1. Если число кратно 3, то возвращаем строку "Fizz"
    2. Если число кратно 5, то возвращаем строку "Buzz"
    3. Если число кратно и 3, и 5, то возвращаем строку "FizzBuzz"
    4. В остальных случаях возвращаем строку, содержащую данное число

    Написать функцию fizzbuzz_list, которая получает число `n: u32` и возвращает
    список строк, содержащих строковые представления fizzbuzz
    для чисел в диапазоне от 1 до n. Написать тесты.
*/

fn fizzbuzz(num: u32) -> String {
    if (num % 3 == 0) && (num % 5 == 0) { "FizzBuzz".into() }
    else if num % 3 == 0 { "Fizz".into() }
    else if num % 5 == 0 { "Buzz".into() } else { num.to_string() }
}

fn fizzbuzz_list(num: u32) -> Vec<String> {
    let mut v: Vec<String> = vec![];
    for i in 1..=num {
        v.push(fizzbuzz(i))
    }
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(&fizzbuzz(1), "1");
        assert_eq!(&fizzbuzz(3), "Fizz");
        assert_eq!(&fizzbuzz(5), "Buzz");
        assert_eq!(&fizzbuzz(7), "7");
        assert_eq!(&fizzbuzz(9), "Fizz");
        assert_eq!(&fizzbuzz(15), "FizzBuzz");
        assert_eq!(&fizzbuzz(30), "FizzBuzz");
        assert_eq!(&fizzbuzz(49), "49");
    }

    #[test]
    fn fizzbuzz_list_works() {
        println!("{:?}", &fizzbuzz_list(3));
        assert_eq!(&fizzbuzz_list(3)[2], "Fizz");
        assert_eq!(&fizzbuzz_list(3)[0], "1");
    }
}

fn main() {}