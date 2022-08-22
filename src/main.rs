extern crate iron;
extern crate router;
extern crate logger;
extern crate env_logger;
extern crate chrono;
extern crate uuid;
extern crate serde;

mod model;
mod database;
mod handler;

use model::*;
use database::Database;
use handler::*;

use iron::Iron;
use iron::prelude::Chain;
use router::Router;
use logger::Logger;
use uuid::Uuid;

const LOCAL_HOST: &str = "127.0.0.1:8000";

fn main() {
    // initializing the environment logger
    env_logger::init();

    // instantiate the loggers
    let (logger_before, logger_after) = Logger::new(None);

    // creating dummy data
    let first_post =
        Post::new(
            "First Post Title",
            "This is the first post's body...",
            "John Smith",
            chrono::offset::Utc::now(),
            Uuid::new_v4(),
        );

    let second_post =
        Post::new(
            "Second Post Title",
            "This is the second post's body...",
            "Leo Messi",
            chrono::offset::Utc::now(),
            Uuid::new_v4(),
        );

    // instantiate the database
    let mut database = Database::new();

    // populating the database with the dummy data
    database.insert_post(first_post);
    database.insert_post(second_post);

    let handlers = Handlers::new(database);
    let json_content_after_middleware = JsonContentAfterMiddleWare;

    // instantiate a router
    let mut router = Router::new();

    // adding the route for GET request for the feed
    router.get("/feed", handlers.feed_getter, "feed");
    // adding the routes for POST/GET requests for the posts
    router.post("/post", handlers.post_poster, "post_post");
    router.get("/post/:id", handlers.post_getter, "post");

    // instantiate a chain
    let mut chain = Chain::new(router);

    // adding the loggers & the middleware to the chain
    chain.link_before(logger_before);
    chain.link_after(json_content_after_middleware);
    chain.link_after(logger_after);

    Iron::new(chain).http(LOCAL_HOST).unwrap();
}
