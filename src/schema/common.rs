use actix::prelude::Message;
use actix_web::{ Error };
use juniper::http::GraphQLRequest;

#[derive(Serialize, Deserialize)]
pub struct GraphQLData(pub GraphQLRequest);

impl Message for GraphQLData {
    type Result = Result<String, Error>;
}