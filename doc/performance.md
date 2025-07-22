# CIM Component Performance Guide

## Overview

This guide provides comprehensive performance optimization strategies for the CIM Component system. It covers memory layout optimization, query performance tuning, storage backend selection, and concurrent access patterns.

## Performance Architecture

```mermaid
graph TB
    subgraph "Performance Layers"
        Application[Application Layer]
        Query[Query Layer]
        Storage[Storage Layer]
        Memory[Memory Layer]
    end
    
    subgraph "Optimization Points"
        AppOpt[API Usage]
        QueryOpt[Query Planning]
        StorageOpt[Storage Strategy]
        MemOpt[Memory Layout]
    end
    
    subgraph "Metrics"
        Latency[Latency]
        Throughput[Throughput]
        MemUsage[Memory Usage]
        CacheHit[Cache Hit Rate]
    end
    
    Application --> AppOpt
    Query --> QueryOpt
    Storage --> StorageOpt
    Memory --> MemOpt
    
    AppOpt --> Latency
    QueryOpt --> Throughput
    StorageOpt --> CacheHit
    MemOpt --> MemUsage
    
    style Application fill:#FF6B6B,stroke:#C92A2A,stroke-width:4px,color:#FFF
    style Query fill:#4ECDC4,stroke:#2B8A89,stroke-width:3px,color:#FFF
    style Storage fill:#4ECDC4,stroke:#2B8A89,stroke-width:3px,color:#FFF
    style Memory fill:#4ECDC4,stroke:#2B8A89,stroke-width:3px,color:#FFF
    style AppOpt fill:#FFE66D,stroke:#FCC419,stroke-width:3px,color:#000
    style QueryOpt fill:#FFE66D,stroke:#FCC419,stroke-width:3px,color:#000
    style StorageOpt fill:#FFE66D,stroke:#FCC419,stroke-width:3px,color:#000
    style MemOpt fill:#FFE66D,stroke:#FCC419,stroke-width:3px,color:#000
    style Latency fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
    style Throughput fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
    style MemUsage fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
    style CacheHit fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
```

## Memory Layout Optimization

### Component Data Layout

```mermaid
graph LR
    subgraph "Layout Strategies"
        AoS[Array of Structs]
        SoA[Struct of Arrays]
        Hybrid[Hybrid Layout]
        Packed[Packed Layout]
    end
    
    subgraph "Access Patterns"
        Random[Random Access]
        Sequential[Sequential Scan]
        Partial[Partial Access]
        Bulk[Bulk Operations]
    end
    
    subgraph "Performance Impact"
        Random --> AoS
        Sequential --> SoA
        Partial --> Hybrid
        Bulk --> Packed
    end
    
    style AoS fill:#FF6B6B,stroke:#C92A2A,stroke-width:3px,color:#FFF
    style SoA fill:#4ECDC4,stroke:#2B8A89,stroke-width:3px,color:#FFF
    style Hybrid fill:#FFE66D,stroke:#FCC419,stroke-width:3px,color:#000
    style Packed fill:#95E1D3,stroke:#63C7B8,stroke-width:3px,color:#000
    style Random fill:#2D3436,stroke:#000,stroke-width:2px,color:#FFF
    style Sequential fill:#2D3436,stroke:#000,stroke-width:2px,color:#FFF
    style Partial fill:#2D3436,stroke:#000,stroke-width:2px,color:#FFF
    style Bulk fill:#2D3436,stroke:#000,stroke-width:2px,color:#FFF
```

### Memory Layout Examples

```rust
// Array of Structs (AoS) - Good for random access
pub struct PositionAoS {
    positions: Vec<Position>,
}

impl PositionAoS {
    fn get(&self, index: usize) -> Option<&Position> {
        self.positions.get(index)
    }
}

// Struct of Arrays (SoA) - Good for SIMD and cache efficiency
pub struct PositionSoA {
    x: Vec<f32>,
    y: Vec<f32>,
    z: Vec<f32>,
}

impl PositionSoA {
    fn update_all_x(&mut self, delta: f32) {
        // Vectorized operation on contiguous memory
        for x in &mut self.x {
            *x += delta;
        }
    }
}

// Hybrid layout - Balance between AoS and SoA
pub struct HybridLayout {
    hot_data: SoA,      // Frequently accessed
    cold_data: AoS,     // Rarely accessed
}
```

