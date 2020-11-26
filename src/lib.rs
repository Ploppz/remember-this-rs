//! An in-memory cache backed by an on-disk cache.
extern crate chrono;
extern crate flexbuffers;
#[macro_use]
extern crate log;
extern crate serde;
extern crate serde_derive;
extern crate tokio;

mod cache;
mod manager;
mod result;

pub use cache::Cache;
pub use manager::CacheManager;
pub use result::Error;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}