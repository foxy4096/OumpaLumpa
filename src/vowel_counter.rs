fn main() {
    let mut s = String::new();

    println!("Enter your sentence: ");

    std::io::stdin()
        .read_line(&mut s)
        .expect("Well, there seems to be an problem");
    let vowels = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
    let mut vowel_count = 0;
    for i in s.chars() {
        if vowels.contains(&i) {
            vowel_count += 1;
        }
    }
    println!("The Number of vowels in the sentence is {}", vowel_count);
}