### Cache Optimization

```mermaid
graph TD
    subgraph "Cache Hierarchy"
        L1[L1 Cache - 64KB]
        L2[L2 Cache - 256KB]
        L3[L3 Cache - 8MB]
        RAM[Main Memory]
    end
    
    subgraph "Access Times"
        L1Time[~4 cycles]
        L2Time[~12 cycles]
        L3Time[~40 cycles]
        RAMTime[~200 cycles]
    end
    
    L1 --> L1Time
    L2 --> L2Time
    L3 --> L3Time
    RAM --> RAMTime
    
    subgraph "Optimization Strategies"
        Locality[Spatial Locality]
        Prefetch[Prefetching]
        Align[Cache Line Alignment]
    end
    
    style L1 fill:#95E1D3,stroke:#63C7B8,stroke-width:3px,color:#000
    style L2 fill:#FFE66D,stroke:#FCC419,stroke-width:3px,color:#000
    style L3 fill:#4ECDC4,stroke:#2B8A89,stroke-width:3px,color:#FFF
    style RAM fill:#FF6B6B,stroke:#C92A2A,stroke-width:3px,color:#FFF
    style L1Time fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
    style L2Time fill:#FFE66D,stroke:#FCC419,stroke-width:2px,color:#000
    style L3Time fill:#4ECDC4,stroke:#2B8A89,stroke-width:2px,color:#FFF
    style RAMTime fill:#FF6B6B,stroke:#C92A2A,stroke-width:2px,color:#FFF
    style Locality fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
    style Prefetch fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
    style Align fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
```

### Cache-Friendly Component Design

```rust
// Cache line size (typically 64 bytes)
const CACHE_LINE_SIZE: usize = 64;

// Aligned component for better cache performance
#[repr(align(64))]
#[derive(Component)]
pub struct CacheAlignedTransform {
    position: Vec3,     // 12 bytes
    rotation: Quat,     // 16 bytes
    scale: Vec3,        // 12 bytes
    _padding: [u8; 24], // Padding to 64 bytes
}

// Hot/Cold data separation
#[derive(Component)]
pub struct OptimizedEntity {
    // Hot data - frequently accessed together
    hot: HotData,
    // Cold data - rarely accessed
    cold: Box<ColdData>,
}

#[repr(C)]
struct HotData {
    position: Vec3,
    velocity: Vec3,
    health: f32,
    _padding: [u8; 4],
}

struct ColdData {
    name: String,
    description: String,
    metadata: HashMap<String, Value>,
}
```

## Query Performance

### Query Optimization Pipeline

```mermaid
flowchart TD
    Query[Query Request]
    Parse[Parse Query]
    Optimize[Optimize Plan]
    Execute[Execute Query]
    
    Query --> Parse
    Parse --> Optimize
    Optimize --> Execute
    
    subgraph "Optimization Steps"
        Predicate[Predicate Pushdown]
        Index[Index Selection]
        Join[Join Ordering]
        Parallel[Parallelization]
    end
    
    Optimize --> Predicate
    Optimize --> Index
    Optimize --> Join
    Optimize --> Parallel
    
    subgraph "Execution Strategies"
        Sequential[Sequential Scan]
        Indexed[Index Scan]
        Partitioned[Partitioned Scan]
        Vectorized[Vectorized Ops]
    end
    
    Execute --> Sequential
    Execute --> Indexed
    Execute --> Partitioned
    Execute --> Vectorized
    
    style Query fill:#2D3436,stroke:#000,stroke-width:4px,color:#FFF
    style Parse fill:#FF6B6B,stroke:#C92A2A,stroke-width:3px,color:#FFF
    style Optimize fill:#4ECDC4,stroke:#2B8A89,stroke-width:3px,color:#FFF
    style Execute fill:#FFE66D,stroke:#FCC419,stroke-width:3px,color:#000
    style Predicate fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
    style Index fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
    style Join fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
    style Parallel fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
    style Sequential fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
    style Indexed fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
    style Partitioned fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
    style Vectorized fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
```

