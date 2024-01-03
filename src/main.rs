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
            let mut name = String::new();
            io::stdin().read_line(&mut name).expect("failed 1");
            let name1 = name.trim().to_string();

            println!("enter your token id");
            let mut TokenId = String::new();
            io::stdin().read_line(&mut TokenId).expect("failed 2");
            let TokenId: u32 = TokenId.trim().parse().expect("please enter a number");

            let user = User {
                name,
                TokenId,
            };
            user_vec.push(user);

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
            let b: usize = b.trim().parse().expect("please enter only number");

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
    let mut num = user_vec.len();
    println!("num = {}", num);
    // for i in 0..(user_vec.len()) {
    //     if let Some(user) = user_vec.get(i) {
    //         if user.TokenId > 0 {
    //             num += 1;
    //         }
    //     }
    // }
    let secretNumber = rand::thread_rng().gen_range(0..num);
    println!("Lottery index is : {:?}", secretNumber);

    if let Some(user) = user_vec.get(secretNumber) {
        let a = &user.name;
        let b = &user.TokenId;
        println!(
            "congratulations the winner index is {} and User's name is {} and Token Id is {}",
            secretNumber, a, b
        );
    }
}
