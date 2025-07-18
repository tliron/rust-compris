use super::generator::*;

use {proc_macro2::*, quote::*};

impl StructGenerator {
    /// Generate field handler.
    pub fn generate_handle_field(
        &self,
        field: &Field,
        key: &TokenStream,
        annotated_parameter: &TokenStream,
    ) -> TokenStream {
        let handle_annotations = if let Some(annotations_field_name) = &self.annotations_field {
            let quoted_field_name = field.name.to_string().to_token_stream();
            quote! {
                if #annotated_parameter::has_annotations()
                    && let ::std::option::Option::Some(annotations) = ::compris::annotate::Annotated::get_annotations(value)
                {
                    resolved.#annotations_field_name.insert(
                        #quoted_field_name.into(),
                        annotations.clone(),
                    );
                }
            }
        } else {
            TokenStream::new()
        };

        let handle_null = Self::generate_handle_null(field, false);

        let handle_required = if field.attribute.required {
            quote! {
                else {
                    errors.give(
                        ::compris::annotate::Annotated::with_annotations_from(
                            ::compris::resolve::MissingRequiredKeyError::new(key.into()),
                            self,
                        )
                    )?;
                }
            }
        } else {
            TokenStream::new()
        };

        let field_name = &field.name;

        quote! {
            let key = #key;
            if let ::std::option::Option::Some(value) = self.into_get(key) {
                #handle_annotations
                #handle_null
                if let ::std::option::Option::Some(value) = ::compris::resolve::Resolve::resolve_with_errors(value, errors)? {
                    resolved.#field_name = value;
                }
            }
            #handle_required
        }
    }
}
