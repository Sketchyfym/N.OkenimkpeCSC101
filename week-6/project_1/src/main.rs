use std::io;

fn main() {
    let p1 = "Poundo Yam/Edinkaido Soup";
    let p = 3200;
    let f1 = "Fried Rice & Chicken";
    let f = 3200;
    let a1 = "Amala & Ewedu Soup";
    let a = 2500;
    let e1 = "Eba & Egusi Soup";
    let e = 2000;
    let w1 = "White Rice & Stew";
    let w = 2500;

    println!(
        "Menu:\n P: {} - N{}\n F: {} - N{}\n A: {} - N{}\n E: {} - N{}\n W: {} - N{}",
        p1, p, f1, f, a1, a, e1, e, w1, w
    );

    let mut input1 = String::new();
    println!("How many P do you want (0,1,2,3): ");
    io::stdin()
        .read_line(&mut input1)
        .expect("Failed to read input");
    let pamount: i32 = input1.trim().parse().expect("Not a valid number");

    let mut input2 = String::new();
    println!("How many F do you want (0,1,2,3): ");
    io::stdin()
        .read_line(&mut input2)
        .expect("Failed to read input");
    let famount: i32 = input2.trim().parse().expect("Not a valid number");

    let mut input3 = String::new();
    println!("How many A do you want (0,1,2,3): ");
    io::stdin()
        .read_line(&mut input3)
        .expect("Failed to read input");
    let aamount: i32 = input3.trim().parse().expect("Not a valid number");

    let mut input4 = String::new();
    println!("How many E do you want (0,1,2,3): ");
    io::stdin()
        .read_line(&mut input4)
        .expect("Failed to read input");
    let eamount: i32 = input4.trim().parse().expect("Not a valid number");

    let mut input5 = String::new();
    println!("How many W do you want (0,1,2,3): ");
    io::stdin()
        .read_line(&mut input5)
        .expect("Failed to read input");
    let wamount: i32 = input5.trim().parse().expect("Not a valid number");

    let price = (p * pamount) + (f * famount) + (a * aamount) + (e * eamount) + (w * wamount);

    if price > 10_000 {
        let new_price = price as f32 * 0.95;
        println!("The price of your meal after discount is: N{:.?}", new_price);
    } else {
        println!("The price of your meal is: N{}", price);
    }
}
