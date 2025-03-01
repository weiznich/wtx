use crate::database::client::postgres::{statements::Statement, Postgres, Record};
use core::ops::Range;

/// Records
#[derive(Debug, Default, Eq, PartialEq)]
pub struct Records<'exec> {
  pub(crate) bytes: &'exec [u8],
  /// Each element represents a record and an offset of `values_bytes_offsets`.
  pub(crate) records_values_offsets: &'exec [usize],
  pub(crate) stmt: Statement<'exec>,
  /// Each element represents a value and an offset of `bytes`.
  pub(crate) values_bytes_offsets: &'exec [Range<usize>],
}

impl<'exec> crate::database::Records for Records<'exec> {
  type Database = Postgres;

  #[inline]
  fn len(&self) -> usize {
    self.records_values_offsets.len()
  }

  #[inline]
  fn record(&self, record_idx: usize) -> Option<Record<'_>> {
    let slice = self.records_values_offsets.get(..record_idx.wrapping_add(1))?;
    let (record_bytes_range, record_values_bytes_offsets) = match slice {
      [] => return None,
      &[to] => {
        let record_values_bytes_offsets = self.values_bytes_offsets.get(..to)?;
        (0..record_values_bytes_offsets.last()?.end, record_values_bytes_offsets)
      }
      &[.., from, to] => {
        let record_values_bytes_offsets = self.values_bytes_offsets.get(from..to)?;
        let [start, end] = match record_values_bytes_offsets {
          [first] => [first.start, first.end],
          [first, .., last] => [first.start, last.end],
          _ => return None,
        };
        (start..end, record_values_bytes_offsets)
      }
    };
    let initial_value_offset = record_bytes_range.start;
    Some(Record {
      bytes: self.bytes.get(record_bytes_range)?,
      initial_value_offset,
      stmt: self.stmt.clone(),
      values_bytes_offsets: record_values_bytes_offsets,
    })
  }
}

#[cfg(test)]
mod tests {
  use crate::database::{
    client::postgres::{statements::Statement, Record, Records},
    Record as _, Records as _,
  };
  use alloc::vec::Vec;

  #[test]
  fn returns_correct_values() {
    let bytes = &[0, 0, 0, 2, 1, 2, 0, 0, 0, 2, 3, 4, 9, 9, 9, 0, 1, 0, 0, 0, 4, 5, 6, 7, 8];
    let mut records_values_offsets = Vec::new();
    let mut values_bytes_offsets = Vec::new();
    let _ =
      Record::parse(bytes, 0..12, Statement::new(&[], &[]), &mut values_bytes_offsets, 2).unwrap();
    records_values_offsets.push(values_bytes_offsets.len());
    let _ =
      Record::parse(bytes, 17..25, Statement::new(&[], &[]), &mut values_bytes_offsets, 1).unwrap();
    records_values_offsets.push(values_bytes_offsets.len());

    let records = Records {
      bytes: &bytes[4..],
      records_values_offsets: &records_values_offsets,
      stmt: Statement::new(&[], &[]),
      values_bytes_offsets: &values_bytes_offsets,
    };
    assert_eq!(records.len(), 2);
    assert_eq!(records.bytes, &bytes[4..]);
    assert_eq!(records.records_values_offsets, &[2, 3]);
    assert_eq!(records.values_bytes_offsets, &[0..2, 6..8, 17..21]);

    let first_record = records.record(0).unwrap();
    assert_eq!(
      &first_record,
      &Record {
        bytes: &[1, 2, 0, 0, 0, 2, 3, 4],
        initial_value_offset: 0,
        stmt: Statement::new(&[], &[]),
        values_bytes_offsets: &[0..2, 6..8]
      }
    );
    assert_eq!(first_record.value(0).unwrap(), &[1, 2]);
    assert_eq!(first_record.value(1).unwrap(), &[3, 4]);

    let second_record = records.record(1).unwrap();
    assert_eq!(
      &second_record,
      &Record {
        bytes: &[5, 6, 7, 8],
        initial_value_offset: 17,
        stmt: Statement::new(&[], &[]),
        values_bytes_offsets: &[17..21]
      }
    );
  }
}
