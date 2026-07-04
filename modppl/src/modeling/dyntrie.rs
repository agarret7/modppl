use crate::{Trace, Trie};
use std::any::Any;
use std::fmt::{self, Debug, Display, Write};
use std::sync::Arc;

///
pub type DynTrie = Trie<Arc<dyn Any + Send + Sync>>;

///
pub type DynTrace<Args, Ret> = Trace<Args, DynTrie, Ret>;

/// Conversion support for [`DynTrie::auto`].
pub trait DynAutoCast: Clone + Sized + 'static {
    /// Try to convert a dynamically stored value into `Self`.
    fn autocast(value: &(dyn Any + Send + Sync)) -> Option<Self>;
}

impl DynTrie {
    /// Safely casCastt the inner `dyn Any` at `addr` into type `V` at runtime.
    pub fn read<V: 'static + Clone>(&self, addr: &str) -> V {
        match self.search(addr) {
            Some(v) => {
                let v_typed = v.ref_inner().unwrap().downcast_ref::<V>();
                match v_typed {
                    Some(v) => v.clone(),
                    None => {
                        panic!("read: failed when downcasting type at address \"{}\"", addr);
                    }
                }
            }
            None => {
                panic!("read: failed when searching empty address \"{}\"", addr);
            }
        }
    }

    /// Cast or convert the inner `dyn Any` at `addr` into type `V` at runtime.
    ///
    /// This first tries the same exact downcast as [`DynTrie::read`]. If that
    /// fails, it tries [`DynAutoCast`] conversions for supported literal types.
    ///
    /// # Safety
    ///
    /// Autocasts may be lossy or surprising. Numeric conversions use Rust's
    /// `as` casts, so callers must be willing to accept truncation, wrapping, or
    /// precision loss where those casts allow it.
    pub unsafe fn auto<V: DynAutoCast>(&self, addr: &str) -> V {
        match self.search(addr) {
            Some(v) => {
                let value = v.ref_inner().unwrap().as_ref();
                if let Some(value) = value.downcast_ref::<V>() {
                    return value.clone();
                }
                V::autocast(value).unwrap_or_else(|| {
                    panic!("auto: failed when autocasting type at address \"{addr}\"")
                })
            }
            None => {
                panic!("auto: failed when searching empty address \"{addr}\"");
            }
        }
    }
}

macro_rules! impl_exact_autocast {
    ($($ty:ty),* $(,)?) => {
        $(
            impl DynAutoCast for $ty {
                fn autocast(_: &(dyn Any + Send + Sync)) -> Option<Self> {
                    None
                }
            }
        )*
    };
}

macro_rules! impl_numeric_autocast {
    ($target:ty; $($source:ty),* $(,)?) => {
        impl DynAutoCast for $target {
            fn autocast(value: &(dyn Any + Send + Sync)) -> Option<Self> {
                $(
                    if let Some(value) = value.downcast_ref::<$source>() {
                        return Some(*value as $target);
                    }
                )*
                None
            }
        }
    };
}

macro_rules! impl_tuple2_autocast {
    (($target:ty, $target2:ty); $(($source:ty, $source2:ty)),* $(,)?) => {
        impl DynAutoCast for ($target, $target2) {
            fn autocast(value: &(dyn Any + Send + Sync)) -> Option<Self> {
                $(
                    if let Some(value) = value.downcast_ref::<($source, $source2)>() {
                        return Some((value.0 as $target, value.1 as $target2));
                    }
                )*
                None
            }
        }
    };
}

macro_rules! impl_tuple3_autocast {
    (($target:ty, $target2:ty, $target3:ty); $(($source:ty, $source2:ty, $source3:ty)),* $(,)?) => {
        impl DynAutoCast for ($target, $target2, $target3) {
            fn autocast(value: &(dyn Any + Send + Sync)) -> Option<Self> {
                $(
                    if let Some(value) = value.downcast_ref::<($source, $source2, $source3)>() {
                        return Some((
                            value.0 as $target,
                            value.1 as $target2,
                            value.2 as $target3,
                        ));
                    }
                )*
                None
            }
        }
    };
}

macro_rules! impl_tuple4_autocast {
    (($target:ty, $target2:ty, $target3:ty, $target4:ty); $(($source:ty, $source2:ty, $source3:ty, $source4:ty)),* $(,)?) => {
        impl DynAutoCast for ($target, $target2, $target3, $target4) {
            fn autocast(value: &(dyn Any + Send + Sync)) -> Option<Self> {
                $(
                    if let Some(value) = value.downcast_ref::<($source, $source2, $source3, $source4)>() {
                        return Some((
                            value.0 as $target,
                            value.1 as $target2,
                            value.2 as $target3,
                            value.3 as $target4,
                        ));
                    }
                )*
                None
            }
        }
    };
}

