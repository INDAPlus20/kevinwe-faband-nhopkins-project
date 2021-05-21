#[derive(Clone)]
pub enum Effect {
    Damage,
    ModStrength,
}
// Consider moving this elsewhere
#[derive(Clone)]
pub enum ActivePlayer {
    One,
    Two,
}

/// The ´Target´ trait applies an effect that is represented by an `isize` in numerical value.
/// To confirm that it works, it returns the modified values, or an error message.
pub trait Target {
    // TODO: Concider this; Should it return an isize? I think the error should return a message at least.
    fn apply_effect(&self, effect: Effect, modifier: isize) -> Result<isize, &'static str>;
}
