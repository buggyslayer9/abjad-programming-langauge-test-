//! Collections module for the Abjad standard library

use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::Hash;

/// Represents a dynamic array
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Array<T> {
    data: Vec<T>,
}

impl<T> Array<T> {
    /// Creates a new empty array
    pub fn new() -> Self {
        Array { data: Vec::new() }
    }

    /// Creates an array with a capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Array {
            data: Vec::with_capacity(capacity),
        }
    }

    /// Adds an element to the end of the array
    pub fn push(&mut self, value: T) {
        self.data.push(value);
    }

    /// Removes and returns the last element
    pub fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }

    /// Adds an element to the beginning of the array
    pub fn push_front(&mut self, value: T) {
        self.data.insert(0, value);
    }

    /// Removes and returns the first element
    pub fn pop_front(&mut self) -> Option<T> {
        if self.data.is_empty() {
            None
        } else {
            Some(self.data.remove(0))
        }
    }

    /// Returns the element at the given index
    pub fn get(&self, index: usize) -> Option<&T> {
        self.data.get(index)
    }

    /// Returns a mutable reference to the element at the given index
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.data.get_mut(index)
    }

    /// Sets the element at the given index
    pub fn set(&mut self, index: usize, value: T) -> Option<T> {
        if index < self.data.len() {
            Some(std::mem::replace(&mut self.data[index], value))
        } else {
            None
        }
    }

    /// Returns the length of the array
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns true if the array is empty
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Clears the array
    pub fn clear(&mut self) {
        self.data.clear();
    }

    /// Sorts the array
    pub fn sort(&mut self)
    where
        T: Ord,
    {
        self.data.sort();
    }

    /// Reverses the array
    pub fn reverse(&mut self) {
        self.data.reverse();
    }

    /// Returns an iterator over the array
    pub fn iter(&self) -> std::slice::Iter<T> {
        self.data.iter()
    }

    /// Returns a mutable iterator over the array
    pub fn iter_mut(&mut self) -> std::slice::IterMut<T> {
        self.data.iter_mut()
    }

    /// Converts the array to a Vec
    pub fn to_vec(self) -> Vec<T> {
        self.data
    }
}

impl<T> Default for Array<T> {
    fn default() -> Self {
        Array::new()
    }
}

impl<T> From<Vec<T>> for Array<T> {
    fn from(data: Vec<T>) -> Self {
        Array { data }
    }
}

/// Represents a doubly-linked list
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct List<T> {
    data: VecDeque<T>,
}

impl<T> List<T> {
    /// Creates a new empty list
    pub fn new() -> Self {
        List {
            data: VecDeque::new(),
        }
    }

    /// Adds an element to the front of the list
    pub fn push_front(&mut self, value: T) {
        self.data.push_front(value);
    }

    /// Adds an element to the back of the list
    pub fn push_back(&mut self, value: T) {
        self.data.push_back(value);
    }

    /// Removes and returns the front element
    pub fn pop_front(&mut self) -> Option<T> {
        self.data.pop_front()
    }

    /// Removes and returns the back element
    pub fn pop_back(&mut self) -> Option<T> {
        self.data.pop_back()
    }

    /// Returns the front element
    pub fn front(&self) -> Option<&T> {
        self.data.front()
    }

    /// Returns the back element
    pub fn back(&self) -> Option<&T> {
        self.data.back()
    }

    /// Returns the length of the list
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns true if the list is empty
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Clears the list
    pub fn clear(&mut self) {
        self.data.clear();
    }
}

impl<T> Default for List<T> {
    fn default() -> Self {
        List::new()
    }
}

/// Represents a key-value map
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Map<K, V> {
    data: HashMap<K, V>,
}

impl<K, V> Map<K, V>
where
    K: Eq + Hash,
{
    /// Creates a new empty map
    pub fn new() -> Self {
        Map {
            data: HashMap::new(),
        }
    }

    /// Inserts a key-value pair
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.data.insert(key, value)
    }

    /// Gets a value by key
    pub fn get(&self, key: &K) -> Option<&V> {
        self.data.get(key)
    }

    /// Gets a mutable value by key
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        self.data.get_mut(key)
    }

    /// Removes a key-value pair
    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.data.remove(key)
    }

    /// Returns true if the map contains the key
    pub fn contains_key(&self, key: &K) -> bool {
        self.data.contains_key(key)
    }

    /// Returns the number of key-value pairs
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns true if the map is empty
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Clears the map
    pub fn clear(&mut self) {
        self.data.clear();
    }

    /// Returns an iterator over the keys
    pub fn keys(&self) -> impl Iterator<Item = &K> {
        self.data.keys()
    }

    /// Returns an iterator over the values
    pub fn values(&self) -> impl Iterator<Item = &V> {
        self.data.values()
    }

    /// Returns an iterator over key-value pairs
    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        self.data.iter()
    }
}

