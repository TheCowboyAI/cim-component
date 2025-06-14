# CIM Component

Component trait and storage for the Composable Information Machine.

## Overview

The Component module provides the foundational trait and utilities for implementing a component-based architecture in CIM. Components are immutable data that can be attached to domain objects (entities, nodes, edges) to extend their functionality without modifying their core structure. This follows the Entity-Component pattern, enabling flexible and extensible domain modeling.

## Key Concepts

### Component Trait

The core abstraction for attachable data:

```rust
pub trait Component: Any + Send + Sync + Debug {
    /// Get the component as Any for downcasting
    fn as_any(&self) -> &dyn Any;

    /// Clone the component into a box
    fn clone_box(&self) -> Box<dyn Component>;

    /// Get the name of this component type
    fn type_name(&self) -> &'static str;
}
```

### Type Safety

Components use Rust's type system for compile-time safety:

```rust
// Define a component
#[derive(Debug, Clone)]
struct Position3D {
    x: f32,
    y: f32,
    z: f32,
}

impl Component for Position3D {
    fn as_any(&self) -> &dyn Any { self }
    fn clone_box(&self) -> Box<dyn Component> { Box::new(self.clone()) }
    fn type_name(&self) -> &'static str { "Position3D" }
}

// Type-safe access
let pos = entity.get_component::<Position3D>();
```

## Usage Patterns

### Defining Components

Components should be:
- **Immutable**: Data that doesn't change after creation
- **Self-contained**: No references to other entities
- **Serializable**: Can be persisted and restored
- **Cloneable**: Can be duplicated when needed

```rust
use cim_component::Component;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Metadata {
    name: String,
    description: Option<String>,
    tags: Vec<String>,
    created_at: chrono::DateTime<chrono::Utc>,
}

impl Component for Metadata {
    fn as_any(&self) -> &dyn Any { self }
    fn clone_box(&self) -> Box<dyn Component> { Box::new(self.clone()) }
    fn type_name(&self) -> &'static str { "Metadata" }
}
```

### Component Storage

While this crate provides the trait, storage is typically implemented by consuming crates:

```rust
// Example storage pattern (implemented in domain crates)
pub struct ComponentStorage {
    components: HashMap<TypeId, Box<dyn Component>>,
}

impl ComponentStorage {
    pub fn add<C: Component>(&mut self, component: C) -> Result<(), ComponentError> {
        let type_id = TypeId::of::<C>();
        if self.components.contains_key(&type_id) {
            return Err(ComponentError::AlreadyExists(component.type_name().to_string()));
        }
        self.components.insert(type_id, Box::new(component));
        Ok(())
    }

    pub fn get<C: Component>(&self) -> Option<&C> {
        self.components
            .get(&TypeId::of::<C>())
            .and_then(|c| c.as_any().downcast_ref::<C>())
    }
}
```

## Common Component Types

### Visual Components

```rust
#[derive(Debug, Clone)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

#[derive(Debug, Clone)]
struct Label {
    text: String,
    font_size: f32,
    font_family: String,
}
```

### Behavioral Components

```rust
#[derive(Debug, Clone)]
struct Velocity {
    dx: f32,
    dy: f32,
    dz: f32,
}

#[derive(Debug, Clone)]
struct Lifetime {
    created_at: Instant,
    expires_at: Option<Instant>,
}
```

### Domain Components

```rust
#[derive(Debug, Clone)]
struct Permissions {
    read: bool,
    write: bool,
    delete: bool,
    share: bool,
}

#[derive(Debug, Clone)]
struct AuditInfo {
    created_by: UserId,
    modified_by: Option<UserId>,
    version: u64,
}
```

## Integration Examples

### With Entities

