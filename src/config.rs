pub trait Config: for<'a> serde::Deserialize<'a> + Default + serde::Serialize {
    fn generate(self) -> Result<Vec<String>, String>;
}
