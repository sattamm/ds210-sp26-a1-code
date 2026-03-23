use kalosm::language::*;
use file_chatbot::solution::file_library;

use crate::solution::Cache;

pub struct ChatbotV5 {
    model: Llama,
    cache: Cache<Chat<Llama>>,
}

impl ChatbotV5 {
    pub fn new(model: Llama) -> ChatbotV5 {
        return ChatbotV5 {
            model: model,
            cache: Cache::new(3),
        };
    }

    pub async fn chat_with_user(&mut self, username: String, message: String) -> String {
        let filename = &format!("{}.txt", username);
        let cached_chat = self.cache.get_chat(&username);

        match cached_chat {
            None => {
                println!("chat_with_user: {username} is not in the cache!");
                // since the convo is not in the cache, we can check if its in the file 
                // try to load previous conversation 
                let mut chat = 
                if let Some(session) = file_library::load_chat_session_from_file(filename) {
                    println!("Loaded {username} from file");
                    //rebuild the chat
                    self.model.chat().with_session(session)
                } 
                // no file found, new user 
                else {
                    println!("Creating new chat for {username}");
                    self.model.chat()
                };
                // add the message to the conversation, and comes up with a response 
                let response = chat.add_message(message).await.unwrap().to_string();
                // save the session to the file 
                // &chat.session.unwrap() here because mismatched types
                file_library::save_chat_session_to_file(filename, &chat.session().unwrap());
                //put into the cache 
                self.cache.insert_chat(username, chat);
                return response; 
            }
            Some(chat_session) => {
                println!("chat_with_user: {username} is in the cache! Nice!");
                //  the chat is already in memory
                let mut chat = chat_session; 
                // add the message to the conversation, and comes up with a response 
                let response = chat.add_message(message).await.unwrap().to_string();
                // save the convo to the file 
                 // save the session to the file 
                // &chat.session.unwrap() here because mismatched types
                file_library::save_chat_session_to_file(filename, &chat.session().unwrap());
                self.cache.insert_chat(username, chat);
                return response; 

            }
        }
    }

    pub fn get_history(&mut self, username: String) -> Vec<String> {
        let filename = &format!("{}.txt", username);
        let cached_chat = self.cache.get_chat(&username);

        match cached_chat {
            None => {
                println!("get_history: {username} is not in the cache!");
                // TODO: The cache does not have the chat. What should you do?
                // Your code goes here.
                return Vec::new();
            }
            Some(chat_session) => {
                println!("get_history: {username} is in the cache! Nice!");
                // TODO: The cache has this chat. What should you do?
                // Your code goes here.
                return Vec::new();

            }
        }
    }
}