### Query Performance Patterns

```rust
// Efficient batch queries
pub fn process_entities_batch(storage: &dyn ComponentStorage) -> Result<()> {
    // Pre-allocate result vector
    let mut results = Vec::with_capacity(10000);
    
    // Use streaming to avoid loading all at once
    let mut stream = storage.query::<(Position, Velocity)>()
        .filter(|(pos, _)| pos.x > 0.0)
        .stream(1000)?;
    
    while let Some(batch) = stream.next_batch()? {
        // Process in parallel
        batch.par_iter()
            .filter(|(_, vel)| vel.magnitude() > 10.0)
            .for_each(|(id, (pos, vel))| {
                // Process entity
            });
    }
    
    Ok(())
}

// Optimized multi-component query
pub fn complex_query_optimized(storage: &dyn ComponentStorage) -> Result<Vec<EntityData>> {
    // Create indices for common queries
    storage.create_index::<Team>("team_id", |t| t.id)?;
    storage.create_index::<Position>("position_quad", |p| {
        let quad_x = (p.x / 100.0) as i32;
        let quad_y = (p.y / 100.0) as i32;
        quad_x * 1000 + quad_y // Spatial hash
    })?;
    
    // Query uses indices automatically
    storage.query::<(Team, Position, Health)>()
        .filter(|(team, _, _)| team.id == player_team)
        .filter(|(_, pos, _)| pos.distance_from(target) < 50.0)
        .filter(|(_, _, health)| health.value > 0)
        .execute()
}
```

### Query Execution Plans

```mermaid
graph TD
    subgraph "Query Plan Visualization"
        Plan[Query Plan]
        
        Stage1[Filter: position.x > 0]
        Stage2[Index Scan: team_index]
        Stage3[Join: Position ⨝ Team]
        Stage4[Filter: health > 50]
        Stage5[Sort: distance]
        Stage6[Limit: 100]
        
        Plan --> Stage1
        Stage1 --> Stage2
        Stage2 --> Stage3
        Stage3 --> Stage4
        Stage4 --> Stage5
        Stage5 --> Stage6
    end
    
    subgraph "Cost Estimates"
        Cost1[Est: 10,000 rows]
        Cost2[Est: 1,000 rows]
        Cost3[Est: 1,000 rows]
        Cost4[Est: 500 rows]
        Cost5[Est: 500 rows]
        Cost6[Est: 100 rows]
    end
    
    Stage1 --> Cost1
    Stage2 --> Cost2
    Stage3 --> Cost3
    Stage4 --> Cost4
    Stage5 --> Cost5
    Stage6 --> Cost6
    
    style Plan fill:#2D3436,stroke:#000,stroke-width:4px,color:#FFF
    style Stage1 fill:#FF6B6B,stroke:#C92A2A,stroke-width:3px,color:#FFF
    style Stage2 fill:#4ECDC4,stroke:#2B8A89,stroke-width:3px,color:#FFF
    style Stage3 fill:#FFE66D,stroke:#FCC419,stroke-width:3px,color:#000
    style Stage4 fill:#4ECDC4,stroke:#2B8A89,stroke-width:3px,color:#FFF
    style Stage5 fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
    style Stage6 fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
    style Cost1 fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
    style Cost2 fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
    style Cost3 fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
    style Cost4 fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
    style Cost5 fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
    style Cost6 fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
```

## Storage Performance

### Storage Backend Comparison

