use blog::Post;

fn main() {}

#[cfg(test)]
mod tests {
    use blog::Post;

    #[test]
    fn test_blog() {
        let mut post = Post::new();

        post.add_text("I ate a salad for lunch today");
        assert_eq!("", post.content());

        post.request_review();
        assert_eq!("", post.content());

        post.reject();
        assert_eq!("", post.content());

        post.approve();
        assert_eq!("I ate a salad for lunch today", post.content());
    }
}
