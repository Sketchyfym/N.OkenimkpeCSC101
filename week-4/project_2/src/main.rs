
use std::io;

fn main() {

    let mut age = String::new();
    let mut experience = String::new();
    loop {
        println!("Enter your age: ");
        io::stdin().read_line(&mut age).expect("Not a valid string");
        let age:f32 = age.trim().parse().expect("Not a valid number");

        println!("Enter your experience: ");
        println!("1 for Experienced");
        println!("2 for Inexperienced");
        io::stdin().read_line(&mut experience).expect("Not a valid string");
        let experience:f32 = experience.trim().parse().expect("Not a valid number");

        if age >= 40.0 && experience == 1.0 {
            println!("Your Incentive is N1,560,000 per month");
            break;
        }

        if age >= 30.0 && age < 40.0 && experience == 1.0 {
            println!("Your Incentive is N1,480,000 per month");
            break;
        }
        if age > 28.0 && experience == 1.0 {
            println!("Your Incentive is N1,300,000 per month");
            break;
        } 
        if experience == 2.0 {
            println!("Your incentive is N100,000 per month");
            break;
        }
        else {
            println!("Re-enter values.");
        }
    }
}
    


