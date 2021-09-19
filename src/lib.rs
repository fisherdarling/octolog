pub mod collector;
pub mod emitter;
pub mod pipeline;
pub mod transformer;

// Great post on generic pipelines:
// https://tokio.rs/blog/2021-05-14-inventing-the-service-trait

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
