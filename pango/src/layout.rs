// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;

use crate::{LayoutLine, LayoutRun};

// rustdoc-stripper-ignore-next
/// The result of [`LayoutLine::x_to_index`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct HitPosition {
    index: i32,
    trailing: i32,
    is_inside: bool,
}

impl HitPosition {
    // rustdoc-stripper-ignore-next
    /// The UTF-8 byte offset of the grapheme closest to the position.
    ///
    /// This position is relative to the start of the [`Layout`]'s text.
    ///
    /// [`Layout`]: crate::Layout
    pub fn index(self) -> i32 {
        self.index
    }

    // rustdoc-stripper-ignore-next
    /// The codepoint within the grapheme of the position.
    ///
    /// This will always be either `0`, or the number of `char`s (*not bytes!*)
    /// in the grapheme. This represents whether the user clicked near the start
    /// of the grapheme or near the end; this is important for things like
    /// resolving cursor positions.
    pub fn trailing(self) -> i32 {
        self.trailing
    }

    // rustdoc-stripper-ignore-next
    /// Whether or not the position was within the bounds of the line.
    ///
    /// If this is `false`, then `index` and `trailing` will always resolve
    /// to either the very first or the very last position in the line; this
    /// behaviour is dependent on the line's resolved writing direction.
    pub fn is_inside(self) -> bool {
        self.is_inside
    }
}

impl LayoutLine {
    // rustdoc-stripper-ignore-next
    /// The byte index of the start of this line into the text used to create
    /// the source [`Layout`].
    ///
    /// [`Layout`]: crate::Layout
    #[cfg(not(any(feature = "v1_50", feature = "dox")))]
    #[cfg_attr(feature = "dox", doc(not(cfg(feature = "v1_50"))))]
    pub fn start_index(&self) -> i32 {
        unsafe { (*self.as_ptr()).start_index }
    }

    // rustdoc-stripper-ignore-next
    /// The length of this line's text, in bytes.
    #[cfg(not(any(feature = "v1_50", feature = "dox")))]
    #[cfg_attr(feature = "dox", doc(not(cfg(feature = "v1_50"))))]
    pub fn length(&self) -> i32 {
        unsafe { (*self.as_ptr()).length }
    }

    #[doc(alias = "pango_layout_line_runs")]
    pub fn runs(&self) -> Vec<LayoutRun> {
        unsafe { FromGlibPtrContainer::from_glib_none((*self.as_ptr()).runs) }
    }
    #[doc(alias = "pango_layout_line_x_to_index")]
    pub fn x_to_index(&self, x_pos: i32) -> HitPosition {
        let mut index = 0;
        let mut trailing = 0;

        let is_inside = unsafe {
            from_glib(ffi::pango_layout_line_x_to_index(
                self.to_glib_none().0,
                x_pos,
                &mut index,
                &mut trailing,
            ))
        };

        HitPosition {
            index,
            trailing,
            is_inside,
        }
    }
}
