
use serde::Serialize;
use sqlx::{Error, Row};
use sqlx::postgres::{PgPoolOptions};
use serde::Deserialize;


#[derive(Deserialize)]
#[derive(Debug, Clone, PartialEq,Serialize)]
pub struct Foo {
    name: String,
}

#[derive(Deserialize)]
#[derive(Debug, Clone, PartialEq,Serialize,sqlx::FromRow)]
pub struct posts{
    pub(crate) id: i32,
    pub(crate) title: String,
    pub(crate) description: String,
}

pub(crate) async fn connection() ->Result<Vec<String>, ()>{


    dotenv::dotenv().expect("Unable to load environment variables from .env file");

    let db_url = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");

    let pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await.expect("Unable to connect to Postgres");


    let mut vect=Vec::new();
    let  rows = sqlx::query("SELECT name FROM categories")
        .fetch_all(&pool)
        .await.expect("Unable to");



    for row in rows{
        let names: String=row.get("name");

        vect.push(names);

    }


    Ok(vect)
}




pub async fn select_all_from_table() -> Result<Vec<String>,Error> {

    dotenv::dotenv().expect("Unable to load environment variables from .env file");

    let db_url = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");

    let mut pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await.expect("Unable to connect to Postgres");

    let mut all_posts = Vec::new();


    let rows = sqlx::query("SELECT id,title,description FROM posts")
        .fetch_all(&pool)
        .await?;
    for row in rows {
        let id: i32 = row.get("id");
        let title: String = row.get("title");
        let description: String = row.get("description");
        let all_posts_string=title+" " +&*description;
        all_posts.push(all_posts_string);
    }

    let  x:i32= all_posts.len() as i32;
    println!(" {:?}",x);

    Ok(all_posts)
}

pub async fn select_posts()->Result<Vec<posts>,Error>
{
    dotenv::dotenv().expect("Unable to load environment variables from .env file");

    let db_url = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");

    let mut pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await.expect("Unable to connect to Postgres");

    let mut posting = sqlx::query_as::<_, posts>("select id, title, description from posts")
        .fetch_all(&pool)
        .await
        .unwrap();


    Ok(posting)
}