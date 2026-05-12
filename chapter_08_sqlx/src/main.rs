use rusqlite::Connection;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conn: Connection = Connection::open("my-db.db3")?;

    // Create table
    conn.execute(
        "CREATE TABLE person(
            id      INTEGER PRIMARY KEY,
            name    TEXT NOT NULL,
            dob     INTEGER, --date of birth 
            date    BLOB
        )STRICT",
        (),
    )?;

    // Insert
    conn.execute("INSERT INTO person(name,dob) VALUES(?1,?2)", ("mukul", &1997))?;

    // Select
    let select_query = " SELECT person.id, person.name, person.dob
                        FROM person
                        WHERE dob > :dob ";
    let mut stmt = conn.prepare(select_query)?;
    let mut rows = stmt.query(&[(":dob", &1990)])?;

    // print selected
    while let Some(selected_row) = rows.next()? {
        let name: String = selected_row.get(1)?;
        println!("->> name : {name}");
        println!("->> row  : {selected_row:?}");
    }

    Ok(())
}
