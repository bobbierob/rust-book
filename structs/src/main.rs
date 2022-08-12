struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

//Tuple structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

//Unit-like struct
struct AlwaysEqual;  //Struct used as 'flag' for obj no additional data

fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1
    };

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    let black = Color(0, 0, 0);
}
