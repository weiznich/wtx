use crate::database::client::postgres::SqlState;
use alloc::string::String;
use core::{
  fmt::{Debug, Formatter},
  ops::Range,
};

/// Position of an error in a query.
#[derive(Debug, Eq, PartialEq)]
pub enum ErrorPosition {
  /// Position in an internally generated query.
  Internal {
    /// Byte position.
    position: u32,
    /// Query generated by the server.
    query: Range<usize>,
  },
  /// Position in the original query.
  Original(u32),
}

create_enum! {
  /// The severity of a Postgres error or notice.
  #[derive(Clone, Copy, Debug, Eq, PartialEq)]
  pub enum Severity<u8> {
    /// Debug
    Debug = (0, "DEBUG"),
    /// Error
    Error = (1, "ERROR"),
    /// Fatal
    Fatal = (2, "FATAL"),
    /// Info
    Info = (3, "INFO"),
    /// Log
    Log = (4, "LOG"),
    /// Notice
    Notice = (5, "NOTICE"),
    /// Panic
    Panic = (6, "PANIC"),
    /// Warning
    Warning = (7, "WARNING"),
  }
}

/// A Postgres error or notice.
#[derive(Eq, PartialEq)]
pub struct DbError {
  buffer: String,
  code: SqlState,
  column: Option<Range<usize>>,
  constraint: Option<Range<usize>>,
  datatype: Option<Range<usize>>,
  detail: Option<Range<usize>>,
  file: Option<Range<usize>>,
  hint: Option<Range<usize>>,
  line: Option<u32>,
  message: Range<usize>,
  position: Option<ErrorPosition>,
  routine: Option<Range<usize>>,
  schema: Option<Range<usize>>,
  severity_localized: Range<usize>,
  severity_nonlocalized: Option<Severity>,
  table: Option<Range<usize>>,
  r#where: Option<Range<usize>>,
}

impl DbError {
  /// The SQLSTATE code for the error
  #[inline]
  pub fn code(&self) -> &SqlState {
    &self.code
  }

  /// If the error was associated with a specific table column, the name of the column.
  #[inline]
  pub fn column(&self) -> Option<&str> {
    self.column.as_ref().and_then(|range| self.buffer.get(range.clone()))
  }

  /// If the error was associated with a specific constraint, the name of the constraint.
  #[inline]
  pub fn constraint(&self) -> Option<&str> {
    self.constraint.as_ref().and_then(|range| self.buffer.get(range.clone()))
  }

  /// If the error was associated with a specific data type, the name of the data type.
  #[inline]
  pub fn datatype(&self) -> Option<&str> {
    self.datatype.as_ref().and_then(|range| self.buffer.get(range.clone()))
  }

  /// An optional secondary error message carrying more detail about the problem. Might run to
  /// multiple lines.
  #[inline]
  pub fn detail(&self) -> Option<&str> {
    self.detail.as_ref().and_then(|range| self.buffer.get(range.clone()))
  }

  /// The file name of the source-code location where the error was reported.
  #[inline]
  pub fn file(&self) -> Option<&str> {
    self.file.as_ref().and_then(|range| self.buffer.get(range.clone()))
  }

  /// An optional suggestion what to do about the problem.
  #[inline]
  pub fn hint(&self) -> Option<&str> {
    self.hint.as_ref().and_then(|range| self.buffer.get(range.clone()))
  }

  /// The line number of the source-code location where the error was reported.
  #[inline]
  pub fn line(&self) -> Option<u32> {
    self.line
  }

  /// The primary human-readable error message.
  #[inline]
  pub fn message(&self) -> &str {
    self.buffer.get(self.message.clone()).unwrap_or_default()
  }

  /// The field value is a decimal ASCII integer, indicating an error cursor position as an index
  /// into the original query string.
  #[inline]
  pub fn position(&self) -> Option<&ErrorPosition> {
    self.position.as_ref()
  }

  /// The name of the source-code routine reporting the error.
  #[inline]
  pub fn routine(&self) -> Option<&str> {
    self.routine.as_ref().and_then(|range| self.buffer.get(range.clone()))
  }

  /// If the error was associated with a specific database object, the name of the schema
  /// containing that object, if any.
  #[inline]
  pub fn schema(&self) -> Option<&str> {
    self.schema.as_ref().and_then(|range| self.buffer.get(range.clone()))
  }

  /// Localized severity.
  #[inline]
  pub fn severity_localized(&self) -> &str {
    self.buffer.get(self.severity_localized.clone()).unwrap_or_default()
  }

