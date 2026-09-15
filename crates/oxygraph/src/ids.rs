use num_traits::{PrimInt, Unsigned};
use oxygraph_derive::serde_feature;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[serde_feature]
pub struct VertexId<T: Unsigned + PrimInt> {
    id: T,
}
impl<T: Unsigned + PrimInt> VertexId<T> {
    pub fn new(id: usize) -> Self {
        Self {
            id: T::from(id).expect("Must always be convertible from usize"),
        }
    }

    pub fn id(&self) -> usize {
        self.id
            .to_usize()
            .expect("Must always be convertible to usize")
    }

    pub fn max() -> T {
        T::max_value()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[serde_feature]
pub struct EdgeView<'a, E, I>
where
    I: Unsigned + PrimInt,
{
    pub from: VertexId<I>,
    pub to: VertexId<I>,
    pub data: &'a E,
}
impl<'a, E, I> EdgeView<'a, E, I>
where
    I: Unsigned + PrimInt,
{
    pub fn new(from: VertexId<I>, to: VertexId<I>, data: &'a E) -> Self {
        Self { from, to, data }
    }
}
