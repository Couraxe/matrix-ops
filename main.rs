//! ```cargo
//! [dependencies]
//! rand = "0.8.3"
//! ```

use std::fmt;
use rand::Rng;

struct Matrix
{
	m: u32,
	n: u32,
	entries: Vec<f64>,
}

impl Matrix
{
	pub fn new(m: u32, n: u32) -> Matrix
	{
		Matrix
		{
			m,
			n,
			entries: vec![0f64; (m * n) as usize],
		}
	}

	/*
		Converts 2D coordinates to a 1D index.
	*/
	fn get_index(&self, i: u32, j: u32) -> usize
    {
        (i * self.n + j) as usize
    }

	/*
		Converts a 1D index to 2D coordinates 
		for easily operating on rows and columns.
	*/
	fn get_coords(&self, idx: usize) -> (u32, u32)
    {
        (idx as u32 / self.n, idx as u32 % self.n)
    }

	/*
		Returns the i-th row vector, indexing from 0 through m - 1.
	*/
	fn get_row_vec(&self, i: usize) -> Vec<f64>
	{
		let mut row = Vec::<f64>::new();

		for j in 0..self.n
		{
			let idx = self.get_index(i as u32, j as u32);
			row.push(self.entries[idx]);
		}

		row
	}

	/*
		Returns the j-th column vector, indexing from 0 through n - 1.
	*/
	fn get_col_vec(&self, j: usize) -> Vec<f64>
	{
		let mut col = Vec::<f64>::new();

		for i in 0..self.m
		{
			let idx = self.get_index(i as u32, j as u32);
			col.push(self.entries[idx]);
		}

		col
	}

	/*
		Produces a submatrix with row i and column j missing.
	*/
	pub fn sub_matrix(&self, i: u32, j: u32) -> Matrix
	{
		Matrix
		{
			m: self.m - 1,
			n: self.n - 1,
			entries: self.entries
						 .iter()
						 .enumerate()
						 .filter(|&(idx, _)| {
							let (row, col) = self.get_coords(idx);
							!(row == i || col == j)
						 })
						 .map(|(_, elem)| *elem)
						 .collect::<Vec<_>>(),
		}
	}

	pub fn add(&self, other: &Matrix) -> Matrix
	{
		if (self.m, self.n) != (other.m, other.n)
		{
			panic!("cannot add matrices of differing dimensions");
		}

		Matrix
		{
			m: self.m,
			n: self.n,
			entries: self.entries
						 .iter()
						 .zip(other.entries.iter())
						 .map(|(l, r)| l + r)
						 .collect::<Vec<_>>(),
		}
	}

	pub fn mul(&self, other: &Matrix) -> Matrix
	{
		if (self.m, self.n) != (other.n, other.m)
		{
			panic!("cannot multiply matrices of differing inverted dimensions");
		}

		let mut res = Matrix::new(self.m, other.n);

		for i in 0..self.m
		{
			let row_vec = self.get_row_vec(i as usize);
			for j in 0..self.n
			{
				let idx      = res.get_index(i as u32, j as u32);
				let col_vec  = other.get_col_vec(j as usize);
				let dot_prod = row_vec.iter()
									  .zip(col_vec.iter())
									  .map(|(l, r)| l * r)
									  .fold(0.0f64, |sum, r| sum + r);

				res.entries[idx] = dot_prod;
			}
		}

		res
	}

	pub fn det(&self) -> f64
	{
		if self.m != self.n
		{
			panic!("cannot compute determinant for non-square matrix")
		}

		match self.m
		{
			0 => 0f64,
			1 => self.entries[0],
			2 => {
				/*
					In the case of a 2x2-matrix A with entries
					[a b]
					[c d]
					we compute the determinant using the formula det(A) = a*d - b*c.
				*/
				let (a, b, c, d) = (
					self.entries[self.get_index(0, 0)],
					self.entries[self.get_index(0, 1)],
					self.entries[self.get_index(1, 0)],
					self.entries[self.get_index(1, 1)]
				);
				a * d - b * c
			}
			_ => {
				/*
					We use a Laplace expansion to compute the cofactors
					of the input matrix and recursively reduce the problem to a
					determinant of a 2x2-matrix.
				*/
				let mut det: f64 = 0f64;

				for col in 0..self.n
				{
					let idx   = self.get_index(0, col);
					let entry = self.entries[idx];
					let subm  = self.sub_matrix(0, col);
					let coef  = (-1f64).powf((0 + col) as f64);	

					let val = coef * entry * subm.det();

					det += val;
				}

				det
			},
		}
	}
}

impl fmt::Display for Matrix
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
	{
		/* 
			We begin at i = 1 such that the first entry
		    is not printed on a separate line. (Since 0 % k = 0 for all k.)
		*/
		for i in 1..=(self.m * self.n)
		{
			let entry = self.entries[(i-1) as usize];

			/* 
				We will produce a newline if we are on the n-th (n = width) 
				element within a row. We use a simple modulus check to determine this.
			*/
			if i % self.n == 0
			{
				write!(f, "{}\n", entry)?;
			}
			else
			{
				write!(f, "{} ", entry)?;
			}
		}

		Ok(())
	}
}

fn main()
{
	let mut mat = Matrix::new(3, 3);
	let mut rng = rand::thread_rng();

	for i in 0..(mat.m * mat.n)
	{
		mat.entries[i as usize] = rng.gen_range(1.0f64..20.0f64);
	}

	let mut mat2 = Matrix::new(3, 3);

	for i in 0..(mat2.m * mat2.n)
	{
		mat2.entries[i as usize] = rng.gen_range(1.0f64..20.0f64);
	}

	println!("{}", mat);
	println!("{}", mat2);
	println!("det(mat) = {}", mat.det());
	println!("sum: {}", mat.add(&mat2));
	println!("mul: {}", mat.mul(&mat2));
}
