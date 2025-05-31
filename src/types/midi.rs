pub type MessageData = [u8; 3];
pub type Message = (u64, MessageData);
pub type MessageLog = Vec<Message>;
