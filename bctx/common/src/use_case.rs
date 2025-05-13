pub trait CasoDeUso<In, Out, E> {
    fn ejecutar(&self, in_: In) -> Result<Out, E>;
}
