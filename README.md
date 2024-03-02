# Command Line Helper

A command-line utility for performing various actions, such as language-specific tasks, file operations, and more.

## Usage

```bash
command_line_helper [OPTIONS]
```

## Examples

Perform language-specific action:

```bash
command_line_helper --language rust --action build
```

Copy files to a specific output path:

```bash
command_line_helper --copy /path/to/source --name my_files
```

Download a YouTube video:

```bash
command_line_helper --url <YOUTUBE_VIDEO_URL>
```

Search for files:

```bash
command_line_helper --search "*.txt" --limit 10
```

## Local Setup

To run Command Line Helper locally, follow these steps:

1. Install Rust:

Ensure that you have Rust and Cargo (Rust's package manager) installed on your system. If not, you can install them by following the official instructions here.

2. Clone the Repository:

Clone this repository to your local machine using the following command:

```bash
git clone https://github.com/your-username/command_line_helper.git
```

3. Navigate to the Project Directory:

 Change your working directory to the project folder:

```bash
cd command_line_helper
```

4. Build the Project:

Build the project using Cargo:

```bash
cargo build --release
```

This will create the executable in the target/release/ directory.

5. Run the Command Line Helper:

Run the command-line helper using:

```bash
./target/release/command_line_helper [OPTIONS]
```

## Notes

Here are some situations and how the tool handles them:

* **Omitting `--language` flag but providing `--action` flag:** The tool attempts to infer the language based on the current directory.
* **Both `--language` and `--action` flags missing:** This results in an error, and the user is prompted to provide both flags.
* For detailed information and additional options, use the --help flag:

```bash
command_line_helper --help
```

## Contributing

This project welcomes contributions! We encourage you to submit pull requests if you find any issues or have suggestions for improvements.