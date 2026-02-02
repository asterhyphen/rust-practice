pub fn run(mut age: i32) {
    //Basically while true version of rust
    loop {
        println!("Currently {}", age);
        age += 1;
        if age > 21 {
            println!("Go get married");
            break;
        }
    }
}
