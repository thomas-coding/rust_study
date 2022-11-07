/* User */
struct User {
    active: bool,
    username: String,
    email: String,
    count: u64,
    flag: Flag,
}

enum Flag {
    FLAG1,
    FLAG2,
}

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("some_user_name"),
        active: true,
        count: 1,
        flag: Flag::FLAG1,
    };

    user1.email = String::from("modify@example.com");
    user1.count = 2;
    user1.flag = Flag::FLAG2;

    match user1.flag {
        Flag::FLAG1 => {
            println!("this is flag1");
        }
        Flag::FLAG2 => {
            println!("this is flag2");
        }
    }

    println!(
        "user1 name:'{}' email:'{}' active:{}",
        user1.username, user1.email, user1.active
    );
}
