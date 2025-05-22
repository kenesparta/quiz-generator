use async_trait::async_trait;

#[async_trait]
pub trait ObservadorEventoDeDominio<Event, Error>: Send + Sync {
    async fn evento_ocurrido(&self, evento: Event) -> Result<(), Error>;
}
