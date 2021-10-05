use crate::state::State;
use crate::review::PendingReview;

pub struct Draft {}

impl State for Draft {

    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview{})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

}