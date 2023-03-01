use crate::db_models::User;
use crate::db_utils::DbActor;
use crate::schema::users::dsl::*;
use crate::messages::{CreateUser, FetchSingleUser};
use crate::insertables::NewUser;
use actix::Handler;
use diesel::{self, prelude::*};


impl Handler<FetchSingleUser> for DbActor {
    type Result = QueryResult<User>;
  
    fn handle(&mut self, msg: FetchSingleUser, _ctx: &mut Self::Context) -> Self::Result {
      let mut conn = self.0.get().expect("Fetch Single User: Unable to establish connection");
  
      users.filter(username.eq(msg.username)).get_result::<User>(&mut conn)
    }
  }


impl Handler<CreateUser> for DbActor {
    type Result = QueryResult<User>;
  
    fn handle(&mut self, msg: CreateUser, _ctx: &mut Self::Context) -> Self::Result {
      let mut conn = self.0.get().expect("Create User: Unable to establish connection");
  
      let new_user = NewUser {
        username: msg.username,
        pwd: msg.pwd,
      };
  
      diesel::insert_into(users)
        .values(new_user)
        .returning((
          id,
          username,
          pwd,
        ))
        .get_result::<User>(&mut conn)
    }
  }