impl_exact_autocast!(bool, char, String, &'static str);

impl_numeric_autocast!(i8; i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64);
impl_numeric_autocast!(i16; i8, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64);
impl_numeric_autocast!(i32; i8, i16, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64);
impl_numeric_autocast!(i64; i8, i16, i32, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64);
impl_numeric_autocast!(i128; i8, i16, i32, i64, isize, u8, u16, u32, u64, u128, usize, f32, f64);
impl_numeric_autocast!(isize; i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, usize, f32, f64);
impl_numeric_autocast!(u8; i8, i16, i32, i64, i128, isize, u16, u32, u64, u128, usize, f32, f64);
impl_numeric_autocast!(u16; i8, i16, i32, i64, i128, isize, u8, u32, u64, u128, usize, f32, f64);
impl_numeric_autocast!(u32; i8, i16, i32, i64, i128, isize, u8, u16, u64, u128, usize, f32, f64);
impl_numeric_autocast!(u64; i8, i16, i32, i64, i128, isize, u8, u16, u32, u128, usize, f32, f64);
impl_numeric_autocast!(u128; i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, usize, f32, f64);
impl_numeric_autocast!(usize; i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, f32, f64);
impl_numeric_autocast!(f32; i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f64);
impl_numeric_autocast!(f64; i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32);

impl_tuple2_autocast!((f32, f32); (f64, f64));
impl_tuple2_autocast!((f64, f64); (f32, f32));
impl_tuple3_autocast!((f32, f32, f32); (f64, f64, f64));
impl_tuple3_autocast!((f64, f64, f64); (f32, f32, f32));
impl_tuple4_autocast!((f32, f32, f32, f32); (f64, f64, f64, f64));
impl_tuple4_autocast!((f64, f64, f64, f64); (f32, f32, f32, f32));

macro_rules! impl_array_autocast {
    ($target:ty; $($source:ty),* $(,)?) => {
        impl<const N: usize> DynAutoCast for [$target; N] {
            fn autocast(value: &(dyn Any + Send + Sync)) -> Option<Self> {
                $(
                    if let Some(value) = value.downcast_ref::<[$source; N]>() {
                        return Some(value.map(|x| x as $target));
                    }
                )*
                None
            }
        }
    };
}

impl_array_autocast!(f32; f64);
impl_array_autocast!(f64; f32);

fn format_tuple2<T: fmt::Display>(value: &(T, T)) -> String {
    format!("({}, {})", value.0, value.1)
}

fn format_tuple3<T: fmt::Display>(value: &(T, T, T)) -> String {
    format!("({}, {}, {})", value.0, value.1, value.2)
}

fn format_tuple4<T: fmt::Display>(value: &(T, T, T, T)) -> String {
    format!("({}, {}, {}, {})", value.0, value.1, value.2, value.3)
}

fn format_array<T: fmt::Display, const N: usize>(value: &[T; N]) -> String {
    let mut out = String::from("[");
    for (i, x) in value.iter().enumerate() {
        if i > 0 {
            out.push_str(", ");
        }
        write!(out, "{x}").unwrap();
    }
    out.push(']');
    out
}

/// Convert a dynamically stored trace leaf into a readable string.
///
/// This recognizes common literal-like values. Custom structs are intentionally
/// reported as `<unknown>` because [`DynTrie`] stores leaves as [`Any`], which
/// preserves type identity for downcasting but not a general `Display`/`Debug`
/// implementation.
pub fn dyn_value_to_string(value: &(dyn Any + Send + Sync)) -> String {
    dyn_value_to_string_with(value, &[])
}

/// A printer-side formatter for a dynamically stored trace leaf.
pub type DynValueFormatter = fn(&(dyn Any + Send + Sync)) -> Option<String>;

/// Options for rendering dynamic traces as strings.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DynTracePrintOptions {
    /// Include the trace arguments.
    pub args: bool,

    /// Include the trace data choices.
    pub data: bool,

    /// Include the trace return value.
    pub retv: bool,

    /// Include the trace log joint probability.
    pub logjp: bool,

    /// Include node and leaf weights in the printed trace data.
    pub weights: bool,
}

impl DynTracePrintOptions {
    /// Set whether trace arguments are printed.
    pub const fn set_args(mut self, args: bool) -> Self {
        self.args = args;
        self
    }

    /// Set whether trace data choices are printed.
    pub const fn set_data(mut self, data: bool) -> Self {
        self.data = data;
        self
    }

