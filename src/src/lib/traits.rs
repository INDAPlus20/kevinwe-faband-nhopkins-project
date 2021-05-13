enum Effect{
    Damage,
    ModStrength,
}

pub trait Target {
    fn apply_effect(&self, Effect, isize) -> Result<T, E>;
}
