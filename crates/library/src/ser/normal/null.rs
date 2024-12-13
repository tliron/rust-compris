use super::super::super::normal::*;

use serde::ser::*;

//
// Null
//

impl Serialize for Null {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_unit()
    }
}
