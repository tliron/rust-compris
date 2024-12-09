use super::super::*;

use serde::ser::*;

//
// Boolean
//

impl Serialize for Boolean {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_bool(self.value)
    }
}
