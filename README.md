## Overview

This utility script allows users to generate terminal commands.

### Installation

To install the script, run the following command:

```sh
npm install -g cmdo-gen
```

or 

```
cargo install cmdo-gen
```

## Usage

### 1. Setting the OpenAI API Key

Before generating commands, you need to set your OpenAI API key. Run the following command:

```sh
cmdo set-key <your-openai-api-key>
```

This will save your API key to a configuration file in your home directory (`~/.cmd.config`).

### 2. Generating Commands

To generate a terminal command based on an action description, run:

```sh
cmdo gen "<action-description>"
```

For example:

```sh
cmdo gen "list all files in the current directory"
```

The script will generate a command and prompt you to either copy it to the clipboard or exit.

### Command Options

- `set-key`: Save the OpenAI API key to the configuration file.
- `gen`: Generate a terminal command based on the provided action description.

## Example

### Setting the API Key

```sh
cmdo set-key sk-xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
```

### Generating a Command

```sh
cmdo gen "restart all the pods using kubectl"
```

Output:

```
cmdo generating command for: restart all the pods using kubectl

Generated command: kubectl rollout restart deployment --all

Press (c) to copy the command to the clipboard or (e) to exit
```

## Contributing

Feel free to open issues or submit pull requests with improvements or bug fixes.

