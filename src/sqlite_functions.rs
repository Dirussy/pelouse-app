use chrono::{Datelike, NaiveDate};
//SQLite function
use rusqlite::{Connection, Result};

#[derive(Debug, Default)]
pub struct Client{
    name : String,
    address : String,
    cost : f64,
    freq : u32,
    bag : bool,
    note : String,
}

#[derive(Debug, Default)]
pub struct TaskClient{
    name: String,
    address: String,
    client_id : u32,
    nb_days: Option<i64>,
    irregulier: bool,
}

#[derive(Debug, Default)]
pub struct Payement{
    pay: f64,
    date: u32,
    month: u32,
    year:i32,
    is_cash: bool,
    note: String,
    client_id: u32,
}

impl Payement {
    pub fn new(pay: f64, date: u32, month: u32, year: i32, note: &str, is_cash:bool, client_id: u32) -> Payement
    {
        Payement { 
            pay, 
            date, 
            month, 
            year,
            is_cash,
            note: note.to_string(),
            client_id
         }
    }
    pub fn add_payement(&self, database : &Connection) -> bool
    { 
        database.execute(
            "INSERT INTO liste_payement (day, month, year, montant, is_cash, note, client_id) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            (&self.date, &self.month, &self.year, &self.pay, &self.is_cash, &self.note, &self.client_id),
        ).is_ok()
    }
}
impl Client{
    pub fn new(name: &str, address: &str, cost: f64, freq : u32, bag : bool, note: &str) -> Client{
        Client{
            name : name.to_string(),
            address : address.to_string(),
            cost,
            freq,
            bag,
            note : note.to_string()
        }
    } 
    pub fn laod_from_name(name: &str) ->Client
    {
        let conn = Connection::open("PelouseData.db").expect("Cannot open database");
            let address: String = conn.query_row_and_then(
                "SELECT address FROM liste_clients WHERE name_client=?1",
                [name],
                |row| row.get(0),
            ).unwrap();
            let cost: f64 = conn.query_row_and_then(
                "SELECT cost FROM liste_clients WHERE name_client=?1",
                [name],
                |row| row.get(0),
            ).unwrap();
            let freq: u32 = conn.query_row_and_then(
                "SELECT freq FROM liste_clients WHERE name_client=?1",
                [name],
                |row| row.get(0),
            ).unwrap();
            let bag: bool = conn.query_row_and_then(
                "SELECT bag FROM liste_clients WHERE name_client=?1",
                [name],
                |row| row.get(0),
            ).unwrap();
            let note: String = conn.query_row_and_then(
                "SELECT note FROM liste_clients WHERE name_client=?1",
                [name],
                |row| row.get(0),
            ).unwrap();

            Client::new(name, &address, cost, freq, bag, &note)
    }

    pub fn name(&self) -> &String{
        &self.name
    }
    pub fn address(&self) -> &String{
        &self.address
    }
    pub fn cost(&self) -> &f64{
        &self.cost
    }
    pub fn freq(&self) -> &u32{
        &self.freq
    }
    pub fn is_bag_use(&self) -> &bool{
        &self.bag
    }
    pub fn note(&self) -> &String{
        &self.note
    }
    pub fn get_client_id(&self, database : &Connection) -> Result<u32>
    {
        get_client_id(self.name().into(), database)
    }
    
    
}

impl TaskClient {
    pub fn name(&self) -> &String{
        &self.name
    }
    pub fn address(&self) -> &String{
        &self.address
    }
    pub fn client_id(&self) -> &u32{
        &self.client_id
    }
    pub fn nb_days(&self) -> &Option<i64>
    {
        &self.nb_days
    }
    pub fn irregular(&self) -> &bool{
        &self.irregulier
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
             freq unsigned interger not null,
             bag bool not null,
             note text
         )",
        [],
    )?;
    conn.execute(
        "create table if not exists liste_pelouse (
             id integer primary key,
             day unsigned integer not null,
             month unsigned integer not null,
             year integer not null,
             client_id unsigned integer not null references liste_clients(id)
         )",
        [],
    )?;
    conn.execute(
        "create table if not exists liste_payement (
             id integer primary key,
             day unsigned integer not null,
             month unsigned integer not null,
             year integer not null,
             montant float not null,
             is_cash bool,
             note text,
             client_id unsigned integer not null references liste_clients(id)
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

pub fn add_pelouse(database : &Connection, day : u32, month: u32, year: i32, client_id: u32) -> bool
{
    database.execute(
        "INSERT INTO liste_pelouse (day, month, year, client_id) VALUES (?1, ?2, ?3, ?4)",
        (day, month, year, client_id),
    ).is_ok()
}
pub fn get_client_id(client_name: &String,  database : &Connection) -> Result<u32> {
    database.query_row_and_then(
        "SELECT id FROM liste_clients WHERE name_client=?1",
        [client_name],
        |row| row.get(0),
    )
}

pub fn load_list_client_last_job(path :&str) -> Result<Vec<TaskClient>>
{
    let conn = connect_database(path)?;
    // Retrieve data from users table
    let mut stmt = conn.prepare("SELECT id, name_client, address, freq FROM liste_clients")?;
    let user_iter = stmt.query_map([], |row| {
        Ok(TaskClient {
            client_id: row.get(0)?,
            name: row.get(1)?,
            address: row.get(2)?,
            nb_days: match row.get(3)? {
                Some(t) => Some(t),
                None => None,
            },
            irregulier: match row.get(3)? {
                Some(0) => true,
                _ => false,
            },
        })
    })?;
    
    let mut liste_task: Vec<TaskClient> = Vec::new();
    for user in user_iter {
        let temp = user?;
        let task_client = TaskClient{
            client_id: temp.client_id.clone(),
            name: temp.name.clone(),
            address : temp.address.clone(),
            nb_days: get_nb_day_last_job(temp.client_id.clone(),temp.nb_days.clone() , &conn),
            irregulier: temp.irregulier.clone(),
        };
        liste_task.push(task_client);
    }
    Ok(liste_task)
} 

fn get_nb_day_last_job(client_id: u32,nb_days : Option<i64>, conn : &Connection) -> Option<i64>
{
    let mut stmt = conn.prepare(&format!("SELECT day, month, year FROM liste_pelouse WHERE client_id = {}", client_id) )
        .expect("Error in preparing queary");
    let job_iter = stmt.query_map([], |row| {
        Ok(NaiveDate::from_ymd_opt(row.get(2)?, row.get(1)?, row.get(0)?).expect("Date invalid!"))
    }).unwrap();
    let mut day_min = i64::MIN;
    let day = match nb_days {
        Some(day) => day,
        None => 0,
    };
    let local_time = chrono::Local::now();
    let naive_local_time = NaiveDate::from_ymd_opt(local_time.year(), local_time.month(), local_time.day());
    // Iterate over the retrieved rows
    for job in job_iter {

        let dif_day = match job {
            Err(_) => i64::MIN,
            Ok(last_job) => {
                    let dif_days = naive_local_time.unwrap() - last_job;
                    day - dif_days.num_days()
            }
        };
        if day_min < dif_day
        {
            day_min = dif_day
        }
    }
    match day_min {
        i64::MIN => None,
        day=> Some(day),
    }

}