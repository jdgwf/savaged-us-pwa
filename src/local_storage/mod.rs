use blob;
use chrono::offset::Utc;
use chrono::{DateTime, TimeZone};
use gloo_console::{log, error};
use indexed_db_futures::js_sys::Uint8Array;
use indexed_db_futures::{js_sys, prelude::*};
use savaged_libs::player_character::game_data_package::GameDataPackage;
use savaged_libs::save_db_row::SaveDBRow;
use standard_components::libs::local_storage_shortcuts::set_local_storage_string;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{DomException, Request, RequestInit, Response};

static INDEX_DB_DB_NAME: &str = "savaged";
static INDEX_DB_BOOKS_STORE_NAME: &str = "books";
static INDEX_DB_SAVES_STORE_NAME: &str = "saves";
static INDEX_DB_VERSION: u32 = 9;

#[derive(Debug)]
pub struct GameDataSyncUpdateResults {
    books: u32,
    edges: u32,
    hindrances: u32,
    gear: u32,
    armor: u32,
    weapons: u32,
    latest_updated_on: DateTime<Utc>,
}

pub struct SavesSyncUpdateResults {
    saves: u32,
    latest_updated_on: DateTime<Utc>,
}

async fn _create_tables(db_req: &mut OpenDbRequest) {
    db_req.set_on_upgrade_needed(Some(|evt: &IdbVersionChangeEvent| -> Result<(), JsValue> {
        // log!("_create_tables?");
        // Check if the object store exists; create it if it doesn't
        if let None = evt
            .db()
            .object_store_names()
            .find(|n| n == INDEX_DB_BOOKS_STORE_NAME)
        {
            let _ = evt.db().create_object_store(INDEX_DB_BOOKS_STORE_NAME);
            // log!("Created indexed_db store", INDEX_DB_BOOKS_STORE_NAME);
        }
        if let None = evt
            .db()
            .object_store_names()
            .find(|n| n == "game_data_edges")
        {
            let _ = evt.db().create_object_store("game_data_edges");
            // log!("Created indexed_db store", "game_data_edges");
        }
        if let None = evt
            .db()
            .object_store_names()
            .find(|n| n == "game_data_hindrances")
        {
            let _ = evt.db().create_object_store("game_data_hindrances");
            // log!("Created indexed_db store", "game_data_hindrances");
        }

        if let None = evt
            .db()
            .object_store_names()
            .find(|n| n == "game_data_gear")
        {
            let _ = evt.db().create_object_store("game_data_gear");
            // log!("Created indexed_db store", "game_data_gear");
        }

        if let None = evt
            .db()
            .object_store_names()
            .find(|n| n == "game_data_armor")
        {
            let _ = evt.db().create_object_store("game_data_armor");
            // log!("Created indexed_db store", "game_data_armor");
        }

        if let None = evt
            .db()
            .object_store_names()
            .find(|n| n == "game_data_weapons")
        {
            let _ = evt.db().create_object_store("game_data_weapons");
            // log!("Created indexed_db store", "game_data_weapons");
        }

        if let None = evt
            .db()
            .object_store_names()
            .find(|n| n == INDEX_DB_SAVES_STORE_NAME)
        {
            let _ = evt
                .db()
                .create_object_store(INDEX_DB_SAVES_STORE_NAME)
                .unwrap();
            // log!("Created indexed_db store", INDEX_DB_SAVES_STORE_NAME);
        }
        // log!("_create_tables end");
        Ok(())
    }));
}

// pub async fn check_and_upgrade_index_db_stores() {
//     let db_req_result = IdbDatabase::open_u32(INDEX_DB_DB_NAME, INDEX_DB_VERSION);
//     log!("check_and_upgrade_index_db_stores called");
//     match db_req_result {
//         Ok( mut db_req ) => {
//             let db: IdbDatabase = db_req.into_future().await.unwrap();

