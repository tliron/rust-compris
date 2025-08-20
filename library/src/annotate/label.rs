use kutil::std::immutable::*;

//
// Label
//

/// Label annotation.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Label {
    /// Integer tag.
    Integer(i64),

    /// String tag.
    String(ByteString),
}
