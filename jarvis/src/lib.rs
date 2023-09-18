pub struct TodoList {
    pub todo: Vec<String>,
    pub todo_path: String,
    pub backup: String,
}

impl TodoList {
    pub fn new() -> Result<Self, String> { 
        let todo = vec!();
        let todo_path = String::new();
        let backup = String::new();

        Ok(Self{todo, backup, todo_path })

    }

    pub fn list() {}    

    pub fn add() {}
    pub fn remove() {}
    pub fn delete() {}
    pub fn restore() {}

}





#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
