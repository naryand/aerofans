use std::sync::atomic::AtomicU64;

use actix_web::{App, HttpResponse, HttpServer, Responder, web::{self, Json}};
use serde::{Serialize, Deserialize};
use dashmap::DashMap;

// Post struct
// contains all info for a post
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Post {
    pub text: String,
}

// Shared mutable state of app server
struct Posts {
    counter: AtomicU64,
    map: DashMap<u64, Post>,
}

// /post
// Takes in POST request with JSON encoded Post in body
// On success, post is created and returns 200 OK with JSON encoded post_id in body
async fn create(post: Json<Post>, posts: web::Data<Posts>) -> impl Responder {
    let posts = web::Data::clone(&posts);

    let id = posts.counter.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    posts.map.insert(id, post.into_inner());

    HttpResponse::Ok().json(id)
}

// /post/{post_id}
// Takes in GET request and post_id from URL
// On success, returns 200 OK with JSON encoded post in body
// If the post_id does not exist, return 404 Not Found
async fn read(web::Path(post_id): web::Path<u64>, posts: web::Data<Posts>) -> impl Responder {
    let posts = web::Data::clone(&posts);
    let post = posts.map.get(&post_id);
    
    match post {
        Some(p) => HttpResponse::Ok().json(p.value()),
        None => HttpResponse::NotFound().finish(),
    }
}

// /post/{post_id}
// Takes in PUT request with JSON encoded Post in body
// On success, returns 200 OK and replaces the post
async fn update(post: Json<Post>, web::Path(post_id): web::Path<u64>, posts: web::Data<Posts>) -> impl Responder {
    let posts = web::Data::clone(&posts);
    posts.map.insert(post_id, post.into_inner());

    HttpResponse::Ok().finish()
}

// /post/{post_id}
// Takes in DELETE request and post_id from URL
// On success, returns 200 OK and deletes the post
// If the post_id does not exist, return 404 Not Found
async fn delete(web::Path(post_id): web::Path<u64>, posts: web::Data<Posts>) -> impl Responder {
    let posts = web::Data::clone(&posts);
    let post = posts.map.remove(&post_id);
    
    match post {
        Some(_) => HttpResponse::Ok().finish(),
        None => HttpResponse::NotFound().finish(),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize shared mutable state
    let map = web::Data::new(Posts { 
        counter: AtomicU64::new(0),
        map: DashMap::new(),
    });

    // Start server
    HttpServer::new( move || {
        App::new()
        .app_data(web::Data::clone(&map))
        .service(web::resource("/post")
            .route(web::post().to(create))
        )
        .service(web::resource("/post/{post_id}")
            .route(web::get().to(read))
            .route(web::put().to(update))
            .route(web::delete().to(delete))
        )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}