```mermaid
graph LR
    subgraph "Storage Backends"
        Memory[Memory]
        IPLD[IPLD]
        SQL[SQL]
        Redis[Redis]
    end
    
    subgraph "Performance Metrics"
        subgraph "Latency (μs)"
            MemLat[0.1]
            IPLDLat[100]
            SQLLat[10]
            RedisLat[5]
        end
        
        subgraph "Throughput (ops/s)"
            MemThr[10M]
            IPLDThr[10K]
            SQLThr[100K]
            RedisThr[500K]
        end
        
        subgraph "Scalability"
            MemScale[Limited]
            IPLDScale[Excellent]
            SQLScale[Good]
            RedisScale[Good]
        end
    end
    
    Memory --> MemLat
    Memory --> MemThr
    Memory --> MemScale
    
    IPLD --> IPLDLat
    IPLD --> IPLDThr
    IPLD --> IPLDScale
    
    style Memory fill:#95E1D3,stroke:#63C7B8,stroke-width:3px,color:#000
    style IPLD fill:#4ECDC4,stroke:#2B8A89,stroke-width:3px,color:#FFF
    style SQL fill:#FFE66D,stroke:#FCC419,stroke-width:3px,color:#000
    style Redis fill:#FF6B6B,stroke:#C92A2A,stroke-width:3px,color:#FFF
    style MemLat fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
    style IPLDLat fill:#4ECDC4,stroke:#2B8A89,stroke-width:2px,color:#FFF
    style SQLLat fill:#FFE66D,stroke:#FCC419,stroke-width:2px,color:#000
    style RedisLat fill:#FF6B6B,stroke:#C92A2A,stroke-width:2px,color:#FFF
    style MemThr fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
    style IPLDThr fill:#4ECDC4,stroke:#2B8A89,stroke-width:2px,color:#FFF
    style SQLThr fill:#FFE66D,stroke:#FCC419,stroke-width:2px,color:#000
    style RedisThr fill:#FF6B6B,stroke:#C92A2A,stroke-width:2px,color:#FFF
    style MemScale fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
    style IPLDScale fill:#4ECDC4,stroke:#2B8A89,stroke-width:2px,color:#FFF
    style SQLScale fill:#FFE66D,stroke:#FCC419,stroke-width:2px,color:#000
    style RedisScale fill:#FF6B6B,stroke:#C92A2A,stroke-width:2px,color:#FFF
```

### Storage Optimization Techniques

```rust
// Batch operations for better performance
pub async fn batch_store_components(
    storage: &dyn ComponentStorage,
    entities: Vec<(Uuid, Position, Velocity)>,
) -> Result<()> {
    // Group by component type
    let mut positions = Vec::with_capacity(entities.len());
    let mut velocities = Vec::with_capacity(entities.len());
    
    for (id, pos, vel) in entities {
        positions.push((id, pos));
        velocities.push((id, vel));
    }
    
    // Batch store each component type
    storage.batch_store(positions).await?;
    storage.batch_store(velocities).await?;
    
    Ok(())
}

// Compression for large components
#[derive(Component)]
pub struct CompressedTerrain {
    compressed_data: Vec<u8>,
    compression_type: CompressionType,
}

impl CompressedTerrain {
    pub fn new(terrain: &Terrain) -> Result<Self> {
        let data = bincode::serialize(terrain)?;
        let compressed = zstd::encode_all(&data[..], 3)?;
        
        Ok(Self {
            compressed_data: compressed,
            compression_type: CompressionType::Zstd,
        })
    }
    
    pub fn decompress(&self) -> Result<Terrain> {
        let decompressed = zstd::decode_all(&self.compressed_data[..])?;
        bincode::deserialize(&decompressed)
    }
}
```

## Concurrent Access

### Concurrency Model

