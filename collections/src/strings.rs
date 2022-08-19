fn strings() {
        // 8.2 Strings
        mut s = String::new();  // define a new string

        let data = "initial contents";  // types that have a Display trait can use to_string()
        let s = data.to_string();
    
        // the method also works on a literal directly:
        let s = "initial contents".to_string();
        // this is the same as
        let s = String::from("initial contents")
    
        
        // Appending to a string
        let mut s = String::from("foo");
        s.push_str("bar") // push_str appends string to the end
    
        // Adding strings together
        let s1 = String::from("Hello, ");
        let s2 = String::from("world!");
        let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    
        // Concatenating using format!
        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");
    
        let s = format!("{}-{}-{}", s1, s2, s3);
    
        // Slicing strings
        let hello = "Здравствуйте";
    
        let s = &hello[0..4];  // you can slice strings like this, but not index into them because of char boundaries in UTF-8
    
        // Operating on chars in strings
        // You can use chars() to iterate through the characters in a string
        for c in "Зд".chars() {
            println!("{}", c);
        }
        // or
        for b in "Зд".bytes() {
            println!("{}", b);
        }
}