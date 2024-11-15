/// Represents a 1-dimensional map folding.
#[derive(Debug)]
pub struct MapFolding {
    /// Representation of the folding in the standard stack form.
    /// The number of the lowest segment is at index 0 in the vector.
    indices: Vec<u64>,
}

impl MapFolding {

    /// Attempts to create a 1-dimensional map folding based on a list of segments
    /// written in the standard stack notation. 
    ///
    /// The standard stack notation requires that:
    /// 1. Segment #1 is facing the same way as it did before the map was folded.
    /// 2. The segments are listed in the same order in which they appear in the stack,
    ///    starting from the bottom. That is, the number of the lowest segment in the
    ///    stack is the first value in the list and vice versa.
    ///
    /// # Errors
    ///
    /// This function returns an error if the supplied list of segments is incoherent.
    /// However, it does not check whether the map folding in question is physically
    /// possible. Impossible foldings can be created with this function!
    ///
    /// # Example:
    /// ```
    /// # use map_folding::MapFolding;
    /// let f = MapFolding::from_standard_stack(&vec![1, 3, 4, 2]).unwrap();
    ///
    /// assert_eq!(format!("{:?}", f.standard_stack()), "[1, 3, 4, 2]");
    /// ```
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
    
    /// Returns the standard stack representation of the map folding as a list
    /// of segments.
    ///
    /// # Example
    /// ```
    /// # use map_folding::MapFolding;
    /// let f = MapFolding::from_standard_stack(&vec![5, 2, 3, 4, 1]).unwrap();
    ///
    /// assert_eq!(f.standard_stack(), vec![5, 2, 3, 4, 1]);
    /// ```
    pub fn standard_stack(&self) -> &[u64] {
        &self.indices[..]
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
