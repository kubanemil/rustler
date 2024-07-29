#[derive(Debug, Clone, Copy)]
enum Race {
    Human,
    Elf,
    Orc,
}

#[derive(Debug)]
enum Team {
    Jedi(String, String), // you can associate some data with enum
    _Sith(String),
    _Rebel(u64),
    _Other(Player), // or even another struct
    _Other2 {
        name: String,
        race: Race,
        score: u64,
    }, // same as above.
}

impl Team {
    // you can also implement methods for enum
    fn is_jedi(&self) -> bool {
        matches!(self, Team::Jedi(_, _))
    }
}
#[derive(Debug)]
struct Player {
    _name: String,
    _race: Race, // usually enums used in struct
    _score: u64,
}

impl Player {
    fn new(name: &str, _race: Race, _score: u64) -> Self {
        Self {
            _name: name.to_string(),
            _race,
            _score,
        }
    }
}

fn main() {
    let race_enums = [Race::Human, Race::Elf, Race::Orc];
    let elf_enum = race_enums[1]; // . is for instance. In impl or enum methods, use ::
    println!("Elf: {:?}\n", elf_enum);
    let player1 = Player::new("Ibroh", elf_enum, 1221);
    dbg!(&player1);

    let team1 = Team::Jedi("Obi-Wan".to_string(), "Anakin".to_string());
    println!("is jedi: {}\n", &team1.is_jedi());

    if let Team::Jedi(name1, name2) = team1 {
        println!("jedis: {name1} and {name2}");
    }

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
