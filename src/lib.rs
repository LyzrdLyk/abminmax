pub struct Algorithm {
    id: u64,
}

impl Algorithm {
    pub fn new() -> Algorithm {
        Algorithm { id: 38 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_alg() {
        let s = Algorithm::new();
        assert_eq!(s.id, 38);
    }
}
