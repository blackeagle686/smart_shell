/* 
    AGENT steps:
        1. Analysze the user request
        2. Thinkning and chose the proper tools
        3. Execute the tools some tools (human in the loop)
        4. Analysze the result
        5. REACT Reflector 
 */ 

use crate::tools::{Tool, ToolResult};
use serde::{Deserialize, Serialize};
use serde_json::json;

use std::fmt;

#[derive(Debug)]
pub enum AgentError {
    InvalidInput(String),
    LlmError(String),
    TaskError(String),
    RateLimitExceeded(String),
}

impl fmt::Display for AgentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AgentError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            AgentError::LlmError(msg) => write!(f, "LLM processing error: {}", msg),
            AgentError::TaskError(msg) => write!(f, "Task error: {}", msg),
            AgentError::RateLimitExceeded(msg) => write!(f, "Rate limit exceeded: {}", msg),
        }
    }
}

impl std::error::Error for AgentError {}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum TaskStatus {
    Pending,
    Completed,
    Failed,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
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

    pub async fn generate_tasks(&self, messages: &[Message]) -> Result<(Vec<Task>, usize), AgentError> {
        let payload = json!({
            "model": self.model,
            "messages": messages, 
            "temperature": 0.7, 
            "max_tokens": 2048,
        });

        let res = self.client.post(&self.base_url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await
            .map_err(|e| AgentError::LlmError(e.to_string()))?; 
            
        if res.status().is_success() {
            let body: serde_json::Value = res.json().await
                .map_err(|e| AgentError::LlmError(e.to_string()))?;
                
            let tokens = body["usage"]["total_tokens"].as_u64().unwrap_or(0) as usize;
                
            if let Some(content) = body["choices"][0]["message"]["content"].as_str() {
                // Parse lightweight format
                let mut cmd = String::new();
                let mut hil = true;
                
                for line in content.lines() {
                    let line = line.trim();
                    if line.starts_with("CMD:") {
                        cmd = line.strip_prefix("CMD:").unwrap().trim().to_string();
                    } else if line.starts_with("HIL:") {
                        let val = line.strip_prefix("HIL:").unwrap().trim();
                        hil = val.eq_ignore_ascii_case("true");
                    }
                }
                
                if cmd.is_empty() {
                    // Fallback if the model just outputted raw text
                    cmd = content.replace('`', "").trim().to_string();
                }

                let task = Task {
                    id: "1".to_string(),
                    title: "Generated Command".to_string(),
                    description: "Direct command execution".to_string(),
                    tool_to_use: vec!["shell".to_string()],
                    command: cmd,
                    priority: 1,
                    human_in_loop: hil,
                    status: TaskStatus::Pending,
                };
                
                return Ok((vec![task], tokens));
            }
            Err(AgentError::LlmError("No content field in LLM response".to_string()))
        } else {
            let status = res.status();
            let text = res.text().await.unwrap_or_default();
            Err(AgentError::LlmError(format!("Request failed {}: {}", status, text)))
        }
    }

    pub async fn generate_action(&self, _prompt: &str) -> Result<ToolResult, AgentError> {
        // TODO: Implement actual LLM action generation
        Ok(ToolResult {
            status: "0".to_string(),
            output: String::new(),
            error: String::new(),
        })
    }

    pub async fn generate_summary(&self, prompt: &str) -> Result<(String, usize), AgentError> {
        let payload = json!({
            "model": self.model,
            "messages": [
                {"role": "system", "content": "You are a concise AI. Summarize or format the shell output beautifully in max 3 sentences to save tokens."},
                {"role": "user", "content": prompt}
            ], 
            "temperature": 0.3, 
            "max_tokens": 150,
        });

        let res = self.client.post(&self.base_url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await
            .map_err(|e| AgentError::LlmError(e.to_string()))?; 
            
        if res.status().is_success() {
            let body: serde_json::Value = res.json().await
                .map_err(|e| AgentError::LlmError(e.to_string()))?;
                
            let tokens = body["usage"]["total_tokens"].as_u64().unwrap_or(0) as usize;
                
            if let Some(content) = body["choices"][0]["message"]["content"].as_str() {
                return Ok((content.to_string(), tokens));
            }
            Err(AgentError::LlmError("No content field in LLM response".to_string()))
        } else {
            let status = res.status();
            let text = res.text().await.unwrap_or_default();
            Err(AgentError::LlmError(format!("Request failed {}: {}", status, text)))
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Message {
    pub role: String,
    pub content: String,
}

pub struct Agent {
    pub llm: Llm,
    pub tools: Vec<Tool>,
    pub history: Vec<Message>,
    pub max_iterations: u32,
    pub total_tokens_used: usize,
    pub token_limit: usize,
}

impl Agent {
    pub fn new(llm: Llm, tools: Vec<Tool>, max_iterations: u32) -> Self {
        Agent {
            llm,
            tools,
            history: Vec::new(),
            max_iterations,
            total_tokens_used: 0,
            token_limit: 1_000_000,
        }
    }
    
    pub async fn think(&mut self, user_request: &str) -> Result<Vec<Task>, AgentError> {
        if self.total_tokens_used >= self.token_limit {
            return Err(AgentError::RateLimitExceeded(format!("Token limit of {} reached. Current usage: {}", self.token_limit, self.total_tokens_used)));
        }

        if user_request.trim().is_empty() {
            return Err(AgentError::InvalidInput("User request cannot be empty".to_string()));
        }
        
        let prompt = format!(
            "You are a fast Linux shell AI. User request: {}\n\n\
            Return exactly ONE bash command to satisfy this request.\n\
            Format your response EXACTLY like this, with NO markdown and NO explanations:\n\
            CMD: <bash command here>\n\
            HIL: <true/false>\n\n\
            RULES FOR HIL:\n\
            - MUST be false for ALL safe/read operations (like ls, cat, echo, pwd, uname, free, df, grep, find).\n\
            - MUST be true ONLY for destructive/write operations (like rm, mv, cp, mkdir, touch, systemctl, chmod).\n\n\
            CRITICAL: DO NOT explain the command. DO NOT write conversational text. Output ONLY the CMD and HIL lines.\n\n\
            Example:\n\
            User request: delete the build folder\n\
            CMD: rm -rf build\n\
            HIL: true",
            user_request
        );

        self.history.push(Message { role: "user".to_string(), content: prompt });
        
        // Maintain a tiny context window (max 3) to guarantee ultra-fast CPU inference
        // High context size causes slow Prompt Evaluation (Prefill) on CPUs
        if self.history.len() > 3 {
            let overflow = self.history.len() - 3;
            self.history.drain(0..overflow);
        }

        let (tasks, tokens) = self.llm.generate_tasks(&self.history).await?;
        self.total_tokens_used += tokens;
        Ok(tasks)
    }
    
    pub async fn act(&mut self, user_request: &str, _tasks: Vec<Task>, _tool: &Tool) -> Result<ToolResult, AgentError> {
        if self.total_tokens_used >= self.token_limit {
            return Err(AgentError::RateLimitExceeded(format!("Token limit of {} reached. Current usage: {}", self.token_limit, self.total_tokens_used)));
        }

        if user_request.trim().is_empty() {
            return Err(AgentError::InvalidInput("User request cannot be empty".to_string()));
        }

        let prompt = format!(    
            "You are a helpful assistant.\nUser request: {}\n\nThink about what needs to be done.",
            user_request
        );
        self.llm.generate_action(&prompt).await
    }

    pub async fn summarize_output(&mut self, command: &str, output: &str) -> Result<String, AgentError> {
        if self.total_tokens_used >= self.token_limit {
            return Err(AgentError::RateLimitExceeded(format!("Token limit of {} reached. Current usage: {}", self.token_limit, self.total_tokens_used)));
        }

        // Truncate output to save tokens safely at char boundaries
        let truncated = if output.len() > 600 {
            let mut end = 600;
            while end > 0 && !output.is_char_boundary(end) {
                end -= 1;
            }
            format!("{}...\n[TRUNCATED to save tokens]", &output[..end])
        } else {
            output.to_string()
        };

        let prompt = format!(
            "Command executed: `{}`\nOutput:\n{}\n\nProvide a very short, beautifully formatted summary of this output.",
            command, truncated
        );

        let (summary, tokens) = self.llm.generate_summary(&prompt).await?;
        self.total_tokens_used += tokens;
        
        // Also keep a record of the AI's summary in the history context window if we want
        self.history.push(Message { role: "assistant".to_string(), content: summary.clone() });
        if self.history.len() > 10 {
            let overflow = self.history.len() - 10;
            self.history.drain(0..overflow);
        }
        
        Ok(summary)
    }
}
