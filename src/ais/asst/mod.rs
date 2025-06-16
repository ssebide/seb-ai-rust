use async_openai::config;
use derive_more::{Deref, Display, From};

use crate::{Result, ais::OaClient};

//region ---Types
pub struct CreateConfig {
    name: String,
    model: String,
}

#[derive(Debug, From, Deref, Display)]
pub struct AsstId(String);
//endregion ---Types

pub async fn create(oac: &OaClient, config: CreateConfig) -> Result<AsstId> {
    todo!()
}
