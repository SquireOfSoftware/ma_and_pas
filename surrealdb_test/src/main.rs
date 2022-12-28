// use serde::{Deserialize, Serialize};
// use serde_json::json;
// use std::borrow::Cow;
// use surrealdb_rs::param::Root;
// use surrealdb_rs::protocol::Ws;
// use surrealdb_rs::{Result, Surreal};
//
// #[derive(Serialize, Deserialize)]
// struct Name {
//     first: Cow<'static, str>,
//     last: Cow<'static, str>,
// }
//
// #[derive(Serialize, Deserialize)]
// struct Person {
//     #[serde(skip_serializing)]
//     id: Option<String>,
//     title: Cow<'static, str>,
//     name: Name,
//     marketing: bool,
// }
//
// #[tokio::main]
// async fn main() -> Result<()> {
//     let client = Surreal::connect::<Ws>("localhost:8000").await?;
//
//     // Signin as a namespace, database, or root user
//     client
//         .signin(Root {
//             username: "root",
//             password: "root",
//         })
//         .await?;
//
//     // Select a specific namespace and database
//     client.use_ns("test").use_db("test").await?;
//
//     // Create a new person with a random ID
//     let tobie: Person = client
//         .create("person")
//         .content(Person {
//             id: None,
//             title: "Founder & CEO".into(),
//             name: Name {
//                 first: "Tobie".into(),
//                 last: "Morgan Hitchcock".into(),
//             },
//             marketing: true,
//         })
//         .await?;
//
//     assert!(tobie.id.is_some());
//
//     // Create a new person with a specific ID
//     let mut jaime: Person = client
//         .create(("person", "jaime"))
//         .content(Person {
//             id: None,
//             title: "Founder & COO".into(),
//             name: Name {
//                 first: "Jaime".into(),
//                 last: "Morgan Hitchcock".into(),
//             },
//             marketing: false,
//         })
//         .await?;
//
//     assert_eq!(jaime.id.unwrap(), "person:jaime");
//
//     // Update a person record with a specific ID
//     jaime = client
//         .update(("person", "jaime"))
//         .merge(json!({ "marketing": true }))
//         .await?;
//
//     assert!(jaime.marketing);
//
//     // Select all people records
//     let people: Vec<Person> = client.select("person").await?;
//
//     assert!(!people.is_empty());
//
//     // Perform a custom advanced query
//     let groups = client
//         .query("
//             SELECT marketing,
//                    count()
//             FROM type::table($table)
//             GROUP BY marketing
//         ")
//         .bind("table", "person")
//         .await?;
//
//     dbg!(groups);
//
//     // Delete all people upto but not including Jaime
//     client.delete("person").range(.."jaime").await?;
//
//     // Delete all people
//     client.delete("person").await?;
//
//     Ok(())
// }

use std::borrow::Cow;
use surrealdb_rs::param::Root;
use surrealdb_rs::protocol::Ws;
use surrealdb_rs::{Result, Surreal};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Name {
    first: Cow<'static, str>,
    last: Cow<'static, str>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    #[serde(skip_serializing)]
    id: Option<String>,
    title: Cow<'static, str>,
    name: Name,
    marketing: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("starting the server");
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let client = Surreal::connect::<Ws>("localhost:8000").await
        .expect("should connect");

    println!("connection established");

    // for some strange reason the code just stops here, something to do with the client which
    // i havent figured out yet
    client.signin(Root {
        username: "root",
        password: "root"
    })
        .await?;

    println!("signed in");

    client.use_ns("test").use_db("test").await?;

    let person: Person = client
        .create("person")
        .content(Person {
            id: None,
            title: "Test".into(),
            name: Name {
                first: "John".into(),
                last: "Smith".into(),
            },
            marketing: false
        })
        .await.expect("should create user");

    println!("created a user");

    assert!(person.id.is_some());
    let people: Vec<Person> = client.select("person").await.expect("should select people");

    println!("Hello world");
    Ok(())
}