```rust
use cim_domain::Entity;

// Extend an entity with components
let mut entity = Entity::new();
entity.add_component(Position3D { x: 0.0, y: 0.0, z: 0.0 });
entity.add_component(Metadata {
    name: "Player".to_string(),
    description: Some("The main player character".to_string()),
    tags: vec!["player".to_string(), "character".to_string()],
    created_at: Utc::now(),
});

// Query components
if let Some(pos) = entity.get_component::<Position3D>() {
    println!("Entity at position: ({}, {}, {})", pos.x, pos.y, pos.z);
}
```

### With Graphs

```rust
// Add components to graph nodes
graph.add_node_component(node_id, Label {
    text: "Start Node".to_string(),
    font_size: 14.0,
    font_family: "Arial".to_string(),
});

// Add components to edges
graph.add_edge_component(edge_id, Weight(1.5));
```

## Error Handling

The module provides error types for component operations:

```rust
use cim_component::{ComponentError, ComponentResult};

fn add_unique_component<C: Component>(storage: &mut Storage, component: C) -> ComponentResult<()> {
    match storage.add(component) {
        Ok(()) => Ok(()),
        Err(ComponentError::AlreadyExists(name)) => {
            eprintln!("Component {} already exists", name);
            Err(ComponentError::AlreadyExists(name))
        }
        Err(e) => Err(e),
    }
}
```

## Best Practices

### 1. Keep Components Small

Components should represent a single concept:

```rust
// Good: Focused components
struct Position { x: f32, y: f32 }
struct Velocity { dx: f32, dy: f32 }

// Bad: Monolithic component
struct PhysicsData {
    position: (f32, f32),
    velocity: (f32, f32),
    acceleration: (f32, f32),
    mass: f32,
    friction: f32,
}
```

### 2. Prefer Composition

Build complex behavior by combining simple components:

```rust
// Entity with multiple components
entity.add_component(Position { x: 0.0, y: 0.0 });
entity.add_component(Velocity { dx: 1.0, dy: 0.0 });
entity.add_component(Sprite { texture: "player.png" });
entity.add_component(Health { current: 100, max: 100 });
```

### 3. Use Type Names Consistently

Component type names are used for debugging and serialization:

```rust
impl Component for MyComponent {
    fn type_name(&self) -> &'static str {
        // Use the actual type name for clarity
        "MyComponent"
    }
}
```

## Performance Considerations

### TypeId Lookups

Component lookups use `TypeId` which is very fast:
- O(1) HashMap lookup in most implementations
- No string comparisons
- Compile-time type safety

### Memory Layout

- Components are stored as trait objects (`Box<dyn Component>`)
- Consider cache locality for frequently accessed components
- Group related components in systems that process them

### Cloning

- Components must be cloneable for entity duplication
- Keep clone operations lightweight
- Consider using `Arc` for large shared data

## Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_component_type_safety() {
        let comp = TestComponent("data".to_string());
        
        // Correct type downcast succeeds
        let any_ref = comp.as_any();
        assert!(any_ref.downcast_ref::<TestComponent>().is_some());
        
        // Incorrect type downcast fails
        assert!(any_ref.downcast_ref::<OtherComponent>().is_none());
    }

    #[test]
    fn test_component_cloning() {
        let original = TestComponent("original".to_string());
        let cloned = original.clone_box();
        
        let cloned_ref = cloned.as_any().downcast_ref::<TestComponent>().unwrap();
        assert_eq!(cloned_ref.0, "original");
    }
}
```

## Future Enhancements

### Planned Features

1. **Component Queries**: Efficient querying of entities by component combinations
2. **Component Serialization**: Standardized serialization format
3. **Component Versioning**: Handle component schema evolution
4. **Component Dependencies**: Express relationships between components
5. **Component Pools**: Object pooling for frequently created/destroyed components

### Integration Goals

- Standardize component usage across all domain modules
- Provide derive macros for common implementations
- Support for component inheritance/composition patterns
- Integration with event sourcing for component changes

## Contributing

1. Keep the trait minimal and focused
2. Ensure thread safety (Send + Sync)
3. Maintain backward compatibility
4. Add tests for new functionality
5. Document component semantics clearly

## License

See the main project LICENSE file. 