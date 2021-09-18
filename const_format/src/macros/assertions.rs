macro_rules! with_shared_docs {(
    $(#[$before_clarification:meta])*
    ;clarification
    $(#[$before_syntax:meta])*
    ;syntax
    $(#[$after_syntax:meta])*
    ;error_message
    $(#[$after_error_message:meta])*
    ;limitations
    $item:item
) => (
    $(#[$before_clarification])*
    ///
    /// This macro requires the "assert" feature to be exported,
    /// because it uses some nightly Rust features.<br>
    ///
    $(#[$before_syntax])*
    /// # Syntax
    ///
    /// This macro uses [the same syntax](./fmt/index.html#fmtsyntax)
    /// for the format string and formatting arguments as the
    /// [`formatc`] macro.
    ///
    $(#[$after_syntax])*
    /// # Error message
    ///
    /// `const_format` uses some workarounds to avoid requiring users to enable the
    /// `#![feature(const_panic)]` feature themselves,
    /// as a result, the error message isn't as good as it could possibly be.
    ///
    /// Compile-time errors with this macro include the formatted error message,
    /// and the module path + line where this macro was invoked.
    ///
    $(#[$after_error_message])*
    /// # Limitations
    ///
    /// This macro has these limitations:
    ///
    /// - It can only use constants that involve concrete types,
    /// so while a `Type::<u8>::FOO` in an argument would be fine,
    /// `Type::<T>::FOO` would not be (`T` being a type parameter).
    ///
    /// - Integer arguments must have a type inferrable from context,
    /// [as described in the integer arguments section in the root module
    /// ](./index.html#integer-args).
    ///
    /// [`PWrapper`]: ./struct.PWrapper.html
    /// [`formatc`]: ./macro.formatc.html
    /// [`FormatMarker`]: ./marker_traits/trait.FormatMarker.html
    ///
    $item
)}

////////////////////////////////////////////////////////////////////////////////

with_shared_docs! {
    /// Compile-time assertions with formatting.
    ///
    ;clarification
    ;syntax
    ;error_message
    ;limitations
    ///
    /// # Examples
    ///
    /// ### Passing assertion
    ///
    /// ```rust
    /// #![feature(const_mut_refs)]
    ///
    /// use const_format::assertc;
    ///
    /// use std::mem::size_of;
    ///
    /// assertc!(
    ///     size_of::<&str>() == size_of::<&[u8]>(),
    ///     "The size of `&str`({} bytes) and `&[u8]`({} bytes) aren't the same?!?!",
    ///     size_of::<&str>(),
    ///     size_of::<&[u8]>(),
    /// );
    ///
    /// # fn main(){}
    /// ```
    ///
    /// ### Failing assertion
    ///
    /// This example demonstrates a failing assertion,
    /// and how the compiler error looks like as of 2020-09-XX.
    ///
    /// ```compile_fail
    /// #![feature(const_mut_refs)]
    ///
    /// use const_format::assertc;
    ///
    /// use std::mem::size_of;
    ///
    /// const L: u64 = 2;
    /// const R: u64 = 2;
    ///
    /// assertc!(L + R == 5, "{} plus {} isn't 5 buddy", L,  R);
    ///
    /// # fn main(){}
    /// ```
    ///
    /// This is the compiler output,
    /// the first compilation error is there to have an indicator of what assertion failed,
    /// and the second is the assertion failure.
    ///
    /// ```text
    /// error: any use of this value will cause an error
    ///   --> src/macros/assertions.rs:59:1
    ///    |
    /// 13 | assertc!(L + R == 5, "{} plus {} isn't 5 buddy", L,  R);
    ///    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ exceeded interpreter step limit (see `#[const_eval_limit]`)
    ///    |
    ///    = note: `#[deny(const_err)]` on by default
    ///    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
    ///
    /// error[E0080]: could not evaluate constant
    ///   --> /const_format/src/panicking.rs:32:5
    ///    |
    /// 32 |     .
    ///    |     ^ the evaluated program panicked at '
    /// --------------------------------------------------------------------------------
    /// module_path: rust_out
    /// line: 13
    ///
    /// assertion failed: L + R == 5
    ///
    /// 2 plus 2 isn't 5 buddy
    /// --------------------------------------------------------------------------------
    /// ', const_format/src/panicking.rs:31:1
    ///    |
    ///    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
    ///
    /// ```
    ///
    #[cfg_attr(feature = "docsrs", doc(cfg(feature = "assert")))]
    #[macro_export]
    macro_rules! assertc {
        ($($parameters:tt)*) => (
            $crate::__assertc_inner!{
                ($($parameters)*)
                ($($parameters)*)
            }
        );
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! __assertc_inner {
    (
        ($($parameters:tt)*)
        ($cond:expr $(, $fmt_literal:expr $(,$fmt_arg:expr)*)? $(,)?)
    ) => {
        const _: () = {
            use $crate::__cf_osRcTFl4A;

            $crate::__assertc_common!{
                ($($parameters)*)
                ($cond)
                (
                    concat!(
                        "assertion failed. ",
                        $($fmt_literal)?
                    ),
                    $($($fmt_arg),*)?
                )
            }
        };
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! __assertc_common {
    (
        ($($span:tt)*)
        ($cond:expr)
        ($($fmt_literal:expr $(,$fmt_arg:expr)*)?)
    ) => (
        const PANIC_IF_TRUE_NHPMWYD3NJA: bool = !($cond);

        const MSG_NHPMWYD3NJA: &str = $crate::pmr::__formatc_if_impl!(
            (PANIC_IF_TRUE_NHPMWYD3NJA),
            ($($fmt_literal,)?),
            $($($fmt_arg,)*)?
        );

        __cf_osRcTFl4A::pmr::respan_to!{
            ($($span)*)
            __cf_osRcTFl4A::panicking::assert_(PANIC_IF_TRUE_NHPMWYD3NJA, MSG_NHPMWYD3NJA)
        }
    );
}

////////////////////////////////////////////////////////////////////////////////

macro_rules! assert_eq_docs {
    (
        $(#[$documentation:meta])*
        ;documentation
        $item:item
    ) => (
        with_shared_docs! {
            $(#[$documentation])*
            ;clarification
            /// # Comparison Arguments
            ///
            /// This macro accepts these types for comparison and debug printing:
            ///
            /// - Standard library types for which  [`PWrapper`] wrapping that type
            /// has a `const_eq` method.
            /// This includes all integer types, `&str`, slices/arrays of integers/`&str`,
            /// Options of integers/`&str`, etc.
            ///
            /// - non-standard-library types that implement [`FormatMarker`] with debug formatting<br>
            /// and have a `const fn const_eq(&self, other:&Self) -> bool` inherent method,
            ///
            ;syntax
            ;error_message
            ;limitations
            $item
        }
    )
}

assert_eq_docs! {
    /// Compile-time equality assertion with formatting.
    ///
    ;documentation
    ///
    /// # Examples
    ///
    /// ### Passing assertion
    ///
    /// ```rust
    /// #![feature(const_mut_refs)]
    ///
    /// use const_format::assertc_eq;
    ///
    /// use std::mem::size_of;
    ///
    /// assertc_eq!(size_of::<usize>(), size_of::<[usize;1]>());
    ///
    /// const TWO: u32 = 2;
    /// assertc_eq!(TWO, TWO, "Oh no {} doesn't equal itself!!", TWO);
    ///
    /// # fn main(){}
    /// ```
    ///
    /// ### Failing assertion
    ///
    /// This example demonstrates a failing assertion,
    /// and how the compiler error looks like as of 2020-09-XX.
    ///
    /// ```compile_fail
    /// #![feature(const_mut_refs)]
    ///
    /// use const_format::assertc_eq;
    ///
    /// use std::mem::size_of;
    ///
    /// assertc_eq!(size_of::<u32>(), size_of::<u8>());
    ///
    /// # fn main(){}
    /// ```
    ///
    /// This is the compiler output,
    /// the first compilation error is there to have an indicator of what assertion failed,
    /// and the second is the assertion failure.
    ///
    /// ```text
    /// error: any use of this value will cause an error
    ///  --> src/macros/assertions.rs:256:1
    ///   |
    /// 9 | assertc_eq!(size_of::<u32>(), size_of::<u8>());
    ///   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ exceeded interpreter step limit (see `#[const_eval_limit]`)
    ///   |
    ///   = note: `#[deny(const_err)]` on by default
    ///   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
    ///
    /// error[E0080]: could not evaluate constant
    ///   --> /const_format/src/panicking.rs:32:5
    ///    |
    /// 32 |     .
    ///    |     ^ the evaluated program panicked at '
    /// --------------------------------------------------------------------------------
    /// module_path: rust_out
    /// line: 9
    ///
    /// assertion failed: LEFT == RIGHT
    ///
    ///  left: `4`
    /// right: `1`
    /// --------------------------------------------------------------------------------
    /// ', /const_format/src/panicking.rs:31:1
    ///    |
    ///    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
    ///
    /// error: aborting due to 2 previous errors
    ///
    /// ```
    ///
    /// ### Comparing user-defined types
    ///
    /// This example demonstrates how you can assert that two values of a
    /// user-defined type are equal.
    ///
    #[cfg_attr(feature = "derive", doc = "```compile_fail")]
    #[cfg_attr(not(feature = "derive"), doc = "```ignore")]
    /// #![feature(const_mut_refs)]
    ///
    /// use const_format::{Formatter, PWrapper};
    /// use const_format::{ConstDebug, assertc_eq, try_};
    ///
    /// const POINT: Point = Point{ x: 5, y: 8, z: 13 };
    /// const OTHER_POINT: Point = Point{ x: 21, y: 34, z: 55 };
    ///
    /// assertc_eq!(POINT, OTHER_POINT);
    ///
    /// #[derive(ConstDebug)]
    /// pub struct Point {
    ///     pub x: u32,
    ///     pub y: u32,
    ///     pub z: u32,
    /// }
    ///
    /// impl Point {
    ///     pub const fn const_eq(&self, other: &Self) -> bool {
    ///         self.x == other.x &&
    ///         self.y == other.y &&
    ///         self.z == other.z
    ///     }
    /// }
    /// ```
    ///
    /// This is the compiler output:
    ///
    /// ```text
    /// error: any use of this value will cause an error
    ///   --> src/macros/assertions.rs:343:1
    ///    |
    /// 12 | assertc_eq!(POINT, OTHER_POINT);
    ///    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ exceeded interpreter step limit (see `#[const_eval_limit]`)
    ///    |
    ///    = note: `#[deny(const_err)]` on by default
    ///    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
    ///
    /// error[E0080]: could not evaluate constant
    ///   --> /const_format/src/panicking.rs:32:5
    ///    |
    /// 32 |     .
    ///    |     ^ the evaluated program panicked at '
    /// --------------------------------------------------------------------------------
    /// module_path: rust_out
    /// line: 12
    ///
    /// assertion failed: LEFT == RIGHT
    ///
    ///  left: `Point {
    ///     x: 5,
    ///     y: 8,
    ///     z: 13,
    /// }`
    /// right: `Point {
    ///     x: 21,
    ///     y: 34,
    ///     z: 55,
    /// }`
    /// --------------------------------------------------------------------------------
    /// ', /const_format/src/panicking.rs:32:5
    ///    |
    ///    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
    ///
    /// error: aborting due to 2 previous errors
    ///
    /// ```
    ///
    #[cfg_attr(feature = "docsrs", doc(cfg(feature = "assert")))]
    #[macro_export]
    macro_rules! assertc_eq {
        ($($parameters:tt)*) => (
            $crate::__assertc_equality_inner!{
                ($($parameters)*)
                ($($parameters)*)
                ( == )
                ("==")
            }
        );
    }
}

assert_eq_docs! {
    /// Compile-time inequality assertion with formatting.
    ///
    ;documentation
    ///
    /// # Examples
    ///
    /// ### Passing assertion
    ///
    /// ```rust
    /// #![feature(const_mut_refs)]
    ///
    /// use const_format::assertc_ne;
    ///
    /// use std::mem::size_of;
    ///
    /// assertc_ne!(size_of::<u32>(), size_of::<[u32; 2]>());
    ///
    /// const TWO: u32 = 2;
    /// const THREE: u32 = 3;
    /// assertc_ne!(TWO, THREE, "Oh no {} somehow equals {}!!", TWO, THREE);
    ///
    /// # fn main(){}
    /// ```
    ///
    /// ### Failing assertion
    ///
    /// This example demonstrates a failing assertion,
    /// and how the compiler error looks like as of 2020-09-XX.
    ///
    /// ```compile_fail
    /// #![feature(const_mut_refs)]
    ///
    /// use const_format::assertc_ne;
    ///
    /// use std::mem::size_of;
    ///
    /// type Foo = u32;
    ///
    /// assertc_ne!(size_of::<u32>(), size_of::<Foo>());
    ///
    /// # fn main(){}
    /// ```
    ///
    /// This is the compiler output,
    /// the first compilation error is there to have an indicator of what assertion failed,
    /// and the second is the assertion failure:
    ///
    /// ```text
    /// error: any use of this value will cause an error
    ///   --> src/macros/assertions.rs:411:1
    ///    |
    /// 11 | assertc_ne!(size_of::<u32>(), size_of::<Foo>());
    ///    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ exceeded interpreter step limit (see `#[const_eval_limit]`)
    ///    |
    ///    = note: `#[deny(const_err)]` on by default
    ///    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
    ///
    /// error[E0080]: could not evaluate constant
    ///   --> /const_format/src/panicking.rs:32:5
    ///    |
    /// 32 |     .
    ///    |     ^ the evaluated program panicked at '
    /// --------------------------------------------------------------------------------
    /// module_path: rust_out
    /// line: 11
    ///
    /// assertion failed: LEFT != RIGHT
    ///
    ///  left: `4`
    /// right: `4`
    /// --------------------------------------------------------------------------------
    /// ', /const_format/src/panicking.rs:31:1
    ///    |
    ///    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
    ///
    /// error: aborting due to 2 previous errors
    ///
    /// ```
    ///
    /// ### Comparing user-defined types
    ///
    /// This example demonstrates how you can assert that two values of a
    /// user-defined type are unequal.
    ///
    #[cfg_attr(feature = "derive", doc = "```compile_fail")]
    #[cfg_attr(not(feature = "derive"), doc = "```ignore")]
    /// #![feature(const_mut_refs)]
    ///
    /// use const_format::{Formatter, PWrapper};
    /// use const_format::{ConstDebug, assertc_ne, try_};
    ///
    /// const POINT: Point = Point{ x: 5, y: 8, z: 13 };
    ///
    /// assertc_ne!(POINT, POINT);
    ///
    /// #[derive(ConstDebug)]
    /// pub struct Point {
    ///     pub x: u32,
    ///     pub y: u32,
    ///     pub z: u32,
    /// }
    ///
    /// impl Point {
    ///     pub const fn const_eq(&self, other: &Self) -> bool {
    ///         self.x == other.x &&
    ///         self.y == other.y &&
    ///         self.z == other.z
    ///     }
    /// }
    /// ```
    ///
    /// This is the compiler output:
    ///
    /// ```text
    /// error: any use of this value will cause an error
    ///   --> src/macros/assertions.rs:522:1
    ///    |
    /// 11 | assertc_ne!(POINT, POINT);
    ///    | ^^^^^^^^^^^^^^^^^^^^^^^^^^ exceeded interpreter step limit (see `#[const_eval_limit]`)
    ///    |
    ///    = note: `#[deny(const_err)]` on by default
    ///    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
    ///
    /// error[E0080]: could not evaluate constant
    ///   --> /const_format/src/panicking.rs:32:5
    ///    |
    /// 32 |     .
    ///    |     ^ the evaluated program panicked at '
    /// --------------------------------------------------------------------------------
    /// module_path: rust_out
    /// line: 11
    ///
    /// assertion failed: LEFT != RIGHT
    ///
    ///  left: `Point {
    ///     x: 5,
    ///     y: 8,
    ///     z: 13,
    /// }`
    /// right: `Point {
    ///     x: 5,
    ///     y: 8,
    ///     z: 13,
    /// }`
    /// --------------------------------------------------------------------------------
    /// ', /const_format/src/panicking.rs:32:5
    ///    |
    ///    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
    ///
    /// error: aborting due to 2 previous errors
    ///
    /// For more information about this error, try `rustc --explain E0080`.
    /// Couldn't compile the test.
    ///
    /// failures:
    ///     src/macros/assertions.rs - assertc_ne (line 514)
    ///
    ///
    /// ```
    ///
    #[cfg_attr(feature = "docsrs", doc(cfg(feature = "assert")))]
    #[macro_export]
    macro_rules! assertc_ne {
        ($($parameters:tt)*) => (
            $crate::__assertc_equality_inner!{
                ($($parameters)*)
                ($($parameters)*)
                ( != )
                ("!=")
            }
        );
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! __assertc_equality_inner {
    (
        ($($parameters:tt)*)
        (
            $left:expr,
            $right:expr
            $(, $fmt_literal:expr $(,$fmt_arg:expr)*)? $(,)?
        )
        ($($op:tt)*)
        ($op_str:expr)
    )=>{
        const _: () = {
            use $crate::pmr::respan_to as __cf_respan_to;
            const LEFT: bool = {
                // Have to use `respan_to` to make the `multiple coerce found` error
                // point at the `$left` argument here.
                use $crate::coerce_to_fmt as __cf_coerce_to_fmt;
                match [&$left, &$right] {
                    __cf_respan_to!(($left) [left, right]) =>
                        __cf_respan_to!(($left) __cf_coerce_to_fmt!(left).const_eq(right)),
                }
            };
            const RIGHT: bool = true;

            $crate::__assertc_common!{
                ($($parameters)*)
                (LEFT $($op)* RIGHT)
                (
                    concat!(
                        "assertion failed: `(left ",
                        $op_str,
                        " right)`\n",
                        " left: `{left_NHPMWYD3NJA:?}`\n\
                         right: `{right_NHPMWYD3NJA:?}`",
                        $("\n", $fmt_literal, "\n")?
                    ),
                    $($($fmt_arg,)*)?
                    left_NHPMWYD3NJA = $left,
                    right_NHPMWYD3NJA = $right
                )
            }
        };
    }
}
