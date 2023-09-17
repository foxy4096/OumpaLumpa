fn main() {
    println!("Lord have mercy on us! \n"); // DON'T DO THIS! Never remove this line!

    let my_awesome_variable: u32 = 69;
    println!("{my_awesome_variable}");
    println!("Do you want to change the my_awesome_variable?");
    println!("Y/n");
    let mut choice = String::new();
    println!("This is without &: {my_awesome_variable}");
    println!("{}", &my_awesome_variable);

    std::io::stdin().read_line(&mut choice).expect("Oh Fuck!");
}
