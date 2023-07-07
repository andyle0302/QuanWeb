pub mod conversions;

use std::collections::HashMap;

use edgedb_tokio::Client;
use serde::{Deserialize, Serialize};
use edgedb_protocol::codec::ShapeElement;
use edgedb_protocol::common::Cardinality;
use axum::extract::FromRef;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::http::header::CONTENT_TYPE;
use minijinja::Environment;
use rust_embed::RustEmbed;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiErrorShape {
    pub message: String,
    pub fields: Option<HashMap<String, String>>,
    pub code: Option<String>,
}

impl Default for ApiErrorShape {
    fn default() -> Self {
        Self {
            message: "Some error".to_string(),
            fields: None,
            code: None,
        }
    }
}

impl From<String> for ApiErrorShape {
    fn from(message: String) -> Self {
        Self {
            message,
            ..Default::default()
        }
    }
}

impl From<HashMap<String, String>> for ApiErrorShape {
    fn from(fields: HashMap<String, String>) -> Self {
        Self {
            fields: Some(fields),
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, FromRef)]
pub struct AppState {
    pub db: Client,
    pub jinja: Environment<'static>,
}

#[derive(RustEmbed)]
#[folder = "static"]
#[exclude = "vendor/alpine*.js"]
#[exclude = "fonts/*"]
pub struct Assets;

pub struct StaticFile<T>(pub T);

impl<T> IntoResponse for StaticFile<T>
where
  T: Into<String>,
{
  fn into_response(self) -> Response {
    let path = self.0.into();

    match Assets::get(path.as_str()) {
      Some(content) => {
        let mime = mime_guess::from_path(path).first_or_octet_stream();
        ([(CONTENT_TYPE, mime.as_ref())], content.data).into_response()
      }
      None => (StatusCode::NOT_FOUND, "File Not Found").into_response(),
    }
  }
}

pub fn create_shape_element<N: ToString>(name: N, cardinality: Cardinality) -> ShapeElement {
    ShapeElement {
        name: name.to_string(),
        cardinality: Some(cardinality),
        flag_link: false,
        flag_link_property: false,
        flag_implicit: false,
    }
}
