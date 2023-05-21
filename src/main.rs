use itertools::Itertools;
use serde::{Deserialize, Serialize};
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::sql::Thing;
use surrealdb::Surreal;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");

    macro_rules! measure_time_inline {
        ($stmt:expr) => {{
            let start = std::time::Instant::now();
            let result = $stmt;
            let duration = start.elapsed();
            println!("Execution time: {:?}", duration);
            result
        }};
    }

    let db = measure_time_inline!(Surreal::new::<Ws>("localhost:8000").await?);

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
        ($stmt:stmt) => {{
            let start = std::time::Instant::now();
            $stmt;
            let duration = start.elapsed();
            println!("Execution time: {:?}", duration);
        }};
    }

    measure_time!({
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

    measure_time!({
        let updated: Record = db
            .update(("person", "jaime"))
            .merge(Responsibility { marketing: true })
            .await?;
        dbg!(&updated);
    });

    measure_time!({
        let people: Vec<Record> = db.select("person").await?;
        dbg!(people);
    });

    measure_time!({
        let groups = db
            .query("SELECT marketing, count() FROM type::table($table) GROUP BY marketing")
            .bind(("table", "person"))
            .await?;
        dbg!(groups);
    });

    Ok(())
}
