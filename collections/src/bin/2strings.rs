// you can use most of Vec methods in String
fn main() {
    let mut s = String::from("EMIL");
    s.push('!'); // with .push() you can append only one character (with '', instead of "")
    dbg!(s);

    let mut new_s = String::new();
    for i in 1..10 {
        new_s.push_str(&i.to_string()); // remember that &String = &str
    }
    dbg!(new_s);

    // string concatenation
    let s1 = "Kuban ".to_string(); // won't be available after s3
    let s2 = "Emil".to_string(); // will be available after s3
                                 // the left always should be owned by the variable
    let s3 = s1 + &s2; // nor (&s1+&s2) | (s1+s2) | (&s1+s2) is possible
    dbg!(&s3);
    let s4 = s3 + " is super"; // because " is super" is already reference
    dbg!(&s4);

    // format!
    let (s1, s2, s3) = ("JoJo", "Bizzare", "Adventure");
    let goat = format!("{s1}'s {s2} {s3}!");
    dbg!(&goat);

    /* Rust doesn't support string indexing (s[2]), bc one letter can
    take more than 1 byte to store (russian letters take 2bytes).
    Use .char() method to separate string into letters: */
    for letter in "Здарова".chars() {
        print!("{letter}-");
    }
    println!();
    // if you want separate string by bytes:
    for byte in "Здарова".bytes() {
        print!("{byte}|");
    }
    println!();
}
