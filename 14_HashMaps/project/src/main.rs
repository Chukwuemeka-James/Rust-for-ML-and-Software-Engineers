use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct SocialNetwork {
    // Maps user name -> set of friends
    users: HashMap<String, HashSet<String>>,
}

impl SocialNetwork {
    // Create a new empty social network
    fn new() -> Self {
        Self {
            users: HashMap::new(),
        }
    }

    // Add a user if not exists
    fn add_user(&mut self, user: &str) {
        self.users.entry(user.to_string()).or_insert_with(HashSet::new);
    }

    // Add friendship between two users
    fn add_friend(&mut self, user1: &str, user2: &str) {
        if user1 == user2 {
            println!("A user cannot friend themselves.");
            return;
        }
        self.add_user(user1);
        self.add_user(user2);

        self.users.get_mut(user1).unwrap().insert(user2.to_string());
        self.users.get_mut(user2).unwrap().insert(user1.to_string());
    }

    // Remove friendship between two users
    fn remove_friend(&mut self, user1: &str, user2: &str) {
        if let Some(friends) = self.users.get_mut(user1) {
            friends.remove(user2);
        }
        if let Some(friends) = self.users.get_mut(user2) {
            friends.remove(user1);
        }
    }

    // Get friends of a user
    fn list_friends(&self, user: &str) -> Option<&HashSet<String>> {
        self.users.get(user)
    }

    // Find mutual friends between two users
    fn mutual_friends(&self, user1: &str, user2: &str) -> HashSet<String> {
        match (self.users.get(user1), self.users.get(user2)) {
            (Some(friends1), Some(friends2)) => friends1.intersection(friends2).cloned().collect(),
            _ => HashSet::new(),
        }
    }

    // Suggest friends (friends of friends who are not already friends or the user)
    fn suggest_friends(&self, user: &str) -> HashSet<String> {
        if let Some(friends) = self.users.get(user) {
            let mut suggestions = HashSet::new();
            for friend in friends {
                if let Some(friends_of_friend) = self.users.get(friend) {
                    for fof in friends_of_friend {
                        if fof != user && !friends.contains(fof) {
                            suggestions.insert(fof.clone());
                        }
                    }
                }
            }
            suggestions
        } else {
            HashSet::new()
        }
    }
}

fn main() {
    let mut network = SocialNetwork::new();

    network.add_friend("Alice", "Bob");
    network.add_friend("Alice", "Charlie");
    network.add_friend("Bob", "David");
    network.add_friend("Charlie", "Eve");
    network.add_friend("Eve", "Frank");

    println!("Alice's friends: {:?}", network.list_friends("Alice"));
    println!("Bob's friends: {:?}", network.list_friends("Bob"));

    println!("Mutual friends of Alice and Bob: {:?}", network.mutual_friends("Alice", "Bob"));
    println!("Mutual friends of Alice and Eve: {:?}", network.mutual_friends("Alice", "Eve"));

    println!("Friend suggestions for Alice: {:?}", network.suggest_friends("Alice"));
    println!("Friend suggestions for Bob: {:?}", network.suggest_friends("Bob"));

    network.remove_friend("Alice", "Charlie");
    println!("Alice's friends after removing Charlie: {:?}", network.list_friends("Alice"));
    println!("Charlie's friends after removal: {:?}", network.list_friends("Charlie"));
}
