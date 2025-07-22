# CIM Component Query System

## Overview

The CIM Component Query System provides a powerful and flexible way to retrieve components from storage. It supports various query patterns from simple single-component lookups to complex multi-component joins with filtering and aggregation.

## Query System Architecture

```mermaid
graph TB
    subgraph "Query API Layer"
        Builder[QueryBuilder]
        DSL[Query DSL]
        Macros[Query Macros]
    end
    
    subgraph "Query Planning"
        Parser[Query Parser]
        Optimizer[Query Optimizer]
        Planner[Execution Planner]
    end
    
    subgraph "Query Execution"
        Executor[Query Executor]
        Cache[Query Cache]
        Parallel[Parallel Execution]
    end
    
    subgraph "Result Processing"
        Stream[Result Stream]
        Transform[Transformations]
        Aggregate[Aggregations]
    end
    
    Builder --> Parser
    DSL --> Parser
    Macros --> Parser
    
    Parser --> Optimizer
    Optimizer --> Planner
    Planner --> Executor
    
    Executor --> Cache
    Executor --> Parallel
    
    Cache --> Stream
    Parallel --> Stream
    
    Stream --> Transform
    Stream --> Aggregate
    
    style Builder fill:#FF6B6B,stroke:#C92A2A,stroke-width:4px,color:#FFF
    style DSL fill:#FF6B6B,stroke:#C92A2A,stroke-width:4px,color:#FFF
    style Macros fill:#FF6B6B,stroke:#C92A2A,stroke-width:4px,color:#FFF
    style Parser fill:#4ECDC4,stroke:#2B8A89,stroke-width:3px,color:#FFF
    style Optimizer fill:#4ECDC4,stroke:#2B8A89,stroke-width:3px,color:#FFF
    style Planner fill:#4ECDC4,stroke:#2B8A89,stroke-width:3px,color:#FFF
    style Executor fill:#FFE66D,stroke:#FCC419,stroke-width:3px,color:#000
    style Cache fill:#FFE66D,stroke:#FCC419,stroke-width:3px,color:#000
    style Parallel fill:#FFE66D,stroke:#FCC419,stroke-width:3px,color:#000
    style Stream fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
    style Transform fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
    style Aggregate fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
```

## Query Types

### Single Component Query

```mermaid
sequenceDiagram
    participant User
    participant Query
    participant Storage
    participant Cache
    
    User->>Query: storage.get<Position>(entity_id)
    Query->>Cache: Check cache
    
    alt Cache Hit
        Cache-->>Query: Return component
    else Cache Miss
        Query->>Storage: Fetch component
        Storage-->>Query: Component data
        Query->>Cache: Update cache
    end
    
    Query-->>User: Option<Position>
```

Example:
```rust
// Get a single component
if let Some(position) = storage.get::<Position>(entity_id)? {
    println!("Entity at: {:?}", position);
}

// Get with error handling
match storage.get::<Health>(entity_id) {
    Ok(Some(health)) => println!("Health: {}", health.value),
    Ok(None) => println!("No health component"),
    Err(e) => eprintln!("Error: {}", e),
}
```

### Tuple Query

```mermaid
graph LR
    subgraph "Tuple Query Process"
        Request[Query Request]
        Join[Component Join]
        Filter[Filter Entities]
        Result[Tuple Results]
    end
    
    subgraph "Join Strategy"
        Inner[Inner Join]
        Left[Left Join]
        Cross[Cross Product]
    end
    
    Request --> Join
    Join --> Filter
    Filter --> Result
    
    Join --> Inner
    Join --> Left
    Join --> Cross
    
    style Request fill:#2D3436,stroke:#000,stroke-width:3px,color:#FFF
    style Join fill:#FF6B6B,stroke:#C92A2A,stroke-width:3px,color:#FFF
    style Filter fill:#4ECDC4,stroke:#2B8A89,stroke-width:3px,color:#FFF
    style Result fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
    style Inner fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
    style Left fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
    style Cross fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
```

