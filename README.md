## Overview

This utility script allows users to generate terminal commands using natural language descriptions.

### Installation

To install the script, run the following command:

![demo](./assets/demo.png)

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
cmdo gen "ping google.com 3 times"
```

Output:

```
crafting command for: ping google.com 3 times

> ping -c 3 google.com

(c) copy, (e) execute, (q) quit
```

## Contributing

Feel free to open issues or submit pull requests with improvements or bug fixes.

