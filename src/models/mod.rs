mod student_model;
mod teacher_model;
pub mod utils;

use serde::{Deserialize, Serialize};
pub use student_model::Student;
pub use teacher_model::Teacher;

#[derive(Debug)]
pub enum Model {
    Student(Student),
    Teacher(Teacher),
}

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub message: String,
    pub error_code: u16,
    pub error_line: u32
}

impl ErrorResponse {
    pub fn new(message: String, error_code: u16) -> Self {
        Self {
            message, 
            error_code,
            error_line:line!()
        }
    }
}