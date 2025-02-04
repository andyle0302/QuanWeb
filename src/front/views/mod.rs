pub mod blog;
pub mod old_urls;

use std::num::NonZeroU16;

use serde::ser::Serialize;
use minijinja::Environment;
use http::{StatusCode, Uri};
use axum::extract::{Query, State, OriginalUri};
use axum::response::{Html, IntoResponse, Result as AxumResult};
use minijinja::context;
use minijinja::value::Value as MJValue;

pub use crate::errors::PageError;
use crate::auth::Auth;
use crate::types::{AppState, Paginator, StaticFile};
use super::structs::LaxPaging;
use crate::stores;
use crate::consts::{DEFAULT_PAGE_SIZE, STATIC_URL};

pub fn render_with<S: Serialize>(template_name: &str, context: S, engine: Environment) -> Result<String, PageError> {
    let tpl = engine.get_template(template_name)?;
    let content = tpl.render(context)?;
    Ok(content)
}


pub async fn fallback_view() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "Not found")
}


pub async fn home(
    auth: Auth,
    OriginalUri(current_url): OriginalUri,
    Query(paging): Query<LaxPaging>,
    State(state): State<AppState>,
) -> AxumResult<Html<String>> {
    let AppState { db, jinja } = state;
    let current_page = paging
        .page
        .and_then(|p| NonZeroU16::new(p.parse().ok()?))
        .unwrap_or(NonZeroU16::MIN);
    let total = stores::blog::count_all_published_posts(&db)
        .await
        .map_err(PageError::EdgeDBQueryError)?;
    let page_size = DEFAULT_PAGE_SIZE;
    let total_pages = NonZeroU16::try_from((total as f64 / page_size as f64).ceil() as u16)
        .unwrap_or(NonZeroU16::MIN);
    let paginator = Paginator {
        current_page,
        total_pages,
    };
    let pagelink_items = paginator.generate_items();
    tracing::debug!("Pagination links: {:?}", pagelink_items);
    let next_page_url = paginator.next_url(&current_url);
    let prev_page_url = paginator.previous_url(&current_url);
    let offset = ((current_page.get() - 1) * (page_size as u16)) as i64;
    let result = stores::blog::get_published_posts(Some(offset), Some(page_size as i64), &db)
        .await
        .map_err(PageError::EdgeDBQueryError)?;
    let posts: Vec<MJValue> = result.into_iter().collect();
    let categories = stores::blog::get_blog_categories(None, None, &db)
        .await
        .map_err(PageError::EdgeDBQueryError)?;
    let no_tracking = auth.current_user.is_some();
    let context = context!(
        posts => posts,
        categories => categories,
        pagelink_items => pagelink_items,
        next_page_url => next_page_url,
        prev_page_url => prev_page_url,
        no_tracking => no_tracking);
    let content = render_with("home.jinja", context, jinja)?;
    Ok(Html(content))
}

pub async fn static_handler(uri: Uri) -> impl IntoResponse {
    // URI is like "/static/css/style.css", we need to strip to "css/style.css"
    let path = uri
        .path()
        .trim_start_matches(&format!("{STATIC_URL}/"))
        .to_string();
    StaticFile(path)
}