impl<K, V> Default for Map<K, V>
where
    K: Eq + Hash,
{
    fn default() -> Self {
        Map::new()
    }
}

/// Represents a set of unique values
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Set<T> {
    data: HashSet<T>,
}

impl<T> Set<T>
where
    T: Eq + Hash,
{
    /// Creates a new empty set
    pub fn new() -> Self {
        Set {
            data: HashSet::new(),
        }
    }

    /// Adds a value to the set
    pub fn insert(&mut self, value: T) -> bool {
        self.data.insert(value)
    }

    /// Removes a value from the set
    pub fn remove(&mut self, value: &T) -> bool {
        self.data.remove(value)
    }

    /// Returns true if the set contains the value
    pub fn contains(&self, value: &T) -> bool {
        self.data.contains(value)
    }

    /// Returns the number of elements in the set
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns true if the set is empty
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Clears the set
    pub fn clear(&mut self) {
        self.data.clear();
    }

    /// Returns an iterator over the set
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter()
    }

    /// Returns the intersection of two sets
    pub fn intersection(&self, other: &Set<T>) -> Set<T> {
        Set {
            data: self.data.intersection(&other.data).cloned().collect(),
        }
    }

    /// Returns the union of two sets
    pub fn union(&self, other: &Set<T>) -> Set<T> {
        Set {
            data: self.data.union(&other.data).cloned().collect(),
        }
    }

    /// Returns the difference of two sets
    pub fn difference(&self, other: &Set<T>) -> Set<T> {
        Set {
            data: self.data.difference(&other.data).cloned().collect(),
        }
    }
}

impl<T> Default for Set<T>
where
    T: Eq + Hash,
{
    fn default() -> Self {
        Set::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array() {
        let mut arr = Array::new();
        arr.push(1);
        arr.push(2);
        arr.push(3);
        
        assert_eq!(arr.len(), 3);
        assert_eq!(arr.get(0), Some(&1));
        assert_eq!(arr.pop(), Some(3));
        assert_eq!(arr.len(), 2);
    }

    #[test]
    fn test_array_sort() {
        let mut arr = Array::new();
        arr.push(3);
        arr.push(1);
        arr.push(2);
        
        arr.sort();
        assert_eq!(arr.get(0), Some(&1));
        assert_eq!(arr.get(1), Some(&2));
        assert_eq!(arr.get(2), Some(&3));
    }

    #[test]
    fn test_list() {
        let mut list = List::new();
        list.push_back(1);
        list.push_back(2);
        list.push_front(0);
        
        assert_eq!(list.front(), Some(&0));
        assert_eq!(list.back(), Some(&2));
        assert_eq!(list.pop_front(), Some(0));
        assert_eq!(list.len(), 2);
    }

    #[test]
    fn test_map() {
        let mut map = Map::new();
        map.insert("one", 1);
        map.insert("two", 2);
        
        assert_eq!(map.get("one"), Some(&1));
        assert!(map.contains_key("two"));
        assert_eq!(map.len(), 2);
    }

    #[test]
    fn test_set() {
        let mut set = Set::new();
        set.insert(1);
        set.insert(2);
        set.insert(3);
        
        assert!(set.contains(&2));
        assert_eq!(set.len(), 3);
        assert!(!set.insert(1)); // Already exists
    }

    #[test]
    fn test_set_operations() {
        let mut set1 = Set::new();
        set1.insert(1);
        set1.insert(2);
        
        let mut set2 = Set::new();
        set2.insert(2);
        set2.insert(3);
        
        let intersection = set1.intersection(&set2);
        assert!(intersection.contains(&2));
        assert!(!intersection.contains(&1));
        
        let union = set1.union(&set2);
        assert!(union.contains(&1));
        assert!(union.contains(&2));
        assert!(union.contains(&3));
    }
}
