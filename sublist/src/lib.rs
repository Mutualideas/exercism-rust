#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    //todo!("Determine if the first list is equal to, sublist of, superlist of or unequal to the second list.");
    if _first_list == _second_list {
        return Comparison::Equal;
    } else if is_sublist(_first_list, _second_list) {
        return Comparison::Sublist;
    } else if is_sublist(_second_list, _first_list) {
        return Comparison::Superlist;
    } else {
        Comparison::Unequal
    }
}

fn is_sublist<T: PartialEq>(sublist: &[T], superlist: &[T]) -> bool {
    if sublist.is_empty() {
        return true;
    }
    superlist
        .windows(sublist.len())
        .any(|window| window == sublist)
}
