use kalosm::language::*;


#[allow(dead_code)]
pub struct ChatbotV2 {
    // What should you store inside your Chatbot type?
    // The model? The chat_session?
    chat_session: Chat<Llama>

    //The struct stores a chat session, so you are storing the entire conversation state, 
    //and not just the model. Now, every time you call add_mesage, the conversation grows. 
}


impl ChatbotV2 {
    #[allow(dead_code)]
    pub fn new(model: Llama) -> ChatbotV2 {
        return ChatbotV2 {
            chat_session: model.chat().with_system_prompt("The assistant will act like a pirate")  
        };
    }


    #[allow(dead_code)]
    pub async fn chat_with_user(&mut self, message: String) -> String {
        let asynchronous_output = self.chat_session.add_message(message);
        let output = asynchronous_output.await.unwrap();
        output.to_string()   
    }
}