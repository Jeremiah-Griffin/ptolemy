
use linked_hash_map::LinkedHashMap;

///if is_err map value must be empty.
/// if not, the map is keyed by place names, and the values are that place's ID.
///
/// However, recognizing that a programmer may reasonably err, the constructor of
/// PLaceNameId *wilL* take a value even if it is supposed to return an error.
/// I'd rather wasted bandwidth than a failed assertion.
pub struct PlaceNameId{
    pub is_err: bool,
    pub names_ids: LinkedHashMap<String, String>,
}

impl PlaceNameId{
    ///Check struct documentation explaining why buf isn't an Option<T>
    pub fn new(is_err: bool, buf: &[(String, String)]) -> PlaceNameId {
        PlaceNameId{
            is_err,
            names_ids: LinkedHashMap::from_iter(buf.iter().map(|(n, i)| (n.to_string(), i.to_string()))),
        }
    }
}

