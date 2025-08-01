use sea_orm::DatabaseConnection;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct AddRoom {
    pub number: i32,
    pub name: String,
    pub capacity: i32,
    pub max_capacity: Option<i32>,
    pub is_apartment: Option<bool>,
    pub has_kitchen: Option<bool>,
    pub bedrooms: Option<i32>,
}

impl AddRoom {
    /// Checks:
    /// - number for duplicates
    /// - capacity != 0
    /// - max capacity bigger or smaller than capacity
    /// - bedrooms bigger than -1
    pub async fn validate(&self, conn: &DatabaseConnection) -> Result<bool, String> {
        let mut is_valid = true;

        // capacity
        if self.capacity <= 0 {
            return Ok(false);
        }

        // max capacity
        if let Some(max) = self.max_capacity {
            if max != 0 && max < self.capacity {
                return Ok(false);
            }
        }

        //bedrooms
        if let Some(bedrooms) = self.bedrooms {
            if bedrooms < 0 {
                return Ok(false);
            }
        }

        // duplicate number
        /*let result = room::Entity::find()
            .filter(
                Condition::all()
                    .add(room::Column::Number.eq(self.number))
                    .add(room::Column::Valid.eq(true)),
            )
            .all(conn)
            .await;
        match result {
            Ok(val) => {
                is_valid = val.len() == 0;
            }
            Err(val) => {
                return Err(val.to_string());
            }
        }*/

        Ok(is_valid)
    }
}
