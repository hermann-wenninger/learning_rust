#[allow(dead_code)]


#![crate_type = "lib"]

#![crate_name = "rary"]


#[cfg(target_os = "linux")]
if cfg!(target_os = "linux")
#[cfg(not(target_os = "linux"))]


#[cfg(some_condition)]
$ rustc --cfg some_condition custom.rs && ./custom

//testing
#[test]
#[ignore = "not yet implemented"]
#[test]
#[should_panic(expected = "values don't match")]

//derive
#[derive(PartialEq, Clone)]

//diagnose
#[allow(missing_docs)]
#[warn(missing_docs)]
#[deny(missing_docs)]
#[forbid(missing_docs)]
#[allow(unused)]
#[deny(unused_must_use)]
#[deny(warnings)]
#[warn(unsafe_code)]
#![warn(clippy::pedantic)]
#![allow(clippy::filter_map)]
#[allow(clippy::cmp_nan)]
#[deprecated(since = "5.2.0", note = "foo was rarely used. Users should instead use bar")]
#[must_use]

//codegen
#[inline] 
#[inline(always)]
#[inline(never)]
#[target_feature(enable = "avx2")]
#[track_caller]

//limits
#![recursion_limit = "4"]
#![type_length_limit = "4"]


//type sytem
#[non_exhaustive]

//debugger
#![debugger_visualizer(natvis_file = "Rectangle.natvis")]
#![debugger_visualizer(gdb_script_file = "printer.py")]