mod traits.rs

enum School{
    EECS,
    ABE,
    ITM,
    SCI,
    CCH,
}

enum CType{
    Event,
    Person,
}

enum Subtype{
    Professor,
    Phoes,
}

pub struct Card{
    position: CardPosition,
    owner: *Player,
    used: bool,
    mana: isize,
    strength: isize,
    health: isize,
    school: School,
    ctype: CType,
    sub_type: Vec<Subtype>,
    effects: Vec<(Effect, isize)>,
    text: String,
}

impl Card {
    fn new(position: CardPosition, owner: *Player, strength: isize, health: isize, school: School,
        ctype: CType, subtype: Vec<Subtype>, effects: Vec<(Effect, isize)>, text: String) -> Card
    {
            let card = Card{
                position: position,
                owner: owner,
                used: false,
                mana: 0,
                strength: strength,
                health: health,
                school: school,
                ctype: ctype,
                sub_type: subtype,
                effects: effects,
                text: text,
            }
            return card
    }

    fn use(&self, target: &impl Target)
    {
        for eff in self.effects {target.apply_effect(eff.0, eff.1)};
    }
}

impl Target for Card{
    fn apply_effect(&self, effect: Effect, value: isize){
        match effect {
            Effect::Damage => {
                &self.health -= value;
            }
            Effect::ModStrength => {
                &self.damage += value;
            }
            _ => {
                println!("The effect {} isn't implemented yet for this target type", effect);
            }
        }
    }
}