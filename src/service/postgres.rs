pub struct PostgresClient {
  client: tokio_postgres::Client,
  pub database_names: Vec<String>,
}

pub struct Config {
  host: String,
  user: String,
}

impl Config {
  pub fn build_str(&self) -> String {
    format!("host={} user={}", self.host, self.user)
  }
}

impl PostgresClient {
  pub async fn new(config: Option<Config>) -> Result<Self, tokio_postgres::Error> {
    let client_config: Config = match config {
      Some(c) => c,
      None => Config { host: String::from("localhost"), user: String::from("alexmatthewcandelario") },
    };
    let (client, connection) =
      tokio_postgres::connect("host=localhost user=alexmatthewcandelario", tokio_postgres::NoTls).await?;

    tokio::spawn(async move {
      if let Err(e) = connection.await {
        eprintln!("connection error: {}", e);
      }
    });

    let mut s = Self { client, database_names: Vec::new() };
    s.get_database_names().await?;
    Ok(s)
  }

  pub async fn print_schemas(&mut self) -> Result<(), tokio_postgres::Error> {
    for row in self.client.query("SELECT datname FROM pg_database", &[]).await? {
      let d: String = row.get("datname");
      println!("Row: {:#?}", d);
    }
    Ok(())
  }

  pub async fn get_database_names(&mut self) -> Result<(), tokio_postgres::Error> {
    for row in self.client.query("SELECT datname FROM pg_database", &[]).await? {
      self.database_names.push(row.get("datname"));
    }
    Ok(())
  }

  pub fn user_query(&mut self, query: &str) {
  }
}
