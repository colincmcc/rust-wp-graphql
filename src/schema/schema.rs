use juniper::{FieldResult, FieldError, RootNode, EmptyMutation};
use crate::schema::wordpress::post::{PostQuery, NewPost, Post};
use reqwest;

pub struct QueryRoot;

graphql_object!(QueryRoot: () as "Query" |&self| {

    field getPost(&executor, id: i32) -> FieldResult<Post> {
                let request_url = format!("https://hauntandhorror.com/wp-json/wp/v2/posts/{id}",
                            id = &id);

                let mut response = reqwest::get(&request_url)?;
                let postData: Post = response.json()?;
                println!("{:?}", postData);

                Ok(postData)
    }

        field getAllPosts(&executor) -> FieldResult<Vec<Post>> {
                let request_url ="https://hauntandhorror.com/wp-json/wp/v2/posts/";

                let mut response = reqwest::get(request_url)?;
                let postData: Vec<Post> = response.json()?;
                println!("{:?}", postData);

                Ok(postData)
    }

});
pub struct MutationRoot;

graphql_object!(MutationRoot: () |&self| {
    field createPost(&executor, new_post: NewPost) -> FieldResult<PostQuery> {
        Ok(PostQuery{
            id: 1234.to_owned(),
            status: new_post.title,
        })
    }
});
pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {})
}