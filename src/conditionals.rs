pub fn run(weight: i32) {
    let msg = "Name: Ahmed, Category: ";
    //if else but assign var write first
    let res = if weight > 40 { "Heavy" } else { "Light" };
    println!("{}{}", msg, res);

    let msg = "Is Ahmed Heavy? ";
    let res = weight > 40; //let is compulsory here because type is diff
    println!("{}{}", msg, res);
}
