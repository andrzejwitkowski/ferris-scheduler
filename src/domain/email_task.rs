use crate::domain::port::task_scheduler::{TaskHandler, TaskSchedulerError};
use serde::{Serialize, Deserialize};
use crate::task_definition;

#[derive(Default, Serialize, Deserialize)]
struct EmailParams {
    recipient: String,
    subject: String,
    body: String,
}

#[task_definition(EmailParams)]
pub struct EmailTask;

impl TaskHandler<EmailParams> for EmailTask {
    fn handle(&self, params: EmailParams) -> Result<(), TaskSchedulerError> {
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