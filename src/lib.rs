///A tuple containing Id 0 the natural language name of the place
///id 1 the id associated with that name.
#[repr(C)]
pub struct PlaceNameId{
    pub name: String,
    pub id: String,
}