    /// Set whether trace return values are printed.
    pub const fn set_retv(mut self, retv: bool) -> Self {
        self.retv = retv;
        self
    }

    /// Set whether trace log joint probabilities are printed.
    pub const fn set_logjp(mut self, logjp: bool) -> Self {
        self.logjp = logjp;
        self
    }

    /// Set whether data node and leaf weights are printed.
    pub const fn set_weights(mut self, weights: bool) -> Self {
        self.weights = weights;
        self
    }

    /// Set maximum or minimum trace printing verbosity.
    ///
    /// Maximum verbosity prints every trace field and data weights. Minimum
    /// verbosity prints only data choices without weights.
    pub const fn verbose(verbose: bool) -> Self {
        Self {
            args: verbose,
            data: true,
            retv: verbose,
            logjp: verbose,
            weights: verbose,
        }
    }
}

impl Default for DynTracePrintOptions {
    fn default() -> Self {
        Self {
            args: true,
            data: true,
            retv: true,
            logjp: true,
            weights: false,
        }
    }
}

/// Build a printer-side formatter for a custom type that implements [`Display`].
pub fn dyn_display_formatter<T: Display + 'static>(
    value: &(dyn Any + Send + Sync),
) -> Option<String> {
    value.downcast_ref::<T>().map(ToString::to_string)
}

/// Build a printer-side formatter for a custom type that implements [`Debug`].
pub fn dyn_debug_formatter<T: Debug + 'static>(value: &(dyn Any + Send + Sync)) -> Option<String> {
    value
        .downcast_ref::<T>()
        .map(|value| format!("{:?}", value))
}

/// Convert a dynamically stored trace leaf into a readable string, using
/// custom printer-side formatters before falling back to built-in literals.
pub fn dyn_value_to_string_with(
    value: &(dyn Any + Send + Sync),
    formatters: &[DynValueFormatter],
) -> String {
    for formatter in formatters {
        if let Some(value) = formatter(value) {
            return value;
        }
    }

    macro_rules! downcast_display {
        ($($ty:ty),* $(,)?) => {
            $(
                if let Some(value) = value.downcast_ref::<$ty>() {
                    return value.to_string();
                }
            )*
        };
    }

    macro_rules! downcast_tuple {
        ($formatter:ident; $($ty:ty),* $(,)?) => {
            $(
                if let Some(value) = value.downcast_ref::<$ty>() {
                    return $formatter(value);
                }
            )*
        };
    }

    downcast_display!(
        bool,
        char,
        String,
        &'static str,
        i8,
        i16,
        i32,
        i64,
        i128,
        isize,
        u8,
        u16,
        u32,
        u64,
        u128,
        usize,
        f32,
        f64,
    );

    downcast_tuple!(format_tuple2; (f32, f32), (f64, f64));
    downcast_tuple!(format_tuple3; (f32, f32, f32), (f64, f64, f64));
    downcast_tuple!(format_tuple4; (f32, f32, f32, f32), (f64, f64, f64, f64));

    macro_rules! downcast_array {
        ($($ty:ty; $n:literal),* $(,)?) => {
            $(
                if let Some(value) = value.downcast_ref::<[$ty; $n]>() {
                    return format_array(value);
                }
            )*
        };
    }

    downcast_array!(
        f32; 2, f32; 3, f32; 4,
        f64; 2, f64; 3, f64; 4,
    );

    "<unknown>".to_string()
}

fn write_dyntrie(
    out: &mut String,
    trie: &DynTrie,
    indent: usize,
    formatters: &[DynValueFormatter],
    options: DynTracePrintOptions,
) {
    let mut entries = trie.iter().collect::<Vec<_>>();
    entries.sort_by(|(left, _), (right, _)| left.cmp(right));

    for (addr, subtrie) in entries {
        let padding = " ".repeat(indent);
        if subtrie.is_leaf() {
            let value = subtrie.ref_inner().unwrap();
            write!(
                out,
                "{padding}\"{addr}\" => {}",
                dyn_value_to_string_with(value.as_ref(), formatters)
            )
            .unwrap();
            if options.weights {
                write!(out, "  [weight = {}]", subtrie.weight()).unwrap();
            }
            writeln!(out).unwrap();
        } else {
            write!(out, "{padding}\"{addr}\" => {{").unwrap();
            if options.weights {
                write!(out, "  [weight = {}]", subtrie.weight()).unwrap();
            }
            writeln!(out).unwrap();
            write_dyntrie(out, subtrie, indent + 2, formatters, options);
            writeln!(out, "{padding}}}").unwrap();
        }
    }
}

