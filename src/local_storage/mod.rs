use chrono::{DateTime, TimeZone};
use chrono::offset::{Utc};
use savaged_libs::player_character::chargen_data::ChargenData;
use wasm_bindgen::JsValue;
use web_sys::DomException;
use std::collections::HashMap;
use indexed_db_futures::{prelude::*, js_sys};
use gloo_console::log;

static INDEX_DB_DB_NAME: &str = "savaged";
static INDEX_DB_BOOKS_STORE_NAME: &str = "books";
static INDEX_DB_SAVES_STORE_NAME: &str = "saves";
static INDEX_DB_VERSION: u32 = 3;

#[derive(Debug)]
pub struct SyncUpdateResults {
    books: u32,
    edges: u32,
    hindrances: u32,
    latest_updated_on: DateTime<Utc>,
}

pub async fn index_db_save_chargen_data(
    chargen_data: ChargenData,
) -> SyncUpdateResults {
    let mut update_stats: SyncUpdateResults = SyncUpdateResults {
        books: 0,
        edges: 0,
        hindrances: 0,
        latest_updated_on: Utc.with_ymd_and_hms(1990, 1, 1, 0, 0, 0).unwrap(),
    };

    let mut db_req: OpenDbRequest = IdbDatabase::open_u32(INDEX_DB_DB_NAME, INDEX_DB_VERSION).unwrap();



    db_req.set_on_upgrade_needed(
        Some(|evt: &IdbVersionChangeEvent| -> Result<(), JsValue> {
            // Check if the object store exists; create it if it doesn't
            if let None = evt.db().object_store_names().find(|n| n == INDEX_DB_BOOKS_STORE_NAME) {
                evt.db().create_object_store(INDEX_DB_BOOKS_STORE_NAME).unwrap();
            }
            if let None = evt.db().object_store_names().find(|n| n == "chargen_edges" ) {
                evt.db().create_object_store("chargen_edges").unwrap();
            }
            if let None = evt.db().object_store_names().find(|n| n == "chargen_hindrances" ) {
                evt.db().create_object_store("chargen_hindrances").unwrap();
            }
            Ok(())
        })
    );


    /* Books */
    let db: IdbDatabase = db_req.into_future().await.unwrap();

    for  book in &chargen_data.books {

        match book.updated_on {
            Some( updated_on ) => {
                if update_stats.latest_updated_on < updated_on {
                    update_stats.latest_updated_on = updated_on;
                }
            }
            None => {}
        }
        let tx: IdbTransaction = db
            .transaction_on_one_with_mode(
                INDEX_DB_BOOKS_STORE_NAME,
                IdbTransactionMode::Readwrite
            ).unwrap();
        let store: IdbObjectStore = tx.object_store(INDEX_DB_BOOKS_STORE_NAME).unwrap();


        let value_to_put: JsValue = serde_json::to_string(&book).unwrap().into();
        store.put_key_val_owned(book.id, &value_to_put).unwrap();
        let _ =  tx.await.into_result();
        update_stats.books += 1;
    }


    /* Edges */
    let mut db_req: OpenDbRequest = IdbDatabase::open_u32(INDEX_DB_DB_NAME, INDEX_DB_VERSION).unwrap();



    let db: IdbDatabase = db_req.into_future().await.unwrap();

    for  item in &chargen_data.edges {

        let tx: IdbTransaction = db
            .transaction_on_one_with_mode(
                "chargen_edges",
                IdbTransactionMode::Readwrite
            ).unwrap();
        let store: IdbObjectStore = tx.object_store("chargen_edges").unwrap();


        match item.updated_on {
            Some( updated_on ) => {
                if update_stats.latest_updated_on < updated_on {
                    update_stats.latest_updated_on = updated_on;
                }
            }
            None => {}
        }

        let value_to_put: JsValue = serde_json::to_string(&item).unwrap().into();
        store.put_key_val_owned(item.id, &value_to_put).unwrap();
        let _ = tx.await.into_result();
        update_stats.edges += 1;
    }

    /* Hindrances */
    let mut db_req: OpenDbRequest = IdbDatabase::open_u32(INDEX_DB_DB_NAME, INDEX_DB_VERSION).unwrap();


    let db: IdbDatabase = db_req.into_future().await.unwrap();

    for  item in &chargen_data.hindrances {

        let tx: IdbTransaction = db
            .transaction_on_one_with_mode(
                "chargen_hindrances",
                IdbTransactionMode::Readwrite
            ).unwrap();
        let store: IdbObjectStore = tx.object_store("chargen_hindrances").unwrap();

        match item.updated_on {
            Some( updated_on ) => {
                if update_stats.latest_updated_on < updated_on {
                    update_stats.latest_updated_on = updated_on;
                }
            }
            None => {}
        }

        let value_to_put: JsValue = serde_json::to_string(&item).unwrap().into();
        store.put_key_val_owned(item.id, &value_to_put).unwrap();
        let _ = tx.await.into_result();
        update_stats.hindrances += 1;
    }

    return update_stats;
}



// pub async fn example() -> Result<(), DomException> {
//     // Open my_db v1
//     let mut db_req: OpenDbRequest = IdbDatabase::open_u32("my_db", 1)?;
//     db_req.set_on_upgrade_needed(Some(|evt: &IdbVersionChangeEvent| -> Result<(), JsValue> {
//         // Check if the object store exists; create it if it doesn't
//         if let None = evt.db().object_store_names().find(|n| n == "my_store") {
//             evt.db().create_object_store("my_store")?;
//         }
//         Ok(())
//     }));

