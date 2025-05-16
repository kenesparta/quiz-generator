use crate::domain::value_objects::id::{ID, IdError};
use crate::domain::value_objects::id_type::IdType;
use std::fmt::Debug;
use uuid::Uuid;

/// Properties that all domain entities should have
pub trait EntityProps: Debug + Clone {
    /// Optionally provides an ID for the entity (used for reconstruction)
    fn id(&self) -> Option<String> {
        None
    }

    /// Returns the type of ID this entity uses
    fn id_type(&self) -> IdType;
}

/// The core Entity trait that encapsulates DDD entity concepts
pub trait Entity: Debug + Clone + Eq {
    type Props: EntityProps;

    /// Returns the entity's unique identifier
    fn id(&self) -> ID;

    /// Returns the entity's properties
    fn props(&self) -> &Self::Props;

    /// Creates a new instance of this entity type
    fn create(props: Self::Props) -> Result<Self, IdError>
    where
        Self: Sized;

    /// Reconstructs an entity from an ID and properties (for persistence)
    fn reconstruct(id: ID, props: Self::Props) -> Self;

    /// Determines if this entity equals another based solely on identity
    fn equals<E: Entity>(&self, other: &E) -> bool {
        self.id().value() == other.id().value()
    }

    /// Determines if this entity is different from another based solely on identity
    fn not_equals<E: Entity>(&self, other: &E) -> bool {
        !self.equals(other)
    }
}

/// Base implementation of an entity that can be used for all domain entities
#[derive(Debug, Clone)]
pub struct EntityBase<P: EntityProps> {
    id: ID,
    props: P,
}

impl<P: EntityProps> EntityBase<P> {
    /// Creates a new entity with the given properties and a generated ID
    pub fn new(props: P) -> Result<Self, IdError> {
        let id = match props.id() {
            Some(id_value) => ID::new(&id_value, props.id_type())?,
            None => {
                let uuid = Uuid::new_v4().to_string();
                ID::new(&uuid, props.id_type())?
            }
        };

        Ok(Self { id, props })
    }

    /// Reconstructs an entity with a specific ID and properties (for persistence)
    pub fn reconstruct(id: ID, props: P) -> Self {
        Self { id, props }
    }

    /// Gets a reference to the entity's properties
    pub fn props(&self) -> &P {
        &self.props
    }

    // /// Gets a reference to the entity's ID
    // pub fn id(&self) -> ID {
    //     self.id
    // }
}

impl<P: EntityProps + PartialEq> PartialEq for EntityBase<P> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<P: EntityProps + PartialEq> Eq for EntityBase<P> {}

/// Macro to simplify implementation of the Entity trait for concrete entity types
#[macro_export]
macro_rules! implement_entity {
    ($entity_type:ty, $props_type:ty) => {
        impl Entity for $entity_type {
            type Props = $props_type;

            fn id(&self) -> ID {
                self.id.clone()
            }

            fn props(&self) -> &Self::Props {
                &self.props
            }

            fn create(props: Self::Props) -> Result<Self, IdError> {
                let base = EntityBase::new(props)?;
                Ok(Self {
                    id: base.id,
                    props: base.props,
                })
            }

            fn reconstruct(id: ID, props: Self::Props) -> Self {
                Self { id, props }
            }
        }
    };
}

// Example usage
#[cfg(test)]
mod tests {
    use super::*;

    // Define User properties
    #[derive(Debug, Clone, PartialEq, Eq)]
    struct UserProps {
        name: String,
        email: String,
    }

    impl EntityProps for UserProps {
        fn id_type(&self) -> IdType {
            IdType::Usuario
        }
    }

    // Define the User entity
    #[derive(Debug, Clone, PartialEq, Eq)]
    struct User {
        id: ID,
        props: UserProps,
    }

    // Implement the Entity trait for User using the macro
    implement_entity!(User, UserProps);

    // Additional User-specific methods
    impl User {
        pub fn new(name: String, email: String) -> Result<Self, IdError> {
            Self::create(UserProps { name, email })
        }

        pub fn with_id(id: String, name: String, email: String) -> Result<Self, IdError> {
            let props = UserProps { name, email };
            let id_value = ID::new(&id, IdType::Usuario)?;
            Ok(Self::reconstruct(id_value, props))
        }

        pub fn name(&self) -> &str {
            &self.props.name
        }

        pub fn email(&self) -> &str {
            &self.props.email
        }

        pub fn change_email(&mut self, new_email: String) {
            let props = UserProps {
                name: self.props.name.clone(),
                email: new_email,
            };
            self.props = props;
        }
    }

    #[test]
    fn test_entity_equality_by_identity() -> Result<(), IdError> {
        // Arrange
        let id = Uuid::new_v4();
        let user1 = User::with_id(
            id.to_string(),
            "John".to_string(),
            "john@example.com".to_string(),
        )?;
        let user2 = User::with_id(
            id.to_string(),
            "John".to_string(),
            "john@example.com".to_string(),
        )?;

        // Assert - entities with same ID should be equal regardless of different properties
        assert_eq!(user1, user2);
        assert!(user1.equals(&user2));
        assert!(!user1.not_equals(&user2));
        Ok(())
    }

    #[test]
    fn test_entity_inequality_by_identity() -> Result<(), IdError> {
        // Arrange
        let user1 = User::new("Jane".to_string(), "jane@example.com".to_string())?;
        let user2 = User::new("Jane".to_string(), "jane@example.com".to_string())?;

        // Assert - entities with different IDs should not be equal even with identical properties
        assert_ne!(user1, user2);
        assert!(!user1.equals(&user2));
        assert!(user1.not_equals(&user2));
        Ok(())
    }

    #[test]
    fn test_entity_can_change_while_preserving_identity() -> Result<(), IdError> {
        // Arrange
        let mut user = User::new("Alice".to_string(), "alice@example.com".to_string())?;
        let id_before = user.id();

        // Act
        user.change_email("alice.new@example.com".to_string());

        // Assert - properties can change while identity is preserved
        assert_eq!(user.id(), id_before);
        assert_eq!(user.email(), "alice.new@example.com");
        Ok(())
    }

    #[test]
    fn test_invalid_id_creation() {
        // Arrange & Act
        let result = User::with_id(
            "not-a-valid-uuid".to_string(),
            "Invalid".to_string(),
            "invalid@example.com".to_string(),
        );

        // Assert
        assert!(result.is_err());
    }
}
