use chrono::{DateTime, TimeZone};
use chrono::offset::{Utc};
use savaged_libs::player_character::chargen_data::ChargenData;
use savaged_libs::save_db_row::SaveDBRow;
use wasm_bindgen::JsValue;
use web_sys::DomException;
use standard_components::libs::local_storage_shortcuts::set_local_storage_string;
use standard_components::libs::local_storage_shortcuts::get_local_storage_string;
use indexed_db_futures::{prelude::*, js_sys};
use gloo_console::log;

static INDEX_DB_DB_NAME: &str = "savaged";
static INDEX_DB_BOOKS_STORE_NAME: &str = "books";
static INDEX_DB_SAVES_STORE_NAME: &str = "saves";
static INDEX_DB_VERSION: u32 = 4;

#[derive(Debug)]
pub struct ChargenSyncUpdateResults {
    books: u32,
    edges: u32,
    hindrances: u32,
    latest_updated_on: DateTime<Utc>,
}

pub struct SavesSyncUpdateResults {
    saves: u32,
    latest_updated_on: DateTime<Utc>,
}

async fn _create_tables( db_req: &mut OpenDbRequest ) {
    db_req.set_on_upgrade_needed(
        Some(|evt: &IdbVersionChangeEvent| -> Result<(), JsValue> {

            log!("_create_tables?");
            // Check if the object store exists; create it if it doesn't
            if let None = evt.db().object_store_names().find(|n| n == INDEX_DB_BOOKS_STORE_NAME) {
                let _ = evt.db().create_object_store(INDEX_DB_BOOKS_STORE_NAME).unwrap();
                log!("Created indexed_db store", INDEX_DB_BOOKS_STORE_NAME);
            }
            if let None = evt.db().object_store_names().find(|n| n == "chargen_edges" ) {
                let _ = evt.db().create_object_store("chargen_edges").unwrap();
                log!("Created indexed_db store", "chargen_edges");

            }
            if let None = evt.db().object_store_names().find(|n| n == "chargen_hindrances" ) {
                let _ = evt.db().create_object_store("chargen_hindrances").unwrap();
                log!("Created indexed_db store", "chargen_hindrances");

            }

            if let None = evt.db().object_store_names().find(|n| n == INDEX_DB_SAVES_STORE_NAME ) {
                let _ = evt.db().create_object_store(INDEX_DB_SAVES_STORE_NAME).unwrap();
                log!("Created indexed_db store", INDEX_DB_SAVES_STORE_NAME);

            }
            log!("_create_tables end");
            Ok(())
        })
    );

}


// pub async fn check_and_upgrade_index_db_stores() {
//     let db_req_option = IdbDatabase::open_u32(INDEX_DB_DB_NAME, INDEX_DB_VERSION);
//     log!("check_and_upgrade_index_db_stores called");
//     match db_req_option {
//         Ok( mut db_req ) => {
//             let db: IdbDatabase = db_req.into_future().await.unwrap();

//             if let None = db.object_store_names().find(|n| n == INDEX_DB_BOOKS_STORE_NAME) {
//                 let _ = db.create_object_store(INDEX_DB_BOOKS_STORE_NAME).unwrap();
//                 log!("Created indexed_db store", INDEX_DB_BOOKS_STORE_NAME);
//             }
//             if let None = db.object_store_names().find(|n| n == "chargen_edges" ) {
//                 let _ = db.create_object_store("chargen_edges").unwrap();
//                 log!("Created indexed_db store", "chargen_edges");
//             }
//             if let None = db.object_store_names().find(|n| n == "chargen_hindrances" ) {
//                 let _ = db.create_object_store("chargen_hindrances").unwrap();
//                 log!("Created indexed_db store", "chargen_hindrances");
//             }


//             if let None = db.object_store_names().find(|n| n == INDEX_DB_SAVES_STORE_NAME ) {
//                 let _ = db.create_object_store(INDEX_DB_SAVES_STORE_NAME).unwrap();
//                 log!("Created indexed_db store", INDEX_DB_SAVES_STORE_NAME);
//             }

//         }
//         Err( _err ) => {


//         }
//     }
//     log!("check_and_upgrade_index_db_stores completed");

// }
pub async fn index_db_save_saves(
    saves: Vec<SaveDBRow>,
) -> SavesSyncUpdateResults {
    let mut update_stats: SavesSyncUpdateResults = SavesSyncUpdateResults {
        saves: 0,
        latest_updated_on: Utc.with_ymd_and_hms(1990, 1, 1, 0, 0, 0).unwrap(),
    };

    let mut db_req: OpenDbRequest = IdbDatabase::open_u32(INDEX_DB_DB_NAME, INDEX_DB_VERSION).unwrap();

    log!("index_db_save_saves 1");

    _create_tables( &mut db_req ).await;


    /* Saves */
    let db: IdbDatabase = db_req.into_future().await.unwrap();

    log!("index_db_save_saves 2");
    for  save in &saves {

        match save.updated_on {
            Some( updated_on ) => {
                if update_stats.latest_updated_on < updated_on {
                    update_stats.latest_updated_on = updated_on;
                }
            }
            None => {}
        }
        let tx: IdbTransaction = db
            .transaction_on_one_with_mode(
                INDEX_DB_SAVES_STORE_NAME,
                IdbTransactionMode::Readwrite
            ).unwrap();
        let store: IdbObjectStore = tx.object_store(INDEX_DB_SAVES_STORE_NAME).unwrap();


        let value_to_put: JsValue = serde_json::to_string(&save).unwrap().into();

        // log!( format!("value_to_put {:?}", value_to_put) );
        let err = store.put_key_val_owned(save.uuid.clone(), &value_to_put);
        match err {
            Ok( _ ) => {
                let _ =  tx.await.into_result();
                log!( format!("index_db_save_saves saved key ID:{} ", save.uuid) );
                update_stats.saves += 1;
            }
            Err( _err ) => {
                log!( format!("index_db_save_saves store data error ID:{}  /{:?}", save.id, _err) );
            }
        }

    }
    db.close();

    return update_stats;
}

