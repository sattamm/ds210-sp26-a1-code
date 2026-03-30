use kalosm::language::*;
use std::collections::HashMap;

#[allow(dead_code)]
pub struct ChatbotV3 {
    model: Llama,
    sessions: HashMap<String, Chat<Llama>>,
    // Chatbot now has memory, but has trouble distinguishing between the histories of different users. 
    // hashmap so the key is a string that holds the username & value is the username’s respective chat session

}

impl ChatbotV3 {
    #[allow(dead_code)]
    pub fn new(model: Llama) -> ChatbotV3 {
        return ChatbotV3 {
            
            // Make sure you initialize your struct members here
            model,
            sessions: HashMap::new(),
        };
    }

    #[allow(dead_code)]
    pub async fn chat_with_user(&mut self, username: String, message: String) -> String {
        // first check if the user is not already in the hashmap, and if not, creates a chat object
        if !self.sessions.contains_key(&username){
            let chat = self.model
                .chat()
                .with_system_prompt("The assistant will act like a pirate");
            self.sessions.insert(username.clone(), chat);
        }
        //create the response 
        let chat = self.sessions.get_mut(&username).unwrap();
        let output = chat.add_message(message).await.unwrap();
        output.to_string()


        
    }

    #[allow(dead_code)]
    pub fn get_history(&self, username: String) -> Vec<String> {
        // Extract the chat message history for the given username
        // if theere exists a chat session, then get it and find the history and session
          if let Some(chat_session) = self.sessions.get(&username) {
        let session = chat_session.session().unwrap();
        let history = session.history();

        //Vector of chat messages → iterate over them, ignore the system prompt, transform the message to a string 
        history
            .iter()
            .skip(1) // skip the system prompt
            .map(|msg| msg.content().to_string())
            .collect()
        //if not in the history, return an empty vector 
    } else {
        Vec::new()
    }
    
    }
}