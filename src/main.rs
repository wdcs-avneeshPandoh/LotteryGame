use rand::Rng;
use std::io;
#[derive(Debug)]
#[warn(unused_imports)]

struct User {
    name: String,
    TokenId: u32,
}

fn main() {
    println!("Avneesh Lottery system");
    let mut active = false;
    let mut passive = false;
    let mut user_vec: Vec<User> = Vec::new();
    // lottery system
    loop {
        if active == false {
            println!("enter your name");
            let mut name1 = String::new();
            io::stdin().read_line(&mut name1).expect("failed 1");
            let name1 = name1.trim().to_string();

            println!("enter your token id");
            let mut token1 = String::new();
            io::stdin().read_line(&mut token1).expect("failed 2");
            let token1: u32 = token1.trim().parse().expect("please a number");

            let user1 = User {
                name: name1,
                TokenId: token1,
            };
            user_vec.push(user1);

            println! {"do you want to ad more users in lottery system"};
            let mut a = String::new();
            io::stdin().read_line(&mut a).expect("failed 3");
            let a: bool = a.trim().parse().expect("please enter true or false");

            if a == false {
                active = true;
            }
            if active == true {
                break;
            }
        }
    }

    loop {
        if passive == false {
            println!("enter the index you want to view the data for");
            let mut b = String::new();
            io::stdin().read_line(&mut b).expect("failed 4");
            let b: usize = b.trim().parse().expect("please enter true or false 2");

            if let Some(user) = user_vec.get(b) {
                println!("User details: {:?}", user);
            }

            println!("do you want to view another index data");
            let mut t = String::new();
            io::stdin().read_line(&mut t).expect("failed 5");
            let t: bool = t.trim().parse().expect("enter true or false only");
            if t == false {
                passive = true;
            }

            if passive == true {
                break;
            }
        }
    }
    let mut num = 0;
    for i in 0..(user_vec.len()) {
        if let Some(user) = user_vec.get(i) {
            if user.TokenId > 0 {
                num += 1;
            }
        }
    }
    let secretNumber = rand::thread_rng().gen_range(0..num);
    println!("Lottery number is : {:?}", secretNumber);

    if let Some(user) = user_vec.get(secretNumber) {
        let a = &user.name;
        println!(
            "congratulations the winner index is {} and User's name is {}",
            secretNumber, a
        );
    }
}
