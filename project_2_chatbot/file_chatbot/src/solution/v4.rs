use kalosm::language::*;
use crate::solution::file_library;

pub struct ChatbotV4 {
    model: Llama,
}

//  if you close the terminal, then restarting it, your history is lost
// put data in a file 
impl ChatbotV4 {
    pub fn new(model: Llama) -> ChatbotV4 {
        return ChatbotV4 {
            model: model,
        };
    }

    pub async fn chat_with_user(&mut self, username: String, message: String) -> String {
        let filename = &format!("{}.txt", username);

        // TODO: You have to implement the rest:
        // You need to load the chat session from the file using file_library::load_chat_session_from_file(...).
        // Think about what needs to happen if the function returns None vs Some(session).
        // Hint: look at https://docs.rs/kalosm/latest/kalosm/language/struct.Chat.html#method.with_session

        let mut chat = match file_library::load_chat_session_from_file(&filename) {
        Some(session) => {
            self.model
                .chat()
                .with_session(session)
        }
        None => {
            self.model
                .chat()
                .with_system_prompt("The assistant will act like a pirate")
            }
        };

        let output = chat.add_message(message).await.unwrap();

        let session = chat.session().unwrap();
        file_library::save_chat_session_to_file(&filename, &session);

        output.to_string()

    }

    pub fn get_history(&self, username: String) -> Vec<String> {
        let filename = &format!("{}.txt", username);

        //tries to load the chat history
        match file_library::load_chat_session_from_file(&filename) {
            None => {
                return Vec::new();
            }
            //if nothing then return an empty vector
            Some(session) => {
              //  If a file is found, you rebuild the chat with from the saved chat
            let chat = self.model.chat().with_session(session);
            let session = chat.session().unwrap();
            let history = session.history();

            let mut messages: Vec<String> = Vec::new();

            for message in history {
            let content = message.content().to_string();

            
            if content == "The assistant will act like a pirate" || content.trim().is_empty() {
                continue;
            }
//Create an empty vector called messages, and look through every message 
//and put in string from and push it in the message vector 
    messages.push(content);
}
            return messages;
        }
    }
}
}