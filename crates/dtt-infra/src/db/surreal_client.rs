use surrealdb::{Surreal, engine::remote::ws::{Ws, Client}};

pub struct SurrealClient;

impl SurrealClient {
    pub async fn connect(url: &str) -> Result<Surreal<Client>, String> {
        let db = Surreal::new::<Ws>(url)
            .await
            .map_err(|e| e.to_string())?;
        
        db.use_ns("decentik").use_db("main")
            .await
            .map_err(|e| e.to_string())?;
        
        // Initialize schema
        let _: surrealdb::Result<surrealdb::Response> = db
            .query("
                DEFINE TABLE users SCHEMAFULL;
                DEFINE FIELD id ON TABLE users TYPE record(users);
                DEFINE FIELD handle ON TABLE users TYPE string;
                
                DEFINE TABLE posts SCHEMAFULL;
                DEFINE FIELD id ON TABLE posts TYPE record(posts);
                DEFINE FIELD user_id ON TABLE posts TYPE record(users);
                DEFINE FIELD media_id ON TABLE posts TYPE object;
                DEFINE FIELD caption ON TABLE posts TYPE option<string>;
                DEFINE FIELD signature ON TABLE posts TYPE option<object>;
                DEFINE FIELD timestamp ON TABLE posts TYPE number;
                
                DEFINE TABLE posted SCHEMAFULL;
                DEFINE FIELD in ON TABLE posted TYPE record(users);
                DEFINE FIELD out ON TABLE posted TYPE record(posts);
                DEFINE FIELD timestamp ON TABLE posted TYPE datetime;
            ")
            .await
            .map_err(|e| e.to_string())?;
        
        Ok(db)
    }
}