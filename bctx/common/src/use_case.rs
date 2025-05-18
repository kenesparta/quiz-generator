use async_trait::async_trait;

#[async_trait]
pub trait CasoDeUso<In, Out, E> {
    async fn ejecutar(&self, in_: In) -> Result<Out, E>;
}
