use super::account_model::Account;
use anyhow::Result;
use async_graphql::Object;

#[derive(Default)]
pub struct AccountQuery;

#[Object]
impl AccountQuery {
    // Get an account by ID from the database.
    async fn account<'a>(&self) -> Result<Account> {
        Ok(Account {
            id: String::from("Test"),
            username: String::from("Lol"),
            nickname: String::from("Hey"),
            email: String::from("lol@gmail.com"),
        })
    }
}

#[Object]
impl Account {
    async fn id(&self) -> Result<&str> {
        Ok(&self.id)
    }

    async fn username(&self) -> Result<&str> {
        Ok(&self.username)
    }

    async fn nickname(&self) -> Result<&str> {
        Ok(&self.nickname)
    }

    async fn email(&self) -> Result<&str> {
        Ok(&self.email)
    }
}
