use std::path::Path;

///这个函数自动忽略错误
#[allow(private_bounds)]
pub fn get_direntry_with_suffix<T: AsRef<Path>, S: Clone + Into<Suffix>>(
    path: T,
    suffix: S,
) -> Vec<walkdir::DirEntry> {
    // let v: Vec<_> = walk
    //     .into_iter()
    //     .filter_map(|f| {
    //         if f.is_ok() {
    //             if is_right_suffix(f.as_ref().unwrap().path(), suffix.clone()) {
    //                 Some(f.unwrap())
    //             } else {
    //                 None
    //             }
    //         } else {
    //             None
    //         }
    //     })
    //     .map(|d| d.clone().into_path())
    //     .collect();

    //忽视了可能的错误
    walkdir::WalkDir::new(path)
        .into_iter()
        .flatten()
        .filter(|d| is_right_suffix(d.path(), suffix.clone()))
        .collect()
}

#[allow(private_bounds)]
pub fn non_empty_entrys<T: AsRef<Path>, S: Clone + Into<Suffix>>(
    path: T,
    suffix: S,
) -> Option<Vec<walkdir::DirEntry>> {
    let v = get_direntry_with_suffix(path, suffix);
    if v.is_empty() {
        None
    } else {
        Some(v)
    }
}
#[derive(Clone)]
struct Suffix(String);

impl<'a> From<&'a str> for Suffix {
    fn from(value: &'a str) -> Self {
        Suffix(value.to_string())
    }
}
impl From<config::Suffix> for Suffix {
    fn from(value: config::Suffix) -> Self {
        Self(value.0)
    }
}
impl From<&config::Suffix> for Suffix {
    fn from(value: &config::Suffix) -> Self {
        Self(value.0.to_string())
    }
}

fn is_right_suffix<T: AsRef<Path>, S: Into<Suffix>>(path: T, suffix: S) -> bool {
    path.as_ref()
        .to_str()
        .unwrap()
        .ends_with(suffix.into().0.as_str())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn show() {
        dbg!(get_direntry_with_suffix("../../zzzzzzzz", ".yaml"));
    }

    #[test]
    fn t_is_right_suffix() {
        assert!(is_right_suffix("../../zzzzzzzz/test.txt", ".txt"));
    }
}
