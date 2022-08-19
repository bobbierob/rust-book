fn main() {
    let v: Vec<i32> = Vec::new();  // define vector
    println!("{:?}", v);

    let v = vec![1, 2, 3];  // macro for defining vector
    println!("{:?}", v);
    
    let mut v = Vec::new();  // create vector and add values to it

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    println!("{:?}", v);  // [5, 6, 7, 8]


    let v = vec![1, 2, 3, 4, 5];  // Reading elements of vectors

    let third: &i32 = &v[2];  // using references
    println!("The third element is {}", third);


    let third: Option<&i32> = v.get(2);  // using Vec.get()
    match third {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }


    let v = vec![100, 32, 57];  // iterating through a vector
    for i in &v {
        println!("{}", i);
    }


    let mut v = vec![100, 32, 57];  // iterating through a vector mutably
    for i in &mut v {
        *i += 50;
        println!("{}", i);
    }


    enum SpreadsheetCell {  // use enum to store multiple types
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

}
