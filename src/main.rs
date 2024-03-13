use std::collections::HashSet;

#[derive(Debug)]
struct User {
    id: usize,
    name: String
}
impl  User {
    fn new(id:usize, name: String) -> Self{
        User {
            id,name
        }
    }
    fn user_interest(&self, interests: &Vec<(usize, &str)>) -> Vec<String> {
        let mut interest: Vec<String> = Vec::new();
        for iters in interests.into_iter() {
            if self.id == iters.0 {
                interest.push(iters.1.to_string());
            }
        }
        interest
    }
}
#[derive(Debug)]
struct  Users {
    users: Vec<User>
}

impl  Users {
    fn new(users: Vec<User>) -> Self {
        Users { users : users}
    }
}


#[derive(Debug)]
struct Friendships {
    friendship: Vec<(usize,Vec<usize>)>
}


impl  Friendships {
    fn new() -> Self {
        Self {friendship: Vec::new()}
    }

    fn friendships(&mut self,  friend_pairs:&mut Vec<(usize, usize)>, users: &Users){
        for user in &users.users{
            self.friendship.push((user.id, Vec::new()));
        }
        for i in friend_pairs{
            self.friendship[i.0].1.push(i.1);
            self.friendship[i.1].1.push(i.0);

        }
    }

    fn number_of_friends(&mut self, user: &User) -> usize {
        let user_id = user.id;
        let friends_ids = &mut self.friendship[user_id].1;
        friends_ids.len()
    }

    fn friend_of_friends(&self, user: &User) -> Vec<usize> {
        let user_id = user.id;
        let mut foaf: HashSet<usize> = HashSet::new();
        for friend_id in &self.friendship[user_id].1 {
            for &foaf_id in &self.friendship[*friend_id].1 {
                if foaf_id != user_id && !self.friendship[user_id].1.contains(&foaf_id) {
                    foaf.insert(foaf_id);
                }
            }
        }
    
        // Convert HashSet to Vec and return
        foaf.into_iter().collect()
    }
    
}

fn main() {
    let users1 = vec![User::new(0, "Hero".to_owned()), 
                            User{id: 1, name: "Dunn".to_owned()},
                            User{id: 2, name: "Sue".to_owned()},
                            User{id: 3, name: "Chi".to_owned()},
                            User{id: 4, name: "Thor".to_owned()},
                            User{id: 5, name: "Clive".to_owned()},
                            User{id: 6, name: "Hicks".to_owned()},
                            User{id: 7, name: "Devin".to_owned()},
                            User{id: 8, name: "Kate".to_owned()},
                            User{id: 9, name: "Klein".to_owned()}];

    let mut friend_ship1 = vec![(0,1),
                                            (0,2),
                                            (1,2),
                                            (1,3),
                                            (2,3),
                                            (3,4),
                                            (4,5),
                                            (5,6),
                                            (5,7),
                                            (6,8),
                                            (7,8),
                                            (8,9)];

    // find the ids of all users who like  a target interest.
    let interests = vec![
        (0, "Hadoop"), (0, "Big Data"), (0, "HBase"), (0, "Java"),
        (0, "Spark"), (0, "Storm"), (0, "Cassandra"),
        (1, "NoSQL"), (1, "MongoDB"), (1, "Cassandra"), (1, "HBase"),
        (1, "Postgres"), (2, "Python"), (2, "scikit-learn"), (2, "scipy"),
        (2, "numpy"), (2, "statsmodels"), (2, "pandas"), (3, "R"), (3, "Python"),
        (3, "statistics"), (3, "regression"), (3, "probability"),
        (4, "machine learning"), (4, "regression"), (4, "decision trees"),
        (4, "libsvm"), (5, "Python"), (5, "R"), (5, "Java"), (5, "C++"),
        (5, "Haskell"), (5, "programming languages"), (6, "statistics"),
        (6, "probability"), (6, "mathematics"), (6, "theory"),
        (7, "machine learning"), (7, "scikit-learn"), (7, "Mahout"),
        (7, "neural networks"), (8, "neural networks"), (8, "deep learning"),
        (8, "Big Data"), (8, "artificial intelligence"), (9, "Hadoop"),
        (9, "Java"), (9, "MapReduce"), (9, "Big Data")];


    let users = Users::new(users1);
    let mut friend_ship = Friendships::new();
    println!("user 0 -->{:?}", users.users[0].user_interest(&interests));
    println!("user 1 --> {:?}", users.users[1].user_interest(&interests));
    println!("{:?}", friend_ship.friendship);
    println!("_________________________________________________");
    println!("{:?}", &users.users[5]);
    println!("{:?}", friend_ship.friendship[5]);
    println!("{:?}", friend_ship.number_of_friends(&users.users[5]));
    println!("{:?}", friend_ship.friend_of_friends(&users.users[5]));

    
}
