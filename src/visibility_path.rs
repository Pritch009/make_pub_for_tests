use syn::{Path, VisRestricted, Visibility, parse::{Parse, ParseStream}, parse_quote, token::In};

pub struct VisibilityPath {
    pub r#in: Option<In>,
    pub path: Path,
}

impl Parse for VisibilityPath {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut r#in = None;
        

        if input.peek(In) {
            r#in = Some(input.parse::<In>()?);
        }

        let path = input.parse::<Path>()?;
        if (path.segments.len() > 1 && r#in.is_none()) || path.leading_colon.is_some() {
            return Err(syn::Error::new_spanned(path, "Just like in `pub(...)`, modules that are not `crate`, `self`, `super` must be written like `pub(__in crate::module::submod__)` (https://doc.rust-lang.org/reference/visibility-and-privacy.html)"));
        }

        Ok(Self {
            r#in,
            path,
        })
    }
}

impl VisibilityPath {
    pub fn new_crate() -> Self {
        Self {
            r#in: None,
            path: parse_quote! { crate }
        }
    }
}

impl Into<Visibility> for VisibilityPath {
    fn into(self) -> Visibility {
        Visibility::Restricted(VisRestricted { 
            pub_token: Default::default(), 
            paren_token: Default::default(), 
            in_token: self.r#in, 
            path: Box::new(self.path) 
        })
    }
}