/// Return a readable tree representation of a dynamic trace's data choices.
pub fn dyntrie_to_string(trie: &DynTrie) -> String {
    dyntrie_to_string_with(trie, &[])
}

/// Return a readable tree representation of a dynamic trace's data choices,
/// using custom printer-side formatters before falling back to built-in literals.
pub fn dyntrie_to_string_with(trie: &DynTrie, formatters: &[DynValueFormatter]) -> String {
    dyntrie_to_string_with_options(trie, formatters, DynTracePrintOptions::default())
}

/// Return a readable tree representation of a dynamic trace's data choices,
/// using custom printer-side formatters and rendering options.
pub fn dyntrie_to_string_with_options(
    trie: &DynTrie,
    formatters: &[DynValueFormatter],
    options: DynTracePrintOptions,
) -> String {
    let mut out = String::from("data: {\n");
    write_dyntrie(&mut out, trie, 2, formatters, options);
    out.push('}');
    out
}

/// Return a readable representation of a dynamic trace.
pub fn dyntrace_to_string<Args: Debug, Ret: Debug>(trace: &DynTrace<Args, Ret>) -> String {
    dyntrace_to_string_with(trace, &[])
}

/// Return a readable representation of a dynamic trace, using custom
/// printer-side formatters before falling back to built-in literals.
pub fn dyntrace_to_string_with<Args: Debug, Ret: Debug>(
    trace: &DynTrace<Args, Ret>,
    formatters: &[DynValueFormatter],
) -> String {
    dyntrace_to_string_with_options(trace, formatters, DynTracePrintOptions::default())
}

/// Return a readable representation of a dynamic trace, using custom
/// printer-side formatters and rendering options.
pub fn dyntrace_to_string_with_options<Args: Debug, Ret: Debug>(
    trace: &DynTrace<Args, Ret>,
    formatters: &[DynValueFormatter],
    options: DynTracePrintOptions,
) -> String {
    let mut out = String::new();
    if options.args {
        writeln!(out, "args: {:?}", trace.args).unwrap();
    }
    if options.data {
        writeln!(
            out,
            "{}",
            dyntrie_to_string_with_options(&trace.data, formatters, options)
        )
        .unwrap();
    }
    if options.retv {
        match trace.retv.as_ref() {
            Some(retv) => writeln!(out, "retv: {:?}", retv).unwrap(),
            None => writeln!(out, "retv: None").unwrap(),
        }
    }
    if options.logjp {
        writeln!(out, "logjp: {}", trace.logjp).unwrap();
    }
    out
}

/// Print a readable representation of a dynamic trace.
pub fn print_dyntrace<Args: Debug, Ret: Debug>(trace: &DynTrace<Args, Ret>) {
    print!("{}", dyntrace_to_string(trace));
}

/// Print a readable representation of a dynamic trace, using custom
/// printer-side formatters before falling back to built-in literals.
pub fn print_dyntrace_with<Args: Debug, Ret: Debug>(
    trace: &DynTrace<Args, Ret>,
    formatters: &[DynValueFormatter],
) {
    print!("{}", dyntrace_to_string_with(trace, formatters));
}

/// Print a readable representation of a dynamic trace, using custom
/// printer-side formatters and rendering options.
pub fn print_dyntrace_with_options<Args: Debug, Ret: Debug>(
    trace: &DynTrace<Args, Ret>,
    formatters: &[DynValueFormatter],
    options: DynTracePrintOptions,
) {
    print!(
        "{}",
        dyntrace_to_string_with_options(trace, formatters, options)
    );
}

#[cfg(test)]
mod tests {
    use super::{
        dyn_debug_formatter, dyn_display_formatter, dyn_value_to_string, dyn_value_to_string_with,
        dyntrace_to_string_with_options, dyntrie_to_string, dyntrie_to_string_with_options,
        DynTracePrintOptions,
    };
    use crate::{Trace, Trie};
    use std::any::Any;
    use std::fmt;
    use std::sync::Arc;

    #[derive(Debug)]
    struct CustomStruct {
        value: i64,
    }

