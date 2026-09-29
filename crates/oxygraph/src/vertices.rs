use std::{fmt::Debug, hash::Hash, num::NonZero};

#[cfg(feature = "serde")]
use oxygraph_derive::serde_feature;

#[cfg_attr(feature = "serde", serde_feature)]
pub trait UnsignedId: Debug + Copy + Eq + Hash {
    type NonZero: Debug + Copy + Eq + Hash + Ord;
    fn to_nz(v: usize) -> Self::NonZero;
    fn from_nz(nz: Self::NonZero) -> usize;
}

macro_rules! impl_unsigned_id {
    ($($t:ty),*) => {
        $(
            impl UnsignedId for $t {
                type NonZero = NonZero<$t>;
                #[inline(always)] fn to_nz(v: usize) -> Self::NonZero {
                    NonZero::new((v as $t).wrapping_add(1)).expect("ID overflow")
                }
                #[inline(always)] fn from_nz(nz: Self::NonZero) -> usize {
                    (nz.get() - 1) as usize
                }
            }
        )*
    };
}

impl_unsigned_id!(u8, u16, u32, u64, u128, usize);

#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", serde_feature)]
pub struct VertexId<I: UnsignedId>(pub I::NonZero);

impl<I: UnsignedId> VertexId<I> {
    pub fn new(id: usize) -> Self {
        Self(I::to_nz(id))
    }

    pub fn id(&self) -> usize {
        I::from_nz(self.0)
    }
}

// Not derived: `#[derive(PartialOrd, Ord)]` would bound `I`, not the `I::NonZero` field it
// actually compares, and fail to compile: ordered by the same dense index `id()` returns.
impl<I: UnsignedId> PartialOrd for VertexId<I> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<I: UnsignedId> Ord for VertexId<I> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}
