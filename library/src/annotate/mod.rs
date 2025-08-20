mod annotate;
mod annotated;
mod annotations;
mod depict;
mod dyn_annotated;
mod errors;
mod label;
mod location;
mod macros;
mod span;
mod r#struct;
mod with;
mod without;

#[allow(unused_imports)]
pub use {
    crate::{impl_annotated, impl_dyn_annotated_error},
    annotate::*,
    annotated::*,
    annotations::*,
    depict::*,
    dyn_annotated::*,
    errors::*,
    label::*,
    location::*,
    span::*,
    r#struct::*,
    with::*,
    without::*,
};