pub async fn index_db_save_chargen_data(
    chargen_data: ChargenData,
) -> ChargenSyncUpdateResults {
    let mut update_stats: ChargenSyncUpdateResults = ChargenSyncUpdateResults {
        books: 0,
        edges: 0,
        hindrances: 0,
        latest_updated_on: Utc.with_ymd_and_hms(1990, 1, 1, 0, 0, 0).unwrap(),
    };

    let mut db_req: OpenDbRequest = IdbDatabase::open_u32(INDEX_DB_DB_NAME, INDEX_DB_VERSION).unwrap();

    log!("index_db_save_chargen_data 1");

    _create_tables( &mut db_req ).await;


    /* Books */
    let db: IdbDatabase = db_req.into_future().await.unwrap();

    log!("index_db_save_chargen_data 2");
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
    db.close();

    /* Edges */
    let db_req: OpenDbRequest = IdbDatabase::open_u32(INDEX_DB_DB_NAME, INDEX_DB_VERSION).unwrap();



    let db: IdbDatabase = db_req.into_future().await.unwrap();
    log!("index_db_save_chargen_data 3");
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
    db.close();

    /* Hindrances */
    let db_req: OpenDbRequest = IdbDatabase::open_u32(INDEX_DB_DB_NAME, INDEX_DB_VERSION).unwrap();


    let db: IdbDatabase = db_req.into_future().await.unwrap();
    log!("index_db_save_chargen_data 4");
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
    db.close();

    log!("index_db_save_chargen_data 5");
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

pub async fn get_saves_from_index_db() -> Option<Vec<SaveDBRow>> {


    let db_req_option = IdbDatabase::open_u32(INDEX_DB_DB_NAME, INDEX_DB_VERSION);


    match db_req_option {
        Ok( mut db_req ) => {

            _create_tables( &mut db_req ).await;

            // Saves.
            let db: IdbDatabase = db_req.into_future().await.unwrap();
            let tx = db.transaction_on_one(INDEX_DB_SAVES_STORE_NAME).unwrap();
            let store = tx.object_store(INDEX_DB_SAVES_STORE_NAME).unwrap();

            let mut saves: Vec<SaveDBRow> = Vec::new();
            // TODO

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
                                saves.push( item );
                            }
                            Err( _err ) => {
                                return None;
                            }
                        }
                    }
                    Err( _err ) => {

                    }
                }

            }

            return Some(saves);
        }
        Err( _err ) => {

        }
    }

    return None;
}

pub async fn get_chargen_data_from_index_db() -> Option<ChargenData> {
    let mut chargen_data = ChargenData{
        books: Vec::new(),
        edges: Vec::new(),
        hindrances: Vec::new(),
    };


    let db_req_option = IdbDatabase::open_u32(INDEX_DB_DB_NAME, INDEX_DB_VERSION);

    match db_req_option {
        Ok( mut db_req ) => {

            _create_tables( &mut db_req ).await;

            // Books.
            let db: IdbDatabase = db_req.into_future().await.unwrap();
            let tx = db.transaction_on_one(INDEX_DB_BOOKS_STORE_NAME).unwrap();
            let store = tx.object_store(INDEX_DB_BOOKS_STORE_NAME).unwrap();

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
                    Err( _err ) => {

                    }
                }

            }
            db.close();
        }

        Err (_err) => {
            return None;
        }
    }


    let db_req_option2 = IdbDatabase::open_u32(INDEX_DB_DB_NAME, INDEX_DB_VERSION);

    match db_req_option2 {
        Ok( mut db_req ) => {
            _create_tables( &mut db_req ).await;
            // Edges.
            let db: IdbDatabase = db_req.into_future().await.unwrap();
            let tx = db.transaction_on_one("chargen_edges").unwrap();
            let store = tx.object_store("chargen_edges").unwrap();


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
                    Err( _err ) => {

                    }
                }

            }
            db.close();
        }

        Err (_err) => {
            return None;
        }
    }

    let db_req_option3 = IdbDatabase::open_u32(INDEX_DB_DB_NAME, INDEX_DB_VERSION);

    match db_req_option3 {
        Ok( mut db_req ) => {
            _create_tables( &mut db_req ).await;
            // Hindrances.
            let db: IdbDatabase = db_req.into_future().await.unwrap();
            let tx = db.transaction_on_one("chargen_hindrances").unwrap();
            let store = tx.object_store("chargen_hindrances").unwrap();


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
                    Err( _err ) => {

                    }
                }

            }
            db.close();

        }

        Err (_err) => {
            return None;
        }
    }



    return Some( chargen_data );
}