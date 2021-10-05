use crate::state::State;
use crate::post::Post;
pub struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

	fn content<'a>(&self, post: &'a Post) -> &'a str {
		&post.content
	}
}