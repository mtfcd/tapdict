use sqlx::SqliteConnection;

#[derive(Debug)]
pub struct StarWord {
    pub word: String,
    pub phonetic: Option<String>,
    pub definition: Option<String>,
    pub translation: Option<String>,
}

pub async fn lookup(word: &str, conn: &mut SqliteConnection) -> Result<StarWord, sqlx::Error> {
    let def = sqlx::query_as!(
        StarWord,
        "SELECT word, phonetic, definition, translation FROM stardict WHERE word = ?",
        word
    )
    .fetch_one(conn)
    .await?;
    Ok(def)
}

#[test]
async fn test_stardict() {
    use sqlx::Connection;
    let mut conn = SqliteConnection::connect("sqlite://./resources/stardict.db")
        .await
        .unwrap();
    let def = lookup("set", &mut conn).await.unwrap();
    println!("{:#?}", def);
}
