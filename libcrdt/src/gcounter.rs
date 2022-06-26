use std::collections::HashMap;
use std::cmp::max;

pub struct GCounter {
    count: HashMap<String, u64>
}

impl GCounter {
    pub fn new(node: &str) -> GCounter {
        let mut count = HashMap::new();
        count.insert(node.to_string(), 0);
        GCounter { count }
    }

    pub fn incr(&mut self, node: &str) {
        if self.count.contains_key(node) {
            *(self.count.get_mut(node).unwrap()) += 1;
        } else {
            self.count.insert(node.to_string(), 1);
        }
    }

    pub fn merge(counters: &[&GCounter]) -> GCounter {
        let mut merged = GCounter{ count: HashMap::new() };
        for counter in counters {
            for (k, v) in &counter.count {
                if merged.count.contains_key(k) {
                    let merged_v = merged.count.get_mut(k).unwrap();
                    *merged_v = max(*merged_v, *v);
                } else {
                    merged.count.insert(k.to_string(), *v);
                }
            }
        }
        merged
    }

    pub fn total(&self) -> u64 {
        self.count.values().sum()
    }

    pub fn get_count(&self, node: &str) -> u64 {
        *self.count.get(node).unwrap_or(&0)
    }

    pub fn set_count(&mut self, node: &str, count: u64) {
        if self.count.contains_key(node) {
            *(self.count.get_mut(node).unwrap()) = count;
        } else {
            self.count.insert(node.to_string(), count);
        }
    }

}

#[cfg(test)]
mod test {
    use super::GCounter;

    #[test]
    fn test_set_count() {
        // Set a node value that is already defined
        let mut counter = GCounter::new("node");
        counter.set_count("node", 2);
        assert_eq!(counter.get_count("node"), 2_u64);

        // set a value for a node that isn't yet defined in the counter
        counter.set_count("node2", 41);
        assert_eq!(counter.get_count("node2"), 41_u64);
    }

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
            counter.incr("node");
        }
        assert_eq!(counter.get_count("node"), 10);

        // increment non-existant counter
        for _ in 0..10 {
            counter.incr("node2");
        }
        assert_eq!(counter.get_count("node2"), 10);
    }

    #[test]
    fn test_merge_non_overlap() {
        let mut counter1 = GCounter::new("node1");
        counter1.set_count("node1", 10);
        counter1.set_count("node2", 10);

        let mut counter2 = GCounter::new("node3");
        counter2.set_count("node3", 20);
        counter2.set_count("node4", 20);

        let counter3 = GCounter::merge(&[&counter1, &counter2]);
        assert_eq!(counter3.total(), 60);
        assert_eq!(counter3.get_count("node1"), 10);
        assert_eq!(counter3.get_count("node2"), 10);
        assert_eq!(counter3.get_count("node3"), 20);
        assert_eq!(counter3.get_count("node4"), 20);
    }

    #[test]
    fn test_merge_overlap() {
        let mut counter1 = GCounter::new("node1");
        counter1.set_count("node1", 10);
        counter1.set_count("node2", 20);

        let mut counter2 = GCounter::new("node1");
        counter2.set_count("node1", 20);
        counter2.set_count("node2", 10);

        let counter3 = GCounter::merge(&[&counter1, &counter2]);
        assert_eq!(counter3.total(), 40);
        assert_eq!(counter3.get_count("node1"), 20);
        assert_eq!(counter3.get_count("node2"), 20);
    }
}