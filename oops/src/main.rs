use oops::Post;

fn main() {
    let mut post: Post = Post::new();
    post.add_test("Hello World");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("Hello World", post.content());
}
