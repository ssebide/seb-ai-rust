use async_openai::types::{
	CreateMessageRequest, CreateMessageRequestContent, MessageContent, MessageObject, MessageRole,
};

use crate::Result;

//region -- Message Constructors
pub fn user_msg(content: impl Into<String>) -> CreateMessageRequest {
	CreateMessageRequest {
		role: MessageRole::User,
		content: CreateMessageRequestContent::Content(content.into()),
		..Default::default()
	}
}
//endregion --- Message constructors

//region ---content extractor
pub fn get_text_content(msg: MessageObject) -> Result<String> {
	let msg_content = msg
		.content
		.into_iter()
		.next()
		.ok_or_else(|| "No message content found".to_string())?;

	//get the text
	let txt = match msg_content {
		MessageContent::Text(text) => text.text.value,
		MessageContent::ImageFile(_) => return Err("Message image not supported yet".into()),
		MessageContent::ImageUrl(_) | MessageContent::Refusal(_) => todo!(),
	};

	Ok(txt)
}
//endregion ----content extractor
