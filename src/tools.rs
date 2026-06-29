use std::process::Command;

#[derive(Debug)]
pub struct ToolResult {
    pub status: String,
    pub output: String,
    pub error: String,
}

pub struct Tool {
    pub name: String, 
    pub description: String, 
    pub is_human_in_loop: bool,
    pub command: String,       
}

pub trait ToolTrait {
    fn execute(&self, command: &str) -> ToolResult;
}

impl ToolTrait for Tool {
    fn execute(&self, command: &str) -> ToolResult {
        // Execute the Linux shell command
        let result = Command::new("bash")
            .arg("-c")
            .arg(command)
            .output();

        match result {
            Ok(output) => {
                // Get the exit status code as a String
                let status_str = match output.status.code() {
                    Some(code) => code.to_string(),
                    None => "Terminated by signal".to_string(),
                };

                ToolResult {
                    status: status_str,
                    // Convert stdout to a String (lossy handles invalid UTF-8 gracefully)
                    output: String::from_utf8_lossy(&output.stdout).to_string(),
                    // Convert stderr to a String
                    error: String::from_utf8_lossy(&output.stderr).to_string(),
                }
            }
            Err(e) => {
                // Handle cases where the command couldn't even be launched
                ToolResult {
                    status: "Execution failed".to_string(),
                    output: String::new(),
                    error: e.to_string(),
                }
            }
        }
    }
}
