use std::path::PathBuf;
use umya_spreadsheet::Spreadsheet;
use umya_spreadsheet::{reader, writer};

pub struct Merge {
    spreadsheet: Spreadsheet,
    file_name: PathBuf,
}

impl Merge {
    pub fn new(file_name: PathBuf) -> Self {
        let spreadsheet = umya_spreadsheet::new_file();
        Self {
            spreadsheet,
            file_name,
        }
    }

    pub fn merge_file(&mut self, file_name: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let spreadsheet = reader::xlsx::read(file_name)?;
        let worksheet = spreadsheet.get_sheet_by_name("Sheet1")?;
        let max_row = worksheet.get_highest_row();
        let max_col = worksheet.get_highest_column();

        let new_worksheet = self.spreadsheet.get_sheet_by_name_mut("Sheet1")?;
        let nrow = new_worksheet.get_highest_row();

        for row in 1..=max_row {
            for col in 1..=max_col {
                let value = worksheet.get_value_by_column_and_row(&col, &row);
                new_worksheet
                    .get_cell_by_column_and_row_mut(&col, &(nrow + row))
                    .set_value(value);
            }
        }

        Ok(())
    }

    pub fn write(&self) -> Result<(), writer::xlsx::XlsxError> {
        writer::xlsx::write(&self.spreadsheet, &self.file_name)
    }
}
