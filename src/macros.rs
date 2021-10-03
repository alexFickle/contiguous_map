/// Macro for creating a [`ContiguousMap`](crate::ContiguousMap).
///
/// ## Example
/// ```
/// use contiguous_map::cmap;
///
/// let map = cmap!(1 => 2);
/// assert_eq!(1, map.len());
///
/// let map = cmap!(
///     1 => 2;
///     5 => 1, 2, 3;
///     9 => 7, 8;
/// );
/// assert_eq!(&[1, 2, 3], map.get_slice(5..).unwrap());
/// ```
#[macro_export]
macro_rules! cmap {
    ($($key:expr => $($value:expr),+ $(,)?);* $(;)?) => {
        {
            let mut _map = $crate::ContiguousMap::new();
            $(
                let mut _key = Some($key);
                $(
                    let k = _key.unwrap();
                    _map.insert(k, $value);
                    _key = <_ as $crate::Key>::add_one(&k);
                )+
            )*
            _map
        }
    };
}
