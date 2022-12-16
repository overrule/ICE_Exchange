pub struct User{
    pub user_id: UserID,
    pub handle_name: String,
    pub cash: i64
}
#[derive(Copy, Clone, Default)]
pub struct UserID{
    pub user_id: usize
}
impl UserID{
    pub fn new(user_id: usize) -> Self{
        Self{ user_id }
    }
}
pub struct UserList{
    userlist: [User; crate::MAX_USERS]
}
impl UserList{
    pub fn change_cash_by(&mut self, user: UserID, delta_cash: i64){
        self.userlist[user.user_id].cash += delta_cash;
    }
}