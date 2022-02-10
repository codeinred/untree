use super::{PathKind::*, *};
use std::io::{BufRead, Lines};
use std::mem::replace;
use std::path::{Path, PathBuf};

pub(crate) enum TreeIterator<BR, F, T>
where
    F: FnMut(Result<(&Path, PathKind)>) -> T,
{
    Good {
        lines: Lines<BR>,
        path: PathBuf,
        depth: i32,
        old_depth: i32,
        filename: PathBuf,
        func: F,
    },
    Bad {
        func: F,
        error: Error,
    },
    Empty,
}

impl<BR: BufRead, F, T> TreeIterator<BR, F, T>
where
    F: FnMut(Result<(&Path, PathKind)>) -> T,
{
    fn step(self) -> (Self, Option<T>) {
        use TreeIterator::*;
        match self {
            Empty => (Empty, None),
            Bad { mut func, error } => return (Empty, Some(func(Err(error)))),
            Good {
                mut lines,
                mut path,
                mut depth,
                mut old_depth,
                mut filename,
                mut func,
            } => {
                if depth <= old_depth {
                    for _ in depth..old_depth {
                        path.pop();
                    }
                    path.set_file_name(&filename);
                } else {
                    path.push(&filename);
                }
                old_depth = depth;
                match lines.next() {
                    Some(Ok(line)) => {
                        if line.is_empty() {
                            let result = (path.as_path(), FilePath);
                            return (Empty, Some((func)(Ok(result))));
                        }
                        let (new_depth, name) = get_entry(line.as_ref());
                        let kind = if new_depth <= depth {
                            FilePath
                        } else {
                            Directory
                        };
                        depth = new_depth;
                        filename = name.into();
                        let result = func(Ok((path.as_path(), kind)));
                        (
                            Good {
                                lines,
                                path,
                                depth,
                                old_depth,
                                filename,
                                func,
                            },
                            Some(result),
                        )
                    }
                    Some(Err(err)) => {
                        let result = func(Ok((path.as_path(), FilePath)));
                        (
                            Bad {
                                func: func,
                                error: err.into(),
                            },
                            Some(result),
                        )
                    }
                    None => {
                        let result = (path.as_path(), FilePath);
                        (Empty, Some((func)(Ok(result))))
                    }
                }
            }
        }
    }
}
impl<BR: BufRead, F, T> Iterator for TreeIterator<BR, F, T>
where
    F: FnMut(Result<(&Path, PathKind)>) -> T,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        use TreeIterator::Empty;
        let (val, result) = replace(self, Empty).step();
        *self = val;
        result
    }
}
