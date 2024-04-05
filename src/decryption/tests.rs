#![cfg(test)]

use super::*;

mod test_get_padding_length {
    use super::*;
    use unpad::get_padding_length;

    /// Tests various conditions related to the padding length, including valid and invalid scenarios.
    #[test]
    fn padding_length_variations() {
        // Valid PKCS#7 padding
        let mut valid_padding = vec![1, 2, 3, 4, 4, 4, 4];
        assert_eq!(get_padding_length(&valid_padding), Some(4));

        // Data with single byte padding
        valid_padding = vec![1, 1];
        assert_eq!(get_padding_length(&valid_padding), Some(1));

        // Data consists only of padding bytes
        valid_padding = vec![2, 2];
        assert_eq!(get_padding_length(&valid_padding), Some(2));

        // Larger padding to ensure it scales correctly
        valid_padding = vec![0, 0, 0, 0, 0, 0, 0, 0, 8, 8, 8, 8, 8, 8, 8, 8];
        assert_eq!(get_padding_length(&valid_padding), Some(8));
    }

    /// Tests invalid padding scenarios where the padding is not according to PKCS#7 rules.
    #[test]
    fn invalid_padding_scenarios() {
        // Data without explicit padding
        let no_padding = vec![1, 2, 3, 4];
        assert_eq!(get_padding_length(&no_padding), None);

        // Last byte is 0, which is invalid in PKCS#7 padding
        let padding_length_zero = vec![1, 2, 3, 4, 0];
        assert_eq!(get_padding_length(&padding_length_zero), None);

        // Padding bytes do not match the last byte's value
        let mismatched_values = vec![1, 2, 3, 4, 3, 2];
        assert_eq!(get_padding_length(&mismatched_values), None);

        // Padding byte value suggests more padding than data, which is invalid
        let longer_than_data = vec![5];
        assert_eq!(get_padding_length(&longer_than_data), None);

        // Empty data slice should return None
        let empty_data: Vec<u8> = vec![];
        assert_eq!(get_padding_length(&empty_data), None);
    }
}

#[cfg(test)]
mod test_unpad_slice {
    use super::*;
    use unpad::unpad_slice;

    #[test]
    fn unpad_mixed() {
        let padded_data = vec![1, 2, 3, 4, 4, 4, 4, 4];

        let unpadded = unpad_slice(&padded_data, Some(4)).unwrap();
        assert_eq!(unpadded, &[1, 2, 3, 4]);

        // flexible block size
        let unpadded = unpad_slice(&padded_data, None).unwrap();
        assert_eq!(unpadded, &[1, 2, 3, 4]);

        // incorrect block size
        assert!(unpad_slice(&padded_data, Some(3)).is_none());
    }

    #[test]
    fn unpad_no_padding_when_optional() {
        let data_without_padding = vec![1, 2, 3, 4];
        assert!(unpad_slice(&data_without_padding, None).is_none());
    }

    #[test]
    fn unpad_invalid_padding() {
        let invalid_padding = vec![1, 2, 3, 4, 5];
        assert!(unpad_slice(&invalid_padding, Some(5)).is_none());
    }

    #[test]
    fn unpad_empty_data() {
        let empty_data = vec![];
        assert!(unpad_slice(&empty_data, Some(8)).is_none());
    }

    #[test]
    fn unpad_padding_longer_than_data() {
        let invalid_data = vec![5];
        assert!(unpad_slice(&invalid_data, Some(1)).is_none());
    }

    #[test]
    fn unpad_big_block_size() {
        let mut padded_data = vec![5; 255]; // 255 bytes in total, last 5 are padding

        let unpadded = unpad_slice(&padded_data, Some(255)).unwrap();
        assert_eq!(unpadded.len(), 250);

        padded_data.push(5); // 256 bytes in total, last 5 are padding
        assert!(unpad_slice(&padded_data, Some(256)).is_none());
    }
}
