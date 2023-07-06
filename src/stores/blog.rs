use uuid::Uuid;
use edgedb_tokio::{Client, Error};
use crate::models::{RawBlogPost, DetailedBlogPost, BlogCategory};

pub async fn get_all_posts_count(client: &Client) -> Result<usize, Error> {
    let q = "SELECT count(BlogPost)";
    tracing::debug!("To query: {}", q);
    let count: i64 = client.query_required_single(q, &()).await?;
    Ok(count.try_into().unwrap_or(0))
}

pub async fn get_blogpost(post_id: Uuid, client: &Client) -> Result<Option<DetailedBlogPost>, Error> {
    // Note: For now, we cannot use EdgeDB splats syntax because the returned field order
    // does not match DetailedBlogPost.
    let q = "
    SELECT BlogPost {
        id,
        title,
        slug,
        is_published,
        published_at,
        created_at,
        updated_at,
        categories: {id, title, slug},
        body,
        format,
        locale,
        excerpt,
        html,
        seo_description,
        og_image,
    }
    FILTER .id = <uuid>$0";
    tracing::debug!("To query: {}", q);
    let post: Option<DetailedBlogPost> = client.query_single(q, &(post_id,)).await?;
    Ok(post)
}

pub async fn get_blogpost_by_slug(slug: String, client: &Client) -> Result<Option<DetailedBlogPost>, Error> {
    // Note: For now, we cannot use EdgeDB splats syntax because the returned field order
    // does not match DetailedBlogPost.
    let q = "
    SELECT BlogPost {
        id,
        title,
        slug,
        is_published,
        published_at,
        created_at,
        updated_at,
        categories: {id, title, slug},
        body,
        format,
        locale,
        excerpt,
        html,
        seo_description,
        og_image,
    }
    FILTER .slug = <str>$0";
    tracing::debug!("To query: {}", q);
    let post: Option<DetailedBlogPost> = client.query_single(q, &(slug,)).await?;
    Ok(post)
}

pub async fn get_blogposts(offset: Option<i64>, limit: Option<i64>, client: &Client) -> Result<Vec<RawBlogPost>, Error> {
    let q = "
    SELECT BlogPost {
        id,
        title,
        slug,
        is_published,
        published_at,
        created_at,
        updated_at,
        categories: {
            id,
            title,
            slug,
        },
    }
    ORDER BY .created_at DESC EMPTY FIRST OFFSET <optional int64>$0 LIMIT <optional int64>$1";
    let posts: Vec<RawBlogPost> = client.query(q, &(offset, limit)).await?;
    Ok(posts)
}

pub async fn get_blog_categories(offset: Option<i64>, limit: Option<i64>, client: &Client) -> Result<Vec<BlogCategory>, Error> {
    let q = "
    SELECT BlogCategory {
        id,
        title,
        slug
    } ORDER BY .title OFFSET <optional int64>$0 LIMIT <optional int64>$1";
    let categories: Vec<BlogCategory> = client.query(q, &(offset, limit)).await?;
    Ok(categories)
}


pub async fn get_all_categories_count(client: &Client) -> Result<usize, Error> {
    let q = "SELECT count(BlogCategory)";
    tracing::debug!("To query: {}", q);
    let count: i64 = client.query_required_single(q, &()).await?;
    Ok(count.try_into().unwrap_or(0))
}

pub async fn get_blog_category(id: Uuid, client: &Client) -> Result<Option<BlogCategory>, Error> {
    let q = "
    SELECT BlogCategory {
        id,
        title,
        slug
    } FILTER .id = <uuid>$0";
    tracing::debug!("To query: {}", q);
    let cat: Option<BlogCategory> = client.query_single(q, &(id,)).await?;
    Ok(cat)
}
