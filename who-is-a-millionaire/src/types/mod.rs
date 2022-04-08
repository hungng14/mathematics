use postgres::{Client, Transaction};

pub struct SQLConnection {
  pub client: Client
}

pub struct SQLTransaction<'a> {
  pub transaction: Transaction<'a>
}

impl SQLConnection {
  pub fn transaction(&mut self) -> SQLTransaction {
    SQLTransaction {
      transaction: self.client.transaction().unwrap()
    }
  }
}