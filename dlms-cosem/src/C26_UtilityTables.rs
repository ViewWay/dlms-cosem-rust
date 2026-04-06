//! IC026 Utility Tables
//!
//! Class ID = 26, Version = 0
//!
//! Attributes:
//! 1: logical_name (octet-string)
//! 2: table_cell_values (array of structures)

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

/// A single cell value in the utility table.
/// Each cell is a structure: (row, column, value)
#[derive(Debug, Clone)]
pub struct TableCellValue {
    pub row: u16,
    pub column: u16,
    pub value: DlmsData,
}

/// Utility Tables (Class ID 26) — holds a grid of cell values.
pub struct UtilityTables26 {
    logical_name: ObisCode,
    cells: Vec<TableCellValue>,
}

impl UtilityTables26 {
    pub fn new(logical_name: ObisCode) -> Self {
        Self {
            logical_name,
            cells: Vec::new(),
        }
    }

    pub fn cells(&self) -> &[TableCellValue] {
        &self.cells
    }

    pub fn add_cell(&mut self, row: u16, column: u16, value: DlmsData) {
        // Replace if exists, else push
        if let Some(existing) = self.cells.iter_mut().find(|c| c.row == row && c.column == column) {
            existing.value = value;
        } else {
            self.cells.push(TableCellValue { row, column, value });
        }
    }

    pub fn cell_count(&self) -> usize {
        self.cells.len()
    }

    pub fn get_cell(&self, row: u16, column: u16) -> Option<&DlmsData> {
        self.cells
            .iter()
            .find(|c| c.row == row && c.column == column)
            .map(|c| &c.value)
    }
}

impl CosemObject for UtilityTables26 {
    fn class_id(&self) -> u16 {
        26
    }
    fn logical_name(&self) -> ObisCode {
        self.logical_name
    }
    fn attribute_count(&self) -> u8 {
        2
    }
    fn method_count(&self) -> u8 {
        0
    }

    fn attribute_to_bytes(&self, attr: u8) -> Option<Vec<u8>> {
        match attr {
            1 => {
                let name = self.logical_name.to_bytes();
                Some(vec![
                    0x09, 0x06, name[0], name[1], name[2], name[3], name[4], name[5],
                ])
            }
            2 => Some(dlms_axdr::encode(&DlmsData::Array(
                self.cells
                    .iter()
                    .map(|c| {
                        DlmsData::Structure(vec![
                            DlmsData::LongUnsigned(c.row),
                            DlmsData::LongUnsigned(c.column),
                            c.value.clone(),
                        ])
                    })
                    .collect(),
            ))),
            _ => None,
        }
    }

    fn attribute_from_bytes(&mut self, attr: u8, _data: &[u8]) -> Result<(), CosemObjectError> {
        match attr {
            2 => Err(CosemObjectError::AttributeNotSupported(attr)),
            _ => Err(CosemObjectError::AttributeNotSupported(attr)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_utility_tables_class_id() {
        let ut = UtilityTables26::new(ObisCode::new(0, 0, 26, 0, 0, 255));
        assert_eq!(ut.class_id(), 26);
        assert_eq!(ut.attribute_count(), 2);
    }

    #[test]
    fn test_utility_tables_add_and_get_cell() {
        let mut ut = UtilityTables26::new(ObisCode::new(0, 0, 26, 0, 0, 255));
        ut.add_cell(1, 1, DlmsData::DoubleLong(42));
        ut.add_cell(1, 2, DlmsData::DoubleLong(99));
        assert_eq!(ut.cell_count(), 2);
        assert_eq!(ut.get_cell(1, 1).unwrap().as_i32(), Some(42));
        assert_eq!(ut.get_cell(1, 2).unwrap().as_i32(), Some(99));
        assert!(ut.get_cell(2, 1).is_none());
    }

    #[test]
    fn test_utility_tables_replace_cell() {
        let mut ut = UtilityTables26::new(ObisCode::new(0, 0, 26, 0, 0, 255));
        ut.add_cell(1, 1, DlmsData::DoubleLong(10));
        ut.add_cell(1, 1, DlmsData::DoubleLong(20)); // overwrite
        assert_eq!(ut.cell_count(), 1);
        assert_eq!(ut.get_cell(1, 1).unwrap().as_i32(), Some(20));
    }

    #[test]
    fn test_utility_tables_attr2_serialization() {
        let mut ut = UtilityTables26::new(ObisCode::new(0, 0, 26, 0, 0, 255));
        ut.add_cell(0, 0, DlmsData::DoubleLong(100));
        let bytes = ut.attribute_to_bytes(2).unwrap();
        assert!(!bytes.is_empty());
    }
}