    impl fmt::Display for CustomStruct {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "CustomStruct(value = {})", self.value)
        }
    }

    #[test]
    fn dyn_value_to_string_prints_literals_and_unknown_custom_values() {
        assert_eq!(dyn_value_to_string(&true), "true");
        assert_eq!(dyn_value_to_string(&42_i64), "42");
        assert_eq!(dyn_value_to_string(&"label"), "label");
        assert_eq!(dyn_value_to_string(&(0.1_f64, 0.2_f64)), "(0.1, 0.2)");
        assert_eq!(
            dyn_value_to_string(&(0.3_f64, 0.5_f64, 0.2_f64)),
            "(0.3, 0.5, 0.2)"
        );
        let custom = CustomStruct { value: 3 };
        assert_eq!(custom.value, 3);
        assert_eq!(dyn_value_to_string(&custom), "<unknown>");
        assert_eq!(
            dyn_value_to_string_with(&custom, &[dyn_display_formatter::<CustomStruct>]),
            "CustomStruct(value = 3)"
        );
        assert_eq!(
            dyn_value_to_string_with(&custom, &[dyn_debug_formatter::<CustomStruct>]),
            "CustomStruct { value: 3 }"
        );
    }

    #[test]
    fn dyntrie_to_string_can_hide_weights() {
        let mut trie = Trie::new();
        trie.w_observe("x", Arc::new(1.0_f64) as Arc<dyn Any + Send + Sync>, -2.0);

        assert!(!dyntrie_to_string(&trie).contains("[weight ="));
        assert!(dyntrie_to_string_with_options(
            &trie,
            &[],
            DynTracePrintOptions::default().set_weights(true)
        )
        .contains("[weight = -2]"));
        assert!(!dyntrie_to_string_with_options(
            &trie,
            &[],
            DynTracePrintOptions::default().set_weights(false)
        )
        .contains("[weight ="));
    }

    #[test]
    fn dyntrace_print_options_control_fields_and_unwrap_retv() {
        let mut trie = Trie::new();
        trie.observe("x", Arc::new(1.0_f64) as Arc<dyn Any + Send + Sync>);
        let trace = Trace::new("args", trie, "retv", -1.5);

        let all_fields =
            dyntrace_to_string_with_options(&trace, &[], DynTracePrintOptions::default());
        assert!(all_fields.contains("args: \"args\""));
        assert!(all_fields.contains("data: {"));
        assert!(all_fields.contains("retv: \"retv\""));
        assert!(!all_fields.contains("retv: Some"));
        assert!(all_fields.contains("logjp: -1.5"));

        let data_only = dyntrace_to_string_with_options(
            &trace,
            &[],
            DynTracePrintOptions::default()
                .set_args(false)
                .set_retv(false)
                .set_logjp(false),
        );
        assert!(!data_only.contains("args:"));
        assert!(data_only.contains("data: {"));
        assert!(!data_only.contains("retv:"));
        assert!(!data_only.contains("logjp:"));
    }

    #[test]
    fn dyntrace_print_options_verbose_sets_max_and_min() {
        let mut trie = Trie::new();
        trie.w_observe("x", Arc::new(1.0_f64) as Arc<dyn Any + Send + Sync>, -2.0);
        let trace = Trace::new("args", trie, "retv", -1.5);

        let max = dyntrace_to_string_with_options(&trace, &[], DynTracePrintOptions::verbose(true));
        assert!(max.contains("args:"));
        assert!(max.contains("data: {"));
        assert!(max.contains("retv:"));
        assert!(max.contains("logjp:"));
        assert!(max.contains("[weight = -2]"));

        let min =
            dyntrace_to_string_with_options(&trace, &[], DynTracePrintOptions::verbose(false));
        assert!(!min.contains("args:"));
        assert!(min.contains("data: {"));
        assert!(!min.contains("retv:"));
        assert!(!min.contains("logjp:"));
        assert!(!min.contains("[weight ="));
    }

    #[test]
    fn dyntrie_auto_converts_supported_literals() {
        let mut trie = Trie::new();
        trie.observe("f64", Arc::new(0.25_f64) as Arc<dyn Any + Send + Sync>);
        trie.observe(
            "rgb",
            Arc::new((0.3_f64, 0.5_f64, 0.2_f64)) as Arc<dyn Any + Send + Sync>,
        );
        trie.observe("n", Arc::new(3_u8) as Arc<dyn Any + Send + Sync>);

        unsafe {
            assert_eq!(trie.auto::<f32>("f64"), 0.25_f32);
            assert_eq!(
                trie.auto::<(f32, f32, f32)>("rgb"),
                (0.3_f32, 0.5_f32, 0.2_f32)
            );
            assert_eq!(trie.auto::<i64>("n"), 3_i64);
        }
    }

    #[test]
    #[should_panic(expected = "read: failed when downcasting type at address \"f64\"")]
    fn dyntrie_read_stays_strict_when_autocast_would_succeed() {
        let mut trie = Trie::new();
        trie.observe("f64", Arc::new(0.25_f64) as Arc<dyn Any + Send + Sync>);

        let _ = trie.read::<f32>("f64");
    }
}
