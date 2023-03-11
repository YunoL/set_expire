use mini_redis::blocking_client;
use std::thread;
use std::time::Duration;

fn main() {
    let ttl = Duration::from_millis(500);
    let mut client = blocking_client::connect("localhost:6379").unwrap();

    client.set_expires("foo", "bar".into(), ttl).unwrap();

    // Getting the value immediately works
    let val = client.get("foo").unwrap().unwrap();
    assert_eq!(val, "bar");

    // Wait for the TTL to expire
    thread::sleep(ttl);

    let val = client.get("foo").unwrap();
    assert!(val.is_some());
}
