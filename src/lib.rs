/*!
This crate provides macros and functions for working assertions as an inherent part of your
development process. Whether you use assertions for specific points in your code or for buffing up
your testing, this crate is for you.

The main motivation for this crate is to make it easy to fill your code with assertions without
taking a performance hit in release or in *tests*. This is because the compiler will optimize away
all assertions in release mode and only *some* in tests, by your choosing.

## Basic Usage
There are different assertion levels defined, and macros are provided for each of them.
For trace assertions, you can use the `tassert!` macro:
```rust
use invariants::tassert;
fn main() {
    tassert!(false, "This will fail when assert level is equal or lower then {}. Current level is {}.",
        assertions::AssertLevel::Trace, assertions::STATIC_MAX_LEVEL);
}
```

See the github repository for more information.
*/

pub type AssertLevel = log::LevelFilter;

pub const STATIC_MAX_LEVEL: AssertLevel = log::STATIC_MAX_LEVEL;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AssertConfig {
    assertion_level: AssertLevel,
}

impl AssertConfig {
    pub fn new(assertion_level: AssertLevel) -> Self {
        Self { assertion_level }
    }

    pub fn assertion_level(&self) -> AssertLevel {
        self.assertion_level
    }
}

/// Asserts that the given expression is true when Error level assertions are enabled.
///
/// If the max assert level is lower than Error, this macro does nothing.
/// If the max and current assert level is equal or higher then Error, this macro panics if the expression is
/// evaluated to false.
///
/// # Examples
///
/// ```rust
/// use invariants::eassert;
/// # fn main() {
///     eassert!(false, "This will fail when assert level is equal or lower then {}. Current level is {}.",
/// assertions::AssertLevel::Error, assertions::STATIC_MAX_LEVEL);
/// # }
/// ```
#[macro_export]
macro_rules! eassert {
    ($config:expr; $($arg:tt)*) =>(
        if $crate::AssertLevel::Error <= $crate::STATIC_MAX_LEVEL
            && $crate::AssertLevel::Error <= $config.assertion_level()  { assert!($($arg)*); });
    ($($arg:tt)*) => (if $crate::AssertLevel::Error <= $crate::STATIC_MAX_LEVEL { assert!($($arg)*); });
}

/// Asserts that the given expression is true when Warn level assertions are enabled.
///
/// If the max assert level is lower than Warn, this macro does nothing.
/// If the max and current assert level is equal or higher then Warn, this macro panics if the expression is
/// evaluated to false.
///
/// # Examples
///
/// ```rust
/// use invariants::wassert;
/// # fn main() {
///     wassert!(false, "This will fail when assert level is equal or lower then {}. Current level is {}.",
/// assertions::AssertLevel::Warn, assertions::STATIC_MAX_LEVEL);
/// # }
/// ```
#[macro_export]
macro_rules! wassert {
    ($config:expr; $($arg:tt)*) =>(
        if $crate::AssertLevel::Warn <= $crate::STATIC_MAX_LEVEL
            && $crate::AssertLevel::Warn <= $config.assertion_level()  { assert!($($arg)*); });
    ($($arg:tt)*) => (if $crate::AssertLevel::Warn <= $crate::STATIC_MAX_LEVEL { assert!($($arg)*); })
}

/// Asserts that the given expression is true when Info level assertions are enabled.
///
/// If the max assert level is lower than Info, this macro does nothing.
/// If the max and current assert level is equal or higher then Info, this macro panics if the expression is
/// evaluated to false.
///
/// # Examples
///
/// ```rust
/// use invariants::iassert;
/// # fn main() {
///     iassert!(false, "This will fail when assert level is equal or lower then {}. Current level is {}.",
/// assertions::AssertLevel::Info, assertions::STATIC_MAX_LEVEL);
/// # }
/// ```
#[macro_export]
macro_rules! iassert {
    ($config:expr; $($arg:tt)*) =>(
        if $crate::AssertLevel::Info <= $crate::STATIC_MAX_LEVEL
            && $crate::AssertLevel::Info <= $config.assertion_level()  { assert!($($arg)*); });
    ($($arg:tt)*) => (if $crate::AssertLevel::Info <= $crate::STATIC_MAX_LEVEL { assert!($($arg)*); })
}

/// Asserts that the given expression is true when Debug level assertions are enabled.
///
/// If the max assert level is lower than Debug, this macro does nothing.
/// If the max and current assert level is equal or higher then Debug, this macro panics if the expression is
/// evaluated to false.
///
/// # Examples
///
/// ```rust
/// use invariants::dassert;
/// # fn main() {
///     dassert!(false, "This will fail when assert level is equal or lower then {}. Current level is {}.",
/// assertions::AssertLevel::Debug, assertions::STATIC_MAX_LEVEL);
/// # }
/// ```
#[macro_export]
macro_rules! dassert {
    ($config:expr; $($arg:tt)*) =>(
        if $crate::AssertLevel::Debug <= $crate::STATIC_MAX_LEVEL
            && $crate::AssertLevel::Debug <= $config.assertion_level()  { assert!($($arg)*); });
    ($($arg:tt)*) => (if $crate::AssertLevel::Debug <= $crate::STATIC_MAX_LEVEL { assert!($($arg)*); })
}

/// Asserts that the given expression is true when Trace level assertions are enabled.
///
/// If the max assert level is lower than Trace, this macro does nothing.
/// If the max and current assert level is equal or higher then Trace, this macro panics if the expression is
/// evaluated to false.
///
/// # Examples
///
/// ```rust
/// use invariants::tassert;
/// # fn main() {
///     tassert!(false, "This will fail when assert level is equal or lower then {}. Current level is {}.",
/// assertions::AssertLevel::Trace, assertions::STATIC_MAX_LEVEL);
/// # }
/// ```
#[macro_export]
macro_rules! tassert {
    ($config:expr; $($arg:tt)*) =>(
        if $crate::AssertLevel::Trace <= $crate::STATIC_MAX_LEVEL
            && $crate::AssertLevel::Trace <= $config.assertion_level()  { assert!($($arg)*); });
    ($($arg:tt)*) => (if $crate::AssertLevel::Trace <= $crate::STATIC_MAX_LEVEL { assert!($($arg)*); })
}

#[cfg(test)]
mod tests {
    use crate::dassert;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        dassert!(result == 4);
        log::info!("{}", result);
    }

    #[test]
    #[should_panic]
    fn it_crashes() {
        let result = 2 + 2;
        dassert!(result == 5);
        log::info!("{}", result);
    }

    #[test]
    fn config_sanity() {
        let result = 2 + 2;
        let config = crate::AssertConfig {
            assertion_level: crate::AssertLevel::Error,
        };
        eassert!(config; result == 4);
        log::info!("{}", result);
    }

    #[test]
    #[should_panic]
    fn config_crashes() {
        let result = 2 + 2;
        let config = crate::AssertConfig {
            assertion_level: crate::AssertLevel::Warn,
        };
        eassert!(config; result == 3);
        log::info!("{}", result);
    }

    #[test]
    fn config_filters() {
        let result = 2 + 3;
        let config = crate::AssertConfig {
            assertion_level: crate::AssertLevel::Warn,
        };
        iassert!(config; result == 4);
        log::info!("{}", result);
    }
}

