fn main() {

    let n1 = "Electrical".to_string();
    let n2 = " Electronic".to_string();
    let n3 = " Engineering".to_string();
    let n4 = n1 + &n2 + &n3; // n2 & n3 referenced is passed

    // About Electrical/Electronic
    println!("\nThe {} is informed by the aspiration to train electrical /electronic engineering professionals in the areas of the design, building and maintenance of electrical control systems,", n4);

    let w1 = "Computer".to_string();
    let w2 = " Science".to_string();
    let w3 = w1 + &w2; // w2 reference is passed
    println!();
    println!("{} is aimed at developing competent, creative, innovating, enetrpreneural and ethically-inded persons, capable of creating value in the diverese fields of computer science.", w3);
}