```mermaid
graph TB
    subgraph "Concurrency Patterns"
        ReadWrite[Read-Write Lock]
        LockFree[Lock-Free]
        Sharded[Sharded Locks]
        Transactional[Transactional]
    end
    
    subgraph "Access Patterns"
        ManyReaders[Many Readers]
        FewWriters[Few Writers]
        Contentious[High Contention]
        Partitionable[Partitionable]
    end
    
    subgraph "Best Match"
        ManyReaders --> ReadWrite
        FewWriters --> ReadWrite
        Contentious --> LockFree
        Partitionable --> Sharded
    end
    
    style ReadWrite fill:#95E1D3,stroke:#63C7B8,stroke-width:3px,color:#000
    style LockFree fill:#4ECDC4,stroke:#2B8A89,stroke-width:3px,color:#FFF
    style Sharded fill:#FFE66D,stroke:#FCC419,stroke-width:3px,color:#000
    style Transactional fill:#FF6B6B,stroke:#C92A2A,stroke-width:3px,color:#FFF
    style ManyReaders fill:#2D3436,stroke:#000,stroke-width:2px,color:#FFF
    style FewWriters fill:#2D3436,stroke:#000,stroke-width:2px,color:#FFF
    style Contentious fill:#2D3436,stroke:#000,stroke-width:2px,color:#FFF
    style Partitionable fill:#2D3436,stroke:#000,stroke-width:2px,color:#FFF
```

### Concurrent Component Access

```rust
use dashmap::DashMap;
use parking_lot::RwLock;

// Lock-free concurrent storage using DashMap
pub struct ConcurrentStorage {
    components: Arc<DashMap<TypeId, Arc<DashMap<Uuid, Box<dyn Any + Send + Sync>>>>>,
}

impl ConcurrentStorage {
    pub fn store<C: Component>(&self, id: Uuid, component: C) -> Result<()> {
        let type_id = TypeId::of::<C>();
        let type_map = self.components
            .entry(type_id)
            .or_insert_with(|| Arc::new(DashMap::new()));
        
        type_map.insert(id, Box::new(component));
        Ok(())
    }
    
    pub fn parallel_query<C: Component>(&self) -> impl ParallelIterator<Item = (Uuid, C)> {
        let type_id = TypeId::of::<C>();
        
        self.components
            .get(&type_id)
            .into_par_iter()
            .flat_map(|type_map| {
                type_map.par_iter()
                    .filter_map(|entry| {
                        let (id, component) = entry.pair();
                        component.downcast_ref::<C>()
                            .map(|c| (*id, c.clone()))
                    })
            })
    }
}

// Sharded storage for reduced contention
pub struct ShardedStorage<const SHARDS: usize = 16> {
    shards: [RwLock<HashMap<Uuid, Box<dyn Any + Send + Sync>>>; SHARDS],
}

impl<const SHARDS: usize> ShardedStorage<SHARDS> {
    fn shard_for(&self, id: Uuid) -> usize {
        let hash = id.as_u128() as usize;
        hash % SHARDS
    }
    
    pub fn store<C: Component>(&self, id: Uuid, component: C) -> Result<()> {
        let shard = self.shard_for(id);
        let mut storage = self.shards[shard].write();
        storage.insert(id, Box::new(component));
        Ok(())
    }
}
```

## Performance Monitoring

### Metrics Collection

```mermaid
graph LR
    subgraph "Metrics Types"
        Counter[Counters]
        Gauge[Gauges]
        Histogram[Histograms]
        Timer[Timers]
    end
    
    subgraph "Component Metrics"
        StoreOps[Store Operations]
        QueryTime[Query Time]
        CacheHits[Cache Hits]
        MemUsage[Memory Usage]
    end
    
    subgraph "Dashboards"
        Grafana[Grafana]
        Prometheus[Prometheus]
        Custom[Custom Dashboard]
    end
    
    StoreOps --> Counter
    QueryTime --> Timer
    CacheHits --> Counter
    MemUsage --> Gauge
    
    Counter --> Prometheus
    Gauge --> Prometheus
    Histogram --> Prometheus
    Timer --> Prometheus
    
    Prometheus --> Grafana
    Prometheus --> Custom
    
    style Counter fill:#FF6B6B,stroke:#C92A2A,stroke-width:3px,color:#FFF
    style Gauge fill:#4ECDC4,stroke:#2B8A89,stroke-width:3px,color:#FFF
    style Histogram fill:#FFE66D,stroke:#FCC419,stroke-width:3px,color:#000
    style Timer fill:#95E1D3,stroke:#63C7B8,stroke-width:3px,color:#000
    style StoreOps fill:#2D3436,stroke:#000,stroke-width:2px,color:#FFF
    style QueryTime fill:#2D3436,stroke:#000,stroke-width:2px,color:#FFF
    style CacheHits fill:#2D3436,stroke:#000,stroke-width:2px,color:#FFF
    style MemUsage fill:#2D3436,stroke:#000,stroke-width:2px,color:#FFF
    style Prometheus fill:#4ECDC4,stroke:#2B8A89,stroke-width:3px,color:#FFF
    style Grafana fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
    style Custom fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
```

