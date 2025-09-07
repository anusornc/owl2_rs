//! # Caching Support for OWL 2 Reasoner
//! 
//! This module provides caching mechanisms to avoid recomputing results for 
//! the same queries in the OWL 2 reasoner.

use crate::{Ontology, reasoner::{ClassHierarchy, IndividualTypes, Individual}};
use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// Configuration for the reasoner caching mechanisms
#[derive(Debug, Clone)]
pub struct CacheConfig {
    /// Whether caching is enabled
    pub enabled: bool,
    /// Maximum number of cached results
    pub max_cache_size: usize,
}

impl Default for CacheConfig {
    fn default() -> Self {
        CacheConfig {
            enabled: true,
            max_cache_size: 1000,
        }
    }
}

/// Cache for storing previously computed results
#[derive(Debug, Clone)]
pub struct ReasonerCache {
    /// Cached consistency results
    consistency_cache: HashMap<u64, bool>,
    /// Cached classification results
    classification_cache: HashMap<u64, ClassHierarchy>,
    /// Cached realization results
    realization_cache: HashMap<u64, HashMap<Individual, IndividualTypes>>,
    /// Cache configuration
    config: CacheConfig,
}

impl ReasonerCache {
    /// Creates a new cache with the given configuration
    pub fn new(config: CacheConfig) -> Self {
        ReasonerCache {
            consistency_cache: HashMap::new(),
            classification_cache: HashMap::new(),
            realization_cache: HashMap::new(),
            config,
        }
    }
    
    /// Computes a hash for an ontology to use as a cache key
    fn compute_ontology_hash(ontology: &Ontology) -> u64 {
        let mut hasher = DefaultHasher::new();
        ontology.hash(&mut hasher);
        hasher.finish()
    }
    
    /// Gets a cached consistency result
    pub fn get_consistency(&self, ontology: &Ontology) -> Option<bool> {
        if !self.config.enabled {
            return None;
        }
        
        let hash = Self::compute_ontology_hash(ontology);
        self.consistency_cache.get(&hash).copied()
    }
    
    /// Stores a consistency result in the cache
    pub fn store_consistency(&mut self, ontology: &Ontology, result: bool) {
        if !self.config.enabled {
            return;
        }
        
        let hash = Self::compute_ontology_hash(ontology);
        self.consistency_cache.insert(hash, result);
        
        // Limit cache size
        if self.consistency_cache.len() > self.config.max_cache_size {
            // Remove oldest entries (simple FIFO approach)
            if let Some(key) = self.consistency_cache.keys().next().copied() {
                self.consistency_cache.remove(&key);
            }
        }
    }
    
    /// Gets a cached classification result
    pub fn get_classification(&self, ontology: &Ontology) -> Option<&ClassHierarchy> {
        if !self.config.enabled {
            return None;
        }
        
        let hash = Self::compute_ontology_hash(ontology);
        self.classification_cache.get(&hash)
    }
    
    /// Stores a classification result in the cache
    pub fn store_classification(&mut self, ontology: &Ontology, result: ClassHierarchy) {
        if !self.config.enabled {
            return;
        }
        
        let hash = Self::compute_ontology_hash(ontology);
        self.classification_cache.insert(hash, result);
        
        // Limit cache size
        if self.classification_cache.len() > self.config.max_cache_size {
            // Remove oldest entries (simple FIFO approach)
            if let Some(key) = self.classification_cache.keys().next().copied() {
                self.classification_cache.remove(&key);
            }
        }
    }
    
    /// Gets a cached realization result
    pub fn get_realization(&self, ontology: &Ontology) -> Option<&HashMap<Individual, IndividualTypes>> {
        if !self.config.enabled {
            return None;
        }
        
        let hash = Self::compute_ontology_hash(ontology);
        self.realization_cache.get(&hash)
    }
    
    /// Stores a realization result in the cache
    pub fn store_realization(&mut self, ontology: &Ontology, result: HashMap<Individual, IndividualTypes>) {
        if !self.config.enabled {
            return;
        }
        
        let hash = Self::compute_ontology_hash(ontology);
        self.realization_cache.insert(hash, result);
        
        // Limit cache size
        if self.realization_cache.len() > self.config.max_cache_size {
            // Remove oldest entries (simple FIFO approach)
            if let Some(key) = self.realization_cache.keys().next().copied() {
                self.realization_cache.remove(&key);
            }
        }
    }
}