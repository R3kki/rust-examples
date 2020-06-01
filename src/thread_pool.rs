/// Naive Thread Pool Implementation
pub struct ThreadPool;

// Public Api
impl ThreadPool {
    pub fn new() -> Self {
        Self
    }

    pub fn execute(&self) {}
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let pool = ThreadPool::new();
        pool.execute();
    }
}