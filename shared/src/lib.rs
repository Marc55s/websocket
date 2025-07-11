
pub enum ChatMessage {
    Text { from: String, content: String },
    Join { user: String },
    Leave { user: String },
}

pub trait ChatTransport {
    fn send(&self, msg: ChatMessage) -> Result<(), String>;
    fn receive(&mut self) -> Option<ChatMessage>;
}

pub fn hellofromlib() {
    println!("Hi from lib")
}

//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
