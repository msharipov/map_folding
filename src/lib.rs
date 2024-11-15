use std::cmp::{max, min};

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
    /// let f = MapFolding::from_stack(&[1, 3, 4, 2]).unwrap();
    ///
    /// assert_eq!(format!("{:?}", f.stack()), "[1, 3, 4, 2]");
    /// ```
    pub fn from_stack(stack: &[u64]) -> Result<Self, ()> {
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
    /// let f = MapFolding::from_stack(&[5, 2, 3, 4, 1]).unwrap();
    ///
    /// assert_eq!(f.stack(), [5, 2, 3, 4, 1]);
    /// ```
    pub fn stack(&self) -> &[u64] {
        &self.indices[..]
    }

    /// Returns a vector that lists the positions of each segment in the standard
    /// stack representation of the map folding.
    ///
    /// The zero-indexed position of the segment N is located at index N in the
    /// returned vector.
    ///
    /// **CAUTION**: the first value in the vector is unused, since the segments are
    /// counted starting from 1.
    ///
    /// # Example
    /// ```
    /// # use map_folding::MapFolding;
    /// let f = MapFolding::from_stack(&[4, 5, 3, 1, 2]).unwrap();
    ///
    /// assert_eq!(f.positions()[3], 2);
    /// assert_eq!(f.positions()[2], 4);
    /// assert_eq!(f.positions()[4], 0);
    /// ```
    pub fn positions(&self) -> Vec<usize> {
        let n = self.indices.len();
        let mut positions: Vec<usize> = vec![0; n + 1]; // 0th value is unused
        for (index, segment) in self.indices.iter().enumerate() {
            positions[*segment as usize] = index;
        }
        positions
    }

    pub fn is_foldable(&self) -> bool {
        let n = self.indices.len();
        let pos = self.positions();

        let mut odd_joints: Vec<(usize, usize)> = vec![];
        for odd_i in (1..n).step_by(2) {
            let cur_pos = pos[odd_i];
            let next_pos = pos[odd_i + 1];
            for joint in &odd_joints {
                let cur_inside = joint.0 < cur_pos && joint.1 > cur_pos;
                let next_inside = joint.0 < next_pos && joint.1 > next_pos;
                let joints_intersect = cur_inside ^ next_inside;
                if joints_intersect {
                    return false;
                }
            }
            let new_joint = (min(cur_pos, next_pos), max(cur_pos, next_pos));
            odd_joints.push(new_joint);
        }

        let mut even_joints: Vec<(usize, usize)> = vec![];
        for even_i in (2..n).step_by(2) {
            let cur_pos = pos[even_i];
            let next_pos = pos[even_i + 1];
            for joint in &even_joints {
                let cur_inside = joint.0 < cur_pos && joint.1 > cur_pos;
                let next_inside = joint.0 < next_pos && joint.1 > next_pos;
                let joints_intersect = cur_inside ^ next_inside;
                if joints_intersect {
                    return false;
                }
            }
            let new_joint = (min(cur_pos, next_pos), max(cur_pos, next_pos));
            even_joints.push(new_joint);
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_stack_8_zigzag() {
        let stack = [1, 2, 3, 4, 5, 6, 7, 8];
        let created = MapFolding::from_stack(&stack).expect("failed to parse stack");
        let expected = vec![1, 2, 3, 4, 5, 6, 7, 8];
        assert_eq!(created.indices, expected);
    }

    #[test]
    fn from_stack_invalid_indices() {
        let indices = [2, 5, 4, 3];
        assert_eq!((), MapFolding::from_stack(&indices).unwrap_err());
    }

    #[test]
    fn from_stack_invalid_indices_2() {
        let indices = [0, 1, 2, 3, 4];
        assert_eq!((), MapFolding::from_stack(&indices).unwrap_err());
    }

    #[test]
    fn from_stack_missing_indices() {
        let indices = [5, 3, 1, 2, 1];
        assert_eq!((), MapFolding::from_stack(&indices).unwrap_err());
    }

    #[test]
    fn from_stack_extra_indices() {
        let indices = [5, 3, 4, 5, 2, 1];
        assert_eq!((), MapFolding::from_stack(&indices).unwrap_err());
    }

    #[test]
    fn is_foldable_8_zigzag() {
        let zigzag =
            MapFolding::from_stack(&[1, 2, 3, 4, 5, 6, 7, 8]).expect("failed to parse stack");
        assert!(zigzag.is_foldable());
    }

    #[test]
    fn is_foldable_impossible_stack() {
        let f = MapFolding::from_stack(&[5, 4, 3, 6, 2, 7, 1, 8]).expect("failed to parse stack");
        assert!(!f.is_foldable());
    }
}
