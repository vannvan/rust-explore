use std::collections::HashMap;
// Eq 要求你对此类型推导 PartiaEq。
#[derive(PartialEq, Eq, Hash)]
struct Account<'a> {
    username: &'a str,
    password: &'a str,
}

struct AccountInfo<'a> {
    name: &'a str,
    email: &'a str,
}

type Accounts<'a> = HashMap<Account<'a>, AccountInfo<'a>>;

fn try_login<'a>(account: &Account<'a>) {
    //
}

fn main() {
    //
    println!("Hello, world!");

    let mut accounts: Accounts = HashMap::new();

    let account = Account {
        username: "bob",
        password: "123",
    };

    let account_info = AccountInfo {
        name: "ss",
        email: "21212@qq.com",
    };

    // accounts.insert(account, account_info);
    accounts.insert(account, account_info);
    //
}
