use std::borrow::Cow;
use std::collections::HashMap;
use std::hash::Hash;

trait Storage<K, V> {
    fn set(&mut self, key: K, val: V);
    fn get(&self, key: &K) -> Option<&V>;
    fn remove(&mut self, key: &K) -> Option<V>;
}

struct User {
    id: u64,
    email: Cow<'static, str>,
    activated: bool,
}

struct UserRepository<K, V>
where
    K: Eq + Hash,
{
    users: HashMap<K, V>,
}

impl<K, V> Storage<K, V> for UserRepository<K, V>
where
    K: Eq + Hash,
{
    fn set(&mut self, key: K, val: V) {
        self.users.insert(key, val);
    }
    fn get(&self, key: &K) -> Option<&V> {
        self.users.get(&key)
    }
    fn remove(&mut self, key: &K) -> Option<V> {
        self.users.remove(&key)
    }
}

impl<K, V> UserRepository<K, V>
where
    K: Eq + Hash,
{
    pub fn new() -> Self {
        let map: HashMap<K, V> = HashMap::new();
        UserRepository { users: map }
    }
    pub fn update(&mut self, key: K, val: V) {
        match self.users.remove(&key) {
            Some(_) => {
                self.users.insert(key, val).unwrap();
            }
            None => {}
        }
    }
}

enum Storages<K, V>
where
    K: Eq + Hash,
{
    UserRepository(UserRepository<K, V>),
}

impl<K, V> Storage<K, V> for Storages<K, V>
where
    K: Eq + Hash,
{
    fn set(&mut self, key: K, val: V) {
        match self {
            Storages::UserRepository(s) => s.set(key, val),
        }
    }
    fn get(&self, key: &K) -> Option<&V> {
        match self {
            Storages::UserRepository(s) => s.get(key),
        }
    }
    fn remove(&mut self, key: &K) -> Option<V> {
        match self {
            Storages::UserRepository(s) => s.remove(key),
        }
    }
}

fn main() {
    let mut dyn_storages: Vec<Box<dyn Storage<u64, User>>> = vec![Box::new(UserRepository::new())];
    let mut static_storages: Vec<Storages<u64, User>> =
        vec![Storages::UserRepository(UserRepository::new())];

    for storage in &mut dyn_storages {
        storage.set(
            0,
            User {
                id: 0,
                email: Cow::Borrowed("uchiha-sasuke@gmail.com"),
                activated: true,
            },
        );
        storage.set(
            1,
            User {
                id: 1,
                email: Cow::Borrowed("uchiha-itachi@gmail.com"),
                activated: true,
            },
        );
        storage.set(
            2,
            User {
                id: 2,
                email: Cow::Borrowed("uchiha-sarada@gmail.com"),
                activated: true,
            },
        );
    }

    for storage in &mut static_storages {
        storage.set(
            0,
            User {
                id: 0,
                email: Cow::Borrowed("uchiha-obito@gmail.com"),
                activated: true,
            },
        );
        storage.set(
            1,
            User {
                id: 1,
                email: Cow::Borrowed("uchiha-madara@gmail.com"),
                activated: true,
            },
        );
        storage.set(
            2,
            User {
                id: 2,
                email: Cow::Borrowed("uchiha-fugaku@gmail.com"),
                activated: true,
            },
        );
    }

    for storage in dyn_storages {
        let user1 = storage.get(&0).unwrap();
        let user2 = storage.get(&1).unwrap();
        let user3 = storage.get(&2).unwrap();

        println!("User ({}, {}, {})", user1.id, user1.email, user1.activated);
        println!("User ({}, {}, {})", user2.id, user2.email, user2.activated);
        println!("User ({}, {}, {})", user3.id, user3.email, user3.activated);
    }

    println!("");

    for storage in static_storages {
        let user1 = storage.get(&0).unwrap();
        let user2 = storage.get(&1).unwrap();
        let user3 = storage.get(&2).unwrap();

        println!("User ({}, {}, {})", user1.id, user1.email, user1.activated);
        println!("User ({}, {}, {})", user2.id, user2.email, user2.activated);
        println!("User ({}, {}, {})", user3.id, user3.email, user3.activated);
    }
}
