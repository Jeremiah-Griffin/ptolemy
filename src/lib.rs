use linked_hash_map::LinkedHashMap;

///if is_err map value must be empty.
/// if not, the map is keyed by place names, and the values are that place's ID.
pub struct PlaceNameId{
    pub is_err: bool,
    pub names_ids: LinkedHashMap<String, String>,
}

