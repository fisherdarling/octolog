#[async_trait::async_trait]
pub trait Emitter {
    type Output;
    type Error;
}
