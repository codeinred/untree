#![doc = ::embed_doc_image::embed_image!("image1", "media/image1.png")]
#![doc = include_str!("../README.md")]

pub mod either;
pub mod errors;
pub mod functions;
pub mod more_context;
pub mod path_action;
pub mod types;

use either::either;
pub use errors::*;
pub use functions::*;
pub use more_context::*;
pub use path_action::*;
pub use types::*;

