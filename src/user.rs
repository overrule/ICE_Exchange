pub struct User{
    pub user_id: UserID,
    pub handle_name: String,
    pub cash: u64
}
#[derive(Copy, Clone)]
pub struct UserID{
    pub user_id: usize
}
