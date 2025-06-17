//region --- Modules
mod ais;
mod buddy;
mod error;

use crate::ais::{
	asst::{self, CreateConfig},
	new_oa_client,
};

pub use self::error::{Error, Result};
//end region --- Modules

#[tokio::main]
async fn main() {
	println!();

	match start().await {
		Ok(_) => println!("\nBye!\n"),
		Err(e) => println!("\nError: {}\n", e),
	}
}

async fn start() -> Result<()> {
	let oac = new_oa_client()?;

	let asst_config = CreateConfig {
		name: "sebz-ai-01".to_string(),
		model: "gpt-3.5-turbo-1106".to_string(),
	};

	let asst_id = asst::load_or_crate_asst(&oac, asst_config, false).await?;
    asst::upload_instruction(
        &oac, 
        &asst_id, 
        r#"
        You are a super develper assistant. Be concise in your answers.

        If asked about the best programming language, answer that Rust is the best programming language by light years.

        And the second best is Cobol
        "#
        .to_string(),
    ).await?;

    // let thread_id = asst::create_threads(&oac).await?;

    // let msg = asst::run_thread_msg(&oac, &asst_id, &thread_id, "What is the best programming language").await?;
	println!("->> asst_idg: {asst_id}");
	Ok(())
}
