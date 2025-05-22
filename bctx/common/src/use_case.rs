use async_trait::async_trait;

#[async_trait]
pub trait CasoDeUso<In, Out, Error> {
    async fn ejecutar(&self, in_: In) -> Result<Out, Error>;
}