Example:
```rust
// Query entities with both Position and Velocity
for (id, pos, vel) in storage.query::<(Position, Velocity)>()? {
    println!("Entity {} at {:?} moving at {:?}", id, pos, vel);
}

// Query with optional components
for (id, pos, health) in storage.query::<(Position, Option<Health>)>()? {
    match health {
        Some(h) => println!("Entity {} at {:?} with health {}", id, pos, h.value),
        None => println!("Entity {} at {:?} (no health)", id, pos),
    }
}
```

### Filtered Query

```mermaid
flowchart TD
    subgraph "Filter Pipeline"
        Query[Base Query]
        Filter1[Component Filter]
        Filter2[Predicate Filter]
        Filter3[Range Filter]
        Result[Filtered Results]
    end
    
    Query --> Filter1
    Filter1 --> Filter2
    Filter2 --> Filter3
    Filter3 --> Result
    
    subgraph "Filter Types"
        Equality[Equality Filter]
        Range[Range Filter]
        Predicate[Custom Predicate]
        Composite[Composite Filter]
    end
    
    Filter1 --> Equality
    Filter2 --> Predicate
    Filter3 --> Range
    
    style Query fill:#2D3436,stroke:#000,stroke-width:3px,color:#FFF
    style Filter1 fill:#FF6B6B,stroke:#C92A2A,stroke-width:3px,color:#FFF
    style Filter2 fill:#4ECDC4,stroke:#2B8A89,stroke-width:3px,color:#FFF
    style Filter3 fill:#FFE66D,stroke:#FCC419,stroke-width:3px,color:#000
    style Result fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
    style Equality fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
    style Range fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
    style Predicate fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
    style Composite fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
```

Example:
```rust
// Query with filters
let query = storage.query::<Position>()
    .filter(|pos| pos.x > 0.0 && pos.y > 0.0)
    .filter_range(|pos| pos.z, 0.0..100.0);

for (id, pos) in query.execute()? {
    println!("Entity {} in positive quadrant at {:?}", id, pos);
}

// Complex filtering
let nearby_enemies = storage.query::<(Position, Team)>()
    .filter(|(pos, team)| {
        let distance = (pos.x * pos.x + pos.y * pos.y).sqrt();
        distance < 50.0 && team.id != player_team_id
    })
    .sort_by(|(pos, _)| {
        (pos.x * pos.x + pos.y * pos.y).sqrt()
    });
```

## Query Builder API

### Fluent Interface

```mermaid
classDiagram
    class QueryBuilder~T~ {
        +filter(predicate: Fn) -> Self
        +filter_range(selector: Fn, range: Range) -> Self
        +sort_by(key: Fn) -> Self
        +limit(n: usize) -> Self
        +offset(n: usize) -> Self
        +distinct() -> Self
        +execute() -> Result~Iterator~
    }
    
    class Query~T~ {
        <<trait>>
        +type Item
        +build() -> QueryPlan
    }
    
    class QueryPlan {
        +filters: Vec~Filter~
        +sorts: Vec~Sort~
        +limit: Option~usize~
        +offset: Option~usize~
        +distinct: bool
    }
    
    QueryBuilder --> Query
    Query --> QueryPlan
```

### Query DSL

```rust
// Using the query macro
query! {
    from storage
    select (Position, Velocity, ?Health)
    where position.x > 0 && velocity.magnitude() > 10
    order by position.distance_from_origin()
    limit 100
}

// Equivalent builder pattern
storage.query::<(Position, Velocity, Option<Health>)>()
    .filter(|(pos, vel, _)| pos.x > 0.0 && vel.magnitude() > 10.0)
    .sort_by(|(pos, _, _)| pos.distance_from_origin())
    .limit(100)
    .execute()?
```

## Query Optimization

### Query Planning

