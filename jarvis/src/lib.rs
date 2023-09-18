use std::{ env, fs };
use std::path::Path;

pub struct TodoList {
    pub todo: Vec<String>,
    pub todo_path: String,
    //pub backup: String,
}

impl TodoList {
    pub fn new() -> Result<Self, String> { 
        //let todo = vec!();
        let todo_path: String = match env::var("JARVIS_PATH") {
            Ok(todo_path) => todo_path,
            Err(_) => {
                let home = env::var("HOME").unwrap();
                format!("{}/.jarvis", &home)
            }
        };

        
        let todo_list = OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .open(&todo_path)
            .expect("Jarvis couldn't open the file");
        
        //let backup = String::new();
        
        Ok(Self{todo, todo_path })

    }

    pub fn list() {}    
    pub fn add() {}
    pub fn remove() {}
    pub fn encrypt() {}
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
