#[cfg(feature = "sm-dev")]
use crate::database::{
  client::postgres::Postgres, client::postgres::Record, sm::fixed_sql_commands::postgres,
  sm::integration_tests, sm::Commands, sm::DbMigration, sm::SchemaManagement, FromRecord,
  Identifier,
};

#[cfg(feature = "sm-dev")]
pub(crate) async fn _clean_drops_all_objs<E>(
  (buffer_cmd, _, buffer_idents): (&mut String, &mut Vec<DbMigration>, &mut Vec<Identifier>),
  c: &mut Commands<E>,
  _: integration_tests::AuxTestParams,
) where
  E: SchemaManagement<Database = Postgres>,
  for<'rec> Identifier: FromRecord<crate::Error, Record<'rec>>,
{
  integration_tests::create_foo_table(buffer_cmd, c, "public.").await;
  let _ = c.executor.execute("CREATE SCHEMA bar", ()).await.unwrap();
  integration_tests::create_foo_table(buffer_cmd, c, "bar.").await;
  let _ = c.executor.execute("CREATE DOMAIN integer0 AS INTEGER CONSTRAINT must_be_greater_than_or_equal_to_zero_chk CHECK(VALUE >= 0);", ()).await.unwrap();
  let _ = c.executor.execute("CREATE FUNCTION time_subtype_diff(x time, y time) RETURNS float8 AS 'SELECT EXTRACT(EPOCH FROM (x - y))' LANGUAGE sql STRICT IMMUTABLE", ()).await.unwrap();
  let _ =
    c.executor.execute("CREATE PROCEDURE something() LANGUAGE SQL AS $$ $$", ()).await.unwrap();
  let _ = c.executor.execute("CREATE SEQUENCE serial START 101;", ()).await.unwrap();
  let _ = c.executor.execute("CREATE TYPE a_type AS (field INTEGER[31])", ()).await.unwrap();
  let _ = c.executor.execute("CREATE TYPE mood AS ENUM ('sad', 'ok', 'happy');", ()).await.unwrap();
  let _ =
    c.executor.execute("CREATE VIEW view AS SELECT * FROM foo WHERE id = 1;", ()).await.unwrap();

  c.executor.table_names(buffer_cmd, buffer_idents, "public").await.unwrap();
  assert_eq!(buffer_idents.len(), 1);
  buffer_idents.clear();

  postgres::_schemas(&mut c.executor, buffer_idents).await.unwrap();
  assert_eq!(buffer_idents.len(), 1);
  buffer_idents.clear();

  c.executor.table_names(buffer_cmd, buffer_idents, "bar").await.unwrap();
  assert_eq!(buffer_idents.len(), 1);
  buffer_idents.clear();

  postgres::_domains(&mut c.executor, buffer_idents).await.unwrap();
  assert_eq!(buffer_idents.len(), 1);
  buffer_idents.clear();

  postgres::_enums(&mut c.executor, buffer_idents).await.unwrap();
  assert_eq!(buffer_idents.len(), 1);
  buffer_idents.clear();

  postgres::_pg_proc((buffer_cmd, buffer_idents), &mut c.executor, 'f').await.unwrap();
  assert_eq!(buffer_idents.len(), 1);
  buffer_idents.clear();

  postgres::_pg_proc((buffer_cmd, buffer_idents), &mut c.executor, 'p').await.unwrap();
  assert_eq!(buffer_idents.len(), 1);
  buffer_idents.clear();

  postgres::_sequences(&mut c.executor, buffer_idents).await.unwrap();
  assert_eq!(buffer_idents.len(), 1);
  buffer_idents.clear();

  postgres::_types(&mut c.executor, buffer_idents).await.unwrap();
  assert_eq!(buffer_idents.len(), 2);
  buffer_idents.clear();

  postgres::_views(&mut c.executor, buffer_idents).await.unwrap();
  assert_eq!(buffer_idents.len(), 1);
  buffer_idents.clear();

  c.clear((buffer_cmd, buffer_idents)).await.unwrap();

  c.executor.table_names(buffer_cmd, buffer_idents, "public").await.unwrap();
  assert_eq!(buffer_idents.len(), 0);
  buffer_idents.clear();

  postgres::_schemas(&mut c.executor, buffer_idents).await.unwrap();
  assert_eq!(buffer_idents.len(), 0);
  buffer_idents.clear();

  c.executor.table_names(buffer_cmd, buffer_idents, "bar").await.unwrap();
  assert_eq!(buffer_idents.len(), 0);
  buffer_idents.clear();

  postgres::_domains(&mut c.executor, buffer_idents).await.unwrap();
  assert_eq!(buffer_idents.len(), 0);
  buffer_idents.clear();

  postgres::_enums(&mut c.executor, buffer_idents).await.unwrap();
  assert_eq!(buffer_idents.len(), 0);
  buffer_idents.clear();

  postgres::_pg_proc((buffer_cmd, buffer_idents), &mut c.executor, 'f').await.unwrap();
  assert_eq!(buffer_idents.len(), 0);
  buffer_idents.clear();

  postgres::_pg_proc((buffer_cmd, buffer_idents), &mut c.executor, 'p').await.unwrap();
  assert_eq!(buffer_idents.len(), 0);
  buffer_idents.clear();

  postgres::_sequences(&mut c.executor, buffer_idents).await.unwrap();
  assert_eq!(buffer_idents.len(), 0);
  buffer_idents.clear();

  postgres::_types(&mut c.executor, buffer_idents).await.unwrap();
  assert_eq!(buffer_idents.len(), 0);
  buffer_idents.clear();

  postgres::_views(&mut c.executor, buffer_idents).await.unwrap();
  assert_eq!(buffer_idents.len(), 0);
  buffer_idents.clear();
}
