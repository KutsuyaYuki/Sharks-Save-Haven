// add a using statement for db.rs
mod db;
fn main() {
    // call the create_db function in db.rs
    let a: Result<(), rusqlite::Error> = db::create_db();

}
