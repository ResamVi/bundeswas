use dip;
use axum::{
    routing::get,
    response::Json,
    Router,
};
use serde::Serialize;
use log::{error, log_enabled, info, Level};
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tower::ServiceBuilder;

/// DTO 
#[derive(Serialize)]
struct Result {
    titel: String,
    datum: String,
    dokumentnummer: String,
    vorgaenge: Vec<Vorgang>,
}

#[derive(Serialize)]
struct Vorgang {
    datum: String,
    titel: String,
    vorgangstyp: String,

    initiative: String,

    // TOOMUCH pub deskriptor: Option<Vec<Deskriptor>>,
    beratungsstand: String,
    sachgebiet: String,
}

async fn handler() -> Json<Vec<Result>> {
    info!("GET /");

    let bundestag = dip::new();
    let mut results = Vec::new();

    for plenarprotokoll in bundestag.plenarprotokolle().take(10) {
        // println!("{}", plenarprotokoll);

        let mut result = Result {
            titel: plenarprotokoll.titel,
            datum: plenarprotokoll.datum,
            dokumentnummer: plenarprotokoll.dokumentnummer,
            vorgaenge: Vec::new(),
        };

        let vorgaenge = bundestag.vorgaenge(plenarprotokoll.id);
        for vorgang in vorgaenge {
            // println!("{}", vorgang);
            result.vorgaenge.push(Vorgang{ 
                datum: vorgang.datum,
                titel: vorgang.titel, 
                vorgangstyp: vorgang.vorgangstyp,
                initiative: vorgang.initiative.unwrap_or_default().join(" "),
                beratungsstand: vorgang.beratungsstand.unwrap_or_default(),
                sachgebiet: vorgang.sachgebiet.unwrap_or_default().join(" "),
            });
        }

        results.push(result);
    };

    
    Json(results)
}

#[tokio::main]
async fn main() {
    env_logger::init();
    // tracing_subscriber::fmt::init();

    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods(Any)
        // allow requests from any origin
        .allow_origin(Any);

    // build our application with a single route
    let app = Router::new()
        .route("/", get(handler))
        .layer( 
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(cors)
        );

    info!("listening");

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
