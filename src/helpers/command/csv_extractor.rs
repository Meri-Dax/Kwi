use std::{fs::File, marker::PhantomData};

use csv::StringRecord;

pub struct CsvExtractor<T> {
    reader: csv::Reader<File>,
    _marker: PhantomData<T>,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Could not read file")]
    CouldNotRead(#[from] std::io::Error),
    #[error("Empty line")]
    Empty,
    #[error("CSV error {0}")]
    CSVError(#[from] csv::Error),
    #[error("CSV invalid values")]
    InvalidRecord,
}

impl<T> CsvExtractor<T>
where
    T: TryFrom<csv::StringRecord>,
{
    pub fn try_new(file: &str) -> Result<Self, Error> {
        let file = File::open(file)?;
        let reader = csv::Reader::from_reader(file);
        Ok(Self {
            reader,
            _marker: PhantomData,
        })
    }

    fn convert(row: Option<Result<StringRecord, csv::Error>>) -> Result<T, Error> {
        match row {
            None => Err(Error::Empty),
            Some(Ok(result)) => {
                let result: Result<T, _> = result.try_into();
                match result {
                    Ok(result) => Ok(result),
                    Err(_) => Err(Error::InvalidRecord),
                }
            }
            Some(Err(e)) => Err(Error::CSVError(e)),
        }
    }

    pub fn get(&mut self) -> Result<T, Error> {
        let result = self.reader.records().next();

        let result = Self::convert(result)?;

        Ok(result)
    }

    pub fn read_all(&mut self) -> Result<Vec<T>, Error> {
        let mut result: Vec<T> = Vec::new();
        for row in self.reader.records() {
            let row_convert = Self::convert(Some(row))?;

            result.push(row_convert);
        }

        Ok(result)
    }
}
