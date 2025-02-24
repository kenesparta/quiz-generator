/// Representa al candidato que postula a obtener una _licencia de conducir_.
pub struct Candidate {
    /// Un identificador numérico único para el candidato. Debe ser único
    /// dentro del contexto de la aplicación. No se debe de confundir con el campo `document_number`
    pub id: u64,

    /// El número de documento del candidato (p. ej., identificación nacional, pasaporte). El tipo
    /// y formato específicos de este número dependerán de los requisitos de la aplicación.
    /// Esta propiedad tambien debe ser único en el contexto de la aplicación.
    pub document_number: String,

    /// Todos los nombres del candidato.
    pub name: String,

    /// Primer apellido del candidato
    pub first_lastname: String,

    /// Segundo apellido del candidato, esta propiedad no necesariamente sera obligatoria,
    /// dependiendo del contexto en el cual se implementa.
    pub second_lastname: String,
}
