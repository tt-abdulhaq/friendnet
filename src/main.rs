
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
}
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
    let users = Users::new(users1);
    let mut friend_ship = Friendships::new();
    friend_ship.friendships( &mut friend_ship1, &users);
    println!("{:?}", friend_ship);
    println!("{:?}", friend_ship.number_of_friends(&users.users[1]));
    
}