//             if let None = db.object_store_names().find(|n| n == INDEX_DB_BOOKS_STORE_NAME) {
//                 let _ = db.create_object_store(INDEX_DB_BOOKS_STORE_NAME).unwrap();
//                 log!("Created indexed_db store", INDEX_DB_BOOKS_STORE_NAME);
//             }
//             if let None = db.object_store_names().find(|n| n == "game_data_edges" ) {
//                 let _ = db.create_object_store("game_data_edges").unwrap();
//                 log!("Created indexed_db store", "game_data_edges");
//             }
//             if let None = db.object_store_names().find(|n| n == "game_data_hindrances" ) {
//                 let _ = db.create_object_store("game_data_hindrances").unwrap();
//                 log!("Created indexed_db store", "game_data_hindrances");
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

pub async fn index_db_put_save(server_root: String, save: SaveDBRow) -> SavesSyncUpdateResults {
    let mut update_stats: SavesSyncUpdateResults = SavesSyncUpdateResults {
        saves: 0,
        latest_updated_on: Utc.with_ymd_and_hms(1990, 1, 1, 0, 0, 0).unwrap(),
    };

    let db_req_result = IdbDatabase::open_u32(INDEX_DB_DB_NAME, INDEX_DB_VERSION);
    match db_req_result {
        Ok(mut db_req) => {
            _create_tables(&mut db_req).await;

            /* Saves */
            let db: IdbDatabase = db_req.into_future().await.unwrap();

            // log!("index_db_put_save 2");

            let mut edit_save = save.clone();
            match save.updated_on {
                Some(updated_on) => {
                    if update_stats.latest_updated_on < updated_on {
                        update_stats.latest_updated_on = updated_on;
                    }
                }
                None => {}
            }

            edit_save.image_base64_mime = None;
            edit_save.image_base64 = None;
            edit_save.image_token_base64_mime = None;
            edit_save.image_token_base64 = None;

            if !save.imageurl.is_empty() {
                // TODO Fetch Image Data
                let image_url = server_root.clone() + &edit_save.imageurl;
                let (image_data, image_mime) = get_image_file(image_url).await;

                // log!("image_data", image_data);
                // log!("image_mime", image_mime);

                edit_save.image_base64 = Some(image_data);
                edit_save.image_base64_mime = Some(image_mime);
                // TODO Encode to base 64

                // TODO Assign Mime

                // TODO Assign Data
            }

            let tx: IdbTransaction = db
                .transaction_on_one_with_mode(
                    INDEX_DB_SAVES_STORE_NAME,
                    IdbTransactionMode::Readwrite,
                )
                .unwrap();
            let store: IdbObjectStore = tx.object_store(INDEX_DB_SAVES_STORE_NAME).unwrap();

            let value_to_put: JsValue = serde_json::to_string(&edit_save).unwrap().into();

            // log!( format!("value_to_put {:?}", value_to_put) );
            let err = store.put_key_val_owned(save.uuid.clone(), &value_to_put);
            match err {
                Ok(_) => {
                    let _ = tx.await.into_result();
                    // log!( format!("index_db_put_save saved key ID:{} ", save.uuid) );
                    update_stats.saves += 1;
                    match edit_save.updated_on {
                        Some(updated_on) => {
                            if update_stats.latest_updated_on < updated_on {
                                update_stats.latest_updated_on = updated_on;
                            }
                        }
                        None => {}
                    }
                }
                Err(_err) => {
                    log!(format!(
                        "index_db_put_save store data error ID: {} / {:?}",
                        save.id, _err
                    ));
                }
            }

            set_local_storage_string(
                "saves_last_updated",
                update_stats.latest_updated_on.to_string(),
            );
            db.close();
        }

        Err(_) => {}
    }

    // log!("index_db_save_saves 1");

    return update_stats;
}

