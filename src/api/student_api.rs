use std::collections::HashMap;

use mongodb::{results::{InsertOneResult, DeleteResult, UpdateResult}, bson::doc};
use rocket::{get, post, put, delete, serde::json::Json, State};
use serde::{Deserialize, Serialize};
use crate::models::{Model, Student, utils::hashmap_to_model_document, ErrorResponse};
use crate::database::Database;

#[get("/student")]
pub fn get_all_students(db: &State<Database>) -> Result<Json<Vec<Student>>, Json<ErrorResponse>> {
    let results = db.student_database.get_datas(doc! {});

    match results {
        Ok(models_data) => {
            let mut students_data:Vec<Student> = Vec::<Student>::new();
            for model in models_data {
                match model {
                    Model::Student(student_data) => {
                        students_data.push(student_data);
                    },
                    Model::Teacher(_) => ()
                }
            }
            Ok(Json(students_data))
        },
        Err(err) => Err(Json(ErrorResponse::new(err, 0)))
    }
}

#[get("/student/search?<_id>&<name>&<class_id>&<card_id>")]
pub fn get_student(db: &State<Database>, _id: Option<String>, name: Option<String>, class_id: Option<String>, card_id: Option<String>) -> Result<Json<Student>, Json<ErrorResponse>> {
    let student_params: HashMap<String, Option<String>> = HashMap::<String, Option<String>>::from([
        (String::from("id"), _id),
        (String::from("name"), name),
        (String::from("card_id"), card_id),
        (String::from("class_id"), class_id),
    ]);

    let student_params: mongodb::bson::Document = hashmap_to_model_document(&student_params, Model::Student(Student::empty()));
    println!("{:?}", student_params);
    
    if student_params.iter().collect::<Vec<(&String, &mongodb::bson::Bson)>>().len() <= 0 {
        return Err(Json(ErrorResponse::new(String::from("Well you should try to give the correct parameters next time.."), 1)));
    }
    
    let result: Result<Model, String> = db.student_database.get_data(student_params);

    match result {
        Ok(data) => {
            match data {
                Model::Student(student) => Ok(Json(student)),
                Model::Teacher(_) => Err(Json(ErrorResponse::new(String::from("Unexpected value : teacher has occured when trying to get student data!"), 2)))
            }
        },
        Err(err) => {
            Err(Json(ErrorResponse::new(format!("Unexpected error has occured. err : {}", err), 0)))
        }
    }
}

#[post("/student", data = "<new_student>")]
pub fn post_student(db: &State<Database>, new_student: Json<Student>) -> Result<Json<InsertOneResult>, Json<ErrorResponse>> {
    let new_student_data: Student = new_student.0;
    let result = db.student_database.insert_data(Model::Student(new_student_data));
    match result {
        Ok(res) => Ok(Json(res)),
        Err(err) => Err(Json(ErrorResponse::new(err, 0)))
    }
}

#[derive(Serialize, Deserialize)]
pub struct PutParamsData {
    params: HashMap<String, String>, 
    new_data: Student,
}

#[put("/student", data = "<params>")]
pub fn put_student(db: &State<Database>, params: Json<PutParamsData>) -> Result<Json<UpdateResult>, Json<ErrorResponse>> {
    // Convert new_data: Student to Model
    let new_data = Model::Student(params.0.new_data);
    
    // Convert HashMap<String, String> to HashMap<String, Bson>
    let params = params.0.params.into_iter().map(|(key, value)| (key, mongodb::bson::Bson::String(value))).collect::<HashMap<String, mongodb::bson::Bson>>();
    // Convert HashMap<String, Bson> to mongodb::bson::Document
    let params = mongodb::bson::Document::from_iter(params.into_iter());
    
    let result = db.student_database.update_data(params, new_data);
    match result {
        Ok(return_value) => Ok(Json(return_value)),
        Err(err) => Err(Json(ErrorResponse::new(err.to_string(), 0)))
    }
}

#[delete("/student", data = "<params>")]
pub fn delete_student(db: &State<Database>, params: Json<Student>) -> Result<Json<DeleteResult>, Json<ErrorResponse>>{
    let params: HashMap<String, Option<String>> = HashMap::<String, Option<String>>::from([
    (String::from("_id"), params.0.id.map(|id| id.to_string())),
        (String::from("name"), params.0.name),
        (String::from("card_id"), params.0.card_id),
        (String::from("class_id"), params.0.class_id.map(|id| id.to_string())),
    ]);
    let params: mongodb::bson::Document = hashmap_to_model_document(&params, Model::Student(Student::empty()));
    let result = db.student_database.delete_data(params);
    match result {
        Ok(return_value) => Ok(Json(return_value)),
        Err(err) => Err(Json(ErrorResponse::new(err.to_string(), 0)))
    }
}