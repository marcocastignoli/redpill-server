#![allow(warnings)]

#![feature(custom_derive)]
#![feature(proc_macro_hygiene, decl_macro)]
// Dico a Rust di utilizzare il suo sistema di plugin
#[macro_use] extern crate rocket;
// Specificare #[macro_use] per importare anche le macro
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;
// Utilizzare mod per includere moduli nel filesystem dell'applicazione
extern crate chrono;
mod db;

// Includo i moduli
mod user;
mod post;
mod reaction;

fn main() {
    let mut rocket = rocket::ignite()
        .manage(db::connect());
    rocket = user::mount(rocket);
    rocket = post::mount(rocket);
    rocket = reaction::mount(rocket);
    rocket.launch();
}