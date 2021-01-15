use std::{ops::Mul, sync::Arc};

#[derive(Copy, Clone)]
pub struct Matrix4x4 {
  pub m: [[f32; 4]; 4],
}

impl Default for Matrix4x4 {
  fn default() -> Self {
    Matrix4x4::from_parts(
      1., 0., 0., 0.,
      0., 1., 0., 0.,
      0., 0., 1., 0.,
      0., 0., 0., 1.,
    )
  }
}

impl Matrix4x4 {
  pub fn new(m: [[f32; 4]; 4]) -> Self {
    Matrix4x4 { m }
  }

  pub fn from_parts(
    m_1_1: f32, m_1_2: f32, m_1_3: f32, m_1_4: f32,
    m_2_1: f32, m_2_2: f32, m_2_3: f32, m_2_4: f32,
    m_3_1: f32, m_3_2: f32, m_3_3: f32, m_3_4: f32,
    m_4_1: f32, m_4_2: f32, m_4_3: f32, m_4_4: f32,
  ) -> Self {
    Matrix4x4 {
      m: [
        [m_1_1, m_1_2, m_1_3, m_1_4],
        [m_2_1, m_2_2, m_2_3, m_2_4],
        [m_3_1, m_3_2, m_3_3, m_3_4],
        [m_4_1, m_4_2, m_4_3, m_4_4],
      ]
    }
  }

  pub fn inverse(&self) -> Option<Self> {
    let mut row_indexes = [0usize; 4];
    let mut col_indexes = [0usize; 4];
    let mut pivot_index = [0usize; 4];

    let mut result = self.m.clone();

    for i in 0..4 {
      let mut curr_row = 0;
      let mut curr_col = 0;
      let mut max = 0.;
      for j in 0..4 {
        if pivot_index[j] != 1 {
          for k in 0..4 {
            if result[j][k].abs() > max {
              max = result[j][k].abs();
              curr_row = j;
              curr_col = k;
            } else if pivot_index[k] > 1 {
              return None;
            }
          }
        }
      }

      pivot_index[curr_col] += 1;

      if curr_row != curr_col {
        result.swap(curr_row, curr_col);
      }

      row_indexes[i] = curr_row;
      col_indexes[i] = curr_col;

      if result[curr_col][curr_col] == 0. {
        return None;
      }

      let pivot_inverse = 1. / result[curr_col][curr_col];
      result[curr_col][curr_col] = 1.;
      for j in 0..4 {
        result[curr_col][j] *= pivot_inverse;
      }

      for j in 0..4 {
        if j != curr_col {
          let temp = result[j][curr_col];
          result[j][curr_col] = 0.;
          for k in 0..4 {
            result[j][k] -= result[curr_col][k] * temp;
          }
        }
      }

      for j in (0..4).rev() {
        if row_indexes[j] != col_indexes[j] {
          for k in 0..4 {
            let row_index = row_indexes[j];
            let col_index = col_indexes[j];
            let tmp = result[k][row_index];
            result[k][row_index] = result[k][col_index];
            result[k][col_index] = tmp;
          }
        }
      }
    }

    Some(Matrix4x4::new(result))
  }
}

impl Mul for Matrix4x4 {
    type Output = Matrix4x4;

    fn mul(self, rhs: Self) -> Self::Output {
      let mut result = [[0.; 4]; 4];
      for i in 0..4 {
        for j in 0..4 {
          for k in 0..4 {
            result[i][j] += self.m[i][k] * rhs.m[k][j];
          }
        }
      }
      Matrix4x4::new(result)
    }
}