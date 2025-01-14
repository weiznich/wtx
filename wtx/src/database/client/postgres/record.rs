use crate::database::{
  client::postgres::{statements::Statement, Postgres},
  Database, ValueIdent,
};
use alloc::vec::Vec;
use core::ops::Range;

/// Record
#[derive(Debug, Eq, PartialEq)]
pub struct Record<'exec> {
  pub(crate) bytes: &'exec [u8],
  pub(crate) initial_value_offset: usize,
  pub(crate) stmt: Statement<'exec>,
  pub(crate) values_bytes_offsets: &'exec [Range<usize>],
}

impl<'exec> Record<'exec> {
  pub(crate) fn parse(
    mut bytes: &'exec [u8],
    bytes_range: Range<usize>,
    stmt: Statement<'exec>,
    values_bytes_offsets: &'exec mut Vec<Range<usize>>,
    values_len: u16,
  ) -> crate::Result<Self> {
    let mut fun = |curr_value_offset: &mut usize, [a, b, c, d]: [u8; 4]| {
      let begin = *curr_value_offset;
      let curr_len = usize::try_from(i32::from_be_bytes([a, b, c, d]))?;
      let end = begin.wrapping_add(curr_len);
      values_bytes_offsets.push(begin..end);
      *curr_value_offset = end;
      crate::Result::Ok(())
    };

    let initial_value_offset = bytes_range.start;
    let mut curr_value_offset = bytes_range.start;

    match (bytes.get(bytes_range), values_len) {
      (Some([a, b, c, d, rest @ ..]), 1..=u16::MAX) => {
        bytes = rest;
        fun(&mut curr_value_offset, [*a, *b, *c, *d])?;
      }
      _ => {
        return Ok(Self { bytes, initial_value_offset: 0, stmt, values_bytes_offsets });
      }
    }

    for _ in 1..values_len {
      let idx = curr_value_offset.wrapping_sub(initial_value_offset);
      let Some(&[a, b, c, d, ..]) = bytes.get(idx..) else {
        return Err(crate::Error::InvalidPostgresRecord);
      };
      curr_value_offset = curr_value_offset.wrapping_add(4);
      fun(&mut curr_value_offset, [a, b, c, d])?;
    }

    Ok(Self { bytes, initial_value_offset, stmt, values_bytes_offsets })
  }
}

impl<'exec> crate::database::Record for Record<'exec> {
  type Database = Postgres;

  #[inline]
  fn len(&self) -> usize {
    self.values_bytes_offsets.len()
  }

  #[inline]
  fn value<'this, CI>(&'this self, ci: CI) -> Option<<Self::Database as Database>::Value<'this>>
  where
    CI: ValueIdent<Record<'this>>,
  {
    let range = self.values_bytes_offsets.get(ci.idx(self)?)?;
    let begin = range.start.wrapping_sub(self.initial_value_offset);
    let end = range.end.wrapping_sub(self.initial_value_offset);
    self.bytes.get(begin..end)
  }
}

impl<'exec> ValueIdent<Record<'exec>> for str {
  #[inline]
  fn idx(&self, input: &Record<'exec>) -> Option<usize> {
    input.stmt.columns.iter().position(|column| column.name.as_str() == self)
  }
}

#[cfg(feature = "arrayvec")]
mod arrayvec {
  use crate::{
    database::{FromRecord, Record},
    misc::_from_utf8_basic_rslt,
  };
  use arrayvec::ArrayString;

  impl<'exec, E, const N: usize> FromRecord<E, crate::database::client::postgres::Record<'exec>>
    for ArrayString<N>
  where
    E: From<crate::Error>,
  {
    #[inline]
    fn from_record(record: crate::database::client::postgres::Record<'exec>) -> Result<Self, E> {
      Ok(
        _from_utf8_basic_rslt(record.value(0).ok_or(crate::Error::NoInnerValue("Record"))?)
          .map_err(From::from)?
          .try_into()
          .map_err(From::from)?,
      )
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::database::{
    client::postgres::{statements::Statement, Record},
    Record as _,
  };
  use alloc::vec;

  #[test]
  fn returns_correct_values() {
    let bytes = &[0, 0, 0, 1, 1, 0, 0, 0, 2, 2, 3, 0, 0, 0, 1, 4];
    let mut values_bytes_offsets = vec![];
    let stmt = Statement::new(&[], &[]);
    let record =
      Record::parse(bytes, 0..bytes.len(), stmt.clone(), &mut values_bytes_offsets, 3).unwrap();
    assert_eq!(
      record,
      Record {
        bytes: &[1, 0, 0, 0, 2, 2, 3, 0, 0, 0, 1, 4],
        initial_value_offset: 0,
        stmt,
        values_bytes_offsets: &[0..1, 5..7, 11..12]
      }
    );
    assert_eq!(record.len(), 3);
    assert_eq!(record.value(0), Some(&[1][..]));
    assert_eq!(record.value(1), Some(&[2, 3][..]));
    assert_eq!(record.value(2), Some(&[4][..]));
    assert_eq!(record.value(3), None);
  }
}
