use async_openai::{
    config::OpenAIConfig,
    types::{
        ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
        CreateChatCompletionRequestArgs,
    },
    Client,
};
use clap::{Parser, Subcommand};
use copypasta::{ClipboardContext, ClipboardProvider};
use std::{fs::File, io::Write};

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[command(subcommand)]
    cmd: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[clap(name = "gen", about = "Generate text")]
    GEN { value: String },
    #[clap(name = "set-key", about = "Set openai api key")]
    SETKEY { value: String },
}

const SYSTEM_PROMPT:&str = "Generate a command to do the given action in the terminal. Only put the command inside ``` and don't put sudo in the command. For example, if you want to generate a command to list all files in the current directory, you would put `ls`.";

async fn generate_command(action: String) -> String {
    // read openai key from ~/.cmd.config
    let home = std::env::var("HOME").unwrap();
    let path = format!("{}/.cmd.config", home);
    let key = std::fs::read_to_string(path).unwrap();

    let config = OpenAIConfig::new().with_api_key(key);

    let client = Client::with_config(config);

    let request = CreateChatCompletionRequestArgs::default()
        .model("gpt-4o-mini")
        .messages([
            ChatCompletionRequestSystemMessageArgs::default()
                .content(SYSTEM_PROMPT)
                .build()
                .unwrap()
                .into(),
            ChatCompletionRequestUserMessageArgs::default()
                .content(action)
                .build()
                .unwrap()
                .into(),
        ])
        .build()
        .unwrap();

    let response = client.chat().create(request).await;

    match response {
        Ok(response) => {
            let completion = response.choices[0].clone();
            let result = completion
                .message
                .content
                .into_iter()
                // remove ``` from the start and end of the command```
                .map(move |s| s.trim_matches('`').to_string())
                .collect();
            result
        }
        Err(e) => {
            panic!("Error: {:?}", e);
        }
    }
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    // if open ai key then save the key in ~./cmd.config
    if let Some(Commands::SETKEY { value }) = &args.cmd {
        println!("Setting OpenAI key to {}", "*".repeat(10),);
        let home = std::env::var("HOME").unwrap();

        let path = format!("{}/.cmd.config", home);

        let mut file = File::create(path).unwrap();

        file.write_all(value.as_bytes()).unwrap();
    }

    if let Some(Commands::GEN { value }) = &args.cmd {
        println!("Generating text with value {}", value);

        let cmd_to_run = generate_command(value.clone()).await;

        // loop and wait for the user to confirm the command
        loop {
            println!("Generated command: {}", cmd_to_run);
            println!("Press (c) to copy the command to the clipboard or (e) to exit");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();
            if input == "c" {
                // copy the command to the clipboard
                let mut ctx = ClipboardContext::new().unwrap();

                ctx.set_contents(cmd_to_run).unwrap();

                println!("Command copied to clipboard");
                break;
            } else if input == "e" {
                return;
            }
        }
    }
}
