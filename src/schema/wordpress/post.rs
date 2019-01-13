use juniper::{FieldResult, FieldError, RootNode, EmptyMutation};

#[derive(Debug, Serialize, Deserialize, GraphQLObject)]
pub struct PostQuery {
    pub id: i32,
    pub status: String
}

#[derive(GraphQLInputObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
pub struct NewPost {
    pub title: String
}

#[derive(Debug, Serialize, Deserialize, GraphQLObject)]
pub struct Rendered {
  rendered: String
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

