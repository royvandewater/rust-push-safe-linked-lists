use std::cell::RefCell;

fn main() {
    with_i32();
    with_string();
}

fn with_i32() {
    let vec = VecRefCell::from([1, 2, 3]);

    // This is OK
    let last: &i32 = vec.last().unwrap();
    vec.push(4);
    println!("last after push: {}", last);

    // This is somehow also OK?
    let last: &i32 = vec.last().unwrap();
    _ = vec.pop();
    println!("last after pop: {}", last);
}

fn with_string() {
    let vec = VecRefCell::from(["a", "b", "c"]);

    // This is OK
    let last: &str = vec.last().unwrap();
    vec.push("d");
    println!("last after push: {}", last);

    // This is somehow also OK?
    let last: &str = vec.last().unwrap();
    _ = vec.pop();
    println!("last after pop: {}", last);
}

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

    pub(crate) fn pop(&self) -> Option<T> {
        self.values.borrow_mut().pop()
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