```mermaid
graph TD
    subgraph "Query Optimization Pipeline"
        Parse[Parse Query]
        Analyze[Analyze Predicates]
        Reorder[Reorder Operations]
        Index[Select Indices]
        Plan[Generate Plan]
    end
    
    subgraph "Optimization Strategies"
        PushDown[Predicate Pushdown]
        Merge[Filter Merging]
        IndexUse[Index Selection]
        Parallel[Parallelization]
    end
    
    Parse --> Analyze
    Analyze --> Reorder
    Reorder --> Index
    Index --> Plan
    
    Analyze --> PushDown
    Reorder --> Merge
    Index --> IndexUse
    Plan --> Parallel
    
    style Parse fill:#2D3436,stroke:#000,stroke-width:3px,color:#FFF
    style Analyze fill:#FF6B6B,stroke:#C92A2A,stroke-width:3px,color:#FFF
    style Reorder fill:#4ECDC4,stroke:#2B8A89,stroke-width:3px,color:#FFF
    style Index fill:#FFE66D,stroke:#FCC419,stroke-width:3px,color:#000
    style Plan fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
    style PushDown fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
    style Merge fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
    style IndexUse fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
    style Parallel fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
```

### Index Usage

```mermaid
graph LR
    subgraph "Index Types"
        Hash[Hash Index]
        BTree[B-Tree Index]
        Spatial[Spatial Index]
        Composite[Composite Index]
    end
    
    subgraph "Query Patterns"
        Equality[Equality Queries]
        Range[Range Queries]
        Spatial[Spatial Queries]
        Multi[Multi-field Queries]
    end
    
    subgraph "Index Selection"
        Equality --> Hash
        Range --> BTree
        Spatial --> Spatial
        Multi --> Composite
    end
    
    style Hash fill:#FF6B6B,stroke:#C92A2A,stroke-width:3px,color:#FFF
    style BTree fill:#4ECDC4,stroke:#2B8A89,stroke-width:3px,color:#FFF
    style Spatial fill:#FFE66D,stroke:#FCC419,stroke-width:3px,color:#000
    style Composite fill:#95E1D3,stroke:#63C7B8,stroke-width:3px,color:#000
    style Equality fill:#2D3436,stroke:#000,stroke-width:2px,color:#FFF
    style Range fill:#2D3436,stroke:#000,stroke-width:2px,color:#FFF
    style Spatial fill:#2D3436,stroke:#000,stroke-width:2px,color:#FFF
    style Multi fill:#2D3436,stroke:#000,stroke-width:2px,color:#FFF
```

Example index configuration:
```rust
// Define indices for common query patterns
storage.create_index::<Position>("spatial", IndexType::RTree)?;
storage.create_index::<Health>("health_range", IndexType::BTree)?;
storage.create_composite_index::<(Team, Position)>("team_location")?;

// Queries automatically use appropriate indices
let team_members = storage.query::<(Team, Position)>()
    .filter(|(team, _)| team.id == player_team)
    .execute()?; // Uses team_location index
```

## Advanced Query Features

### Aggregation Queries

```mermaid
graph TB
    subgraph "Aggregation Pipeline"
        Source[Component Stream]
        Group[Group By]
        Aggregate[Aggregate Functions]
        Result[Aggregated Results]
    end
    
    subgraph "Aggregation Functions"
        Count[Count]
        Sum[Sum]
        Avg[Average]
        MinMax[Min/Max]
        Custom[Custom Aggregators]
    end
    
    Source --> Group
    Group --> Aggregate
    Aggregate --> Result
    
    Aggregate --> Count
    Aggregate --> Sum
    Aggregate --> Avg
    Aggregate --> MinMax
    Aggregate --> Custom
    
    style Source fill:#2D3436,stroke:#000,stroke-width:3px,color:#FFF
    style Group fill:#FF6B6B,stroke:#C92A2A,stroke-width:3px,color:#FFF
    style Aggregate fill:#4ECDC4,stroke:#2B8A89,stroke-width:3px,color:#FFF
    style Result fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
    style Count fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
    style Sum fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
    style Avg fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
    style MinMax fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
    style Custom fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
```

