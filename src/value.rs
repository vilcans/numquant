use std::marker::PhantomData;

use crate::{
    normalize::{Normalize, NormalizeError},
    Quantize,
};

/// Contains a normalized and quantized value.
///
/// ```
/// use numquant::{Value, Linear, quantizer::Quantizer};
/// let q = Value::<Quantizer<u8, 0xff>, Linear<0, 1000>>::from_f64(250.0);
/// let v = q.to_f64();
/// approx::assert_abs_diff_eq!(v, 250.0, epsilon = 1.0);
/// ```
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Value<Q: Quantize, N: Normalize>(Q::Type, PhantomData<(Q, N)>);

impl<Q: Quantize, N: Normalize> Value<Q, N>
where
    Q::Type: Into<f64> + Clone,
{
    /// Create an instance from an `f64`.
    pub fn from_f64(v: f64) -> Self
    where
        Self: Sized,
    {
        let normalized = N::normalize(v);
        Self::from_raw(Q::quantize(normalized))
    }

    /// Convert the stored value into an `f64`.
    pub fn to_f64(&self) -> f64 {
        let normalized = Q::dequantize(self.0.clone());
        N::denormalize(normalized)
    }

    /// Create an instance from an `f32`.
    pub fn from_f32(v: f32) -> Self
    where
        Self: Sized,
    {
        Self::from_f64(v as f64)
    }

    /// Convert the stored value into an `f32`.
    pub fn to_f32(&self) -> f32 {
        self.to_f64() as f32
    }

    pub fn max_error() -> f64
    where
        N: NormalizeError,
    {
        N::max_error(Q::max_error())
    }

    pub fn from_raw(v: Q::Type) -> Self {
        Self(v, PhantomData)
    }

    pub fn raw(&self) -> Q::Type {
        self.0.clone()
    }
}

impl<Q: Quantize, N: Normalize> Default for Value<Q, N>
where
    Q::Type: Default,
{
    fn default() -> Self {
        Self(Default::default(), PhantomData)
    }
}

impl<Q: Quantize, N: Normalize> Clone for Value<Q, N>
where
    Q::Type: Clone,
{
    fn clone(&self) -> Self {
        Self(self.0.clone(), PhantomData)
    }
}

impl<Q: Quantize, N: Normalize> std::fmt::Debug for Value<Q, N>
where
    Q::Type: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple(std::any::type_name::<Self>())
            .field(&self.0)
            .finish()
    }
}
