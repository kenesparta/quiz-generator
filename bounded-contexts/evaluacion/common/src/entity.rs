use crate::Id;

/// A trait for Domain-Driven Design (DDD) entities.
///
/// In DDD, an Entity is an object with a unique identity that persists over time.
/// Unlike Value Objects, entities are compared by their identity (ID), not their attributes.
///
/// # Entity vs Value Object
///
/// - **Entity**: Has unique identity, mutable over time, compared by ID
///   - Examples: User, Person, Order, Product
/// - **Value Object**: No identity, immutable, compared by value
///   - Examples: PersonName, Email, Money, Address
///
/// # Rust Implementation: Composition over Inheritance
///
/// Rust doesn't have inheritance, so we use **composition** via traits.
/// Each entity type:
/// 1. Contains an `Id` field
/// 2. Implements this `Entity` trait
/// 3. Implements `PartialEq` based on ID only (not all fields)
///
/// # Examples
///
/// ```
/// use education_platform_common::{Entity, Id};
///
/// #[derive(Debug, Clone)]
/// struct Product {
///     id: Id,
///     name: String,
///     price: u32,
/// }
///
/// impl Entity for Product {
///     fn id(&self) -> Id {
///         self.id
///     }
/// }
///
/// // Entity equality based on ID only
/// impl PartialEq for Product {
///     fn eq(&self, other: &Self) -> bool {
///         self.id == other.id
///     }
/// }
///
/// impl Eq for Product {}
///
/// // Usage
/// let product1 = Product { id: Id::new(), name: "Laptop".to_string(), price: 1000 };
/// let product2 = product1.clone();
///
/// // Same ID = same entity, even if cloned
/// assert_eq!(product1, product2);
/// assert_eq!(product1.id(), product2.id());
/// ```
///
/// # Why Not Use Associated Types or Generics?
///
/// We could define `trait Entity { type Id; }`, but this adds complexity without benefit:
/// - All entities use the same `Id` type (ULID-based)
/// - Simple method is more ergonomic
/// - Easier to understand and use
///
/// # Pattern in This Codebase
///
/// All entities should:
/// 1. Have an `id: Id` field
/// 2. Implement `Entity` trait (return the ID)
/// 3. Implement `PartialEq` + `Eq` based on ID only
/// 4. Derive `Debug` and `Clone`
pub trait Entity {
    /// Returns the unique identifier for this entity.
    ///
    /// This ID should never change during the entity's lifetime.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_common::{Entity, Id};
    ///
    /// # #[derive(Debug, Clone)]
    /// # struct User { id: Id, name: String }
    /// # impl Entity for User {
    /// #     fn id(&self) -> Id { self.id }
    /// # }
    /// # impl PartialEq for User {
    /// #     fn eq(&self, other: &Self) -> bool { self.id == other.id }
    /// # }
    /// # impl Eq for User {}
    /// let user = User { id: Id::new(), name: "Alice".to_string() };
    /// let id = user.id();
    ///
    /// assert_eq!(id.to_string().len(), 26); // ULID is 26 characters
    /// ```
    fn id(&self) -> Id;
}
