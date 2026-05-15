use crate::value::{GCValue, Value, ValueExt};

pub struct ListIter {
    current: GCValue,
}

impl ListIter {
    pub(crate) fn new(list: GCValue) -> Self {
        Self { current: list }
    }
}

impl Iterator for ListIter {
    type Item = GCValue;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.is_null() {
            return None;
        }

        if let Value::Pair { car, cdr, is_list: true } = self.current.as_ref() {
            let item = car.clone();
            self.current = cdr.clone();
            Some(item)
        } else {
            // If we encounter a non-list pair, we treat it as the last item in the list and then end the iteration.
            let item = self.current.clone();
            self.current = Value::Null.into_gc_value();
            Some(item)
        }
    }
}
