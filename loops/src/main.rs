fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("value of element : {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("Launch!");

    for elem in a.iter().rev() {
        println!("{}!", elem);
    }
    println!("Done.");
}
