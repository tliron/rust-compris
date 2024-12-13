/// True if any of the attributes contain an identifer in their path.
pub fn attributes_have_ident(attributes: &Vec<syn::Attribute>, name: &str) -> bool {
    for attribute in attributes {
        if attribute.path().is_ident(name) {
            return true;
        }
    }
    false
}
