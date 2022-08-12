#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {  //borrowing self immutably here with reference
        self.width * self.height
    }

    fn square(size: u32) -> Self {  //Associated function (static?)
        Self {
            width: size,
            height: size
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let sq = Rectangle::square(5);  //Call associated functions with ::

    println!("rect1 is {:?}", rect1);
    
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
