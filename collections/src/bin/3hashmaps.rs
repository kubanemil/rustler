use std::collections::HashMap;

fn main() {
    // create
    let mut ages: HashMap<String, i32> = HashMap::new(); // always annotate
    let (me, my_age) = ("Emil".to_string(), 777); // 'me' wil be owned by hashmap
    ages.insert(me, my_age); // insert(key, value)
                             // dbg!(me); dbg!(my_age); // 'me' is moved. 'my_age' is copied
    ages.insert("Guts".to_string(), 25);
    ages.insert("Dio".to_string(), 120);
    dbg!(&ages);

    // get
    let guts_age = ages.get("Guts").unwrap(); // to unwrap from 'Some()'
    dbg!(&guts_age);

    // iterate
    for (k, v) in &ages {
        println!("key: `{k}` - value: `{v}`")
    }

    // update value
    ages.insert("Emil".to_string(), 20);
    dbg!(&ages);

    // insert a new key, if no old key
    ages.entry("Guts".to_string()).or_insert(777);
    ages.entry("Gutuso".to_string()).or_insert(19);
    dbg!(&ages);

    // iterative update
    for v in ages.values_mut() {
        *v -= 25;
        // *k = *k + "-sama"; // can't change key
    }
    dbg!(&ages);
}
