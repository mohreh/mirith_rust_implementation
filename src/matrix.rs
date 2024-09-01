use std::fmt;
use std::ops;

use crate::common::numeric::Numeric;

#[derive(Debug)]
pub enum MatrixDefinitionError {
    UndefinedRows,
    UndefinedCols,
    NoDataProvided,
    DataLengthDoesNotMatchRowsTimesCols,
}

impl fmt::Display for MatrixDefinitionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MatrixDefinitionError::UndefinedRows => {
                write!(f, "Number of rows is not defined")
            }
            MatrixDefinitionError::UndefinedCols => {
                write!(f, "Number of cols is not defined")
            }
            MatrixDefinitionError::NoDataProvided => {
                write!(f, "Data was not provided to data field of matrix instance")
            }
            MatrixDefinitionError::DataLengthDoesNotMatchRowsTimesCols => {
                write!(f, "Data length is not consistent with dimensions")
            }
        }
    }
}

impl std::error::Error for MatrixDefinitionError {}

#[derive(Debug)]
pub struct MatrixIndexOutOfBoundsError;

impl fmt::Display for MatrixIndexOutOfBoundsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Trying to access with out of bounds elements")
    }
}

impl std::error::Error for MatrixIndexOutOfBoundsError {}

#[derive(Debug)]
pub enum MatrixCopyToError {
    DefinitionError(MatrixDefinitionError),
}

impl fmt::Display for MatrixCopyToError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MatrixCopyToError::DefinitionError(_) => {
                write!(f, "Matrix Copy failed because of a definition error")
            }
        }
    }
}

impl std::error::Error for MatrixCopyToError {}

#[derive(Debug)]
pub struct Matrix<T>
where
    T: Numeric,
{
    rows: usize,
    cols: usize,
    data: Vec<T>,
}

impl<T> Matrix<T>
where
    T: Numeric,
{
    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
    }

    pub fn data(&self) -> &Vec<T> {
        &self.data
    }

    pub fn copy_to<S>(&self) -> Result<Matrix<S>, MatrixCopyToError>
    where
        S: Numeric,
    {
        MatrixBuilder::<S>::new()
            .from_vec(
                self.rows,
                self.cols,
                self.data.iter().map(|&element| element.to::<S>()).collect(),
            )
            .build()
            .map_err(MatrixCopyToError::DefinitionError)
    }
}

impl<T> ops::Index<(usize, usize)> for Matrix<T>
where
    T: Numeric,
{
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (row, col) = index;
        if row >= self.rows || col >= self.cols {
            panic!("Index out of bounds");
        }
        &self.data[row * self.cols + col]
    }
}

impl<T> ops::IndexMut<(usize, usize)> for Matrix<T>
where
    T: Numeric,
{
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let (row, col) = index;
        if row >= self.rows || col >= self.cols {
            panic!("Index out of bounds");
        }
        &mut self.data[row * self.cols + col]
    }
}

impl<T> PartialEq for Matrix<T>
where
    T: Numeric,
{
    fn eq(&self, other: &Self) -> bool {
        self.rows == other.rows
            && self.cols == other.cols
            && self.data.iter().zip(other.data.iter()).all(|(a, b)| a == b)
    }
}

impl<T> fmt::Display for Matrix<T>
where
    T: Numeric,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f)?;
        for (n, element) in self.data.iter().enumerate() {
            let i = n / self.cols;
            let j = n - i * self.cols;
            if j > 0 {
                write!(f, " ")?;
            } else {
                write!(f, "|")?;
            }
            write!(f, "{:10}", element)?;
            if j == self.cols - 1 {
                write!(f, "|")?;
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

pub trait Swap {
    fn swap_cols(&mut self, j1: usize, j2: usize) -> Result<(), MatrixIndexOutOfBoundsError>;
    fn swap_rows(&mut self, i1: usize, i2: usize) -> Result<(), MatrixIndexOutOfBoundsError>;
}

impl<T> Swap for Matrix<T>
where
    T: Numeric,
{
    fn swap_cols(&mut self, j1: usize, j2: usize) -> Result<(), MatrixIndexOutOfBoundsError> {
        if j1 < self.cols && j2 < self.cols {
            for i in 0..self.rows {
                self.data.swap(i * self.cols + j1, i * self.cols + j2);
            }
            Ok(())
        } else {
            Err(MatrixIndexOutOfBoundsError)
        }
    }

    fn swap_rows(&mut self, i1: usize, i2: usize) -> Result<(), MatrixIndexOutOfBoundsError> {
        if i1 < self.rows && i2 < self.rows {
            for j in 0..self.cols {
                self.data.swap(i1 * self.cols + j, i2 * self.cols + j);
            }
            Ok(())
        } else {
            Err(MatrixIndexOutOfBoundsError)
        }
    }
}

#[derive(Default)]
pub struct MatrixBuilder<T> {
    rows: Option<usize>,
    cols: Option<usize>,
    data: Option<Vec<T>>,
}

impl<T> MatrixBuilder<T>
where
    T: Numeric,
{
    pub fn new() -> Self {
        MatrixBuilder {
            rows: None,
            cols: None,
            data: None,
        }
    }

    pub fn rows(mut self, rows: usize) -> Self {
        self.rows = if rows > 0 { Some(rows) } else { None };
        self
    }

    pub fn cols(mut self, cols: usize) -> Self {
        self.cols = if cols > 0 { Some(cols) } else { None };
        self
    }

    pub fn data(mut self, data: Vec<T>) -> Self {
        self.data = if !data.is_empty() { Some(data) } else { None };
        self
    }

    pub fn from_vec(self, rows: usize, cols: usize, data: Vec<T>) -> Self {
        self.rows(rows).cols(cols).data(data)
    }

    pub fn from_mat(self, data: Vec<Vec<T>>) -> Self {
        println!("data = {:?}", data);
        if let Some(first_row) = data.first() {
            let rows = data.len();
            let cols = first_row.len();
            let flat_data: Vec<T> = data.into_iter().flatten().collect();
            self.rows(rows).cols(cols).data(flat_data)
        } else {
            self
        }
    }

    pub fn zeros(self, rows: usize, cols: usize) -> Self {
        self.rows(rows)
            .cols(cols)
            .data(vec![T::default(); rows * cols])
    }

    pub fn identity(self, n: usize) -> Self {
        let mut data = vec![T::default(); n * n];
        for k in 0..n {
            data[k * n + k] = T::one();
        }
        self.rows(n).cols(n).data(data)
    }

    pub fn build(self) -> Result<Matrix<T>, MatrixDefinitionError> {
        let data = self.data.ok_or(MatrixDefinitionError::NoDataProvided)?;
        let rows = self.rows.ok_or(MatrixDefinitionError::UndefinedRows)?;
        let cols = self.cols.ok_or(MatrixDefinitionError::UndefinedCols)?;
        if data.len() != rows * cols {
            Err(MatrixDefinitionError::DataLengthDoesNotMatchRowsTimesCols)
        } else {
            Ok(Matrix { rows, cols, data })
        }
    }
}
