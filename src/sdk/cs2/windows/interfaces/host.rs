/*
 * Created using https://github.com/a2x/cs2-dumper
 * Sun, 18 Feb 2024 04:14:17 +0000
 */

 #![allow(non_snake_case, non_upper_case_globals, dead_code)]

 pub mod CAnimScriptBase {
     pub const m_bIsValid: usize = 0x8; // bool
 }
 
 pub mod EmptyTestScript { // CAnimScriptBase
     pub const m_hTest: usize = 0x10; // CAnimScriptParam<float>
 }