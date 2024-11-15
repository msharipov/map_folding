// Represents a 1-dimensional map folding.
pub struct MapFolding {
    // Representation of the folding in the standard stack form.
    // The number of the lowest segment is at index 0 in the vector.
    indices: Vec<u64>,
}

impl MapFolding {
    pub fn from_standard_stack(stack: &[u64]) -> Result<Self, ()> {
        let indices = Vec::from(stack);
        let n = indices.len();

        if *indices.iter().min().unwrap() != 1 || *indices.iter().max().unwrap() as usize != n {
            return Err(())
        }

        let mut present = vec![true; n + 1]; // 0th value is unused
        for segment in indices[0..n].iter() {
            present[*segment as usize] = true;
        }

        let all_present = present.iter().all(|&p| p);
        if all_present {
            return Ok(MapFolding { indices })
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
