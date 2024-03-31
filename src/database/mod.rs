use std::collections::HashMap;

use crate::{Student, Teacher, models::Model};

use mongodb::{bson::doc, results::{DeleteResult, InsertOneResult, UpdateResult}, sync::{Client, Collection}};
use crate::models::utils::hashmap_to_model_document;

pub struct Database {
    pub student_database: DatabaseType,
    pub teacher_database: DatabaseType
}

impl Database{
    pub fn new(mongodb_uri: String, database_name: String) -> Self {
        let student_database: DatabaseType = DatabaseType::new(mongodb_uri.clone(), database_name.clone(), DatabaseTypeVariant::StudentDB { collection: None });
        let teacher_database: DatabaseType = DatabaseType::new(mongodb_uri.clone(), database_name.clone(), DatabaseTypeVariant::TeacherDB { collection: None });

        Self {
            student_database,
            teacher_database,
        }
    }
}

pub struct DatabaseType {
    mongodb_uri: String,
    database_name: String,
    database: DatabaseTypeVariant,
}


enum DatabaseTypeVariant {
    StudentDB {
        collection: Option<Collection<Student>>
    },
    TeacherDB {
        collection: Option<Collection<Teacher>>
    }
}

impl DatabaseType {
    fn new(mongodb_uri: String, database_name: String, variant: DatabaseTypeVariant) -> Self {
        Self {
            mongodb_uri,
            database_name,
            database: variant
        }
    }

    pub fn init(&mut self, collection_name:&str) -> Result<bool, bool> {
        let client = Client::with_uri_str(&self.mongodb_uri);
        match client {
            Ok(client) => {
                let db = client.database(self.database_name.as_str());
                match &mut self.database {
                    DatabaseTypeVariant::StudentDB { collection } => {
                        let col: Collection<Student> = db.collection(&collection_name);
                        *collection = Some(col);
                    },
                    DatabaseTypeVariant::TeacherDB { collection } => {
                        let col: Collection<Teacher> = db.collection(&collection_name);
                        *collection = Some(col);
                    }
                };
                Ok(true)
            }
            Err(_) => Err(false)
        }
    }

    pub fn get_data(&self, params: mongodb::bson::Document) -> Result<Model, String> {
        match &self.database {
            //? HANDLE STUDENT DATABASE VARIANT
            DatabaseTypeVariant::StudentDB { collection } => {
                match collection {
                    Some(col) => {
                        let result: Result<Option<Student>, mongodb::error::Error>= col.find_one(params, None);
                        match result {
                            Ok(student_data) => {
                                match student_data {
                                    Some(data) => Ok(Model::Student(data)),
                                    None => Err(String::from("Student's not found!"))
                                }
                            },
                            Err(err) => Err(String::from(format!("Error has found when trying to get student data! Error: {}", err)))
                        }
                    }
                    None => {
                        Err(String::from("student collection is not initialized yet!"))
                    }
                }
            },
            //? HANDLE TEACHER DATABASE VARIANT
            DatabaseTypeVariant::TeacherDB { collection } => {
                match collection {
                    Some(col) => {
                        let result: Result<Option<Teacher>, mongodb::error::Error>= col.find_one(params, None);
                        match result {
                            Ok(teacher_data) => {
                                match teacher_data {
                                    Some(data) => Ok(Model::Teacher(data)),
                                    None => Err(String::from("Teacher's not found!"))
                                }
                            },
                            Err(err) => Err(String::from(format!("Error has found when trying to get teacher data! Error: {}", err)))
                        }

                    }
                    None => {
                        Err(String::from("teacher collection is not initialized yet!"))
                    }
                }
            }
        }
    }

    pub fn get_datas(&self, params: mongodb::bson::Document) -> Result<Vec<Model>, String>{
        match &self.database {
            //? HANDLE STUDENT DATABASE VARIANT
            DatabaseTypeVariant::StudentDB { collection } => {
                match collection {
                    Some(col) => {
                        let result: Result<mongodb::sync::Cursor<Student>, mongodb::error::Error> = col.find(params, None);
                        match result {
                            Ok(cursor) => {
                                let mut students_data: Vec<Model> = Vec::<Model>::new();
                                for cursor_item in cursor {
                                    match cursor_item {
                                        Ok(data) => {
                                            students_data.push(Model::Student(data))
                                        },
                                        Err(_) => ()
                                    }
                                }

                                Ok(students_data)
                            },
                            Err(err) => Err(err.to_string())
                        }
                    }
                    None => {
                        Err(String::from("student collection is not initialized yet!"))
                    }
                }
            },
            //? HANDLE TEACHER DATABASE VARIANT
            DatabaseTypeVariant::TeacherDB { collection } => {
                match collection {
                    Some(col) => {
                        let result: Result<mongodb::sync::Cursor<Teacher>, mongodb::error::Error> = col.find(params, None);
                        match result {
                            Ok(cursor) => {
                                let mut teachers_data: Vec<Model> = Vec::<Model>::new();
                                for cursor_item in cursor {
                                    match cursor_item {
                                        Ok(data) => {
                                            teachers_data.push(Model::Teacher(data))
                                        },
                                        Err(_) => ()
                                    }
                                }

                                Ok(teachers_data)
                            },
                            Err(err) => Err(err.to_string())
                        }
                    }
                    None => {
                        Err(String::from("teacher collection is not initialized yet!"))
                    }
                }
            }
        }
    }

