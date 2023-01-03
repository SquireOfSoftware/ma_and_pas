use tokio_postgres::{Error, NoTls};

struct Person {
    id: i32,
    name: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let (client, connection) = tokio_postgres::connect(
        "host=localhost user=postgres password=test port=5432",
        NoTls,
    )
    .await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    // client.batch_execute("
    // CREATE TABLE person (
    //     id      SERIAL PRIMARY KEY,
    //     name    TEXT NOT NULL
    // )
    // ")
    //     .await?;

    // let name = "Ferris";
    // client.execute(
    //     "INSERT INTO person (name) VALUES ($1)",
    //     &[&name],
    // )
    //     .await?;

    for row in client.query("SELECT id, name FROM person", &[]).await? {
        let id: i32 = row.get(0);
        let name: &str = row.get(1);

        println!("found person: id={} name={}", id, name);
    }

    Ok(())
}