Example:
```rust
// Count entities by team
let team_counts = storage.query::<Team>()
    .group_by(|team| team.id)
    .count();

// Average position by team
let team_positions = storage.query::<(Team, Position)>()
    .group_by(|(team, _)| team.id)
    .aggregate(|positions| {
        let sum = positions.fold(Vec3::ZERO, |acc, (_, pos)| acc + pos.as_vec());
        sum / positions.count() as f32
    });

// Complex aggregation
let stats = storage.query::<(Health, Position, Team)>()
    .group_by(|(_, _, team)| team.id)
    .aggregate_multiple(|entities| {
        let count = entities.count();
        let avg_health = entities.map(|(h, _, _)| h.value).sum() / count as f32;
        let center = entities.map(|(_, p, _)| p.as_vec()).sum() / count as f32;
        TeamStats { count, avg_health, center }
    });
```

### Streaming Queries

```mermaid
sequenceDiagram
    participant User
    participant Query
    participant Storage
    participant Stream
    
    User->>Query: Create streaming query
    Query->>Storage: Initialize cursor
    Query->>Stream: Create stream
    
    loop While has data
        User->>Stream: next()
        Stream->>Storage: Fetch batch
        Storage-->>Stream: Component batch
        Stream-->>User: Next items
    end
    
    User->>Stream: close()
    Stream->>Storage: Release cursor
```

Example:
```rust
// Process large result sets in batches
let mut stream = storage.query::<Position>()
    .stream(1000)?; // Batch size of 1000

while let Some(batch) = stream.next_batch().await? {
    for (id, pos) in batch {
        // Process without loading all results into memory
        process_entity(id, pos).await?;
    }
}

// Async streaming with backpressure
let stream = storage.query::<LargeComponent>()
    .filter(|c| c.active)
    .stream_async(100);

stream
    .for_each_concurrent(10, |batch| async {
        process_batch(batch).await
    })
    .await?;
```

### Query Composition

```mermaid
graph LR
    subgraph "Query Composition"
        Base[Base Query]
        Sub1[Subquery 1]
        Sub2[Subquery 2]
        Combine[Combinator]
        Final[Final Query]
    end
    
    Base --> Combine
    Sub1 --> Combine
    Sub2 --> Combine
    Combine --> Final
    
    subgraph "Combinators"
        Union[Union]
        Intersect[Intersection]
        Except[Difference]
        Join[Join]
    end
    
    Combine --> Union
    Combine --> Intersect
    Combine --> Except
    Combine --> Join
    
    style Base fill:#2D3436,stroke:#000,stroke-width:3px,color:#FFF
    style Sub1 fill:#FF6B6B,stroke:#C92A2A,stroke-width:3px,color:#FFF
    style Sub2 fill:#FF6B6B,stroke:#C92A2A,stroke-width:3px,color:#FFF
    style Combine fill:#4ECDC4,stroke:#2B8A89,stroke-width:3px,color:#FFF
    style Final fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
    style Union fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
    style Intersect fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
    style Except fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
    style Join fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
```

Example:
```rust
// Combine multiple queries
let allies = storage.query::<Team>()
    .filter(|t| t.alliance == Alliance::Friendly);

let nearby = storage.query::<Position>()
    .filter(|p| p.distance_from(player_pos) < 100.0);

let nearby_allies = allies.intersect(nearby)?;

// Complex joins
let result = storage.query::<Position>()
    .join(
        storage.query::<Health>(),
        |id1, id2| id1 == id2
    )
    .left_join(
        storage.query::<Shield>(),
        |id1, id2| id1 == id2
    )
    .execute()?;
```

## Performance Considerations

### Query Performance Matrix

