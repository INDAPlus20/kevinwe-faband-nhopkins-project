pub trait Target {
    fn modify_field(&self, String, isize) -> Result<T, E>;
}