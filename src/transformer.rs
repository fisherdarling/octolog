#[async_trait::async_trait]
pub trait Transformer {
    type Input;
    type Output;
    type Error;

    async fn transform(&mut self, input: Self::Input) -> Result<Self::Output, Self::Error>;
}

// pub struct Feeder {
//     in: Transformer,
//     out: Transformer
// }

// impl Transformer for Feeder {

// }

// pub trait TransformerExt: Transformer {
//     fn feed_into<T: Transformer>(self, next: T) -> Feeder {
//         Feeder {
//             in: self,
//             out: next
//         }
//     }
// }

// ValidateJson;

// let json_validator = Default::default();

// json_validator.feed_into(enricher).feed_into(schema_transformer).feed_into()
