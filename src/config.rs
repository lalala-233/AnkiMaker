pub trait Config: serde::de::DeserializeOwned {
    fn generate(self) -> Result<Vec<String>, String>;
}
