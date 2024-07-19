## Overview

This utility script allows users to generate terminal commands.

## Usage

### 1. Setting the OpenAI API Key

Before generating commands, you need to set your OpenAI API key. Run the following command:

```sh
cargo run -- set-key <your-openai-api-key>
```

This will save your API key to a configuration file in your home directory (`~/.cmd.config`).

### 2. Generating Commands

To generate a terminal command based on an action description, run:

```sh
cargo run -- gen "<action-description>"
```

For example:

```sh
cargo run -- gen "list all files in the current directory"
```

The script will generate a command and prompt you to either copy it to the clipboard or exit.

### Command Options

- `set-key`: Save the OpenAI API key to the configuration file.
- `gen`: Generate a terminal command based on the provided action description.

## Example

### Setting the API Key

```sh
cargo run -- set-key sk-xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
```

### Generating a Command

```sh
cargo run gen "restart all the pods using kubectl"
```

Output:

```
cmdo generating command for: restart all the pods using kubectl

Generated command: kubectl rollout restart deployment --all

Press (c) to copy the command to the clipboard or (e) to exit
```

## Contributing

Feel free to open issues or submit pull requests with improvements or bug fixes.

