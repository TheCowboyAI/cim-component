# CIM Module Guide for Claude

PLEASE READ main.mdc IN .claude
**YOU MUST FOLLOW THESE DIRECTIVES**

## Overview

The `cim-component` module is a foundational type-erased component system for CIM. It provides the building blocks for Entity Component System (ECS) patterns used throughout the CIM architecture. This module has zero dependencies on other CIM modules and serves as a core abstraction.

## Key Concepts

### Type Erasure
- Components are stored without compile-time type knowledge
- Uses `Any` trait for runtime type checking
- Enables heterogeneous component storage

### Component Storage
- Pluggable storage backends (in-memory, IPLD, custom)
- Thread-safe access (`Send + Sync`)
- Query system for efficient component retrieval

## Architecture Diagrams

### Component System Architecture

```mermaid
graph TB
    subgraph "Component Layer"
        trait[Component Trait]
        any[Any + Send + Sync]
        serde[Serialization]
        
        trait --> any
        trait --> serde
    end
    
    subgraph "Storage Layer"
        storage[ComponentStorage Trait]
        memory[InMemoryStorage]
        ipld[IPLDStorage]
        custom[Custom Backends]
        
        storage --> memory
        storage --> ipld
        storage --> custom
    end
    
    subgraph "Query Layer"
        query[Query System]
        single[Single Component]
        multi[Multiple Components]
        filter[Filtered Queries]
        
        query --> single
        query --> multi
        query --> filter
    end
    
    trait --> storage
    storage --> query
    
    style trait fill:#FF6B6B,stroke:#C92A2A,stroke-width:4px,color:#FFF
    style storage fill:#4ECDC4,stroke:#2B8A89,stroke-width:3px,color:#FFF
    style query fill:#FFE66D,stroke:#FCC419,stroke-width:3px,color:#000
    style any fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
    style serde fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
    style memory fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
    style ipld fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
    style custom fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
    style single fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
    style multi fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
    style filter fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
```

### Storage Backend Comparison

```mermaid
graph LR
    subgraph "Storage Backends"
        subgraph "InMemoryStorage"
            mem_fast[Fast Access]
            mem_temp[Temporary]
            mem_hash[HashMap Based]
        end
        
        subgraph "IPLDStorage"
            ipld_persist[Persistent]
            ipld_content[Content Addressed]
            ipld_merkle[Merkle DAG]
        end
        
        subgraph "Custom Storage"
            custom_sql[SQL Backend]
            custom_redis[Redis Backend]
            custom_hybrid[Hybrid Storage]
        end
    end
    
    style mem_fast fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
    style mem_temp fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
    style mem_hash fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
    style ipld_persist fill:#4ECDC4,stroke:#2B8A89,stroke-width:2px,color:#FFF
    style ipld_content fill:#4ECDC4,stroke:#2B8A89,stroke-width:2px,color:#FFF
    style ipld_merkle fill:#4ECDC4,stroke:#2B8A89,stroke-width:2px,color:#FFF
    style custom_sql fill:#FFE66D,stroke:#FCC419,stroke-width:2px,color:#000
    style custom_redis fill:#FFE66D,stroke:#FCC419,stroke-width:2px,color:#000
    style custom_hybrid fill:#FFE66D,stroke:#FCC419,stroke-width:2px,color:#000
```

## Module Structure

```
cim-component/
├── src/
│   ├── lib.rs              # Public API exports
│   ├── component.rs        # Component trait definition
│   ├── storage/
│   │   ├── mod.rs         # Storage trait
│   │   ├── memory.rs      # In-memory implementation
│   │   └── ipld.rs        # IPLD storage implementation
│   ├── query/
│   │   ├── mod.rs         # Query system
│   │   ├── single.rs      # Single component queries
│   │   └── multi.rs       # Multi-component queries
│   ├── error.rs           # Error types
│   └── serde.rs           # Serialization support
├── tests/
│   ├── component_tests.rs
│   ├── storage_tests.rs
│   └── query_tests.rs
└── benches/
    └── component_bench.rs
```

## Common Tasks

### Implementing a New Component

```rust
use cim_component::Component;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MyComponent {
    pub value: String,
}

impl Component for MyComponent {
    fn component_type() -> &'static str {
        "MyComponent"
    }
}
```

### Creating a Storage Backend

