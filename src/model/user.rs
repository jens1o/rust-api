#[derive(FromForm)]
pub struct User {
    name: String,
    age: u8,
    is_student: bool,
}

impl User {
    pub fn greet(&self) -> String {
        format!(
            "Hello, I'm {} and {} years old! I'm a student: {}.",
            self.name, self.age, self.is_student
        )
    }
}
