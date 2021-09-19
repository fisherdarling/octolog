use futures::Stream;

#[async_trait::async_trait]
pub trait Collector {
    type Item;
    type Error;

    async fn produce(
        &mut self,
    ) -> Result<Box<dyn Stream<Item = Result<Self::Item, Self::Error>>>, Self::Error>;
}
