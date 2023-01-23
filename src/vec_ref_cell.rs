use std::cell::RefCell;

pub(crate) struct VecRefCell<T> {
    values: RefCell<Vec<T>>,
}

impl<T> VecRefCell<T> {
    pub(crate) fn new() -> Self {
        VecRefCell {
            values: RefCell::new(Vec::new()),
        }
    }

    pub(crate) fn last(&self) -> Option<&T> {
        unsafe { self.values.try_borrow_unguarded().unwrap().last() }
    }

    pub(crate) fn push(&self, value: T) {
        self.values.borrow_mut().push(value)
    }
}

impl<T, const N: usize> From<[T; N]> for VecRefCell<T> {
    fn from(values: [T; N]) -> Self {
        let vec_ref_cell = Self::new();

        for value in values {
            vec_ref_cell.push(value);
        }

        vec_ref_cell
    }
}
