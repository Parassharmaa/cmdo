use async_openai::{
    config::OpenAIConfig,
    types::{
        ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
        CreateChatCompletionRequestArgs,
    },
    Client,
};
use clap::{Parser, Subcommand};
use colored::*;
use copypasta::{ClipboardContext, ClipboardProvider};
use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
    process::Stdio,
};

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

const SYSTEM_PROMPT:&str = "Generate a command to do the given action in the terminal. \
Only put the command inside ``` and don't put sudo in the command. \
For example, if you want to generate a command to list all files in the current directory, you would put `ls`.";

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
                .map(move |s| s.replace('`', "").to_string())
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
        println!("Setting OpenAI key to {}.", "*".repeat(value.len()).green(),);
        let home = std::env::var("HOME").unwrap();

        let path = format!("{}/.cmd.config", home);

        let mut file = File::create(path).unwrap();

        file.write_all(value.as_bytes()).unwrap();
    }

    if let Some(Commands::GEN { value }) = &args.cmd {
        println!("{}: {}", "crafting command for".bold(), value.blue());

        let mut cmd_to_run = generate_command(value.clone()).await;

        cmd_to_run = cmd_to_run.trim().to_string();

        loop {
            println!("\n{} {}", ">".green().bold(), cmd_to_run.green().bold());

            println!("\n{}", "(c) copy, (e) execute, (q) quit".blue().bold());

            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();

            // convert input to lower case
            let input = input.to_lowercase();

            let input = input.trim();

            match input {
                "c" => {
                    // copy the command to the clipbonard
                    let mut ctx = ClipboardContext::new().unwrap();

                    ctx.set_contents(cmd_to_run).unwrap();

                    println!("{}", "Command copied to clipboard".green());
                    break;
                }
                "e" => {
                    // run the command
                    let mut child = std::process::Command::new("bash")
                        .arg("-c")
                        .arg(cmd_to_run)
                        .stdout(Stdio::piped())
                        .spawn()
                        .expect("failed to execute process");

                    // Ensure we have access to the child's stdout
                    let stdout = child.stdout.take().expect("failed to capture stdout");

                    // Create a BufReader for the stdout
                    let reader = BufReader::new(stdout);

                    println!("\n{}", "-".repeat(10).bold());
                    // Stream the output line by line
                    for line in reader.lines() {
                        match line {
                            Ok(line) => println!("{}", line.bold()),
                            Err(e) => eprintln!(
                                "{}: {}",
                                "Error reading line: {}".red(),
                                e.to_string().red()
                            ),
                        }
                    }

                    println!("{}", "-".repeat(10).bold());

                    break;
                }
                "q" => {
                    return;
                }
                _ => {
                    println!("Invalid input");
                }
            }
        }
    }
}