//     let db: IdbDatabase = db_req.into_future().await?;

//     // Insert/overwrite a record
//     let tx: IdbTransaction = db
//       .transaction_on_one_with_mode("my_store", IdbTransactionMode::Readwrite)?;
//     let store: IdbObjectStore = tx.object_store("my_store")?;

//     let value_to_put: JsValue = "Hi there".into();
//     store.put_key_val_owned("my_key", &value_to_put)?;

//     // IDBTransactions can have an Error or an Abort event; into_result() turns both into a
//     // DOMException
//     tx.await.into_result()?;

//     // Delete a record
//     let tx = db.transaction_on_one_with_mode("my_store", IdbTransactionMode::Readwrite)?;
//     let store = tx.object_store("my_store")?;
//     store.delete_owned("my_key")?;
//     tx.await.into_result()?;

//     // Get a record
//     let tx = db.transaction_on_one("my_store")?;
//     let store = tx.object_store("my_store")?;

//     let value: Option<JsValue> = store.get_owned("my_key")?.await?;
//     log!( format!("hello? {:?}", value)) ;

//     // All of the requests in the transaction have already finished so we can just drop it to
//     // avoid the unused future warning, or assign it to _.
//     let _ = tx;

//     Ok(())
// }

pub async fn get_chargen_data_from_index_db() -> Option<ChargenData> {
    let mut chargen_data = ChargenData{
        books: Vec::new(),
        edges: Vec::new(),
        hindrances: Vec::new(),
    };


    let mut db_req_option = IdbDatabase::open_u32(INDEX_DB_DB_NAME, INDEX_DB_VERSION);

    match db_req_option {
        Ok( db_req ) => {

            // Books.
            let db: IdbDatabase = db_req.into_future().await.unwrap();
            let tx = db.transaction_on_one(INDEX_DB_BOOKS_STORE_NAME).unwrap();
            let store = tx.object_store(INDEX_DB_BOOKS_STORE_NAME).unwrap();

            // let moo: Vec<String> = store.get_all().unwrap().into();
            let result = store.get_all()
                .unwrap()
                .await
                .unwrap();


            tx.await.into_result().unwrap();

            let iterator = js_sys::try_iter(&result).unwrap().ok_or_else(|| {
                "need to pass iterable JS values!"
            }).unwrap();


            for row_result in iterator {
                match row_result {
                    Ok( row ) => {
                        let val = row.as_string().unwrap();
                        let item_result = serde_json::from_str(val.as_str() );
                        match item_result {
                            Ok( item ) => {
                                chargen_data.books.push( item );
                            }
                            Err( _err ) => {
                                return None;
                            }
                        }
                    }
                    Err( err_ ) => {

                    }
                }

            }

        }

        Err (_err) => {
            return None;
        }
    }


            let mut db_req_option2 = IdbDatabase::open_u32(INDEX_DB_DB_NAME, INDEX_DB_VERSION);

            match db_req_option2 {
                Ok( db_req ) => {

            // Edges.
            let db: IdbDatabase = db_req.into_future().await.unwrap();
            let tx = db.transaction_on_one("chargen_edges").unwrap();
            let store = tx.object_store("chargen_edges").unwrap();

            // let moo: Vec<String> = store.get_all().unwrap().into();
            let result = store.get_all()
                .unwrap()
                .await
                .unwrap();


            tx.await.into_result().unwrap();

            let iterator = js_sys::try_iter(&result).unwrap().ok_or_else(|| {
                "need to pass iterable JS values!"
            }).unwrap();


            for row_result in iterator {
                match row_result {
                    Ok( row ) => {
                        let val = row.as_string().unwrap();
                        let item_result = serde_json::from_str(val.as_str() );
                        match item_result {
                            Ok( item ) => {
                                chargen_data.edges.push( item );
                            }
                            Err( _err ) => {
                                return None;
                            }
                        }
                    }
                    Err( err_ ) => {

                    }
                }

            }
        }

        Err (_err) => {
            return None;
        }
    }

            let mut db_req_option3 = IdbDatabase::open_u32(INDEX_DB_DB_NAME, INDEX_DB_VERSION);

            match db_req_option3 {
                Ok( db_req ) => {

            // Hindrances.
            let db: IdbDatabase = db_req.into_future().await.unwrap();
            let tx = db.transaction_on_one("chargen_hindrances").unwrap();
            let store = tx.object_store("chargen_hindrances").unwrap();

            // let moo: Vec<String> = store.get_all().unwrap().into();
            let result = store.get_all()
                .unwrap()
                .await
                .unwrap();


            tx.await.into_result().unwrap();

            let iterator = js_sys::try_iter(&result).unwrap().ok_or_else(|| {
                "need to pass iterable JS values!"
            }).unwrap();

            for row_result in iterator {
                match row_result {
                    Ok( row ) => {
                        let val = row.as_string().unwrap();
                        let item_result = serde_json::from_str(val.as_str() );
                        match item_result {
                            Ok( item ) => {
                                chargen_data.hindrances.push( item );
                            }
                            Err( _err ) => {
                                return None;
                            }
                        }
                    }
                    Err( err_ ) => {

                    }
                }

            }

        }

        Err (_err) => {
            return None;
        }
    }



    return Some( chargen_data );
}