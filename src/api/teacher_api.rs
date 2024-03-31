use std::collections::HashMap;

use mongodb::{results::{DeleteResult, InsertOneResult, UpdateResult}, bson::doc};
use rocket::{get, post, serde::json::Json, State};
use serde::{Serialize, Deserialize};
use crate::models::{Model, Teacher, utils::hashmap_to_model_document, ErrorResponse};
use crate::database::Database;


#[get("/teacher")]
pub fn get_all_teachers(db: &State<Database>) -> Result<Json<Vec<Teacher>>, Json<ErrorResponse>> {
    let results = db.teacher_database.get_datas(doc! {});

    match results {
        Ok(models_data) => {
            let mut teachers_data:Vec<Teacher> = Vec::<Teacher>::new();
            for model in models_data {
                match model {
                    Model::Teacher(teacher_data) => {
                        teachers_data.push(teacher_data);
                    },
                    Model::Student(_) => ()
                }
            }
            Ok(Json(teachers_data))
        },
        Err(err) => Err(Json(ErrorResponse::new(err.to_string(), 0)))
    }
}

#[get("/teacher/search?<_id>&<name>&<pass>")]
pub fn get_teacher(db: &State<Database>, _id: Option<String>, name: Option<String>, pass: Option<String>) -> Result<Json<Teacher>, Json<ErrorResponse>> {
    let teacher_params: HashMap<String, Option<String>> = HashMap::<String, Option<String>>::from([
        (String::from("id"), _id),
        (String::from("name"), name),
        (String::from("pass"), pass),
    ]);

    let teacher_params: mongodb::bson::Document = hashmap_to_model_document(&teacher_params, Model::Teacher(Teacher::empty()));
    println!("{:?}", teacher_params);
    
    if teacher_params.iter().collect::<Vec<(&String, &mongodb::bson::Bson)>>().len() <= 0 {
        return Err(Json(ErrorResponse::new("Well you should try to give the correct parameters next time..".to_string(), 1)));
    }
    
    let result: Result<Model, String> = db.teacher_database.get_data(teacher_params);

    match result {
        Ok(data) => {
            match data {
                Model::Teacher(teacher) => Ok(Json(teacher)),
                Model::Student(_) => Err(Json(ErrorResponse::new("Unexpected value : student has occured when trying to get teacher data!".to_string(), 2)))
            }
        },
        Err(err) => {
            Err(Json(ErrorResponse::new(format!("Unexpected error has occured. err : {}", err), 0)))
        }
    }
}

#[post("/teacher", data = "<new_teacher>")]
pub fn post_teacher(db: &State<Database>, new_teacher: Json<Teacher>) -> Result<Json<InsertOneResult>, Json<ErrorResponse>> {
    let new_teacher_data: Teacher = new_teacher.0;
    let result = db.teacher_database.insert_data(Model::Teacher(new_teacher_data));
    match result {
        Ok(res) => Ok(Json(res)),
        Err(err) => Err(Json(ErrorResponse::new(err, 0)))
    }
}

#[derive(Serialize, Deserialize)]
pub struct PutParamsData {
    params: HashMap<String, String>, 
    new_data: Teacher,
}


#[put("/teacher", data = "<params>")]
pub fn put_teacher(db: &State<Database>, params: Json<PutParamsData>) -> Result<Json<UpdateResult>, Json<ErrorResponse>> {
    let new_data = Model::Teacher(params.0.new_data);
    let params = params.0.params.into_iter().map(|(key, value)| (key, mongodb::bson::Bson::String(value))).collect::<HashMap<String, mongodb::bson::Bson>>();
    let params = mongodb::bson::Document::from_iter(params.into_iter());
    let result = db.teacher_database.update_data(params, new_data);
    match result {
        Ok(return_value) => Ok(Json(return_value)),
        Err(err) => Err(Json(ErrorResponse::new(err.to_string(), 0)))
    }
}

#[delete("/teacher", data = "<params>")]
pub fn delete_teacher(db: &State<Database>, params: Json<Teacher>) -> Result<Json<DeleteResult>, Json<ErrorResponse>>{
    let params: HashMap<String, Option<String>> = HashMap::<String, Option<String>>::from([
        (String::from("_id"), params.0.id.map(|id| id.to_string())),
        (String::from("name"), params.0.name),
        (String::from("pass"), params.0.pass),
    ]);
    let params: mongodb::bson::Document = hashmap_to_model_document(&params, Model::Teacher(Teacher::empty()));
    let result = db.teacher_database.delete_data(params);
    match result {
        Ok(return_value) => Ok(Json(return_value)),
        Err(err) => Err(Json(ErrorResponse::new(err.to_string(), 0)))
    }
}