use ::bstr::{BStr, ByteSlice};

use super::*;

impl<'src> Input<'src> for &'src BStr {
    type Cursor = usize;
    type Span = SimpleSpan<usize>;

    type Token = char;
    type MaybeToken = char;

    type Cache = Self;

    #[inline]
    fn begin(self) -> (Self::Cursor, Self::Cache) {
        (0, self)
    }

    #[inline]
    fn cursor_location(cursor: &Self::Cursor) -> usize {
        *cursor
    }

    #[inline(always)]
    unsafe fn next_maybe(
        this: &mut Self::Cache,
        cursor: &mut Self::Cursor,
    ) -> Option<Self::MaybeToken> {
        if *cursor < this.len() {
            // SAFETY: `cursor < self.len()` above guarantees cursor is in-bounds
            //         We only ever return cursors that are at a character boundary
            let c = this
                .get_unchecked(*cursor..)
                .chars()
                .next()
                .unwrap_unchecked();
            *cursor += c.len_utf8();
            Some(c)
        } else {
            None
        }
    }

    #[inline(always)]
    unsafe fn span(_this: &mut Self::Cache, range: Range<&Self::Cursor>) -> Self::Span {
        (*range.start..*range.end).into()
    }
}

impl<'src> ExactSizeInput<'src> for &'src BStr {
    #[inline(always)]
    unsafe fn span_from(this: &mut Self::Cache, range: RangeFrom<&Self::Cursor>) -> Self::Span {
        (*range.start..this.len()).into()
    }
}

impl<'src> ValueInput<'src> for &'src BStr {
    #[inline(always)]
    unsafe fn next(this: &mut Self::Cache, cursor: &mut Self::Cursor) -> Option<Self::Token> {
        Self::next_maybe(this, cursor)
    }
}

impl Sealed for &BStr {}
impl<'src> StrInput<'src> for &'src BStr {
    #[doc(hidden)]
    fn stringify(slice: Self::Slice) -> String {
        slice.to_string()
    }
}

impl<'src> SliceInput<'src> for &'src BStr {
    type Slice = &'src BStr;

    #[inline(always)]
    fn full_slice(this: &mut Self::Cache) -> Self::Slice {
        *this
    }

    #[inline(always)]
    unsafe fn slice(this: &mut Self::Cache, range: Range<&Self::Cursor>) -> Self::Slice {
        &this[*range.start..*range.end]
    }

    #[inline(always)]
    unsafe fn slice_from(this: &mut Self::Cache, from: RangeFrom<&Self::Cursor>) -> Self::Slice {
        &this[*from.start..]
    }
}
