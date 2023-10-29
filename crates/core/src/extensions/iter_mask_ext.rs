pub trait IterMaskExt: Iterator {
    // An iterator that maps the value of the iterator if the `mask.next()` yields 1 and `destination.next()` otherwise.

    fn masked_map<Mask, Destination, Func>(
        self,
        mask: Mask,
        dest: Destination,
        func: Func,
    ) -> MaskedMap<Self, Mask, Destination, Func>
    where
        Self: Sized,
        Mask: Iterator,
        Destination: Iterator,
        Func: FnMut(Self::Item) -> Destination::Item,
    {
        MaskedMap {
            values: self,
            mask,
            dest,
            func,
        }
    }
}

pub struct MaskedMap<I, Mask, Destination, Func>
where
    I: Iterator,
    Mask: Iterator,
    Destination: Iterator,
    Func: FnMut(I::Item) -> Destination::Item,
{
    values: I,
    mask: Mask,
    dest: Destination,
    func: Func,
}

impl<I, Mask, Destination, Func> Iterator for MaskedMap<I, Mask, Destination, Func>
where
    I: Iterator,
    Mask: Iterator,
    Mask::Item: PartialEq<u64>,
    Destination: Iterator,
    Func: FnMut(I::Item) -> Destination::Item,
{
    type Item = Destination::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let iter_item = self.values.next()?;
        let dest_item = self.dest.next()?;

        self.mask.next().map(|m| {
            if m == 1 {
                (self.func)(iter_item)
            } else {
                dest_item
            }
        })
    }
}

impl<I: Iterator> IterMaskExt for I {}