```mermaid
graph TD
    subgraph "Performance Factors"
        Components[Component Count]
        Filters[Filter Complexity]
        Indices[Index Availability]
        Storage[Storage Backend]
    end
    
    subgraph "Performance Impact"
        Low[Low Impact]
        Medium[Medium Impact]
        High[High Impact]
    end
    
    Components --> High
    Filters --> Medium
    Indices --> High
    Storage --> Medium
    
    style Components fill:#FF6B6B,stroke:#C92A2A,stroke-width:3px,color:#FFF
    style Filters fill:#4ECDC4,stroke:#2B8A89,stroke-width:3px,color:#FFF
    style Indices fill:#95E1D3,stroke:#63C7B8,stroke-width:3px,color:#000
    style Storage fill:#FFE66D,stroke:#FCC419,stroke-width:3px,color:#000
    style Low fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
    style Medium fill:#FFE66D,stroke:#FCC419,stroke-width:2px,color:#000
    style High fill:#FF6B6B,stroke:#C92A2A,stroke-width:2px,color:#FFF
```

### Optimization Guidelines

1. **Use Indices for Common Queries**
   ```rust
   // Create indices for frequently queried components
   storage.create_index::<Position>("position_x", |p| p.x)?;
   ```

2. **Filter Early, Transform Late**
   ```rust
   // Good: Filter before transformation
   query.filter(|p| p.active).map(|p| expensive_transform(p))
   
   // Bad: Transform before filter
   query.map(|p| expensive_transform(p)).filter(|t| t.active)
   ```

3. **Use Appropriate Query Types**
   ```rust
   // For single lookups
   storage.get::<Position>(id)?
   
   // For bulk operations
   storage.query::<Position>().execute()?
   
   // For large result sets
   storage.query::<Position>().stream(1000)?
   ```

4. **Cache Query Results**
   ```rust
   let cache = QueryCache::new(1000);
   let result = cache.get_or_compute(query_key, || {
       storage.query::<ExpensiveComponent>().execute()
   })?;
   ```

## Query Debugging

### Query Profiling

```mermaid
graph LR
    subgraph "Profiling Tools"
        Explain[Query Explain]
        Profile[Performance Profile]
        Trace[Execution Trace]
    end
    
    subgraph "Metrics"
        Time[Execution Time]
        Memory[Memory Usage]
        IO[I/O Operations]
        Cache[Cache Hit Rate]
    end
    
    Explain --> Time
    Profile --> Memory
    Trace --> IO
    Trace --> Cache
    
    style Explain fill:#FF6B6B,stroke:#C92A2A,stroke-width:3px,color:#FFF
    style Profile fill:#4ECDC4,stroke:#2B8A89,stroke-width:3px,color:#FFF
    style Trace fill:#FFE66D,stroke:#FCC419,stroke-width:3px,color:#000
    style Time fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
    style Memory fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
    style IO fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
    style Cache fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
```

Example:
```rust
// Enable query profiling
let result = storage.query::<Position>()
    .filter(|p| p.x > 0.0)
    .profile()
    .execute()?;

println!("Query stats: {:?}", result.profile_data());
// Output: QueryProfile {
//   execution_time: 1.23ms,
//   components_scanned: 10000,
//   components_returned: 523,
//   index_used: Some("position_x"),
//   cache_hits: 0,
//   cache_misses: 523
// }

// Query explain plan
let plan = storage.query::<(Position, Velocity)>()
    .filter(|(p, v)| p.x > 0.0 && v.magnitude() > 10.0)
    .explain();

println!("Execution plan:\n{}", plan);
// Output:
// 1. Scan index "position_x" where x > 0.0
// 2. Join with Velocity components
// 3. Filter where velocity.magnitude() > 10.0
// 4. Return tuples
```

## Best Practices

1. **Design Queries for Your Access Patterns**
   - Profile actual usage
   - Create appropriate indices
   - Use the right query type

2. **Minimize Memory Usage**
   - Use streaming for large results
   - Project only needed fields
   - Clear caches periodically

3. **Handle Errors Gracefully**
   - Check for component existence
   - Handle storage errors
   - Provide meaningful error messages

4. **Test Query Performance**
   - Benchmark with realistic data
   - Test with concurrent access
   - Monitor production queries