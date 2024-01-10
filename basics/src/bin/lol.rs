use std::collections::HashMap;

fn main() {
    let mut hp: HashMap<String, usize> = HashMap::new();
    hp.insert("a".to_string(), 1);
    hp.insert("k".to_string(), 12);
    hp.insert("kl".to_string(), 3);
    let s: usize = hp.iter().map(|v| *v.1).sum();
    dbg!(s);
}
