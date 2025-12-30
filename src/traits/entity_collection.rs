pub trait EntityCollection {
  fn get_titles(&self) -> Vec<&str>;
  fn get_titles_as_string(&self) -> String {
    self.get_titles_as_string_with(", ")
  }
  fn get_titles_as_string_with(&self, separator: &str) -> String {
    self.get_titles().join(separator)
  }
}
