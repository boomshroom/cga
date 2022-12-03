







macro_rules! impl_simd_value {
    ($t:ident{$($field:ident,)* [$($extra:ident: $e:expr),*]}) => {
        impl<T> simba::simd::SimdValue for $t<T>
        where
            T: simba::simd::SimdValue,
            <T as simba::simd::SimdValue>::Element: simba::scalar::RealField,
        {
            type Element = $t<T::Element>;
            type SimdBool = T::SimdBool;
            fn lanes() -> usize {
                T::lanes()
            }
            fn splat(val: Self::Element) -> Self {
                $t { $($field: T::splat(val.$field),)* $($extra: $e,)* }
            }
            fn extract(&self, i: usize) -> Self::Element {
                $t { $($field: self.$field.extract(i),)* $($extra: $e,)* }
            }
            unsafe fn extract_unchecked(&self, i: usize) -> Self::Element {
                $t { $($field: self.$field.extract_unchecked(i),)* $($extra: $e,)* }
            }
            fn replace(&mut self, i: usize, val: Self::Element) {
                $(self.$field.replace(i, val.$field);)*
            }
            unsafe fn replace_unchecked(&mut self, i: usize, val: Self::Element) {
                $(self.$field.replace_unchecked(i, val.$field);)*
            }
            fn select(self, cond: Self::SimdBool, other: Self) -> Self {
                $t { $($field: self.$field.select(cond, other.$field),)* $($extra: $e,)* }
            }
        }
    };
}

macro_rules! impl_subset_of {
    ($t:ident{$($field:ident,)* [$($extra:ident: $e:expr),*]}) => {
        impl<T, U> simba::scalar::SubsetOf<$t<U>> for $t<T>
        where
            U: simba::scalar::SupersetOf<T>,
        {
            fn to_superset(&self) -> $t<U> {
                $t { $($field: U::from_subset(&self.$field),)* $($extra: $e,)* }
            }
            fn from_superset_unchecked(element: &$t<U>) -> Self {
                $t { $($field: element.$field.to_subset_unchecked(),)* $($extra: $e,)* }
            }
            fn is_in_subset(element: &$t<U>) -> bool {
                $(element.$field.is_in_subset())&&*
            }
        }
    }
}

macro_rules! impl_traits {
    ($t:ident{$($field:ident,)* [$($extra:ident: $e:expr),*]}) => {
        impl_simd_value! { $t { $($field,)* [$($extra: $e),*] } }
        impl_subset_of! { $t { $($field,)* [$($extra: $e),*] } }
    }
}
