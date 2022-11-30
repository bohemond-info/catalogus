//noinspection RsMainFunctionNotFound
#[macro_use]
extern crate rocket;

use std::env;
use std::error::Error;
use std::io;
use std::path::PathBuf;
use std::str::FromStr;

use path_clean::PathClean;
use rocket::figment::{Figment, Profile, providers::{Env, Format, Yaml, Toml}};
use rocket::fs::FileServer;
use rocket::http::Method;
use rocket_cors::{AllowedHeaders, AllowedOrigins, Cors, CorsOptions};
use rocket_okapi::openapi_get_routes;
use tracing::{debug, info};

use crate::application_configuration::BohemondConfig;

mod application_configuration;
mod routes;

// Wiring for configuration, asset loading, etc.
const APPLICATION_CONFIG_FILE: &str = "../bohemond.yaml";  // Business logic config-by-file.
const APPLICATION_ENV_PREFIX: &str = "BOHEMOND_";  // Business logic config-by-env-var.
const DEV_ASSET_DIRECTORY: &str = "./resources/";
const SERVER_CONFIG_FILE: &str = "Rocket.toml";  // Web server config-by-file.
const SERVER_ENV_PREFIX: &str = "ROCKET_";  // Web server config-by-env-var.

#[launch]
fn rocket() -> _ {
    let config = Figment::from(rocket::Config::default())
        .merge(Yaml::file(APPLICATION_CONFIG_FILE).nested())
        .merge(Env::prefixed(APPLICATION_ENV_PREFIX).global())
        .merge(Toml::file(SERVER_CONFIG_FILE).nested())
        .merge(Env::prefixed(SERVER_ENV_PREFIX).global());
    // Set env var APP_PROFILE to 'production' for production deployment.
    let profile = Profile::from_env_or("APP_PROFILE", "default");  // i.e. default/dev/staging/production
    let rocket_config = config.select(&*profile);
    println!("Loaded config: {:#?}", profile);
    let bohemond_config = rocket_config
        .extract();
    let bohemond_config = bohemond_config.expect("Could not load application configuration file.");
    // Must come first to initialize global logger.
    // Note: When Rocket switches to Tracing this should be already included.
    // See: https://github.com/SergioBenitez/Rocket/issues/21
    setup_logging(&bohemond_config).expect("failed to initialize logging.");
    info!("Initializing in mode={}", &profile);
    let assets_dir = assets_directory(&bohemond_config).unwrap();  // Panic here is fine.

    rocket::custom(rocket_config)
        .mount("/", rocket_cors::catch_all_options_routes())
        // .mount("/", routes![index])
        .mount("/", openapi_get_routes![routes::get_catalogus])
        .mount("/", FileServer::from(assets_dir))
        .manage(make_cors())
}

fn setup_logging(config: &BohemondConfig) -> Result<(), Box<dyn Error + 'static>> {
    // Set up logging
    let log_level = config.log_level.unwrap();
    tracing_subscriber::fmt().with_max_level(log_level).init();
    debug!("Application log level set to {:#?}", log_level);
    Ok(())
}

fn assets_directory(config: &BohemondConfig) -> Result<PathBuf, io::Error> {
    let dir = env::current_dir()
        .map_err(|_| io::Error::new(io::ErrorKind::Other, "Bad io."))?
        .join(config.static_asset_directory
            .as_ref()
            .unwrap_or({
                &PathBuf::from_str(DEV_ASSET_DIRECTORY).unwrap()
            }))
        .clean();
    debug!("Static asset directory: {:#?}", dir);
    Ok(dir)
}


fn make_cors() -> Cors {
    // The CORS localhost allowance is for development purposes.
    // TODO: Find a way to turn it off when in production mode.
    let allowed_origins = AllowedOrigins::some_exact(&[
        "http://localhost:8080",
        "http://127.0.0.1:8080",
        "http://0.0.0.0:8080",
        "http://localhost:8000",
        "http://127.0.0.1:8000",
        "http://0.0.0.0:8000",
    ]);

    CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post]
            .into_iter()
            .map(From::from)
            .collect(),
        allowed_headers: AllowedHeaders::all(),
        allow_credentials: true,
        // send_wildcard: true,
        ..Default::default()
    }
        .to_cors()
        .expect("Error occurred while building CORS")
}
