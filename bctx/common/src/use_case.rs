use async_trait::async_trait;

#[async_trait]
pub trait CasoDeUso<In, Out, E> {
    async fn ejecutar(&self, input: In) -> Result<Out, E>;
}
