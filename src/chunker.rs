pub fn chunk_pdf(data: &[u8], chunk_size: usize) -> Vec<Vec<u8>> {
    if chunk_size == 0 {
        return vec![data.to_vec()];
    }
    
    data.chunks(chunk_size)
        .map(|chunk| chunk.to_vec())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_chunk_pdf_exact_chunks() {
        let data = vec![1, 2, 3, 4, 5, 6];
        let chunks = chunk_pdf(&data, 2);
        
        assert_eq!(chunks.len(), 3);
        assert_eq!(chunks[0], vec![1, 2]);
        assert_eq!(chunks[1], vec![3, 4]);
        assert_eq!(chunks[2], vec![5, 6]);
    }
    
    #[test]
    fn test_chunk_pdf_uneven_chunks() {
        let data = vec![1, 2, 3, 4, 5];
        let chunks = chunk_pdf(&data, 2);
        
        assert_eq!(chunks.len(), 3);
        assert_eq!(chunks[0], vec![1, 2]);
        assert_eq!(chunks[1], vec![3, 4]);
        assert_eq!(chunks[2], vec![5]);
    }
    
    #[test]
    fn test_chunk_pdf_zero_size() {
        let data = vec![1, 2, 3];
        let chunks = chunk_pdf(&data, 0);
        
        assert_eq!(chunks.len(), 1);
        assert_eq!(chunks[0], vec![1, 2, 3]);
    }
    
    #[test]
    fn test_chunk_pdf_large_chunk_size() {
        let data = vec![1, 2, 3];
        let chunks = chunk_pdf(&data, 10);
        
        assert_eq!(chunks.len(), 1);
        assert_eq!(chunks[0], vec![1, 2, 3]);
    }
}