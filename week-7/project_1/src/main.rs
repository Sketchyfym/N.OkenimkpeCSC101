use std::io;

fn trapezium(){
    let mut input1 = String::new();
    println!("Enter the height of the trapezium:");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let h:f32 = input1.trim().parse().expect("Invalid input");

    let mut input2 = String::new();
    println!("Enter the first base:");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let b1:f32 = input2.trim().parse().expect("Invalid input");

    let mut input3 = String::new();
    println!("Enter the second base:");
    io::stdin().read_line(&mut input3).expect("Failed to read input");
    let b2:f32 = input3.trim().parse().expect("Invalid input");

    let result = h / 2.0 * ( b1 + b2 );
    println!("The area of the trapezium is :{}",result);
}

fn rhombus(){
    let mut input1 = String::new();
    println!("Enter the first diagonal of the rhombus:");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let d1:f32 = input1.trim().parse().expect("Invalid input");

    let mut input2 = String::new();
    println!("Enter the second diagonal:");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let d2:f32 = input2.trim().parse().expect("Invalid input");

    let result = 0.5 * d1 * d2;
    println!("The area of the rhombus is: {}",result);
}

fn parallogram() {
    let mut input1 = String::new();
    println!("Enter the base of the parallogram:");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let b:f32 = input1.trim().parse().expect("Invalid input");

    let mut input2 = String::new();
    println!("Enter the altitude of the parallogram");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let a:f32 = input2.trim().parse().expect("Invalid input");

    let result = b * a;
    println!("The area of the parallogram is:{}",result);
}

fn cube(){
    let mut input1 = String::new();
    println!("Enter the length of cube:");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let l:f32 = input1.trim().parse().expect("Invalid input");

    let result = 6.0 * ( l * l);
    println!("The volume of the cube is: {}",result);
}

fn cylinder(){
    let mut input1 = String::new();
    println!("Enter the radius of the cylinder:");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let r:f32 = input1.trim().parse().expect("Invalid input");

    let mut input2 = String::new();
    println!("Enter the height:");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let h:f32 = input2.trim().parse().expect("Invalid input");

    let result = 3.14 * r * r * h;
    println!("The volume of the cylinder is :{}",result);
}

fn main() {

    let mut question = String::new();
    println!("What are you trying to calculate: \n1.trapezium \n2.rhombus \n3.parallogram \n4.cube \n5.cylinder \n(Just the number behind it eg.1,2,3)");
    io::stdin().read_line(&mut question).expect("Failed to read input");
    let formula:f32 = question.trim().parse().expect("Invalid input");

    if formula == 1.0 {
        trapezium()
    }
    else if formula == 2.0 {
        rhombus()
    }
    else if formula == 3.0 {
        parallogram()
    }
    else if formula == 4.0 {
        cube()
    }
    else if formula == 5.0 {
        cylinder()
    }
    else {
        println!("Incorrect Input.");
    }
}

