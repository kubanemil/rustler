fn main () {
    let elf_enum = Race::Elf; // . is for instance. In class or enum methods, use ::
    println!("Elf: {:?}", elf_enum);
    let player1 = Player::new("Ibroh", elf_enum, 1221);
    dbg!(&player1); println!();

    let team1 = Team::Jedi("Obi-Wan".to_string(), "Anakin".to_string());
    let team2 = Team::Other(player1);
    dbg!(&team1); dbg!(&team2); println!("is jedi: {}", &team1.is_jedi()); println!();

    /*
    Option<T> enum annotates that value might be null.
    let my_num: Option<i32> = get_num();
    Rust has no null, so you need to always handle Option<T> value!
    You can't do any operation with Option<T> directly, you need to unwrap it first:
    let result = my_num + 5; // error
    Rust docs says that it is better to not have nulls for some reason.
    Also, when you specify that variable is Option<T>, you need to asign in only to
    Som(val) or None:
    let my_num1: Option<i32> = Some(5);
    let my_num2: Option<i32> = None;
     */
}

#[derive(Debug)]
enum Race {
    Human,
    Elf,
    Orc,
}

#[derive(Debug)]
enum Team {
    Jedi(String, String), // you can associate some data with enum
    Sith(String),
    Rebel(u64),
    Other(Player), // or even another struct
    Other2 { name: String, race: Race, score: u64 }, // same as above.
}

impl Team { // you can also implement methods for enum
    fn is_jedi(&self) -> bool {
        match self {
            Team::Jedi(_, _) => true,
            _ => false,
        }
    }
}
#[derive(Debug)]
struct Player {
    name: String,
    race: Race, // usually enums used in struct
    score: u64
}

impl Player {
    fn new(name: &str, race: Race, score: u64) -> Self {
        Self { name: name.to_string(), race, score }
    }
}