struct Post {
    content: String
}

struct DraftPost {
    content: String
}

struct PendingPost {
    content: String,
    approvals: u32

}

impl Post {
    fn new() -> DraftPost {
        DraftPost { content: String::new() }
    }

    fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    fn req_review(self) -> PendingPost {
        PendingPost {
            content: self.content,
            approvals: 0
        }
    }

    fn add_text(&mut self, content: &str) {
        self.content.push_str(content);
    }
}

enum PublishResult {
    PendingPost(PendingPost),
    Post(Post)
}

impl PendingPost {
    fn approve(&mut self) {
        self.approvals += 1;
    }
    fn reject(self) -> DraftPost {
        DraftPost { content: self.content }
    }
    fn publish(self) -> PublishResult {
        if self.approvals > 1 {
            PublishResult::Post(Post{content: self.content})
        } else {
            PublishResult::PendingPost(self)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn publish_workflow() {
        let mut draft = Post::new();
        draft.add_text("ashish first post");
        let mut pending = draft.req_review();
        pending.approve();
        pending.approve();
        let publish = pending.publish();
        match publish {
            PublishResult::Post(p) => assert_eq!(p.content(),
                                                 "ashish first post"),
            _ => assert!(false)
        }
    }

    #[test]
    fn reject_workflow() {
        let mut draft = Post::new();
        draft.add_text("ashish first post");
        let pending = draft.req_review();
        let mut again_draft = pending.reject();
        again_draft.add_text(".. after first one..");
    }

    #[test]
    fn two_approvals_workflow() {
        let mut draft = Post::new();
        draft.add_text("ashish first post");
        let mut pending = draft.req_review();
        pending.approve();
        match pending.publish() {
            PublishResult::PendingPost(_) => assert!(true),
            _ => assert!(false)
        }
    }
}
