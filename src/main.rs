fn main(){
    //mutables
    let mut msg="Name: Ahmed, Category: ";
    let mut weight = 55;
    let mut age = 19;
    weight = weight / 2 ;

    //if else but assign var write first
    let res = if weight > 40 { "Heavy" } else { "Light" };
    println!("{}{}", msg, res);

     //same var but diferent type
    msg = "Is Ahmed Heavy? ";
    let res = if weight > 40 { true } else { false }; //let is compulsory here because type is diff
    println!("{}{}", msg, res);

    //loop keyword loop
    loop {
        println!("Currently {}", age);
        age+=1;
        if age > 21{
            println!("Go get married");
            break;
        }
    }

    //optional stuff like using None
    let maybe_number: Option<Option<()>> = Some(None); 
    if let Some(num) = maybe_number {
      println!("The number is {:?}", num); //When None insert else cho statement parin
    } else {
        println!("No number provided.");
    }
}