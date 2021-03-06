#[derive(Clone, Copy, Debug)]
pub enum Effect {
    Damage,
    ModStrength,
    ModMana,
}
// Consider moving this elsewhere, and renaming
#[derive(Clone, Copy, Debug)]
pub enum PlayerType {
    One,
    Two,
}

/// The ´Target´ trait applies an effect that is represented by an `isize` in numerical value.
/// To confirm that it works, it returns the modified values, or an error message.
pub trait Target {
    // TODO: Concider this; Should it return an isize? I think the error should return a message at least.
    fn apply_effect(&mut self, effect: Effect, modifier: isize) -> Result<isize, &'static str>;
}
