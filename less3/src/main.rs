struct Account {
    balance: i64,
}

impl Drop for Account {
    fn drop(&mut self) {
        if self.balance != 0 {
            println!("Attention! Balance is not zero")
        } else {
            println!("Account was delete")
        }
    }
}

fn main() {
    let a1 = Account { balance: 0 };
    // drop(a1);
    let a2 = Account { balance: 42 };
}
