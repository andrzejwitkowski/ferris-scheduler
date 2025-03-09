# Ferris Scheduler
MongoDB-based task scheduling library for Rust applications, built with a hexagonal architecture approach.

## Overview

Ferris Scheduler provides a flexible and type-safe way to define, schedule, and execute tasks in a Rust application. It uses MongoDB for persistence and follows hexagonal architecture principles to separate business logic from infrastructure concerns

## Features

- Type-safe task parameters using Serde serialization/deserialization
- Hexagonal architecture for clean separation of concerns
- MongoDB persistence for task storage
- Custom procedural macros for easy task definition
- Generic task handling with strong typing## Project Structure


## Usage

### Defining a Task

```rustuse
serde::{Serialize, Deserialize};
use ferris_scheduler::task_definition;
use ferris_scheduler::domain::port::task_scheduler::{TaskHandler, TaskSchedulerError};

#[derive(Default, Serialize, Deserialize)]
struct EmailParams {    
    recipient: String,    
    subject: String,    
    body: String,
}
    
#[task_definition(EmailParams)]
pub struct EmailTask;

impl TaskHandler<EmailParams> for EmailTask {
    fn handle(&self, params: EmailParams) -> Result<(),     TaskSchedulerError> {
                // Implement email sending logic        
                println!("Sending email to: {}", params.recipient);        
                println!("Subject: {}", params.subject);        
                println!("Body: {}", params.body);                
                Ok(())    
    }    
                
    fn task_type(&self) -> &'static str {        
        "email-task"    
    }            
}
```

### Scheduling a Task

```rustuse

use ferris_scheduler::domain::task::Task;// Create task parameters
let email_params = EmailParams {    
    recipient: "user@example.com".to_string(),    
    subject: "Hello".to_string(),    
    body: "This is a test email".to_string(),
};// Create and schedule the task
let task = Task::new(
    "task-123".to_string(), 
    "email-task".to_string()
    ).with_parameters(&email_params)?;scheduler.schedule_task(task)?;
```

### Registering Task Handlers

```rustuaw
// Initialize the scheduler
let mut scheduler = MongoDbScheduler::new("mongodb://localhost:27017");
// Register task handlers
EmailTask::register(&mut scheduler)?;
```