# Smart Shell

Smart Shell is an intelligent, AI-powered command-line interface built in Rust. It bridges the gap between natural language and Linux shell commands, allowing users to interact with their system using plain English while maintaining the power and flexibility of a traditional terminal.

## Core Philosophy

The project is built in Rust to ensure memory safety, high performance, and reliable execution. Smart Shell is designed not just to automate tasks, but to act as an educational and protective layer over the standard Linux shell environment.

## Key Features

*   **Natural Language Processing**: Type your intent in plain language, and the agent will process, think, and convert your request into actionable Linux shell commands.
*   **Task Grouping**: Complex requests are broken down into logical groups of tasks and executed sequentially.
*   **Auto-Correction**: Automatically detects and fixes syntax errors or incorrect flags in shell commands before execution.
*   **Command Explanation**: Can explain what a command does before running it, making it an excellent tool for learning Linux system administration.
*   **Safety and Monitoring**: 
    *   Executes safe and accurate commands directly while the agent monitors the process.
    *   Implements a strict blacklist for dangerous commands (e.g., `rm -rf /`) to prevent accidental system damage.
*   **Human-in-the-Loop**: Supports interactive prompts for tasks that require user verification or decision-making.

## Architecture

*   **Agent**: The core decision engine that analyzes requests, determines the required tools, and manages the execution flow.
*   **Tools**: Modular execution units (like the shell executor) that the Agent utilizes to perform system operations.
*   **LLM Integration**: Leverages Large Language Models via HTTP clients to parse natural language, plan task execution, and generate accurate shell commands.

## Getting Started

### Prerequisites

*   Rust
*   Cargo
*   Valid API key for the LLM backend

### Installation

1.  Clone the repository.
2.  Set up your environment variables for your LLM provider.
3.  Build the project using Cargo:

```bash
cargo build --release
```

4. Run the executable:

```bash
cargo run
```

## Security

Smart Shell prioritizes system safety. The internal blacklist prevents destructive commands from being passed to the underlying shell. However, users should still exercise caution and review complex task chains when prompted by the Human-in-the-Loop system.

## Contributing

Contributions, issues, and feature requests are welcome.

## License

Distributed under the MIT License.