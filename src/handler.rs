use std::io::Read;
use std::sync::{Arc, Mutex};

use iron::{
    AfterMiddleware,
    Handler,
    IronResult,
    Request,
    Response,
    status,
};
use iron::headers::ContentType;
use router::Router;
use uuid::Uuid;

use database::Database;
use model::Post;

use crate::database;
use crate::model;

macro_rules! lock {
    ($expression: expr) => { $expression.lock().unwrap() };
}

macro_rules! try_handler {
    ($expression: expr) => {
        match $expression {
            Ok(data) => data,
            Err(error) => return Ok(Response::with((status::InternalServerError, error.to_string())))
        }
    };

    ($expression: expr, $error_status: expr) => {
        match $expression {
            Ok(data) => data,
            Err(error) => return Ok(Response::with(($error_status, error.to_string())))
        }
    };
}

macro_rules! get_http_request_parameter {
    ($request: expr, $parameter_name: expr) => {
        match $request.extensions.get::<Router>() {
            Some(router) => {
                match router.find($parameter_name) {
                    Some(data) => data,
                    None => return Ok(Response::with(status::BadRequest))
                }
            },
            None => return Ok(Response::with(status::InternalServerError))
        }
    };
}

pub struct Handlers {
    pub feed_getter: GetFeedHandler,
    pub post_poster: PostPostHandler,
    pub post_getter: GetPostHandler,
}

impl Handlers {

    pub fn new(database: Database) -> Handlers {
        let database = Arc::new(Mutex::new(database));
        return Handlers {
            feed_getter: GetFeedHandler::new(database.clone()),
            post_poster: PostPostHandler::new(database.clone()),
            post_getter: GetPostHandler::new(database.clone()),
        }
    }
}

pub struct GetFeedHandler {
    database: Arc<Mutex<Database>>,
}

impl GetFeedHandler {

    pub fn new(database: Arc<Mutex<Database>>) -> GetFeedHandler {
        GetFeedHandler { database }
    }
}

impl Handler for GetFeedHandler {

    fn handle(&self, _: &mut Request) -> IronResult<Response> {
        let payload = try_handler!(serde_json::to_string(lock!(self.database).posts()));
        Ok(Response::with((status::Ok, payload)))
    }
}

pub struct PostPostHandler {
    database: Arc<Mutex<Database>>,
}

impl PostPostHandler {

    pub fn new(database: Arc<Mutex<Database>>) -> PostPostHandler {
        PostPostHandler { database }
    }
}

impl Handler for PostPostHandler {

    fn handle(&self, request: &mut Request) -> IronResult<Response> {
        let mut payload: String = String::new();
        try_handler!(request.body.read_to_string(&mut payload));

        let post = try_handler!(serde_json::from_str::<Post>(&payload), status::BadRequest);
        lock!(self.database).insert_post(post);

        Ok(Response::with((status::Created, payload)))
    }
}

pub struct GetPostHandler {
    database: Arc<Mutex<Database>>,
}

impl GetPostHandler {

    pub fn new(database: Arc<Mutex<Database>>) -> GetPostHandler {
        GetPostHandler { database }
    }

    pub fn fetch(&self, id: &Uuid) -> Option<Post> {
        lock!(self.database).posts().iter()
            .find(|post| id == post.uuid())
            .map(|post| post.clone())
    }
}

impl Handler for GetPostHandler {

    fn handle(&self, request: &mut Request) -> IronResult<Response> {
        let ref post_di = get_http_request_parameter!(request, "id");
        let post_id = try_handler!(Uuid::parse_str(post_di), status::BadRequest);

        if let Some(post) = self.fetch(&post_id) {
            let payload = try_handler!(serde_json::to_string(&post));
            Ok(Response::with((status::Ok, payload)))
        } else {
            Ok(Response::with(status::NotFound))
        }
    }
}

pub struct JsonContentAfterMiddleWare;

impl AfterMiddleware for JsonContentAfterMiddleWare {

    fn after(&self, _: &mut Request, mut response: Response) -> IronResult<Response> {
        response.headers.set(ContentType::json());
        Ok(response)
    }
}