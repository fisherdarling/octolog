use crate::{collector::Collector, emitter::Emitter, transformer::Transformer};

#[async_trait::async_trait]
pub trait Pipeline {
    type C: Collector;
    type T: Transformer;
    type E: Emitter;

    type Error;

    async fn run(self) -> Result<<<Self as Pipeline>::E as Emitter>::Output, Self::Error>;
}
