use std::{marker::PhantomData, mem::MaybeUninit};

use serde::{de::Visitor, ser::SerializeTuple, Deserialize, Deserializer, Serialize, Serializer};

struct ArrayVisitor<T, const N: usize> {
    _phantom: PhantomData<T>,
}

impl<'de, T, const N: usize> Visitor<'de> for ArrayVisitor<T, N>
where
    T: Deserialize<'de>,
{
    type Value = FixedSizeArray<T, N>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "an array of size {}", N)
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let mut arr: [MaybeUninit<T>; N] = unsafe { MaybeUninit::uninit().assume_init() };

        let mut place_iter = arr.iter_mut();
        let mut cnt_filled = 0;
        let err = loop {
            match (seq.next_element(), place_iter.next()) {
                (Ok(Some(val)), Some(place)) => *place = MaybeUninit::new(val),
                // no error, we're done
                (Ok(None), None) => break None,
                // error from serde, propagate it
                (Err(e), _) => break Some(e),
                // lengths do not match, report invalid_length
                (Ok(None), Some(_)) | (Ok(Some(_)), None) => {
                    break Some(serde::de::Error::invalid_length(cnt_filled, &self))
                }
            }
            cnt_filled += 1;
        };
        if let Some(err) = err {
            if std::mem::needs_drop::<T>() {
                for elem in arr.into_iter().take(cnt_filled) {
                    unsafe {
                        elem.assume_init();
                    }
                }
            }
            return Err(err);
        }

        let ret: [T; N] = unsafe { std::mem::transmute_copy(&arr) };
        std::mem::forget(arr);

        Ok(FixedSizeArray::from(ret))
    }
}

#[derive(Debug)]
pub(crate) struct FixedSizeArray<T, const N: usize> {
    value: [T; N],
}

impl<T, const N: usize> From<[T; N]> for FixedSizeArray<T, N> {
    fn from(value: [T; N]) -> Self {
        Self { value }
    }
}

impl<'de, T, const N: usize> Deserialize<'de> for FixedSizeArray<T, N>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_tuple(
            N,
            ArrayVisitor {
                _phantom: PhantomData,
            },
        )
    }
}

impl<T, const N: usize> Serialize for FixedSizeArray<T, N>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_tuple(N)?;
        for value in &self.value {
            s.serialize_element(value)?;
        }
        s.end()
    }
}