pub async fn index_db_save_saves(
    server_root: String,
    saves: Vec<SaveDBRow>,
) -> SavesSyncUpdateResults {
    let mut update_stats: SavesSyncUpdateResults = SavesSyncUpdateResults {
        saves: 0,
        latest_updated_on: Utc.with_ymd_and_hms(1990, 1, 1, 0, 0, 0).unwrap(),
    };

    let db_req_result = IdbDatabase::open_u32(INDEX_DB_DB_NAME, INDEX_DB_VERSION);
    match db_req_result {
        Ok(mut db_req) => {
            _create_tables(&mut db_req).await;

            /* Saves */
            let db: IdbDatabase = db_req.into_future().await.unwrap();

            // log!("index_db_save_saves 2");
            for save in &saves {
                let mut edit_save = save.clone();
                match save.updated_on {
                    Some(updated_on) => {
                        if update_stats.latest_updated_on < updated_on {
                            update_stats.latest_updated_on = updated_on;
                        }
                    }
                    None => {}
                }

                edit_save.image_base64_mime = None;
                edit_save.image_base64 = None;
                edit_save.image_token_base64_mime = None;
                edit_save.image_token_base64 = None;

                if !save.imageurl.is_empty() {
                    // TODO Fetch Image Data
                    let image_url = server_root.clone() + &edit_save.imageurl;
                    let (image_data, image_mime) = get_image_file(image_url).await;

                    // log!("image_data", image_data);
                    // log!("image_mime", image_mime);

                    edit_save.image_base64 = Some(image_data);
                    edit_save.image_base64_mime = Some(image_mime);
                    // TODO Encode to base 64

                    // TODO Assign Mime

                    // TODO Assign Data
                }

                let tx: IdbTransaction = db
                    .transaction_on_one_with_mode(
                        INDEX_DB_SAVES_STORE_NAME,
                        IdbTransactionMode::Readwrite,
                    )
                    .unwrap();
                let store: IdbObjectStore = tx.object_store(INDEX_DB_SAVES_STORE_NAME).unwrap();

                let value_to_put: JsValue = serde_json::to_string(&edit_save).unwrap().into();

                // log!( format!("value_to_put {:?}", value_to_put) );
                let err = store.put_key_val_owned(save.uuid.clone(), &value_to_put);
                match err {
                    Ok(_) => {
                        let _ = tx.await.into_result();
                        // log!( format!("index_db_save_saves saved key ID:{} ", save.uuid) );
                        update_stats.saves += 1;
                        match edit_save.updated_on {
                            Some(updated_on) => {
                                if update_stats.latest_updated_on < updated_on {
                                    update_stats.latest_updated_on = updated_on;
                                }
                            }
                            None => {}
                        }
                    }
                    Err(_err) => {
                        log!(format!(
                            "index_db_save_saves store data error ID: {} / {:?}",
                            save.id, _err
                        ));
                    }
                }
            }
            set_local_storage_string(
                "saves_last_updated",
                update_stats.latest_updated_on.to_string(),
            );
            db.close();
        }

        Err(_) => {}
    }

    // log!("index_db_save_saves 1");

    return update_stats;
}

