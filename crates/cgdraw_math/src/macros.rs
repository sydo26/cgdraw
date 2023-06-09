#![macro_use]

macro_rules! impl_tuple_conversions {
    ($ArrayN:ident <$S:ident> { $($field:ident),+ }, $Tuple:ty) => {

        #[allow(clippy::from_over_into)]
        impl<$S> Into<$Tuple> for $ArrayN<$S> {
            #[inline]
            fn into(self) -> $Tuple {
                match self { $ArrayN { $($field),+ } => ($($field),+,) }
            }
        }

        impl<$S> AsRef<$Tuple> for $ArrayN<$S> {
            #[inline]
            fn as_ref(&self) -> &$Tuple {
                unsafe { mem::transmute(self) }
            }
        }

        impl<$S> AsMut<$Tuple> for $ArrayN<$S> {
            #[inline]
            fn as_mut(&mut self) -> &mut $Tuple {
                unsafe { mem::transmute(self) }
            }
        }

        impl<$S> From<$Tuple> for $ArrayN<$S> {
            #[inline]
            fn from(v: $Tuple) -> $ArrayN<$S> {
                match v { ($($field),+,) => $ArrayN { $($field),+ } }
            }
        }

        impl<'a, $S> From<&'a $Tuple> for &'a $ArrayN<$S> {
            #[inline]
            fn from(v: &'a $Tuple) -> &'a $ArrayN<$S> {
                unsafe { mem::transmute(v) }
            }
        }

        impl<'a, $S> From<&'a mut $Tuple> for &'a mut $ArrayN<$S> {
            #[inline]
            fn from(v: &'a mut $Tuple) -> &'a mut $ArrayN<$S> {
                unsafe { mem::transmute(v) }
            }
        }
    }
}

macro_rules! impl_index_operators {
    ($VectorN:ident<$S:ident>, $n:expr, $Output:ty, $I:ty) => {
        impl<$S> Index<$I> for $VectorN<$S> {
            type Output = $Output;

            #[inline]
            fn index(&self, i: $I) -> &$Output {
                let v: &[$S; $n] = self.as_ref();
                &v[i]
            }
        }

        impl<$S> IndexMut<$I> for $VectorN<$S> {
            #[inline]
            fn index_mut(&mut self, i: $I) -> &mut $Output {
                let v: &mut [$S; $n] = self.as_mut();
                &mut v[i]
            }
        }
    };
}

macro_rules! impl_fixed_array_conversions {
    ($ArrayN:ident <$S:ident> { $($field:ident : $index:expr),+ }, $n:expr) => {
        #[allow(clippy::from_over_into)]
        impl<$S> Into<[$S; $n]> for $ArrayN<$S> {
            #[inline]
            fn into(self) -> [$S; $n] {
                match self { $ArrayN { $($field),+ } => [$($field),+] }
            }
        }

        impl<$S> AsRef<[$S; $n]> for $ArrayN<$S> {
            #[inline]
            fn as_ref(&self) -> &[$S; $n] {
                unsafe { mem::transmute(self) }
            }
        }

        impl<$S> AsMut<[$S; $n]> for $ArrayN<$S> {
            #[inline]
            fn as_mut(&mut self) -> &mut [$S; $n] {
                unsafe { mem::transmute(self) }
            }
        }

        impl<$S: Clone> From<[$S; $n]> for $ArrayN<$S> {
            #[inline]
            fn from(v: [$S; $n]) -> $ArrayN<$S> {
                // We need to use a clone here because we can't pattern match on arrays yet
                $ArrayN { $($field: v[$index].clone()),+ }
            }
        }

        impl<'a, $S> From<&'a [$S; $n]> for &'a $ArrayN<$S> {
            #[inline]
            fn from(v: &'a [$S; $n]) -> &'a $ArrayN<$S> {
                unsafe { mem::transmute(v) }
            }
        }

        impl<'a, $S> From<&'a mut [$S; $n]> for &'a mut $ArrayN<$S> {
            #[inline]
            fn from(v: &'a mut [$S; $n]) -> &'a mut $ArrayN<$S> {
                unsafe { mem::transmute(v) }
            }
        }
    }
}

macro_rules! fold_array {
    (&$method:ident, { $x:expr }) => {
        *$x
    };
    (&$method:ident, { $x:expr, $y:expr }) => {
        $x.$method(&$y)
    };
    (&$method:ident, { $x:expr, $y:expr, $z:expr }) => {
        $x.$method(&$y).$method(&$z)
    };
    (&$method:ident, { $x:expr, $y:expr, $z:expr, $w:expr }) => {
        $x.$method(&$y).$method(&$z).$method(&$w)
    };
    ($method:ident, { $x:expr }) => {
        $x
    };
    ($method:ident, { $x:expr, $y:expr }) => {
        $x.$method($y)
    };
    ($method:ident, { $x:expr, $y:expr, $z:expr }) => {
        $x.$method($y).$method($z)
    };
    ($method:ident, { $x:expr, $y:expr, $z:expr, $w:expr }) => {
        $x.$method($y).$method($z).$method($w)
    };
}
