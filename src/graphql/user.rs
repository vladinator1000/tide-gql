#[derive(Clone)]
pub struct User {
    pub id: u16,
    pub name: String,
}

#[juniper::object]
impl User {
    fn id(&self) -> i32 {
        self.id as i32
    }

    fn name(&self) -> &str {
        &self.name
    }
}
