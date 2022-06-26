use std::collections::HashMap;
use std::cmp::max;

pub struct GCounter {
    /// Internal counters for all nodes
    count: HashMap<String, u64>,

    /// Node the counter was initialized. This is key for the value modified
    /// when calling `incr`
    node: String,
}

impl GCounter {
    // When creating a new counter, this should be the only time we need to specify a node. I don't see a reason why we need to include 'node'
    // in any of ther other public interfaces.

    /// Create a new GCounter and specify the node value that will be used for tracking
    /// local operations. This will be implicit in calls to `incr`.
    pub fn new(node: &str) -> GCounter {
        let mut count = HashMap::new();
        count.insert(node.to_string(), 0);
        GCounter { count: count, node: node.to_string(), }
    }

    /// Increment the counter. Note that the increment operation is tied to the node used
    /// in the call to GCounter::new
    pub fn incr(&mut self) {
        // This should never be None since the node is created in the call to GCounter::new
        debug_assert!(self.count.get_mut(&self.node).is_some());
        *(self.count.get_mut(&self.node).unwrap()) += 1;
    }

    /// Calculate the total count
    pub fn total(&self) -> u64 {
        self.count.values().sum()
    }

    /// Merge a list of GCounters into the current GCounter. This updates the current
    /// GCounter in-place. This function is safe to run multiple times or with duplicate
    /// input values.
    pub fn merge(&mut self, counters: &[&GCounter]) {
        for counter in counters {
            for (k, v) in &counter.count {
                self.set_count(k, max(self.get_count(k), *v));
            }
        }
    }

    /// Fetch the count for a given node. Returns zero if no entry is found for the
    /// requested node.
    pub fn get_count(&self, node: &str) -> u64 {
        *self.count.get(node).unwrap_or(&0)
    }

    /// Get all the nodes of the GCounter. These can be used with calls to `get_count()`
    /// however, this is primarily for debugging purposes.
    pub fn get_nodes(&self) -> Vec::<&String> {
        self.count.keys().clone().collect()
    }

    fn set_count(&mut self, node: &str, count: u64) {
        if self.count.contains_key(node) {
            *(self.count.get_mut(node).unwrap()) = count;
        } else {
            self.count.insert(node.to_string(), count);
        }
    }

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_total() {
        let mut counter = GCounter::new("node");
        counter.set_count("node1", 1);
        counter.set_count("node2", 2);
        counter.set_count("node3", 3);
        counter.set_count("node4", 4);
        counter.set_count("node5", 5);

        assert_eq!(counter.total(), 1+2+3+4+5);
    }

    #[test]
    fn test_incr() {
        let mut counter = GCounter::new("node");

        // increment existing counter
        for _ in 0..10 {
            counter.incr();
        }
        assert_eq!(counter.get_count("node"), 10);
    }

    #[test]
    fn test_merge_non_overlap() {
        let mut counter1 = GCounter::new("node1");
        counter1.set_count("node1", 10);
        counter1.set_count("node2", 10);

        let mut counter2 = GCounter::new("node3");
        counter2.set_count("node3", 20);
        counter2.set_count("node4", 20);

        counter1.merge(&[&counter2]);
        assert_eq!(counter1.total(), 60);
        assert_eq!(counter1.get_count("node1"), 10);
        assert_eq!(counter1.get_count("node2"), 10);
        assert_eq!(counter1.get_count("node3"), 20);
        assert_eq!(counter1.get_count("node4"), 20);
    }

    #[test]
    fn test_merge_overlap() {
        let mut counter1 = GCounter::new("node1");
        counter1.set_count("node1", 10);
        counter1.set_count("node2", 20);

        let mut counter2 = GCounter::new("node1");
        counter2.set_count("node1", 20);
        counter2.set_count("node2", 10);

        counter1.merge(&[&counter2]);
        assert_eq!(counter1.total(), 40);
        assert_eq!(counter1.get_count("node1"), 20);
        assert_eq!(counter1.get_count("node2"), 20);
    }
}