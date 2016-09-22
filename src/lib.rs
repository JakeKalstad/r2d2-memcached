extern crate memcached;

use memcached::Client;
use memcached::proto::ProtoType;
use memcached::proto::Error;

extern crate r2d2;

pub struct MemcacheConnectionManager {
  pub url: String,
}

impl r2d2::ManageConnection for MemcacheConnectionManager {
  type Connection = Client;
  type Error = Error;

  fn connect(&self) -> Result<Client, Error> {
    Client::connect(&self.url)
  }

  fn is_valid(&self, conn: &mut Client) -> Result<(), Error> {

    match conn.proto.set(key, value, flags, expiration) {
      Ok(_)     => Ok(()),
      Err(err)  => Err(err)
    }
  }

  fn has_broken(&self, _: &mut Client) -> bool {
    false
  }
}