pub async fn index_db_save_game_data(game_data: GameDataPackage) -> GameDataSyncUpdateResults {
    let mut update_stats: GameDataSyncUpdateResults = GameDataSyncUpdateResults {
        books: 0,
        edges: 0,
        gear: 0,
        armor: 0,
        weapons: 0,
        hindrances: 0,
        latest_updated_on: Utc.with_ymd_and_hms(1990, 1, 1, 0, 0, 0).unwrap(),
    };

    let db_req_result = IdbDatabase::open_u32(INDEX_DB_DB_NAME, INDEX_DB_VERSION);
    match db_req_result {
        Ok(mut db_req) => {
            // log!("index_db_save_game_data 1");

            _create_tables(&mut db_req).await;

            /* Books */
            let db: IdbDatabase = db_req.into_future().await.unwrap();

            // log!("index_db_save_game_data 2");
            for book in &game_data.books {
                match book.updated_on {
                    Some(updated_on) => {
                        if update_stats.latest_updated_on < updated_on {
                            update_stats.latest_updated_on = updated_on;
                        }
                    }
                    None => {}
                }
                let tx: IdbTransaction = db
                    .transaction_on_one_with_mode(
                        INDEX_DB_BOOKS_STORE_NAME,
                        IdbTransactionMode::Readwrite,
                    )
                    .unwrap();

                let store_result: Result<IdbObjectStore, DomException> =
                    tx.object_store(INDEX_DB_BOOKS_STORE_NAME);

                match store_result {
                    Ok(store) => {
                        let value_to_put: JsValue = serde_json::to_string(&book).unwrap().into();
                        let _ = store.put_key_val_owned(book.id, &value_to_put);
                        let _ = tx.await.into_result();
                    }
                    Err(_err) => {}
                }

                update_stats.books += 1;
            }
            db.close();
        }
        Err(_err) => {}
    }
    /* Edges */
    let db_req_result = IdbDatabase::open_u32(INDEX_DB_DB_NAME, INDEX_DB_VERSION);
    match db_req_result {
        Ok(db_req) => {
            let db: IdbDatabase = db_req.into_future().await.unwrap();
            // log!("index_db_save_game_data 3");
            for item in &game_data.edges {
                let tx: IdbTransaction = db
                    .transaction_on_one_with_mode("game_data_edges", IdbTransactionMode::Readwrite)
                    .unwrap();
                let store_result: Result<IdbObjectStore, DomException> =
                    tx.object_store("game_data_edges");

                match store_result {
                    Ok(store) => {
                        match item.updated_on {
                            Some(updated_on) => {
                                if update_stats.latest_updated_on < updated_on {
                                    update_stats.latest_updated_on = updated_on;
                                }
                            }
                            None => {}
                        }

                        let value_to_put: JsValue = serde_json::to_string(&item).unwrap().into();
                        let _ = store.put_key_val_owned(item.id, &value_to_put);
                        let _ = tx.await.into_result();
                    }
                    Err(_err) => {}
                }

                update_stats.edges += 1;
            }
            db.close();
        }
        Err(_err) => {}
    }
    /* Hindrances */
    let db_req_result = IdbDatabase::open_u32(INDEX_DB_DB_NAME, INDEX_DB_VERSION);
    match db_req_result {
        Ok(db_req) => {
            let db: IdbDatabase = db_req.into_future().await.unwrap();
            // log!("index_db_save_game_data 4");
            for item in &game_data.hindrances {
                let tx: IdbTransaction = db
                    .transaction_on_one_with_mode(
                        "game_data_hindrances",
                        IdbTransactionMode::Readwrite,
                    )
                    .unwrap();
                let store_result: Result<IdbObjectStore, DomException> =
                    tx.object_store("game_data_hindrances");

                match store_result {
                    Ok(store) => {
                        match item.updated_on {
                            Some(updated_on) => {
                                if update_stats.latest_updated_on < updated_on {
                                    update_stats.latest_updated_on = updated_on;
                                }
                            }
                            None => {}
                        }

                        let value_to_put: JsValue = serde_json::to_string(&item).unwrap().into();
                        let _ = store.put_key_val_owned(item.id, &value_to_put);
                        let _ = tx.await.into_result();
                    }
                    Err(_err) => {}
                }

                update_stats.hindrances += 1;
            }
            db.close();
        }
        Err(_err) => {}
    }

    /* weapons */
    let db_req_result = IdbDatabase::open_u32(INDEX_DB_DB_NAME, INDEX_DB_VERSION);
    match db_req_result {
        Ok(db_req) => {
            let db: IdbDatabase = db_req.into_future().await.unwrap();
            // log!("index_db_save_game_data 4");
            for item in &game_data.weapons {
                let tx: IdbTransaction = db
                    .transaction_on_one_with_mode(
                        "game_data_weapons",
                        IdbTransactionMode::Readwrite,
                    )
                    .unwrap();
                let store_result: Result<IdbObjectStore, DomException> =
                    tx.object_store("game_data_weapons");

                match store_result {
                    Ok(store) => {
                        match item.updated_on {
                            Some(updated_on) => {
                                if update_stats.latest_updated_on < updated_on {
                                    update_stats.latest_updated_on = updated_on;
                                }
                            }
                            None => {}
                        }

                        let value_to_put: JsValue = serde_json::to_string(&item).unwrap().into();
                        let _ = store.put_key_val_owned(item.id, &value_to_put);
                        let _ = tx.await.into_result();
                    }
                    Err(_err) => {}
                }

                update_stats.weapons += 1;
            }
            db.close();
        }
        Err(_err) => {}
    }

    /* armor */
    let db_req_result = IdbDatabase::open_u32(INDEX_DB_DB_NAME, INDEX_DB_VERSION);
    match db_req_result {
        Ok(db_req) => {
            let db: IdbDatabase = db_req.into_future().await.unwrap();
            // log!("index_db_save_game_data armor 4");
            for item in &game_data.armor {
                let tx: IdbTransaction = db
                    .transaction_on_one_with_mode("game_data_armor", IdbTransactionMode::Readwrite)
                    .unwrap();
                let store_result: Result<IdbObjectStore, DomException> =
                    tx.object_store("game_data_armor");

                match store_result {
                    Ok(store) => {
                        match item.updated_on {
                            Some(updated_on) => {
                                if update_stats.latest_updated_on < updated_on {
                                    update_stats.latest_updated_on = updated_on;
                                }
                            }
                            None => {}
                        }
                        // log!(format!("index_db_save_game_data armor 5 {} {} {}",  &item.id, &item.uuid, &item.name) );

                        let value_to_put: JsValue = serde_json::to_string(&item).unwrap().into();
                        let _ = store.put_key_val_owned(item.id, &value_to_put);
                        let _ = tx.await.into_result();
                    }
                    Err(_err) => {}
                }

                update_stats.armor += 1;
            }
            db.close();
        }
        Err(_err) => {}
    }

    /* gear */
    let db_req_result = IdbDatabase::open_u32(INDEX_DB_DB_NAME, INDEX_DB_VERSION);
    match db_req_result {
        Ok(db_req) => {
            let db: IdbDatabase = db_req.into_future().await.unwrap();
            // log!("index_db_save_game_data 4");
            for item in &game_data.gear {
                let tx: IdbTransaction = db
                    .transaction_on_one_with_mode("game_data_gear", IdbTransactionMode::Readwrite)
                    .unwrap();
                let store_result: Result<IdbObjectStore, DomException> =
                    tx.object_store("game_data_gear");

                match store_result {
                    Ok(store) => {
                        match item.updated_on {
                            Some(updated_on) => {
                                if update_stats.latest_updated_on < updated_on {
                                    update_stats.latest_updated_on = updated_on;
                                }
                            }
                            None => {}
                        }

                        let value_to_put: JsValue = serde_json::to_string(&item).unwrap().into();
                        let _ = store.put_key_val_owned(item.id, &value_to_put);
                        let _ = tx.await.into_result();
                    }
                    Err(_err) => {}
                }

                update_stats.gear += 1;
            }
            db.close();
        }
        Err(_err) => {}
    }

    set_local_storage_string(
        "game_data_user_level",
        format!("{:?}", &game_data.data_level).to_owned(),
    );
    set_local_storage_string(
        "game_data_last_updated",
        update_stats.latest_updated_on.to_string(),
    );

    // log!("index_db_save_game_data 5", format!("{:?}", &game_data.data_level ).to_owned());
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
    log!("get_saves_from_index_db called");
    let db_req_result = IdbDatabase::open_u32(INDEX_DB_DB_NAME, INDEX_DB_VERSION);

    match db_req_result {
        Ok(mut db_req) => {
            _create_tables(&mut db_req).await;

            // Saves.
            let db: IdbDatabase = db_req.into_future().await.unwrap();
            let tx = db.transaction_on_one(INDEX_DB_SAVES_STORE_NAME).unwrap();
            let store = tx.object_store(INDEX_DB_SAVES_STORE_NAME).unwrap();

            let mut saves: Vec<SaveDBRow> = Vec::new();
            // TODO

            let result = store.get_all().unwrap().await.unwrap();

            tx.await.into_result().unwrap();

            let iterator = js_sys::try_iter(&result)
                .unwrap()
                .ok_or_else(|| "need to pass iterable JS values!")
                .unwrap();

            for row_result in iterator {
                match row_result {
                    Ok(row) => {
                        let val = row.as_string().unwrap();
                        let item_result = serde_json::from_str(val.as_str());
                        match item_result {
                            Ok(item) => {
                                saves.push(item);
                            }
                            Err(_err) => {
                                return None;
                            }
                        }
                    }
                    Err(_err) => {}
                }
            }

            return Some(saves);
        }
        Err(_err) => {}
    }

    return None;
}

