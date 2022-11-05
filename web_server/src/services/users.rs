use crate::entities::User;
use crate::repositories::Users;

pub async fn get_user(repo: &impl Users, user_id: i32) -> User {
   repo.find_user(user_id)
       .await
       .unwrap_or(
            User::new(
               "ng".to_string(),
               user_id,
               "".to_string(),
               Some("user not found".to_string()),
            )
        )
}

pub async fn get_all_users(repo: &impl Users) -> Vec<User> {
   repo.all_users().await
}