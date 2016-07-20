fn group_letter(name: &str) -> Option<char> {

    match name {
        "Plantex" => Some('C'),
        "AVZ-Run" => Some('A'),
        "Space Game" => Some('B'),
        _ => None,
    }
}

fn main() {

    println!("AVZ-Run ist Gruppe {:?}.", group_letter("AVZ-Run"));
    println!("Space Game ist Gruppe {:?}.", group_letter("Space Game"));
    println!("Plantex ist Gruppe {:?}.", group_letter("Plantex"));
    println!("Hello ist Gruppe {:?}.", group_letter("Hello"));

}
