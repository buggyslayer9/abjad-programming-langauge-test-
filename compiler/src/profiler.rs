use crate::error::{AbjadError, Result};
use std::collections::HashMap;
use std::path::Path;
use std::time::{Duration, Instant};

/// Function profiling data
#[derive(Debug, Clone)]
pub struct FunctionProfile {
    pub name: String,
    pub calls: usize,
    pub total_time: Duration,
    pub self_time: Duration,
    pub avg_time: Duration,
}

/// Memory allocation data
#[derive(Debug, Clone)]
pub struct MemoryProfile {
    pub total_allocated: usize,
    pub peak_usage: usize,
    pub allocations: usize,
    pub deallocations: usize,
}

/// Profiler for Abjad
pub struct Profiler {
    /// Function profiles
    function_profiles: HashMap<String, FunctionProfile>,
    
    /// Memory profile
    memory_profile: MemoryProfile,
    
    /// Call graph
    call_graph: HashMap<String, Vec<String>>,
    
    /// Hotspots (most time-consuming functions)
    hotspots: Vec<(String, Duration)>,
    
    /// Profiling state
    profiling: bool,
}

impl Profiler {
    /// Create a new profiler
    pub fn new() -> Self {
        Profiler {
            function_profiles: HashMap::new(),
            memory_profile: MemoryProfile {
                total_allocated: 0,
                peak_usage: 0,
                allocations: 0,
                deallocations: 0,
            },
            call_graph: HashMap::new(),
            hotspots: Vec::new(),
            profiling: false,
        }
    }

    /// Start profiling
    pub fn start(&mut self) -> Result<()> {
        println!("Starting profiler");
        self.profiling = true;
        Ok(())
    }

    /// Stop profiling
    pub fn stop(&mut self) -> Result<()> {
        println!("Stopping profiler");
        self.profiling = false;
        self.analyze_hotspots();
        Ok(())
    }

    /// Record a function call
    pub fn record_function_call(&mut self, name: &str, duration: Duration) {
        let profile = self.function_profiles.entry(name.to_string()).or_insert_with(|| FunctionProfile {
            name: name.to_string(),
            calls: 0,
            total_time: Duration::ZERO,
            self_time: Duration::ZERO,
            avg_time: Duration::ZERO,
        });
        
        profile.calls += 1;
        profile.total_time += duration;
        profile.avg_time = profile.total_time / profile.calls as u32;
    }

    /// Record a memory allocation
    pub fn record_allocation(&mut self, size: usize) {
        self.memory_profile.total_allocated += size;
        self.memory_profile.allocations += 1;
        
        if self.memory_profile.total_allocated > self.memory_profile.peak_usage {
            self.memory_profile.peak_usage = self.memory_profile.total_allocated;
        }
    }

    /// Record a memory deallocation
    pub fn record_deallocation(&mut self, size: usize) {
        self.memory_profile.total_allocated -= size;
        self.memory_profile.deallocations += 1;
    }

    /// Add a call graph edge
    pub fn add_call_edge(&mut self, caller: &str, callee: &str) {
        self.call_graph
            .entry(caller.to_string())
            .or_insert_with(Vec::new)
            .push(callee.to_string());
    }

    /// Analyze hotspots
    fn analyze_hotspots(&mut self) {
        self.hotspots = self.function_profiles
            .iter()
            .map(|(name, profile)| (name.clone(), profile.total_time))
            .collect();
        
        self.hotspots.sort_by(|a, b| b.1.cmp(&a.1));
    }

    /// Generate function report
    pub fn generate_function_report(&self) -> String {
        let mut report = String::new();
        report.push_str("Function Profile Report\n");
        report.push_str("======================\n\n");
        
        for (name, profile) in &self.function_profiles {
            report.push_str(&format!(
                "Function: {}\n  Calls: {}\n  Total Time: {:?}\n  Avg Time: {:?}\n\n",
                name, profile.calls, profile.total_time, profile.avg_time
            ));
        }
        
        report
    }

