//! Postgres client

#[path = "./common/mod.rs"]
mod common;
#[path = "./tls_stream/mod.rs"]
mod tls_stream;

use tokio::net::TcpStream;
use wtx::{
  database::{
    client::postgres::{Config, Executor, ExecutorBuffer},
    Executor as _, Record, Records, TransactionManager,
  },
  misc::UriPartsRef,
  rng::StdRng,
};

#[tokio::main]
async fn main() {
  let uri = common::_uri_from_args();
  let uri_parts = UriPartsRef::new(uri.as_str());
  let mut rng = StdRng::default();
  let config = Config::from_uri_parts(&uri_parts).unwrap();
  let eb = ExecutorBuffer::with_default_params(&mut rng);
  let initial_stream = TcpStream::connect(uri_parts.host()).await.unwrap();
  let mut exec =
    Executor::connect_encrypted(&config, eb, initial_stream, &mut rng, |stream| async {
      Ok(tls_stream::_tls_stream_stream(uri_parts.hostname(), stream).await)
    })
    .await
    .unwrap();
  let mut transaction = exec.transaction().await.unwrap();
  let _ = transaction
    .executor()
    .execute::<wtx::Error, _>("CREATE TABLE foo(id INT, name VARCHAR)", ())
    .await
    .unwrap();
  let _ = transaction
    .executor()
    .execute::<wtx::Error, _>(
      "INSERT INTO foo VALUES ($1, $2), ($3, $4);",
      (1u32, "one", 2u32, "two"),
    )
    .await
    .unwrap();
  transaction.commit().await.unwrap();
  let records = exec.records("SELECT * FROM foo;", (), |_| Ok::<_, wtx::Error>(())).await.unwrap();
  assert_eq!(records.record(0).as_ref().and_then(|record| record.decode("id").ok()), Some(1));
  assert_eq!(records.record(1).as_ref().and_then(|record| record.decode("name").ok()), Some("two"));
}
