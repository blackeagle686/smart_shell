/* 
    AGENT steps:
        1. Analysze the user request
        2. Thinkning and chose the proper tools
        3. Execute the tools some tools (human in the loop)
        4. Analysze the result
        5. REACT Reflector 
 */ 

use crate::tools::{Tool, ToolResult};
use reqwest::Client;
use serde_json::json;

use std::fmt;

#[derive(Debug)]
pub enum AgentError {
    InvalidInput(String),
    LlmError(String),
    TaskError(String),
}

impl fmt::Display for AgentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AgentError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            AgentError::LlmError(msg) => write!(f, "LLM processing error: {}", msg),
            AgentError::TaskError(msg) => write!(f, "Task error: {}", msg),
        }
    }
}

impl std::error::Error for AgentError {}

pub enum TaskStatus {
    Pending,
    Completed,
    Failed,
}

pub struct Task {
    pub id: String,
    pub title: String, 
    pub description: String,
    pub tool_to_use: Vec<String>, 
    pub command: String,
    pub priority: u32,
    pub human_in_loop: bool,  
    pub status: TaskStatus,
}

impl Task {
    pub fn new() -> Self {
        Task {
            id: String::new(),
            title: String::new(),
            description: String::new(),
            tool_to_use: Vec::new(),
            command: String::new(),
            priority: 0,
            human_in_loop: false,
            status: TaskStatus::Pending,
        }
    }

    pub fn update_task_status(&mut self, status: TaskStatus) {
        self.status = status;
    }

    pub fn update_task_command(&mut self, command: String) -> Result<(), AgentError> {
        if command.trim().is_empty() {
            return Err(AgentError::TaskError("Command cannot be empty".to_string()));
        }
        self.command = command;
        Ok(())
    }

    pub fn update_task_human_in_loop(&mut self, human_in_loop: bool) {
        self.human_in_loop = human_in_loop;
    }

    pub fn update_task_tool_to_use(&mut self, tool_to_use: Vec<String>) -> Result<(), AgentError> {
        if tool_to_use.is_empty() {
            return Err(AgentError::TaskError("Tool list cannot be empty".to_string()));
        }
        self.tool_to_use = tool_to_use;
        Ok(())
    }

    pub fn update_task_priority(&mut self, priority: u32) -> Result<(), AgentError> {
        if priority > 10 {
            return Err(AgentError::TaskError("Priority cannot exceed 10".to_string()));
        }
        self.priority = priority;
        Ok(())
    }

    pub fn update_task_description(&mut self, description: String) -> Result<(), AgentError> {
        if description.trim().is_empty() {
            return Err(AgentError::TaskError("Description cannot be empty".to_string()));
        }
        self.description = description;
        Ok(())
    }

    pub fn update_task_title(&mut self, title: String) -> Result<(), AgentError> {
        if title.trim().is_empty() {
            return Err(AgentError::TaskError("Title cannot be empty".to_string()));
        }
        self.title = title;
        Ok(())
    }

    pub fn update_task_id(&mut self, id: String) -> Result<(), AgentError> {
        if id.trim().is_empty() {
            return Err(AgentError::TaskError("ID cannot be empty".to_string()));
        }
        self.id = id;
        Ok(())
    }
}

pub struct Llm {
    // Placeholder fields for LLM
    client: reqwest::Client,
    base_url: String,
    model: String,
    api_key: String,
}

impl Llm {
    pub fn new(base_url: String, model: String, api_key: String) -> Self {
        Llm {
            client: reqwest::Client::new(),
            base_url,
            model,
            api_key,
        }
    }

    pub fn generate_tasks(&self, _prompt: &str) -> String {
        // TODO: Implement actual LLM generation
        let payload = json!({
            "model": self.model,
            "messages": Message, 
            "temprature": 0.7, 
            "max_tokens": 2048,
        });

        let res = slef.client.post(self.base_url)
        .header("Authorization", format!("Bearer {}", self.api_key))
        .json(&payload)
        .send()
        .await?
        .map_err(|e| AgentError::LlmError(e.to_string()))?;
        String::new()
    }

}

pub struct Message {
    pub role: String,
    pub content: String,
}

pub struct Agent {
    pub llm: Llm,
    pub tools: Vec<Tool>,
    pub history: Vec<Message>,
    pub max_iterations: u32,
}

impl Agent {
    pub fn new(llm: Llm, tools: Vec<Tool>, max_iterations: u32) -> Self {
        Agent {
            llm,
            tools,
            history: Vec::new(),
            max_iterations,
        }
    }
    
    pub fn think(&self, user_request: &str) -> Result<Vec<Task>, AgentError> {
        if user_request.trim().is_empty() {
            return Err(AgentError::InvalidInput("User request cannot be empty".to_string()));
        }
        
        let prompt = format!(
            "You are a helpful assistant.\nUser request: {}\n\nThink about what needs to be done.",
            user_request
        );
        self.llm.generate_tasks(&prompt)
    }
    
    pub fn act(&self, user_request: &str, _tasks: Vec<Task>, _tool: &Tool) -> Result<ToolResult, AgentError> {
        if user_request.trim().is_empty() {
            return Err(AgentError::InvalidInput("User request cannot be empty".to_string()));
        }

        let prompt = format!(    
            "You are a helpful assistant.\nUser request: {}\n\nThink about what needs to be done.",
            user_request
        );
        self.llm.generate_action(&prompt)
    }
}
