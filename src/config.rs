pub trait Config: serde::de::DeserializeOwned {
    fn generate(self) -> Vec<String>;
}
