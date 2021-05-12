mod traits.rs

/// card type
enum CType{
    Event,
    Person,
}
/// card subtype, should include school
enum Subtype{
    Professor,
    Phoes,
    EECS,
    ABE,
    ITM,
    SCI,
    CCH,
}
/// holds all relevant data for a card
pub struct Card{
    position: CardPosition,
    owner: *Player,
    used: bool,
    mana: isize,
    strength: isize,
    health: isize,
    ctype: CType,
    sub_type: Vec<Subtype>,
    effects: Vec<(Effect, isize)>,
    text: String,
}

impl Card {
    /// creates a new card, should use a helper function to not need to type as much.
    fn new(position: CardPosition, owner: *Player, strength: isize, health: isize,
        ctype: CType, subtype: Vec<Subtype>, effects: Vec<(Effect, isize)>, text: String) -> Card
    {
            let card = Card{
                position: position,
                owner: owner,
                used: false,
                mana: 0,
                strength: strength,
                health: health,
                ctype: ctype,
                sub_type: subtype,
                effects: effects,
                text: text,
            };
            return card
    }
    /// given a target, applies its effects to that target
    fn use(&self, target: &impl Target)
    {
        for eff in self.effects {target.apply_effect(eff.0, eff.1)};
        self.used = true;
    }
    /// Sets used to false, should be called at start of turn
    fn refresh(&self){
        self.used = false;
    }
}

impl Target for Card{
    /// applies an instance of an effect with a scalar value to self
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