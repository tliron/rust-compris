use super::super::super::normal::*;

use serde::ser::*;

//
// String
//

impl Serialize for Text {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&*self.value)
    }
}
