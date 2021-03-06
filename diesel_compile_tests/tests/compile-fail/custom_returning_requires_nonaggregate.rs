#![feature(custom_derive, plugin, custom_attribute)]
#![plugin(diesel_codegen)]
#[macro_use]
extern crate diesel;

use diesel::*;
use diesel::expression::count;

table! {
    users {
        id -> Integer,
        name -> VarChar,
    }
}

#[insertable_into(users)]
pub struct NewUser {
    name: String,
}

fn main() {
    use self::users::dsl::*;

    let stmt = update(users.filter(id.eq(1))).set(name.eq("Bill")).returning(count(id));
    //~^ ERROR NonAggregate

    let new_user = NewUser {
        name: "Foobar".to_string(),
    };
    let stmt = insert(&new_user).into(users).returning((name, count(name)));
    //~^ ERROR NonAggregate
}