```rust
use cim_component::{ComponentStorage, StorageError};

pub struct MyStorage {
    // Implementation details
}

impl ComponentStorage for MyStorage {
    fn store<C: Component>(&self, id: Uuid, component: C) -> Result<(), StorageError> {
        // Store component
    }
    
    fn get<C: Component>(&self, id: Uuid) -> Result<Option<C>, StorageError> {
        // Retrieve component
    }
}
```

### Component Query Patterns

```mermaid
graph TD
    subgraph "Query Types"
        Single[Single Component Query]
        Tuple[Tuple Query]
        Optional[Optional Components]
        Filtered[Filtered Query]
    end
    
    subgraph "Examples"
        Single --> |"get<Position>(id)"| GetPos[Get Position]
        Tuple --> |"query<(Pos, Vel)>()"| GetBoth[Get Position + Velocity]
        Optional --> |"query<(Pos, Option<Vel>)>()"| GetOpt[Position + Maybe Velocity]
        Filtered --> |"query_filtered<Pos>(filter)"| GetFiltered[Filtered Positions]
    end
    
    style Single fill:#FF6B6B,stroke:#C92A2A,stroke-width:3px,color:#FFF
    style Tuple fill:#4ECDC4,stroke:#2B8A89,stroke-width:3px,color:#FFF
    style Optional fill:#FFE66D,stroke:#FCC419,stroke-width:3px,color:#000
    style Filtered fill:#95E1D3,stroke:#63C7B8,stroke-width:3px,color:#000
    style GetPos fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
    style GetBoth fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
    style GetOpt fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
    style GetFiltered fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
```

## Performance Optimization

### Storage Strategy Selection

```mermaid
flowchart TD
    Start[Component Type] --> Size{Size?}
    Size -->|Small < 64B| Dense[Dense Storage]
    Size -->|Large > 64B| Sparse[Sparse Storage]
    
    Dense --> Access{Access Pattern?}
    Access -->|Sequential| Array[Array Storage]
    Access -->|Random| HashMap[HashMap Storage]
    
    Sparse --> Lifetime{Lifetime?}
    Lifetime -->|Short| Memory[In-Memory]
    Lifetime -->|Long| Persistent[IPLD/Database]
    
    Array --> |Best for| Iteration[Fast Iteration]
    HashMap --> |Best for| Lookup[Fast Lookup]
    Memory --> |Best for| Speed[Maximum Speed]
    Persistent --> |Best for| Durability[Data Durability]
    
    style Start fill:#2D3436,stroke:#000,stroke-width:3px,color:#FFF
    style Size fill:#FF6B6B,stroke:#C92A2A,stroke-width:3px,color:#FFF
    style Dense fill:#4ECDC4,stroke:#2B8A89,stroke-width:3px,color:#FFF
    style Sparse fill:#4ECDC4,stroke:#2B8A89,stroke-width:3px,color:#FFF
    style Access fill:#FFE66D,stroke:#FCC419,stroke-width:3px,color:#000
    style Lifetime fill:#FFE66D,stroke:#FCC419,stroke-width:3px,color:#000
    style Array fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
    style HashMap fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
    style Memory fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
    style Persistent fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
    style Iteration fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
    style Lookup fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
    style Speed fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
    style Durability fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
```

## Error Handling

### Error Hierarchy

```mermaid
graph TD
    ComponentError[ComponentError]
    ComponentError --> Storage[StorageError]
    ComponentError --> Type[TypeError]
    ComponentError --> Serde[SerdeError]
    
    Storage --> NotFound[NotFound]
    Storage --> Backend[BackendError]
    Storage --> Capacity[CapacityError]
    
    Type --> Mismatch[TypeMismatch]
    Type --> NotRegistered[NotRegistered]
    
    Serde --> Serialize[SerializeError]
    Serde --> Deserialize[DeserializeError]
    
    style ComponentError fill:#FF6B6B,stroke:#C92A2A,stroke-width:4px,color:#FFF
    style Storage fill:#4ECDC4,stroke:#2B8A89,stroke-width:3px,color:#FFF
    style Type fill:#4ECDC4,stroke:#2B8A89,stroke-width:3px,color:#FFF
    style Serde fill:#4ECDC4,stroke:#2B8A89,stroke-width:3px,color:#FFF
    style NotFound fill:#FFE66D,stroke:#FCC419,stroke-width:2px,color:#000
    style Backend fill:#FFE66D,stroke:#FCC419,stroke-width:2px,color:#000
    style Capacity fill:#FFE66D,stroke:#FCC419,stroke-width:2px,color:#000
    style Mismatch fill:#FFE66D,stroke:#FCC419,stroke-width:2px,color:#000
    style NotRegistered fill:#FFE66D,stroke:#FCC419,stroke-width:2px,color:#000
    style Serialize fill:#FFE66D,stroke:#FCC419,stroke-width:2px,color:#000
    style Deserialize fill:#FFE66D,stroke:#FCC419,stroke-width:2px,color:#000
```

