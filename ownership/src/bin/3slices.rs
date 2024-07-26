/* 
Slicing is same as in python:
s[:] = s[..] | s[0:5] = s[0..5] | s[2:] = s[2..] | s[:5] = s[..5] 
But no negative indexing in rust:
s[-5:-1] = s[s.len()-5..s.len()-1]

Always reference slices! (return &slice, not slice)
*/

fn main() {
    let mut full_name = String::from("Emil Kuban");
    /* you could do something like this:
    let slice = full_name
    but you can't do the same with slices:
    let slice = full_name[..5]
    bc you can't take ownership of part of the string, you can only borrow it.*/
    let slice = &full_name[..8]; 
    println!("slice = {}", slice);

    let first_name = get_first_word(&full_name);
    // full_name.clear(); // error, bc first_name references it, so you can't change it
    println!("first_name = {}", first_name);

    let arr = [1; 10];
    let arr_slice = &arr[..5];
    println!("arr_slice = {:?}", arr_slice);

}

// if 's' is immutable, then &String == &str, so it will accept both
fn get_first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i]; // you should return reference to slice, bc you shouldn't make copies.
        }
    }
    &s
}
