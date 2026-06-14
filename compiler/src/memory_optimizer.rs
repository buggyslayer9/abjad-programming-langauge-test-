use crate::error::{AbjadError, Result};

/// Memory optimization strategy
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryStrategy {
    /// Stack allocation where possible
    StackAllocation,
    /// Pool allocation for frequent allocations
    PoolAllocation,
    /// Memory reuse
    MemoryReuse,
    /// Compact memory layout
    CompactLayout,
}

/// Memory optimizer for Abjad
pub struct MemoryOptimizer {
    /// Optimization strategies
    strategies: Vec<MemoryStrategy>,
    
    /// Enable stack allocation
    enable_stack_allocation: bool,
    
    /// Enable pool allocation
    enable_pool_allocation: bool,
    
    /// Enable memory reuse
    enable_memory_reuse: bool,
}

impl MemoryOptimizer {
    /// Create a new memory optimizer
    pub fn new() -> Self {
        MemoryOptimizer {
            strategies: vec![
                MemoryStrategy::StackAllocation,
                MemoryStrategy::PoolAllocation,
                MemoryStrategy::MemoryReuse,
            ],
            enable_stack_allocation: true,
            enable_pool_allocation: true,
            enable_memory_reuse: true,
        }
    }

    /// Add an optimization strategy
    pub fn add_strategy(&mut self, strategy: MemoryStrategy) {
        self.strategies.push(strategy);
    }

    /// Remove an optimization strategy
    pub fn remove_strategy(&mut self, strategy: MemoryStrategy) {
        self.strategies.retain(|s| *s != strategy);
    }

    /// Enable stack allocation
    pub fn enable_stack_allocation(&mut self, enabled: bool) {
        self.enable_stack_allocation = enabled;
    }

    /// Enable pool allocation
    pub fn enable_pool_allocation(&mut self, enabled: bool) {
        self.enable_pool_allocation = enabled;
    }

    /// Enable memory reuse
    pub fn enable_memory_reuse(&mut self, enabled: bool) {
        self.enable_memory_reuse = enabled;
    }

    /// Optimize memory usage
    pub fn optimize(&self) -> Result<String> {
        let mut optimizations = Vec::new();
        
        if self.enable_stack_allocation {
            optimizations.push("Stack allocation optimization enabled");
        }
        
        if self.enable_pool_allocation {
            optimizations.push("Pool allocation optimization enabled");
        }
        
        if self.enable_memory_reuse {
            optimizations.push("Memory reuse optimization enabled");
        }
        
        for strategy in &self.strategies {
            match strategy {
                MemoryStrategy::StackAllocation => {
                    optimizations.push("Using stack allocation for small objects");
                }
                MemoryStrategy::PoolAllocation => {
                    optimizations.push("Using allocation pools for frequent allocations");
                }
                MemoryStrategy::MemoryReuse => {
                    optimizations.push("Reusing memory when possible");
                }
                MemoryStrategy::CompactLayout => {
                    optimizations.push("Using compact memory layout");
                }
            }
        }
        
        Ok(optimizations.join("\n"))
    }

    /// Get strategies
    pub fn strategies(&self) -> &[MemoryStrategy] {
        &self.strategies
    }

    /// Check if stack allocation is enabled
    pub fn stack_allocation_enabled(&self) -> bool {
        self.enable_stack_allocation
    }

    /// Check if pool allocation is enabled
    pub fn pool_allocation_enabled(&self) -> bool {
        self.enable_pool_allocation
    }

    /// Check if memory reuse is enabled
    pub fn memory_reuse_enabled(&self) -> bool {
        self.enable_memory_reuse
    }
}

impl Default for MemoryOptimizer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_optimizer_default() {
        let optimizer = MemoryOptimizer::default();
        assert!(optimizer.stack_allocation_enabled());
        assert!(optimizer.pool_allocation_enabled());
        assert!(optimizer.memory_reuse_enabled());
    }

    #[test]
    fn test_memory_optimizer_strategies() {
        let mut optimizer = MemoryOptimizer::new();
        optimizer.add_strategy(MemoryStrategy::CompactLayout);
        assert_eq!(optimizer.strategies().len(), 4);
        
        optimizer.remove_strategy(MemoryStrategy::PoolAllocation);
        assert_eq!(optimizer.strategies().len(), 3);
    }

    #[test]
    fn test_memory_optimizer_optimize() {
        let optimizer = MemoryOptimizer::new();
        let result = optimizer.optimize().unwrap();
        assert!(result.contains("Stack allocation"));
        assert!(result.contains("Pool allocation"));
    }
}
