pub mod post;

pub mod review;
pub mod state;
pub mod draft;
pub mod published;

/*

The implementation using the state pattern is easy to extend to add more functionality. 
To see the simplicity of maintaining code that uses the state pattern, try a few of these suggestions:

- Add a reject method that changes the post’s state from PendingReview back to Draft.
- Require two calls to approve before the state can be changed to Published.
- Allow users to add text content only when a post is in the Draft state. 
Hint: have the state object responsible for what might change about the content but not responsible for modifying the Post.
*/

#[cfg(test)]
mod tests {

    use crate::post::Post;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_post_states() {
        let mut post = Post::new();

        post.add_text("I ate a salad for lunch today");
        assert_eq!("", post.content());
    
        post.request_review();
        assert_eq!("", post.content());
    
        post.approve();
        assert_eq!("I ate a salad for lunch today", post.content());
    }
}