    /// Generate memory report
    pub fn generate_memory_report(&self) -> String {
        format!(
            "Memory Profile Report\n\
             ====================\n\n\
             Total Allocated: {} bytes\n\
             Peak Usage: {} bytes\n\
             Allocations: {}\n\
             Deallocations: {}\n",
            self.memory_profile.total_allocated,
            self.memory_profile.peak_usage,
            self.memory_profile.allocations,
            self.memory_profile.deallocations
        )
    }

    /// Generate call graph report
    pub fn generate_call_graph_report(&self) -> String {
        let mut report = String::new();
        report.push_str("Call Graph Report\n");
        report.push_str("=================\n\n");
        
        for (caller, callees) in &self.call_graph {
            report.push_str(&format!("{} calls:\n", caller));
            for callee in callees {
                report.push_str(&format!("  -> {}\n", callee));
            }
            report.push('\n');
        }
        
        report
    }

    /// Generate hotspot report
    pub fn generate_hotspot_report(&self) -> String {
        let mut report = String::new();
        report.push_str("Hotspot Report\n");
        report.push_str("==============\n\n");
        
        for (i, (name, duration)) in self.hotspots.iter().take(10).enumerate() {
            report.push_str(&format!("{}. {} - {:?}\n", i + 1, name, duration));
        }
        
        report
    }

    /// Generate combined report
    pub fn generate_report(&self) -> String {
        let mut report = String::new();
        report.push_str(&self.generate_function_report());
        report.push_str(&self.generate_memory_report());
        report.push_str(&self.generate_call_graph_report());
        report.push_str(&self.generate_hotspot_report());
        report
    }

    /// Save report to file
    pub fn save_report(&self, path: &Path) -> Result<()> {
        let report = self.generate_report();
        std::fs::write(path, report)
            .map_err(|e| AbjadError::internal(format!("Failed to write report: {}", e)))?;
        Ok(())
    }

    /// Get function profile
    pub fn get_function_profile(&self, name: &str) -> Option<&FunctionProfile> {
        self.function_profiles.get(name)
    }

    /// Get memory profile
    pub fn get_memory_profile(&self) -> &MemoryProfile {
        &self.memory_profile
    }

    /// Get hotspots
    pub fn get_hotspots(&self) -> &[(String, Duration)] {
        &self.hotspots
    }

    /// Check if profiling is active
    pub fn is_profiling(&self) -> bool {
        self.profiling
    }
}

impl Default for Profiler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_profiler_default() {
        let profiler = Profiler::default();
        assert!(!profiler.is_profiling());
    }

    #[test]
    fn test_profiler_start_stop() {
        let mut profiler = Profiler::new();
        profiler.start().unwrap();
        assert!(profiler.is_profiling());
        profiler.stop().unwrap();
        assert!(!profiler.is_profiling());
    }

    #[test]
    fn test_function_call_recording() {
        let mut profiler = Profiler::new();
        profiler.record_function_call("test_func", Duration::from_millis(100));
        profiler.record_function_call("test_func", Duration::from_millis(200));
        
        let profile = profiler.get_function_profile("test_func").unwrap();
        assert_eq!(profile.calls, 2);
        assert_eq!(profile.total_time, Duration::from_millis(300));
    }

    #[test]
    fn test_memory_recording() {
        let mut profiler = Profiler::new();
        profiler.record_allocation(1024);
        profiler.record_allocation(2048);
        profiler.record_deallocation(1024);
        
        let mem_profile = profiler.get_memory_profile();
        assert_eq!(mem_profile.total_allocated, 2048);
        assert_eq!(mem_profile.peak_usage, 3072);
        assert_eq!(mem_profile.allocations, 2);
        assert_eq!(mem_profile.deallocations, 1);
    }

    #[test]
    fn test_call_graph() {
        let mut profiler = Profiler::new();
        profiler.add_call_edge("main", "func1");
        profiler.add_call_edge("main", "func2");
        profiler.add_call_edge("func1", "helper");
        
        let report = profiler.generate_call_graph_report();
        assert!(report.contains("main calls:"));
        assert!(report.contains("func1"));
        assert!(report.contains("func2"));
    }
}
