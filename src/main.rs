use core::fmt;

#[derive(Debug)]
enum Gem {
    Diamond,
    Sapphire,
    Ruby,
    Topaz,
    Onyx,
    Jade,
}

impl fmt::Display for Gem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Gem::Diamond => write!(f, "Diamond"),
            Gem::Sapphire => write!(f, "Sapphire"),
            Gem::Ruby => write!(f, "Ruby"),
            Gem::Topaz => write!(f, "Jade"),
            Gem::Onyx => write!(f, "Onyx"),
            Gem::Jade => write!(f, "Jade"),
        }
    }
}
fn main() {
    let mut map = [[false; 5]; 5];
    println!("{map:?}");

    // map[4][2] = true;
    // map[1][2] = true;
    // map[3][3] = true;
    // map[0][2] = true;
    // map[1][4] = true;

    println!("{map:?}");

    for row in map{
        println!("{row:}");
    }

    // let gems = [
    //     (Gem::Onyx, 25.00),
    //     (Gem::Diamond, 100.00),
    //     (Gem::Onyx, 50.00),
    //     (Gem::Ruby, 10.00),
    // ];

    // for gem in gems {
    //     println!("This {:?} is worth {}", gem.0, gem.1);
    // }
}
