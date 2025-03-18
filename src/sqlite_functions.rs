//SQLite function
use rusqlite::{Connection, Result};

#[derive(Debug, Default)]
pub struct Client{
    name : String,
    address : String,
    cost : f64,
    freq : i32,
    bag : bool,
    note : String,
}
impl Client{
    pub fn new(name: &str, address: &str, cost: f64, freq : i32, bag : bool, note: &str) -> Client{
        Client{
            name : name.to_string(),
            address : address.to_string(),
            cost : cost,
            freq : freq,
            bag: bag,
            note : note.to_string()
        }
    } 
}
pub fn connect_database(path_to_file : &str) -> Result<Connection>
{
    let conn = Connection::open(path_to_file)?;
    conn.execute(
        "create table if not exists liste_clients (
             id integer primary key,
             name_client text not null unique,
             address text not null unique,
             cost float not null,
             freq interger not null,
             bag bool not null,
             note text
         )",
        [],
    )?;
    conn.execute(
        "create table if not exists liste_pelouse (
             id integer primary key,
             day integer not null,
             month integer not null,
             year integer not null,
             client_id integer not null references liste_clients(id)
         )",
        [],
    )?;
    conn.execute(
        "create table if not exists liste_payement (
             id integer primary key,
             day integer not null,
             month integer not null,
             year integer not null,
             montant float not null,
             nbm_pelouse integer not null,
             note text,
             client_id integer not null references liste_clients(id)
         )",
        [],
    )?;
    Ok(conn)
} 

pub fn add_client(database : &Connection, new_client : &Client) -> bool
{
    database.execute(
        "INSERT INTO liste_clients (name_client, address, cost, freq, bag, note) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        (&new_client.name, &new_client.address, &new_client.cost, &new_client.freq, &new_client.bag, &new_client.note),
    ).is_ok()
}