  /// Nonlocalized `severity`.
  #[inline]
  pub fn severity_nonlocalized(&self) -> Option<Severity> {
    self.severity_nonlocalized
  }

  /// If the error was associated with a specific table, the name of the table.
  #[inline]
  pub fn table(&self) -> Option<&str> {
    self.table.as_ref().and_then(|range| self.buffer.get(range.clone()))
  }

  /// An indication of the context in which the error occurred. Presently this includes a call
  /// stack traceback of active procedural language functions and internally-generated queries.
  #[inline]
  pub fn r#where(&self) -> Option<&str> {
    self.r#where.as_ref().and_then(|range| self.buffer.get(range.clone()))
  }
}

impl Debug for DbError {
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    f.debug_struct("DbError")
      .field("code", &self.code())
      .field("column", &self.column())
      .field("constraint", &self.constraint())
      .field("datatype", &self.datatype())
      .field("detail", &self.detail())
      .field("file", &self.file())
      .field("hint", &self.hint())
      .field("line", &self.line())
      .field("message", &self.message())
      .field("position", &self.position())
      .field("routine", &self.routine())
      .field("schema", &self.schema())
      .field("severity_localized", &self.severity_localized())
      .field("severity_nonlocalized", &self.severity_nonlocalized())
      .field("table", &self.table())
      .field("r#where", &self.r#where())
      .finish()
  }
}

impl TryFrom<&str> for DbError {
  type Error = crate::Error;

  #[inline]
  fn try_from(from: &str) -> Result<Self, Self::Error> {
    let mut code = None;
    let mut column = None;
    let mut constraint = None;
    let mut datatype = None;
    let mut detail = None;
    let mut file = None;
    let mut hint = None;
    let mut internal_position = None;
    let mut internal_query = None;
    let mut line = None;
    let mut message = None;
    let mut normal_position = None;
    let mut routine = None;
    let mut schema = None;
    let mut severity_localized = None;
    let mut severity_nonlocalized = None;
    let mut table = None;
    let mut r#where = None;

    let mut idx: usize = 0;
    loop {
      let Some(curr) = from.get(idx..) else {
        break;
      };
      if curr.is_empty() {
        break;
      }
      idx = idx.wrapping_add(1);
      let (ty, rest) = curr.split_at(1);
      if ty == "\0" {
        if rest.is_empty() {
          break;
        }
        return Err(crate::Error::UnexpectedString { length: rest.len() });
      }
      let Some((data, _)) = rest.split_once('\0') else {
        return Err(crate::Error::UnexpectedEOF);
      };
      let begin = idx;
      let end = idx.wrapping_add(data.len());
      let range = begin..end;
      idx = end.wrapping_add(1);
      match ty {
        "C" => code = Some(SqlState::try_from(data)?),
        "D" => detail = Some(range),
        "H" => hint = Some(range),
        "L" => line = Some(data.parse::<u32>()?),
        "M" => message = Some(range),
        "P" => normal_position = Some(data.parse::<u32>()?),
        "R" => routine = Some(range),
        "S" => severity_localized = Some(range),
        "V" => severity_nonlocalized = Some(Severity::try_from(data)?),
        "W" => r#where = Some(range),
        "c" => column = Some(range),
        "d" => datatype = Some(range),
        "F" => file = Some(range),
        "n" => constraint = Some(range),
        "p" => internal_position = Some(data.parse::<u32>()?),
        "q" => internal_query = Some(range),
        "s" => schema = Some(range),
        "t" => table = Some(range),
        _ => {
          return Err(crate::Error::UnexpectedUint { received: ty.parse()? });
        }
      }
    }

    Ok(Self {
      buffer: from.get(..idx).unwrap_or_default().into(),
      code: code.ok_or(crate::Error::NoInnerValue("No code"))?,
      column,
      constraint,
      datatype,
      detail,
      file,
      hint,
      line,
      message: message.ok_or(crate::Error::NoInnerValue("No message"))?,
      severity_localized: severity_localized.ok_or(crate::Error::NoInnerValue("No severity"))?,
      severity_nonlocalized,
      position: match normal_position {
        None => match internal_position {
          Some(position) => Some(ErrorPosition::Internal {
            position,
            query: internal_query.ok_or(crate::Error::NoInnerValue("No internal query"))?,
          }),
          None => None,
        },
        Some(position) => Some(ErrorPosition::Original(position)),
      },
      routine,
      schema,
      table,
      r#where,
    })
  }
}
