pub fn run() {
    let maybe_number: Option<Option<()>> = Some(None);

    if let Some(num) = maybe_number {
        println!("The number is {:?}", num); //When None insert else cho statement parin
    } else {
        println!("No number provided.");
    }
}
