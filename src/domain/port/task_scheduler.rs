use crate::domain::task::Task;
use serde::{Serialize, Deserialize};
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub enum TaskSchedulerError {
    TaskAlreadyDefined,
    TaskNotFound,
    TaskHandlerNotFound,
    SerializationError,
    DatabaseError(String),
    ParameterError(String),
}

pub trait TaskHandler<P: Serialize + for<'de> Deserialize<'de>> {
    fn handle(&self, params: P) -> Result<(), TaskSchedulerError>;
    fn task_type(&self) -> &'static str;
}

pub trait TaskScheduler {
    fn register_handler<P>(&mut self, handler: Box<dyn TaskHandler<P>>) -> Result<(), TaskSchedulerError>
    where 
        P: Serialize + for<'de> Deserialize<'de> + 'static;
        
    fn schedule_task<P>(&self, task_type: &str, task_id: String, params: P) -> Result<(), TaskSchedulerError>
    where 
        P: Serialize + 'static;
        
    fn get_task(&self, task_id: &str) -> Result<Option<Task>, TaskSchedulerError>;
    
    fn update_task(&self, task: Task) -> Result<(), TaskSchedulerError>;
    
    fn execute_task(&self, task: &Task) -> Result<(), TaskSchedulerError>;
}

pub struct TypedTaskDefinition<P> 
where 
    P: Serialize + for<'de> Deserialize<'de>
{
    pub task_type: String,
    pub handler: Box<dyn TaskHandler<P>>,
    _marker: PhantomData<P>,
}

impl<P> TypedTaskDefinition<P> 
where 
    P: Serialize + for<'de> Deserialize<'de>
{
    pub fn new(task_type: String, handler: Box<dyn TaskHandler<P>>) -> Self {
        Self {
            task_type,
            handler,
            _marker: PhantomData,
        }
    }
}