pub async fn clear_all_local_data() {
    clear_game_data_local_data().await;
    clear_saves_local_data().await;
}

pub async fn clear_saves_local_data() {
    clear_data_store(INDEX_DB_SAVES_STORE_NAME).await;
}

pub async fn clear_data_store(store_name: &str) {
    let db_req_result = IdbDatabase::open_u32(INDEX_DB_DB_NAME, INDEX_DB_VERSION);

    // log!("clear_data_store called", store_name);
    match db_req_result {
        Ok(db_req) => {
            let db: IdbDatabase = db_req.into_future().await.unwrap();
            // let tx = db.transaction_on_one(store_name).unwrap();
            let tx: IdbTransaction = db
                .transaction_on_one_with_mode(store_name, IdbTransactionMode::Readwrite)
                .unwrap();
            let store = tx.object_store(store_name).unwrap();

            let rv = store.clear();

            match rv {
                Ok(_val) => {
                    // log!( format!("clear_data_store cleared {}", store_name) );
                }
                Err(err) => {
                    error!(format!("clear_data_store errpr {} {:?}", store_name, err));
                }
            }
        }
        Err(err) => {
            error!(format!("clear_data_store error! {:?}", err));
        }
    }
}

pub async fn clear_game_data_local_data() {
    clear_data_store(INDEX_DB_BOOKS_STORE_NAME).await;
    clear_data_store("game_data_edges").await;
    clear_data_store("game_data_hindrances").await;
    clear_data_store("game_data_gear").await;
    clear_data_store("game_data_weapons").await;
    clear_data_store("game_data_armor").await;
}

