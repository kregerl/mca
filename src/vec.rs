use serde::{
    ser::SerializeSeq,
    Deserialize, Serialize,
};

#[derive(Debug, Deserialize)]
pub struct Vec3D([f64; 3]);

#[derive(Debug, Deserialize)]
pub struct Vec2F([f32; 2]);

#[derive(Debug, Deserialize)]
pub struct Vec2I([i32; 2]);

#[derive(Debug, Deserialize)]
pub struct Vec3I([i32; 3]);

macro_rules! serialize_vec {
    ($typ:ty, $amt:expr) => {
        impl Serialize for $typ {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                let mut ser = serializer.serialize_seq(Some($amt))?;
                for value in self.0 {
                    ser.serialize_element(&value)?;
                }
                ser.end()
            }
        }
    };
}

macro_rules! impl_vec_getters {
    ( $typ:ty, $internal:ty, ($($var:ident),+ $(,)?)) => {
        impl $typ {
            pub fn new($($var: $internal,)*) -> Self {
                Self ([$($var,)*])
            }
        }
        impl_vec_getters!(@step 0usize, $typ, $internal, $($var,)*);
    };

    (@step $index:expr, $typ:ty, $internal:ty, $($var:ident,)*) => {
        $(impl_vec_getters!(@step $index + 1usize, $typ, $internal, $var);)*
    };

    (@step $index:expr, $typ:ty, $internal:ty, $var:ident) => {
        impl $typ {
            pub fn $var(&self) -> $internal {
                self.0[$index]
            }
        }
    };
    (@step $_index:expr) => {};
}

serialize_vec!(Vec3D, 3);
impl_vec_getters!(Vec3D, f64, (x, y, z));

serialize_vec!(Vec3I, 3);
impl_vec_getters!(Vec3I, i32, (x, y, z));


serialize_vec!(Vec2F, 2);
impl_vec_getters!(Vec2F, f32, (x, y));

serialize_vec!(Vec2I, 2);
impl_vec_getters!(Vec2I, i32, (x, y));
