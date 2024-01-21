# README

This is a simple package hunter. It automates some boring parts of coding, like generating commit messages based on Git diffs.

## Features

- `commit`: Writes commit messages based on Git diffs.
- `presets`: Creates a Google Slide presentation demonstration the paths of the code.
- `comment`: Writes JSDocs for TypeScript functions.

## Usage

To use this tool, follow these steps:
1. Install the required dependencies:

```bash
pip install gitpython
```

2. Clone the repository:

```bash
git clone https://github.com/your-username/your-repo.git
```

3. Navigate to the project directory:

```bash
cd your-repo
```

4. Run the desired command:

```bash
# Generate commit messages
python -m commit

# Create a Google Slide presentation
python -m presets --path /path/to/code

# Write JSDocs for TypeScript functions
python -m comment --path /path/to/code
```

Replace `/path/to/code` with the path to the code you want to generate comments for.

## Configuration

The tool uses a configuration file located at `~/.acm/config.toml`. You can customize the API base URL, API key, model name, commit prompt, and the maximum number of characters for generated commit messages in this file.

## Contributing

Contributions are welcome! If you find any bugs or have suggestions for improvements, please open an issue or submit a pull request.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.