use crate::gcounter::GCounter;
pub struct PNCounter {
    p: GCounter,
    n: GCounter,
}

impl PNCounter {
    pub fn new(node: &str) -> PNCounter {
        PNCounter {
            p: GCounter::new(node),
            n: GCounter::new(node),
        }
    }

    pub fn incr(&mut self, node: &str) {
        self.p.incr(node);
    }

    pub fn decr(&mut self, node: &str) {
        self.n.incr(node);
    }

    pub fn get_count(&self, node: &str) -> i64 {
        PNCounter::diff(self.p.get_count(node), self.n.get_count(node))
    }

    pub fn total(&self) -> i64 {
        PNCounter::diff(self.p.total(), self.n.total())
    }

    fn diff(p_count: u64, n_count: u64) -> i64 {
        // Convert to i64 after finding difference between positive and negative
        // counts in a mostly futile attempt to reduce unintended wrapping.
        if n_count > p_count {
            -((n_count - p_count) as i64)
        } else {
            (p_count - n_count) as i64
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_incr() {

    }

    fn test_decr() {

    }

    fn test_incr_decr() {

    }
}