### Performance Monitoring Implementation

```rust
use metrics::{counter, histogram, gauge};
use std::time::Instant;

pub struct MetricsStorage<S: ComponentStorage> {
    inner: S,
}

impl<S: ComponentStorage> ComponentStorage for MetricsStorage<S> {
    fn store<C: Component>(&self, id: Uuid, component: C) -> Result<()> {
        let start = Instant::now();
        let result = self.inner.store(id, component);
        
        histogram!("component.store.duration", start.elapsed());
        counter!("component.store.total", 1);
        
        if result.is_err() {
            counter!("component.store.errors", 1);
        }
        
        result
    }
    
    fn query<Q: Query>(&self) -> Result<QueryIter<Q>> {
        let start = Instant::now();
        let result = self.inner.query::<Q>();
        
        histogram!("component.query.duration", start.elapsed());
        counter!("component.query.total", 1);
        
        gauge!("component.active_queries", 1.0);
        
        result
    }
}

// Performance profiling
pub fn profile_query<T, F: FnOnce() -> T>(name: &str, f: F) -> T {
    let start = Instant::now();
    let result = f();
    let duration = start.elapsed();
    
    println!("Query '{}' took: {:?}", name, duration);
    
    if duration > Duration::from_millis(100) {
        warn!("Slow query detected: {} ({:?})", name, duration);
    }
    
    result
}
```

## Performance Best Practices

### Component Design Guidelines

```mermaid
graph TD
    subgraph "Component Design"
        Small[Keep Components Small]
        Value[Use Value Types]
        Copy[Prefer Copy Types]
        Align[Consider Alignment]
    end
    
    subgraph "Benefits"
        Cache[Better Cache Usage]
        SIMD[SIMD Friendly]
        Memory[Less Memory]
        Speed[Faster Access]
    end
    
    Small --> Cache
    Value --> Memory
    Copy --> Speed
    Align --> SIMD
    
    style Small fill:#FF6B6B,stroke:#C92A2A,stroke-width:3px,color:#FFF
    style Value fill:#4ECDC4,stroke:#2B8A89,stroke-width:3px,color:#FFF
    style Copy fill:#FFE66D,stroke:#FCC419,stroke-width:3px,color:#000
    style Align fill:#95E1D3,stroke:#63C7B8,stroke-width:3px,color:#000
    style Cache fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
    style SIMD fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
    style Memory fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
    style Speed fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
```

### Optimization Checklist

1. **Profile Before Optimizing**
   ```rust
   #[cfg(feature = "profiling")]
   puffin::profile_scope!("complex_query");
   ```

2. **Use Appropriate Data Structures**
   - Dense storage for common components
   - Sparse storage for rare components
   - Indexed storage for frequently queried data

3. **Batch Operations**
   ```rust
   // Good: Batch operations
   storage.batch_store(components)?;
   
   // Bad: Individual operations
   for component in components {
       storage.store(id, component)?;
   }
   ```

4. **Minimize Allocations**
   ```rust
   // Reuse allocations
   let mut results = Vec::with_capacity(expected_size);
   query.collect_into(&mut results)?;
   ```

5. **Use Parallel Processing**
   ```rust
   use rayon::prelude::*;
   
   components.par_iter()
       .filter(|c| c.is_active())
       .for_each(|c| process(c));
   ```

## Performance Troubleshooting

### Common Performance Issues

