use std::iter::{repeat, FusedIterator};

pub(super) struct UnpadByValue<I, const B: usize>
where
    I: ExactSizeIterator<Item = [u8; B]> + FusedIterator,
{
    inner: I,
    current_block: [u8; B],
    blocks_left: usize,
    index: usize,
}

impl<I, const B: usize> Iterator for UnpadByValue<I, B>
where
    I: ExactSizeIterator<Item = [u8; B]> + FusedIterator,
{
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= B {
            self.index = 0;
            self.current_block = self.inner.next()?;
            self.blocks_left -= 1;
            debug_assert_eq!(self.blocks_left, self.inner.len());
        }
        if self.blocks_left == 0 {
            if self.index >= B - get_padding_length(&self.current_block).unwrap() {
                self.index = B;
                return None;
            }
        }
        let ret = self.current_block[self.index];
        self.index += 1;
        Some(ret)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.blocks_left == 0 {
            let padding_len = get_padding_length(&self.current_block).unwrap();
            let len_left = (B - self.index).saturating_sub(padding_len);
            (len_left, Some(len_left))
        } else {
            let lo = match (self.blocks_left - 1).checked_mul(B){
                Some(x) => x.checked_add(B - self.index),
                None => None,
            };
            match lo {
                Some(x) => (x, x.checked_add(B - 1)),
                None => (usize::MAX, None),
            }
        }
    }
}

impl<I, const B: usize> FusedIterator for UnpadByValue<I, B> where
    I: ExactSizeIterator<Item = [u8; B]> + FusedIterator
{
}

fn get_padding_length(in_slice: &[u8]) -> Option<usize> {
    let last = *in_slice.last()?;
    if in_slice
        .into_iter()
        .rev()
        .take(last as _)
        .ne(repeat(&last).take(last as _))
    {
        return None;
    }

    Some(last as _)
}