pub async fn get_game_data_from_index_db() -> Option<GameDataPackage> {
    let mut game_data = GameDataPackage::default();



    let db_req_result_books = IdbDatabase::open_u32(INDEX_DB_DB_NAME, INDEX_DB_VERSION);

    match db_req_result_books {
        Ok(mut db_req) => {
            // log!("get_game_data_from_index_db CALLED");
            _create_tables(&mut db_req).await;

            // log!("get_game_data_from_index_db CALLED 2");
            // Books.
            let db: IdbDatabase = db_req.into_future().await.unwrap();
            let tx = db.transaction_on_one(INDEX_DB_BOOKS_STORE_NAME).unwrap();
            let store = tx.object_store(INDEX_DB_BOOKS_STORE_NAME).unwrap();

            let result = store.get_all().unwrap().await.unwrap();

            tx.await.into_result().unwrap();

            let iterator_result = js_sys::try_iter(&result)
                .unwrap()
                .ok_or_else(|| "need to pass iterable JS values!");

            match iterator_result {
                Ok(iterator) => {
                    for row_result in iterator {
                        match row_result {
                            Ok(row) => {
                                let val = row.as_string().unwrap();
                                let item_result = serde_json::from_str(val.as_str());
                                match item_result {
                                    Ok(item) => {
                                        game_data.books.push(item);
                                    }
                                    Err( err) => {
                                        error!( format!("books error 1 {:?}", err ));
                                        return None;
                                    }
                                 }
                            }
                            Err(err) => {
                                error!("books error 2", err);
                            }
                        }
                    }
                    db.close();
                }

                Err( err) => {
                    error!("books error 3", err);
                    return None;
                }
            }

            }
            Err( err) => {
                error!("books error 4", err);
                return None;
            }
        }

    let db_req_result_edges = IdbDatabase::open_u32(INDEX_DB_DB_NAME, INDEX_DB_VERSION);

    match db_req_result_edges {
        Ok(mut db_req) => {
            // Edges.
            let db: IdbDatabase = db_req.into_future().await.unwrap();
            let tx = db.transaction_on_one("game_data_edges").unwrap();
            let store = tx.object_store("game_data_edges").unwrap();

            let result = store.get_all().unwrap().await.unwrap();

            tx.await.into_result().unwrap();

            let iterator = js_sys::try_iter(&result)
                .unwrap()
                .ok_or_else(|| "need to pass iterable JS values!")
                .unwrap();

            for row_result in iterator {
                match row_result {
                    Ok(row) => {
                        let val = row.as_string().unwrap();
                        let item_result = serde_json::from_str(val.as_str());
                        match item_result {
                            Ok(item) => {
                                game_data.edges.push(item);
                            }
                            Err(err) => {
                                error!( format!("game_data_edges error 1 {:?}", err ));
                                return None;
                            }
                        }
                    }
                    Err(err) => {
                        error!("game_data_edges error 2", err);
                    }
                }
            }
            db.close();
        }

        Err( err) => {
            error!("game_data_edges error 3", err);
            return None;
        }
    }

    let db_req_result_hindrances = IdbDatabase::open_u32(INDEX_DB_DB_NAME, INDEX_DB_VERSION);

    match db_req_result_hindrances {
        Ok(mut db_req) => {
            // Hindrances.
            let db: IdbDatabase = db_req.into_future().await.unwrap();
            let tx = db.transaction_on_one("game_data_hindrances").unwrap();
            let store = tx.object_store("game_data_hindrances").unwrap();

            let result = store.get_all().unwrap().await.unwrap();

            tx.await.into_result().unwrap();

            let iterator = js_sys::try_iter(&result)
                .unwrap()
                .ok_or_else(|| "need to pass iterable JS values!")
                .unwrap();

            for row_result in iterator {
                match row_result {
                    Ok(row) => {
                        let val = row.as_string().unwrap();
                        let item_result = serde_json::from_str(val.as_str());
                        match item_result {
                            Ok(item) => {
                                game_data.hindrances.push(item);
                            }
                            Err(err) => {
                                error!( format!("game_data_hindrances error 1 {:?}", err ));
                                return None;
                            }
                        }
                    }
                    Err(err) => {
                        error!("game_data_hindrances error 2", err);
                    }
                }
            }
            db.close();
        }

        Err( err) => {
            error!("game_data_hindrances error 3", err);
            return None;
        }
    }

    let db_req_result_armor = IdbDatabase::open_u32(INDEX_DB_DB_NAME, INDEX_DB_VERSION);

    match db_req_result_armor {
        Ok(mut db_req) => {
            // Armor.
            let db: IdbDatabase = db_req.into_future().await.unwrap();
            let tx = db.transaction_on_one("game_data_armor").unwrap();
            let store = tx.object_store("game_data_armor").unwrap();

            let result = store.get_all().unwrap().await.unwrap();

            tx.await.into_result().unwrap();

            let iterator = js_sys::try_iter(&result)
                .unwrap()
                .ok_or_else(|| "need to pass iterable JS values!")
                .unwrap();

            for row_result in iterator {
                match row_result {
                    Ok(row) => {
                        let val = row.as_string().unwrap();
                        let item_result = serde_json::from_str(val.as_str());
                        match item_result {
                            Ok(item) => {
                                game_data.armor.push(item);
                            }
                            Err(err) => {
                                error!( format!("game_data_armor error 1 {:?}", err ));
                                return None;
                            }
                        }
                    }
                    Err(err) => {
                        error!("game_data_armor error 2", err);
                    }
                }
            }
            db.close();
        }

        Err( err) => {
            error!("game_data_armor error 3", err);
            return None;
        }
    }

    let db_req_result_weapons = IdbDatabase::open_u32(INDEX_DB_DB_NAME, INDEX_DB_VERSION);

    match db_req_result_weapons {
        Ok(mut db_req) => {
            // Weapons.
            let db: IdbDatabase = db_req.into_future().await.unwrap();
            let tx = db.transaction_on_one("game_data_weapons").unwrap();
            let store = tx.object_store("game_data_weapons").unwrap();

            let result = store.get_all().unwrap().await.unwrap();

            tx.await.into_result().unwrap();

            let iterator = js_sys::try_iter(&result)
                .unwrap()
                .ok_or_else(|| "need to pass iterable JS values!")
                .unwrap();

            for row_result in iterator {
                match row_result {
                    Ok(row) => {
                        let val = row.as_string().unwrap();
                        let item_result = serde_json::from_str(val.as_str());
                        match item_result {
                            Ok(item) => {
                                game_data.weapons.push(item);
                            }
                            Err(err) => {
                                error!( format!("game_data_weapons error 1 {:?}", err ));
                                return None;
                            }
                        }
                    }
                    Err(err) => {
                        error!("game_data_weapons error 2", err);
                    }
                }
            }
            db.close();
        }

        Err( err) => {
            error!("game_data_weapons error 3", err);
            return None;
        }
    }

    let db_req_result_gear = IdbDatabase::open_u32(INDEX_DB_DB_NAME, INDEX_DB_VERSION);

    match db_req_result_gear {
        Ok(mut db_req) => {
            // Gear.
            let db: IdbDatabase = db_req.into_future().await.unwrap();
            let tx = db.transaction_on_one("game_data_gear").unwrap();
            let store = tx.object_store("game_data_gear").unwrap();

            let result = store.get_all().unwrap().await.unwrap();

            tx.await.into_result().unwrap();

            let iterator = js_sys::try_iter(&result)
                .unwrap()
                .ok_or_else(|| "need to pass iterable JS values!")
                .unwrap();

            for row_result in iterator {
                match row_result {
                    Ok(row) => {
                        let val = row.as_string().unwrap();
                        let item_result = serde_json::from_str(val.as_str());
                        match item_result {
                            Ok(item) => {
                                game_data.gear.push(item);
                            }
                            Err(err) => {
                                error!( format!("game_data_gear error 1 {:?}", err ));
                                return None;
                            }
                        }
                    }
                    Err(err) => {
                        error!("game_data_gear error 2", err);
                    }
                }
            }
            db.close();
        }

        Err( err) => {
            error!("game_data_gear error 3", err);
            return None;
        }
    }

    // log!("local_storage game_data.books.len()", game_data.books.len());
    // log!("local_storage game_data.edges.len()", game_data.edges.len());
    // log!("local_storage game_data.hindrances.len()", game_data.hindrances.len());
    // log!("local_storage game_data.gear.len()", game_data.gear.len());
    // log!("local_storage game_data.armor.len()", game_data.armor.len());
    // log!("local_storage game_data.weapons.len()", game_data.weapons.len());

    return Some(game_data);
}

pub async fn get_image_file(url: String) -> (String, String) {
    let mut opts = RequestInit::new();

    opts.method("GET");

    let request = Request::new_with_str_and_init(&url, &opts).unwrap();

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request))
        .await
        .unwrap();

    let resp: Response = resp_value.dyn_into().unwrap();

    let data = JsFuture::from(resp.blob().unwrap()).await.unwrap();

    let blob: web_sys::Blob = data.dyn_into().unwrap();

    let array_buffer = JsFuture::from(blob.array_buffer()).await.unwrap();

    let array = Uint8Array::new(&array_buffer);
    let bytes: Vec<u8> = array.to_vec();

    let native_blob: blob::Blob = blob::Blob::from_vec(bytes);

    return (native_blob.encode_base64(), blob.type_().to_string());
}
