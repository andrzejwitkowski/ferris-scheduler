use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub task_id: String,
    pub task_type: String,
    pub status: TaskStatus,
    pub parameters: JsonValue
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskStatus {
    Pending,
    Completed,
    InProgress,
    Error,
}

impl Task {
    pub fn new(task_id: String, task_type: String) -> Self {
        Self {
            task_id,
            task_type,
            status: TaskStatus::Pending,
            parameters: JsonValue::Null,
        }
    }
    
    pub fn with_parameters<T: Serialize>(mut self, params: &T) -> Result<Self, serde_json::Error> {
        self.parameters = serde_json::to_value(params)?;
        Ok(self)
    }
    
    pub fn get_parameters<T: for<'de> Deserialize<'de>>(&self) -> Result<T, serde_json::Error> {
        serde_json::from_value(self.parameters.clone())
    }
}