use crate::{
    core::{db::PlaceIndexer, prelude::*, usecases, util::sort::Rated},
    infrastructure::error::AppError,
};
use rocket::{config::Config, Rocket, Route};
use rocket_contrib::json::Json;
use std::result;

pub mod api;
#[cfg(feature = "frontend")]
mod frontend;
mod guards;
#[cfg(test)]
mod mockdb;
mod sqlite;
mod tantivy;

type Result<T> = result::Result<Json<T>, AppError>;

fn index_all_places<D: PlaceRepo + RatingRepository>(
    db: &D,
    entry_indexer: &mut dyn PlaceIndexer,
) -> Result<()> {
    // TODO: Split into chunks with fixed size instead of
    // loading all entries at once!
    let entries = db.all_places()?;
    for (entry, _) in entries {
        let ratings = db.load_ratings_of_entry(entry.uid.as_ref())?;
        if let Err(err) =
            entry_indexer.add_or_update_place(&entry, &entry.avg_ratings(&ratings[..]))
        {
            error!("Failed to index entry {:?}: {}", entry, err);
        }
    }
    if let Err(err) = entry_indexer.flush() {
        error!("Failed to build entry index: {}", err);
    }
    Ok(Json(()))
}

pub(crate) fn rocket_instance(
    connections: sqlite::Connections,
    mut search_engine: tantivy::SearchEngine,
    mounts: Vec<(&str, Vec<Route>)>,
    cfg: Option<Config>,
) -> Rocket {
    info!("Indexing all entries...");
    index_all_places(&*connections.exclusive().unwrap(), &mut search_engine).unwrap();

    info!("Discarding expired user e-mail nonces...");
    usecases::discard_expired_user_tokens(&*connections.exclusive().unwrap()).unwrap();

    info!("Initialization finished");
    let r = match cfg {
        Some(cfg) => rocket::custom(cfg),
        None => rocket::ignite(),
    };
    let mut instance = r.manage(connections).manage(search_engine);

    for (m, r) in mounts {
        instance = instance.mount(m, r);
    }
    instance
}

#[cfg(not(feature = "frontend"))]
fn mounts() -> Vec<(&'static str, Vec<Route>)> {
    vec![("/api", api::routes())]
}

#[cfg(feature = "frontend")]
fn mounts() -> Vec<(&'static str, Vec<Route>)> {
    vec![("/api", api::routes()), ("/", frontend::routes())]
}

pub fn run(
    connections: sqlite::Connections,
    search_engine: tantivy::SearchEngine,
    enable_cors: bool,
) {
    if enable_cors {
        panic!(
            "enable-cors is currently not available until\
             \nhttps://github.com/SergioBenitez/Rocket/pull/141\nis merged :("
        );
    }
    rocket_instance(connections, search_engine, mounts(), None).launch();
}

#[cfg(test)]
mod tests {
    use crate::core::{prelude::*, usecases};
    use crate::infrastructure::db::{sqlite, tantivy};
    use rocket::{
        config::{Config, Environment},
        local::Client,
        logger::LoggingLevel,
        Route,
    };

    pub mod prelude {
        pub use crate::core::db::*;
        pub use rocket::{
            http::{ContentType, Cookie, Status},
            local::Client,
            response::Response,
        };
    }

    embed_migrations!();

    pub fn setup(
        mounts: Vec<(&'static str, Vec<Route>)>,
    ) -> (
        rocket::local::Client,
        sqlite::Connections,
        tantivy::SearchEngine,
    ) {
        let cfg = Config::build(Environment::Development)
            .log_level(LoggingLevel::Debug)
            .finalize()
            .unwrap();
        let connections = sqlite::Connections::init(":memory:", 1).unwrap();
        embedded_migrations::run(&*connections.exclusive().unwrap()).unwrap();
        let search_engine = tantivy::SearchEngine::init_in_ram().unwrap();
        let rocket = super::rocket_instance(
            connections.clone(),
            search_engine.clone(),
            mounts,
            Some(cfg),
        );
        let client = Client::new(rocket).unwrap();
        (client, connections, search_engine)
    }

    pub fn register_user(pool: &sqlite::Connections, email: &str, pw: &str, confirmed: bool) {
        let db = pool.exclusive().unwrap();
        usecases::create_new_user(
            &*db,
            usecases::NewUser {
                email: email.to_string(),
                password: pw.to_string(),
            },
        )
        .unwrap();
        let email_nonce = EmailNonce {
            email: email.to_string(),
            nonce: Nonce::new(),
        };
        let token = email_nonce.encode_to_string();
        if confirmed {
            usecases::confirm_email_address(&*db, &token).unwrap();
        }
    }
}
