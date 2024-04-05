//! ## Overview of PKCS#7 Padding
//!
//! PKCS#7 padding is applicable to data of any length and is used to ensure that the length of the
//! data is a multiple of a specific block size. The value of each padding byte is the total number
//! of padding bytes added. For instance, if the block size is 16 bytes and the data is 13 bytes long,
//! three bytes of padding will be added, each with the value `03`.
//!
//! For our purposes, the block size will always be 16 bytes (or 128 bits) because that is the
//! blocksize of AES-192.

use std::iter::repeat;

/// This function inspects the padding and,
/// if valid, returns a subslice of the original data without the padding.
///
/// # Parameters
///
/// * `padded`: The data slice that may contain PKCS#7 padding.
/// * `block_size`: An optional block size used for padding. If specified, the function checks whether
///   the padding is valid for this specific block size:
///   - If `block_size` is `Some(size)`, the function returns a subslice only if the input is correctly
///     padded according to PKCS#7 rules for the given block size. This includes ensuring that the `padded`
///     slice's length is a multiple of `block_size` and that the padding length does not exceed `block_size`.
///   - If `block_size` is `None`, the function is more lenient and only checks the padding's validity
///     according to PKCS#7 rules, without enforcing a specific block size. This can be useful when the
///     block size is unknown or variable.
///
/// # Returns
///
/// * `Some(&[u8])`: A subslice of the original data without PKCS#7 padding if the padding is valid.
/// * `None`: If the padding is invalid, the block size is not respected, or other conditions prevent
///   unpadded data from being safely returned. This includes scenarios where the padding length is zero,
///   exceeds the block size, or the padding bytes do not conform to PKCS#7 rules.
///
/// # Examples
///
/// ```
/// use libproj3::decryption::unpad::unpad_slice;
///
/// let padded_data = [1, 2, 3, 4, 4, 4, 4, 4];
/// let unpadded = unpad_slice(&padded_data, Some(8)).unwrap();
/// assert_eq!(unpadded, &[1, 2, 3, 4]);
///
/// let invalid_padding = [1, 2, 3, 4, 5];
/// assert!(unpad_slice(&invalid_padding, Some(8)).is_none());
/// ```
pub fn unpad_slice(padded: &[u8], block_size: Option<usize>) -> Option<&[u8]> {
    let padding_len = get_padding_length(padded)?;

    if let Some(b) = block_size {
        if b > u8::MAX as _ || padded.len() % b != 0 || padding_len > b {
            return None;
        }
    }

    let (subslice, _) = padded.split_at(padded.len() - padding_len);
    Some(subslice)
}

// pub(super) struct UnpadByValue<I, const B: usize>
// where
//     I: ExactSizeIterator<Item = [u8; B]> + FusedIterator,
// {
//     inner: I,
//     current_block: [u8; B],
//     blocks_left: usize,
//     index: usize,
// }

// impl<I, const B: usize> Iterator for UnpadByValue<I, B>
// where
//     I: ExactSizeIterator<Item = [u8; B]> + FusedIterator,
// {
//     type Item = u8;

//     fn next(&mut self) -> Option<Self::Item> {
//         if self.index >= B {
//             self.index = 0;
//             self.current_block = self.inner.next()?;
//             self.blocks_left -= 1;
//             debug_assert_eq!(self.blocks_left, self.inner.len());
//         }
//         if self.blocks_left == 0 {
//             if self.index >= B - get_padding_length(&self.current_block).unwrap() {
//                 self.index = B;
//                 return None;
//             }
//         }
//         let ret = self.current_block[self.index];
//         self.index += 1;
//         Some(ret)
//     }

//     fn size_hint(&self) -> (usize, Option<usize>) {
//         if self.blocks_left == 0 {
//             let padding_len = get_padding_length(&self.current_block).unwrap();
//             let len_left = (B - self.index).saturating_sub(padding_len);
//             (len_left, Some(len_left))
//         } else {
//             let lo = match (self.blocks_left - 1).checked_mul(B) {
//                 Some(x) => x.checked_add(B - self.index),
//                 None => None,
//             };
//             match lo {
//                 Some(x) => (x, x.checked_add(B - 1)),
//                 None => (usize::MAX, None),
//             }
//         }
//     }
// }

// impl<I, const B: usize> FusedIterator for UnpadByValue<I, B> where
//     I: ExactSizeIterator<Item = [u8; B]> + FusedIterator
// {
// }

/// Calculates the length of PKCS#7 padding in a given data slice.
///
/// # Arguments
///
/// * `in_slice` - A slice of bytes that potentially contains PKCS#7 padding.
///
/// # Returns
///
/// * `Some(usize)` - The length of the padding if the input slice is correctly padded
///   according to PKCS#7 rules. The padding length is determined based on the value
///   of the last byte in the slice, and the function verifies that all padding bytes
///   have the same value.
///
/// * `None` - If the input slice is not correctly padded according to PKCS#7 rules.
///   This includes cases where the last byte is 0 (indicating an invalid padding length),
///   or the padding bytes do not all have the same value as required by PKCS#7.
pub(super) fn get_padding_length(in_slice: &[u8]) -> Option<usize> {
    let last = *in_slice.last()?;
    println!("{}", last);
    if last == 0
        || in_slice
            .iter()
            .rev()
            .take(last as _)
            .ne(repeat(&last).take(last as _))
    {
        return None;
    }

    Some(last as _)
}