    pub fn insert_data(&self, new_data: Model) -> Result<InsertOneResult, String> {
        match &self.database {
            //? HANDLE STUDENT DATABASE VARIANT
            DatabaseTypeVariant::StudentDB { collection } => {
                match collection {
                    Some(col) => {
                        match new_data {
                            Model::Student(data) => {
                                match col.insert_one(data, None) {
                                    Ok(res) => Ok(res),
                                    Err(err) => Err(err.to_string())
                                }
                            },
                            _ => {
                                Err(String::from("Unexpected variable type when trying to insert data"))
                            }
                        }
                    },
                    None => {
                        Err(String::from("teacher collection is not initialized yet!"))
                    }
                }
            },
            //? HANDLE TEACHER DATABASE VARIANT
            DatabaseTypeVariant::TeacherDB { collection } => {
                match collection {
                    Some(col) => {
                        match new_data {
                            Model::Teacher(data) => {
                                match col.insert_one(data, None) {
                                    Ok(res) => Ok(res),
                                    Err(err) => Err(err.to_string())
                                }
                            }
                            _ => {
                                Err(String::from("Unexpected variable type when trying to insert data"))
                            }
                        }
                    },
                    None => {
                        Err(String::from("teacher collection is not initialized yet!"))
                    }
                }
            }
        }
    }

    pub fn update_data(&self, params: mongodb::bson::Document, new_data: Model) -> Result<UpdateResult, String> {
        match &self.database {
            //? HANDLE STUDENT DATABASE VARIANT
            DatabaseTypeVariant::StudentDB { collection } => {
                match collection {
                    Some(col) => {
                        match new_data {
                            Model::Student(student) => {
                                let student_data_hashmap = HashMap::<String, Option<String>>::from([
                                    (String::from("_id"), student.id.and_then(|id| Some(id.to_string()))),
                                    (String::from("name"), student.name),
                                    (String::from("class_id"), student.class_id.and_then(|id| Some(id.to_string()))),
                                    (String::from("card_id"), student.card_id),
                                ]);
                                
                                let student: mongodb::bson::Document = hashmap_to_model_document(&student_data_hashmap, Model::Student(Student::empty()));
                                let result = col.update_many(params, doc! {"$set": student}, None);
                                match result {
                                    Ok(res) => Ok(res),
                                    Err(err) => Err(err.to_string())
                                }
                            },
                            _ => Err("Unexpected variable type when trying to insert data".to_string())
                        }
                    },
                    None => Err("student collection is not initialized yet!".to_string())
                }
            },
            //? HANDLE TEACHER DATABASE VARIANT
            DatabaseTypeVariant::TeacherDB { collection } => {
                match collection {
                    Some(col) => {
                        match new_data {
                            Model::Teacher(teacher) => {
                                let teacher_data_hashmap = HashMap::<String, Option<String>>::from([
                                    (String::from("_id"), teacher.id.and_then(|id| Some(id.to_string()))),
                                    (String::from("name"), teacher.name),
                                    (String::from("pass"), teacher.pass),
                                ]);
                                
                                let teacher: mongodb::bson::Document = hashmap_to_model_document(&teacher_data_hashmap, Model::Teacher(Teacher::empty()));
                                let result = col.update_many(params, doc! {"$set": teacher}, None);
                                match result {
                                    Ok(res) => Ok(res),
                                    Err(err) => Err(err.to_string())
                                }
                            },
                            _ => Err("Unexpected variable type when trying to insert data".to_string())
                        }
                    },
                    None => Err("teacher collection is not initialized yet!".to_string())
                }
            }
        }
    }

    pub fn delete_data(&self, params: mongodb::bson::Document) -> Result<DeleteResult, String> {
        match &self.database {
            DatabaseTypeVariant::StudentDB { collection } => {
                match collection {
                    Some(col) => {
                        let result = col.delete_one(params, None);
                        match result {
                            Ok(return_value) => Ok(return_value),
                            Err(err) => Err(err.to_string())
                        }
                    },
                    None => Err("student collection is not initialized yet!".to_string())
                }
            },
            DatabaseTypeVariant::TeacherDB { collection } => {
                match collection {
                    Some(col) => {
                        let result = col.delete_one(params, None);
                        match result {
                            Ok(return_value) => Ok(return_value),
                            Err(err) => Err(err.to_string())
                        }
                    },
                    None => Err("teacher collection is not initialized yet!".to_string())
                }
            }
        }
    }
}