#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    match (_first_list, _second_list) {
        (_first_list, _second_list) if _first_list == _second_list => Comparison::Equal,
        (_first_list, _second_list) if is_sublist(_first_list, _second_list) => Comparison::Sublist,
        (_first_list, _second_list) if is_sublist(_second_list, _first_list) => Comparison::Superlist,
        _ => Comparison::Unequal,
    }
}

pub fn is_sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> bool {
    let _first_list_len = _first_list.len();
    let _second_list_len = _second_list.len();

    if _first_list_len > _second_list_len {
        return false;

    }

    for i in 0..=_second_list_len - _first_list_len {
        if _first_list == &_second_list[i..i + _first_list_len] {
            return true;
        }
    }
    false
}   
