mod models;
mod database;
mod api;

#[macro_use]
extern crate rocket;

use models::{Student, Teacher};
use database::Database;
use api::{
        student_api::{
        get_student, 
        post_student, 
        get_all_students,
        put_student,
        delete_student
    }, 
    teacher_api::{
        get_teacher, 
        post_teacher, 
        get_all_teachers,
        put_teacher,
        delete_teacher
    }
};

#[launch]
fn rocket() -> _ {
    let mut db = Database::new(String::from("mongodb+srv://iwancode64:9wJdR9ae6JnltxRI@iwannn.87t4mdg.mongodb.net/"), String::from("mi-attendance-database"));
    db.student_database.init("students").unwrap();
    db.teacher_database.init("teachers").unwrap();
    rocket::build().manage(db).mount("/", routes![
        get_student, 
        get_teacher, 
        post_teacher, 
        post_student, 
        get_all_students, 
        get_all_teachers,
        put_student,
        put_teacher,
        delete_student,
        delete_teacher
    ])
}