```mermaid
flowchart TD
    Issue[Performance Issue]
    
    Issue --> Identify{Identify Type}
    
    Identify -->|High Latency| Latency[Latency Issues]
    Identify -->|Low Throughput| Throughput[Throughput Issues]
    Identify -->|High Memory| Memory[Memory Issues]
    
    Latency --> CacheMiss[Cache Misses]
    Latency --> LockContention[Lock Contention]
    
    Throughput --> Serialization[Serialization Overhead]
    Throughput --> IO[I/O Bottleneck]
    
    Memory --> Leaks[Memory Leaks]
    Memory --> Fragmentation[Fragmentation]
    
    style Issue fill:#FF6B6B,stroke:#C92A2A,stroke-width:4px,color:#FFF
    style Identify fill:#2D3436,stroke:#000,stroke-width:3px,color:#FFF
    style Latency fill:#4ECDC4,stroke:#2B8A89,stroke-width:3px,color:#FFF
    style Throughput fill:#4ECDC4,stroke:#2B8A89,stroke-width:3px,color:#FFF
    style Memory fill:#4ECDC4,stroke:#2B8A89,stroke-width:3px,color:#FFF
    style CacheMiss fill:#FFE66D,stroke:#FCC419,stroke-width:2px,color:#000
    style LockContention fill:#FFE66D,stroke:#FCC419,stroke-width:2px,color:#000
    style Serialization fill:#FFE66D,stroke:#FCC419,stroke-width:2px,color:#000
    style IO fill:#FFE66D,stroke:#FCC419,stroke-width:2px,color:#000
    style Leaks fill:#FFE66D,stroke:#FCC419,stroke-width:2px,color:#000
    style Fragmentation fill:#FFE66D,stroke:#FCC419,stroke-width:2px,color:#000
```

### Debugging Performance Issues

```rust
// Performance debugging utilities
pub mod debug {
    use std::sync::atomic::{AtomicU64, Ordering};
    
    pub struct PerformanceStats {
        queries: AtomicU64,
        cache_hits: AtomicU64,
        cache_misses: AtomicU64,
        slow_queries: AtomicU64,
    }
    
    impl PerformanceStats {
        pub fn report(&self) {
            let total_queries = self.queries.load(Ordering::Relaxed);
            let cache_hits = self.cache_hits.load(Ordering::Relaxed);
            let cache_misses = self.cache_misses.load(Ordering::Relaxed);
            let hit_rate = cache_hits as f64 / (cache_hits + cache_misses) as f64;
            
            println!("Performance Report:");
            println!("  Total Queries: {}", total_queries);
            println!("  Cache Hit Rate: {:.2}%", hit_rate * 100.0);
            println!("  Slow Queries: {}", self.slow_queries.load(Ordering::Relaxed));
        }
    }
}
```

## Performance Benchmarks

### Benchmark Suite

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_component_store(c: &mut Criterion) {
    let storage = InMemoryStorage::new();
    
    c.bench_function("store_single_component", |b| {
        b.iter(|| {
            let id = Uuid::new_v4();
            storage.store(id, Position { 
                x: black_box(1.0), 
                y: black_box(2.0), 
                z: black_box(3.0) 
            })
        })
    });
    
    c.bench_function("query_1000_components", |b| {
        // Setup
        for i in 0..1000 {
            let id = Uuid::new_v4();
            storage.store(id, Position { x: i as f32, y: 0.0, z: 0.0 }).unwrap();
        }
        
        b.iter(|| {
            let count = storage.query::<Position>()
                .filter(|p| p.x > 500.0)
                .count();
            black_box(count);
        })
    });
}

criterion_group!(benches, bench_component_store);
criterion_main!(benches);
```

## Future Performance Improvements

1. **GPU Acceleration**
   - Component processing on GPU
   - Parallel query execution
   - Physics simulation offloading

2. **Advanced Caching**
   - Predictive prefetching
   - Query result caching
   - Adaptive cache sizing

3. **Zero-Copy Operations**
   - Memory-mapped storage
   - Direct buffer access
   - Shared memory components

4. **SIMD Optimization**
   - Vectorized component operations
   - Batch processing
   - Custom allocators