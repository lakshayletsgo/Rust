pub fn helper_option() -> Option<u8> {
    let mut opt1: Option<u8> = None;
    opt1 = Some(21);
    return opt1;
}

pub fn helper_option_string() -> Option<String> {
    let opt12: Option<String> = Some("Lakshay".to_string());

    return opt12;
}

pub enum CharacterType {
    Archer,
    Warrior,
    Mage,
}

pub fn helper_option_character() -> Option<CharacterType> {
    let mut charatype: Option<CharacterType> = None;
    charatype = Some(CharacterType::Archer);
    return charatype;
}

impl ToString for CharacterType {
    fn to_string(&self) -> String {
        match self {
            CharacterType::Archer => "Archer",
            CharacterType::Warrior => "Warrior",
            CharacterType::Mage => "Mage",
        }
        .to_string()
    }
}
