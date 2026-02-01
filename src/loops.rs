pub fn run(mut age: i32) {
    loop {
        println!("Currently {}", age);
        age += 1;
        if age > 21 {
            println!("Go get married");
            break;
        }
    }
}
