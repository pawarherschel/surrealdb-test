use serde::{Deserialize, Serialize};
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::sql::Thing;
use surrealdb::Surreal;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");

    macro_rules! measure_time_inline {
        ($comment:expr => $stmt:expr) => {{
            let start = std::time::Instant::now();
            let result = $stmt;
            let duration = start.elapsed();
            println!("Execution time for {}: {:?}", $comment, duration);
            result
        }};
    }

    let db = measure_time_inline!( 
        "Creating connection to database with WebSocket" 
        => Surreal::new::<Ws>("localhost:8000").await?);

    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;

    db.use_ns("test").use_db("test").await?;

    #[derive(Debug, Serialize, Deserialize)]
    struct Name {
        first: String,
        last: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct Person {
        title: String,
        name: Name,
        marketing: bool,
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct Responsibility {
        marketing: bool,
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct Record {
        #[allow(dead_code)]
        id: Thing,
    }

    macro_rules! measure_time {
        ($comment:expr => $stmt:expr) => {{
            let start = std::time::Instant::now();
            {
                $stmt
            }
            let duration = start.elapsed();
            println!("Execution time for {}: {:?}", $comment, duration);
        }};
    }

    measure_time!("creating person record" => {
        let created: Record = db
            .create("person")
            .content(Person {
                title: "CEO".into(),
                name: Name {
                    first: "John".into(),
                    last: "Doe".into(),
                },
                marketing: true,
            })
            .await?;
        dbg!(&created);
    });

    measure_time!("updating person record" => {
        let updated: Record = db
            .update(("person", "jaime"))
            .merge(Responsibility { marketing: true })
            .await?;
        dbg!(&updated);
    });

    measure_time!("selection all people" => {
        let people: Vec<Record> = db.select("person").await?;
        dbg!(people);
    });

    measure_time!("complex select" => {
        let groups = db
            .query("SELECT marketing, count() FROM type::table($table) GROUP BY marketing")
            .bind(("table", "person"))
            .await?;
        dbg!(groups);
    });

    Ok(())
}
