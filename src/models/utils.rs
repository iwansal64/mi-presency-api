use std::collections::HashMap;
use mongodb::bson::oid::ObjectId;
use crate::models::Model;

/// For converting from user request to be understood by MongoDB
pub fn string_to_oid(oid_text: &str) -> Option<ObjectId> {
    match ObjectId::parse_str(oid_text) {
        Ok(result_oid) => {
            Some(result_oid)
        }
        Err(_) => {
            None
        }
    }
}

fn insert_to_document<T: Into<mongodb::bson::Bson>>(target_document: &mut mongodb::bson::Document, key: &str, value: Option<T>) {
    match value {
        Some(val) => {
            target_document.insert(key, val);
        },
        None => ()
    }
}

fn hashmap_item_to_oid(hashmap: &HashMap<String, Option<String>>, key: &str) -> Option<ObjectId> {
    hashmap.get(key)
    .and_then(|k| k.as_ref())
    .and_then(|k_str| string_to_oid(k_str))
}

/// For communication between Rust and MongoDB
pub fn hashmap_to_model_document(hashmap: &HashMap<String, Option<String>>, model: Model) -> mongodb::bson::Document {
    match model {
        Model::Student(_) => {
            let mut retval: mongodb::bson::Document = mongodb::bson::Document::new();
            
            /*

            "_id": hashmap_item_to_oid(&hashmap, "id"),
            "name": hashmap.get("name"),
            "class_id": hashmap_item_to_oid(&hashmap, "class_id"),
            "card_id": hashmap.get("card_id"),
            
            */

            insert_to_document(&mut retval, "_id", hashmap_item_to_oid(&hashmap, "id"));
            insert_to_document(&mut retval, "name", hashmap.get("name").and_then(|x| x.clone()));
            insert_to_document(&mut retval, "class_id", hashmap_item_to_oid(&hashmap, "class_id"));
            insert_to_document(&mut retval, "card_id", hashmap.get("card_id").and_then(|x| x.clone()));

            retval
        },
        Model::Teacher(_) => {
            // doc! {
            //     "_id": hashmap_item_to_oid(&hashmap, "id"),
            //     "name": hashmap.get("name"),
            //     "pass": hashmap.get("pass"),
            // };
            let mut retval: mongodb::bson::Document = mongodb::bson::Document::new();
            
            /*
            
            "_id": hashmap_item_to_oid(&hashmap, "id"),
            "name": hashmap.get("name"),
            "pass": hashmap.get("pass"),
            
            */

            insert_to_document(&mut retval, "_id", hashmap_item_to_oid(&hashmap, "id"));
            insert_to_document(&mut retval, "name", hashmap.get("name").and_then(|x| x.clone()));
            insert_to_document(&mut retval, "pass", hashmap.get("pass").and_then(|x| x.clone()));

            retval
        }
    }
}