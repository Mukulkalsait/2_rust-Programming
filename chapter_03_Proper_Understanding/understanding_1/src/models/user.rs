
pub struct User {
    pub name:String,
}

impl User {
    pub fn new(name:&str)->Self{
        Self{name:name.to_string(),}
    }
}


