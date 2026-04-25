use syn::{Path, VisRestricted, Visibility, parse_quote, punctuated::Punctuated, token::PathSep};

pub trait VisiblyComparable {
    fn greated_common_factor(&self, other: &Self) -> Self where Self: Sized;
}

impl VisiblyComparable for Visibility {
    fn greated_common_factor(&self, other: &Self) -> Self where Self: Sized {
        match (self, other) {
            (Self::Inherited, _) => other.clone(),
            (Self::Public(_), _) => self.clone(),
            (_, Self::Public(_)) => other.clone(),
            (_, Self::Inherited) => self.clone(),
            (Self::Restricted(vr_self), Self::Restricted(vr_other)) => {
                // get greatest common factor for each path.
               match greatest_common_path(&vr_self.path, &vr_other.path) {
                    Some(path) => Visibility::Restricted(VisRestricted { 
                        pub_token: Default::default(), 
                        paren_token: Default::default(), 
                        in_token: match path.get_ident().is_some() {
                            true => None,
                            false => Some(Default::default()),
                        }, 
                        path: Box::new(path) 
                    }),
                    None => parse_quote! { crate },
                }
            },
        }
    }
}

/// Returns true if the path is relative (starts with `crate`, `self`, or `super`)
fn is_relative(path: &Path) -> bool {
    if let Some(first) = path.segments.first() {
        let ident = first.ident.to_string();
        matches!(ident.as_str(), "crate" | "self" | "super")
    } else {
        false
    }
}

/// Returns the greatest common path prefix of two `syn::Path` objects.
///
/// Rules:
/// - Leading `::` paths are not supported (caller must not pass them).
/// - If one path is relative (`crate`/`self`/`super`) and the other is absolute
///   (an external crate name like `std`), the absolute path is returned because
///   the two cannot share a meaningful common prefix.
/// - Otherwise, the longest shared segment prefix is returned.
/// - If there is no common prefix at all, `None` is returned.
pub fn greatest_common_path(path1: &Path, path2: &Path) -> Option<Path> {
    let rel1 = is_relative(path1);
    let rel2 = is_relative(path2);

    // One relative, one absolute → they can never share a prefix.
    // Return the absolute one per the spec.
    if rel1 != rel2 {
        if rel1 {
            // path2 is absolute
            return Some(path2.clone());
        } else {
            // path1 is absolute
            return Some(path1.clone());
        }
    }

    // Both relative or both absolute: find the longest common segment prefix.
    let segs1: Vec<_> = path1.segments.iter().collect();
    let segs2: Vec<_> = path2.segments.iter().collect();

    let common_len = segs1
        .iter()
        .zip(segs2.iter())
        .take_while(|(s1, s2)| {
            // Compare only the identifier; ignore any generic arguments on the
            // segments for path-prefix purposes.
            s1.ident == s2.ident
        })
        .count();

    if common_len == 0 {
        return None;
    }

    // Build a new Path from the common segments.
    let mut segments: Punctuated<syn::PathSegment, PathSep> = Punctuated::new();
    for seg in segs1.iter().take(common_len) {
        segments.push((*seg).clone());
    }

    Some(Path {
        leading_colon: None, // spec says no leading colons
        segments,
    })
}


#[cfg(test)]
mod tests {
    use super::*;
    use quote::ToTokens;
    use syn::parse_quote;

    fn assert_equal_paths(path1: &Path, path2: &Path) {
        assert_eq!(
            path1.to_token_stream().to_string(),
            path2.to_token_stream().to_string(),
        )
    }

    #[test]
    fn test_common_prefix_basic() {
        let p1: Path = parse_quote! { crate::mod1::submod };
        let p2: Path = parse_quote! { crate::mod1 };
        let result = greatest_common_path(&p1, &p2).unwrap();
        let expected: Path = parse_quote! { crate::mod1 };
        assert_equal_paths(&result, &expected);
    }

    #[test]
    fn test_common_is_just_crate() {
        let p1: Path = parse_quote! { crate::mod1::submod };
        let p2: Path = parse_quote! { crate };
        let result = greatest_common_path(&p1, &p2).unwrap();
        let expected: Path = parse_quote! { crate };
        assert_equal_paths(&result, &expected);
    }

    #[test]
    fn test_diverging_relative_paths() {
        // crate::a::b vs crate::c::d → common is just `crate`
        let p1: Path = parse_quote! { crate::a::b };
        let p2: Path = parse_quote! { crate::c::d };
        let result = greatest_common_path(&p1, &p2).unwrap();
        let expected: Path = parse_quote! { crate };
        assert_equal_paths(&result, &expected);
    }

    #[test]
    fn test_relative_vs_absolute_returns_absolute() {
        // crate-relative vs external absolute → return the absolute one
        let p1: Path = parse_quote! { crate::mod1 };
        let p2: Path = parse_quote! { std::collections };
        let result = greatest_common_path(&p1, &p2).unwrap();
        let expected: Path = parse_quote! { std::collections };
        assert_equal_paths(&result, &expected);
    }

    #[test]
    fn test_absolute_vs_relative_returns_absolute() {
        let p1: Path = parse_quote! { serde::de };
        let p2: Path = parse_quote! { crate::types };
        let result = greatest_common_path(&p1, &p2).unwrap();
        let expected: Path = parse_quote! { serde::de };
        assert_equal_paths(&result, &expected);
    }

    #[test]
    fn test_no_common_prefix_returns_none() {
        // Two absolute paths with completely different roots
        let p1: Path = parse_quote! { std::vec };
        let p2: Path = parse_quote! { serde::de };
        assert!(greatest_common_path(&p1, &p2).is_none());
    }

    #[test]
    fn test_identical_paths() {
        let p1: Path = parse_quote! { crate::foo::bar };
        let p2: Path = parse_quote! { crate::foo::bar };
        let result = greatest_common_path(&p1, &p2).unwrap();
        let expected: Path = parse_quote! { crate::foo::bar };
        assert_equal_paths(&result, &expected);
    }
}