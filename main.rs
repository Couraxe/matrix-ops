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
			entries: vec![0.0; (m * n) as usize],
		}
	}

	fn get_index(&self, i: u32, j: u32) -> usize
    {
        (i * self.n + j) as usize
    }

	fn get_coords(&self, idx: usize) -> (u32, u32)
    {
        (idx as u32 / self.n, idx as u32 % self.n)
    }

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

	pub fn det(&self) -> f64
	{
		match self.m == 2
		{
			true  => {
				let (a, b, c, d) = (
					self.entries[self.get_index(0, 0)],
					self.entries[self.get_index(0, 1)],
					self.entries[self.get_index(1, 0)],
					self.entries[self.get_index(1, 1)]
				);
				a * d - b * c
			}
			false => {
				let mut det: f64 = 0.0;

				for col in 0..self.n
				{
					let idx   = self.get_index(0, col);
					let entry = self.entries[idx];
					let subm  = self.sub_matrix(0, col);
					let coef  = (-1.0f64).powf((0 + col) as f64);	

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

	println!("{}", mat);
	println!("det(mat) = {}", mat.det());
}
