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

impl PendingPost {
    fn approve(self) -> Post {
        Post { content: self.content }
    }
    fn reject(self) -> DraftPost {
        DraftPost { content: self.content }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn publish_workflow() {
        let mut draft = Post::new();
        draft.add_text("ashish first post");
        let pending = draft.req_review();
        let published = pending.approve();
        assert_eq!(published.content(), "ashish first post");
    }

    #[test]
    fn reject_workflow() {
        let mut draft = Post::new();
        draft.add_text("ashish first post");
        let pending = draft.req_review();
        let mut again_draft = pending.reject();
        again_draft.add_text(".. after first one..");
        assert_eq!(again_draft.req_review().approve().content(),
                   "ashish first post.. after first one..");
    }

}
