#![no_std]

extern crate alloc;

mod macroman;
mod str_255;

pub use str_255::Str255;

#[cfg(test)]
mod tests {
    use super::*;
}
