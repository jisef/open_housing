use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct AddRoom {
    pub number: i32,
    pub name: String,
    pub capacity: i32,
    pub maxCapacity: Option<i32>,
    pub isApartment: Option<bool>,
    pub hasKitchen: Option<bool>,
    pub bedrooms: Option<i32>,
}

