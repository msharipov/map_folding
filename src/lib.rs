// Represents a 1-dimensional map folding.
#[derive(Debug)]
pub struct MapFolding {
    // Representation of the folding in the standard stack form.
    // The number of the lowest segment is at index 0 in the vector.
    indices: Vec<u64>,
}

impl MapFolding {
    pub fn from_standard_stack(stack: &[u64]) -> Result<Self, ()> {
        let indices = Vec::from(stack);
        let n = indices.len();

        let min_index = match indices.iter().min() {
            Some(&index) => index,
            None => return Err(()),
        };
        let max_index = match indices.iter().max() {
            Some(&index) => index as usize,
            None => return Err(()),
        };

        if min_index != 1 || max_index != n {
            return Err(());
        }

        let mut present = vec![false; n + 1]; // 0th value is unused
        for segment in indices[0..n].iter() {
            present[*segment as usize] = true;
        }

        let all_present = present.iter().skip(1).all(|&p| p);
        if all_present {
            return Ok(MapFolding { indices });
        }
        Err(())
    }

    pub fn positions(&self) -> Vec<usize> {
        let n = self.indices.len();
        let mut positions: Vec<usize> = vec![0; n + 1]; // 0th value is unused
        for (index, segment) in self.indices.iter().enumerate() {
            positions[*segment as usize] = index;
        }
        positions
    }

    pub fn is_valid_fold(&self) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_standard_stack_8_zigzag() {
        let indices = [1, 2, 3, 4, 5, 6, 7, 8];
        let created = MapFolding::from_standard_stack(&indices).expect("failed to parse indices");
        let expected = vec![1, 2, 3, 4, 5, 6, 7, 8];
        assert_eq!(created.indices, expected, "indices do not match");
    }

    #[test]
    fn from_standard_stack_invalid_indices() {
        let indices = [2, 5, 4, 3];
        assert_eq!((), MapFolding::from_standard_stack(&indices).unwrap_err());
    }

    #[test]
    fn from_standard_stack_invalid_indices_2() {
        let indices = [0, 1, 2, 3, 4];
        assert_eq!((), MapFolding::from_standard_stack(&indices).unwrap_err());
    }

    #[test]
    fn from_standard_stack_missing_indices() {
        let indices = [5, 3, 1, 2, 1];
        assert_eq!((), MapFolding::from_standard_stack(&indices).unwrap_err());
    }

    #[test]
    fn from_standard_stack_extra_indices() {
        let indices = [5, 3, 4, 5, 2, 1];
        assert_eq!((), MapFolding::from_standard_stack(&indices).unwrap_err());
    }
}