## Integration Points

### With Other CIM Modules

```mermaid
graph LR
    Component[cim-component]
    
    Component --> |stored in| IPLD[cim-ipld]
    Component --> |used by| Domain[cim-domain]
    Component --> |foundation for| Person[cim-domain-person]
    Component --> |foundation for| Org[cim-domain-organization]
    Component --> |enables| Graph[cim-contextgraph]
    
    IPLD --> |content addressing| Component
    Domain --> |defines| Component
    
    style Component fill:#FF6B6B,stroke:#C92A2A,stroke-width:4px,color:#FFF
    style IPLD fill:#4ECDC4,stroke:#2B8A89,stroke-width:3px,color:#FFF
    style Domain fill:#4ECDC4,stroke:#2B8A89,stroke-width:3px,color:#FFF
    style Person fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
    style Org fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
    style Graph fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
```

## Testing Strategy

### Test Categories

1. **Unit Tests**
   - Component trait implementation
   - Storage backend operations
   - Serialization/deserialization

2. **Integration Tests**
   - Multi-component queries
   - Storage backend switching
   - Error propagation

3. **Performance Tests**
   - Component access speed
   - Query performance
   - Storage overhead

### Test Patterns

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_component_storage() {
        let storage = InMemoryStorage::new();
        let id = Uuid::new_v4();
        let component = Position { x: 1.0, y: 2.0 };
        
        storage.store(id, component.clone()).unwrap();
        let retrieved = storage.get::<Position>(id).unwrap().unwrap();
        
        assert_eq!(component.x, retrieved.x);
    }
}
```

## Debugging Tips

### Common Issues

1. **Type Mismatch Errors**
   - Ensure component type names are unique
   - Check serialization round-trip compatibility

2. **Storage Performance**
   - Profile component access patterns
   - Consider switching storage backends

3. **Query Performance**
   - Use appropriate query types
   - Consider component layout optimization

### Debug Visualization

```mermaid
graph TD
    subgraph "Debug Flow"
        Issue[Performance Issue]
        Issue --> Profile[Profile Code]
        Profile --> Identify[Identify Bottleneck]
        
        Identify --> CompAccess[Component Access]
        Identify --> Storage[Storage Backend]
        Identify --> Queries[Query Pattern]
        
        CompAccess --> OptLayout[Optimize Layout]
        Storage --> SwitchBackend[Switch Backend]
        Queries --> RefactorQuery[Refactor Queries]
    end
    
    style Issue fill:#FF6B6B,stroke:#C92A2A,stroke-width:3px,color:#FFF
    style Profile fill:#4ECDC4,stroke:#2B8A89,stroke-width:3px,color:#FFF
    style Identify fill:#FFE66D,stroke:#FCC419,stroke-width:3px,color:#000
    style CompAccess fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
    style Storage fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
    style Queries fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
    style OptLayout fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
    style SwitchBackend fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
    style RefactorQuery fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
```

## Best Practices

1. **Component Design**
   - Keep components small and focused
   - Use value semantics when possible
   - Avoid component interdependencies

2. **Storage Selection**
   - Match storage to access patterns
   - Consider persistence requirements
   - Profile before optimizing

3. **Query Optimization**
   - Use tuple queries for related components
   - Leverage optional queries for sparse data
   - Cache query results when appropriate

## Future Considerations

- Parallel query execution
- Component versioning system
- Advanced indexing strategies
- GPU-friendly component layouts

## Documentation Standards

### Mermaid Graph Styling
All mermaid graphs in this project must follow the high-contrast color scheme defined in [.claude/mermaid-styling.md](.claude/mermaid-styling.md). This ensures:
- Visual consistency across all documentation
- High contrast for better accessibility
- Semantic color mapping for intuitive understanding

## Related Documentation

- [Architecture Overview](doc/cim-component/architecture.md)
- [Storage Backends](doc/cim-component/storage-backends.md)
- [Query System](doc/cim-component/query-system.md)
- [Performance Guide](doc/cim-component/performance.md)