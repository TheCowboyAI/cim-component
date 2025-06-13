//! Component trait for attaching data to domain objects
//!
//! This module provides the foundational Component trait that enables
//! attaching immutable data to entities in the Composable Information Machine.

use std::any::{Any, TypeId};
use std::fmt::Debug;

/// Trait for components that can be attached to domain objects
///
/// Components are immutable data that can be attached to entities, nodes, or edges.
/// They provide a way to extend domain objects with additional data without modifying
/// their core structure.
///
/// # Example
///
/// ```
/// use cim_component::Component;
/// use std::any::Any;
///
/// #[derive(Debug, Clone)]
/// struct Label(String);
///
/// impl Component for Label {
///     fn as_any(&self) -> &dyn Any { self }
///     fn clone_box(&self) -> Box<dyn Component> { Box::new(self.clone()) }
///     fn type_name(&self) -> &'static str { "Label" }
/// }
/// ```
pub trait Component: Any + Send + Sync + Debug {
    /// Get the component as Any for downcasting
    fn as_any(&self) -> &dyn Any;

    /// Clone the component into a box
    fn clone_box(&self) -> Box<dyn Component>;

    /// Get the name of this component type
    fn type_name(&self) -> &'static str;
}

/// Error type for component operations
#[derive(Debug, Clone, PartialEq)]
pub enum ComponentError {
    /// Component of this type already exists
    AlreadyExists(String),
    /// Component not found
    NotFound(String),
}

impl std::fmt::Display for ComponentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ComponentError::AlreadyExists(name) => write!(f, "Component already exists: {}", name),
            ComponentError::NotFound(name) => write!(f, "Component not found: {}", name),
        }
    }
}

impl std::error::Error for ComponentError {}

/// Result type for component operations
pub type ComponentResult<T> = Result<T, ComponentError>;

/// Get the TypeId of a component type
pub fn component_type_id<T: Component + 'static>() -> TypeId {
    TypeId::of::<T>()
}
