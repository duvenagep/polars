use super::Scalar;
use crate::datatypes::ArrowDataType;
use crate::types::NativeType;

/// The implementation of [`Scalar`] for primitive, semantically equivalent to [`Option<T>`]
/// with [`ArrowDataType`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrimitiveScalar<T: NativeType> {
    value: Option<T>,
    dtype: ArrowDataType,
}

impl<T: NativeType> PrimitiveScalar<T> {
    /// Returns a new [`PrimitiveScalar`].
    #[inline]
    pub fn new(dtype: ArrowDataType, value: Option<T>) -> Self {
        if !dtype.to_physical_type().eq_primitive(T::PRIMITIVE) {
            panic!(
                "Type {} does not support logical type {:?}",
                std::any::type_name::<T>(),
                dtype
            )
        }
        Self { value, dtype }
    }

    /// Returns the optional value.
    #[inline]
    pub fn value(&self) -> &Option<T> {
        &self.value
    }

    /// Returns a new `PrimitiveScalar` with the same value but different [`ArrowDataType`]
    /// # Panic
    /// This function panics if the `dtype` is not valid for self's physical type `T`.
    pub fn to(self, dtype: ArrowDataType) -> Self {
        Self::new(dtype, self.value)
    }
}

impl<T: NativeType> From<Option<T>> for PrimitiveScalar<T> {
    #[inline]
    fn from(v: Option<T>) -> Self {
        Self::new(T::PRIMITIVE.into(), v)
    }
}

impl<T: NativeType> Scalar for PrimitiveScalar<T> {
    #[inline]
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    #[inline]
    fn is_valid(&self) -> bool {
        self.value.is_some()
    }

    #[inline]
    fn dtype(&self) -> &ArrowDataType {
        &self.dtype
    }
}
