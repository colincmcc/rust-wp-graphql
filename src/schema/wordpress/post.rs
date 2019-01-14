use crate::schema::wordpress::common::{Rendered};

// At the moment serde does not serialize traits or impl
// NewPost and NewPostMut are placeholder structs for an empty RootMutation

#[derive(GraphQLInputObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
pub struct NewPost {
    pub id: i32
}

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
pub struct NewPostMut {
    pub id: i32
}

#[derive(Debug, Serialize, Deserialize, GraphQLObject)]
#[graphql(description="A Wordpress Post")]
pub struct Post {
  pub id: i32,
  pub date: String,
  pub slug: String,
  pub status: String,
  pub link: String,
  pub title: Rendered,
  pub content: Rendered,
  pub author: i32,
  pub featured_media: i32
}


