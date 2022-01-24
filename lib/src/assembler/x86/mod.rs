/*
 * Permission is hereby granted, free of charge, to any human obtaining a copy of this software and associated documentation files
 * (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify,
 * merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit humans to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES
 * OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
 * LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 * CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

//! Documentation doesn't exist yet but we have some doctests.
//!
//! This code will compile:
//!
//! ```
//! # #![allow(uncommon_codepoints)]
//! # #![allow(non_camel_case_types)]
//! # use yace::𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝘅𝟴𝟲::{𝔞𝔡𝔡𝔯𝔢𝔰𝔰_32ᵇⁱᵗ,
//! #                            𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯64_𝔡𝔞𝔱𝔞32,
//! #                            𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯64_𝔡𝔞𝔱𝔞32,
//! #                            𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓};
//! #
//! # struct 𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫 {
//! #     𝖼𝗈𝗇𝗍𝖾𝗇𝗍: Vec<u8>,
//! # }
//! #
//! # impl 𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫 {
//! #     pub const fn new() -> Self {
//! #         𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫 {
//! #             𝖼𝗈𝗇𝗍𝖾𝗇𝗍: Vec::<u8>::new(),
//! #         }
//! #     }
//! # }
//! #
//! # impl 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓 for 𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫 {
//! #     type 𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞 = ();
//! #     type 𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞 = ();
//! #
//! #     fn emit_u8(&mut self, value: u8) -> Result<(), ()> {
//! #         self.𝖼𝗈𝗇𝗍𝖾𝗇𝗍.push(value);
//! #         Ok(())
//! #     }
//! # }
//! #
//! # type 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_64ᵇⁱᵗ =
//! #     𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯64_𝔡𝔞𝔱𝔞32<'static, 𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫>;
//! # let mut raw_emitter = 𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫::new();
//! 𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯64_𝔡𝔞𝔱𝔞32::from(&mut raw_emitter)
//!     .add((
//!         𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_64ᵇⁱᵗ::𝔞𝔩,
//!         𝔞𝔡𝔡𝔯𝔢𝔰𝔰_32ᵇⁱᵗ
//!             .with_base(𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_64ᵇⁱᵗ::𝔢𝔰𝔭)
//!             .with_index(𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_64ᵇⁱᵗ::𝔯15𝔡)
//!             .with_disp(0x12345678),
//!     ))
//!     .expect("Testing assembler");
//! # assert_eq!(
//! #     &[0x67, 0x42, 0x02, 0x84, 0x3c, 0x78, 0x56, 0x34, 0x12],
//! #     &raw_emitter.𝖼𝗈𝗇𝗍𝖾𝗇𝗍[0..raw_emitter.𝖼𝗈𝗇𝗍𝖾𝗇𝗍.len()]
//! # );
//! ```
//!
//! This code works fine, too:
//!
//! ```
//! # #![allow(uncommon_codepoints)]
//! # #![allow(non_camel_case_types)]
//! # use yace::𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝘅𝟴𝟲::{𝔞𝔡𝔡𝔯𝔢𝔰𝔰_32ᵇⁱᵗ,
//! #                            𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯64_𝔡𝔞𝔱𝔞32,
//! #                            𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯64_𝔡𝔞𝔱𝔞32,
//! #                            𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓};
//! #
//! # struct 𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫 {
//! #     𝖼𝗈𝗇𝗍𝖾𝗇𝗍: Vec<u8>,
//! # }
//! #
//! # impl 𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫 {
//! #     pub const fn new() -> Self {
//! #         𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫 {
//! #             𝖼𝗈𝗇𝗍𝖾𝗇𝗍: Vec::<u8>::new(),
//! #         }
//! #     }
//! # }
//! #
//! # impl 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓 for 𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫 {
//! #     type 𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞 = ();
//! #     type 𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞 = ();
//! #
//! #     fn emit_u8(&mut self, value: u8) -> Result<(), ()> {
//! #         self.𝖼𝗈𝗇𝗍𝖾𝗇𝗍.push(value);
//! #         Ok(())
//! #     }
//! # }
//! #
//! # type 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_64ᵇⁱᵗ =
//! #     𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯64_𝔡𝔞𝔱𝔞32<'static, 𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫>;
//! # let mut raw_emitter = 𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫::new();
//! 𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯64_𝔡𝔞𝔱𝔞32::from(&mut raw_emitter)
//!     .add((
//!         𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_64ᵇⁱᵗ::𝔞𝔥,
//!         𝔞𝔡𝔡𝔯𝔢𝔰𝔰_32ᵇⁱᵗ
//!             .with_base(𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_64ᵇⁱᵗ::𝔢𝔰𝔭)
//!             .with_index(𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_64ᵇⁱᵗ::𝔢𝔟𝔭)
//!             .with_disp(0x12345678),
//!     ))
//!     .expect("Testing assembler");
//! # assert_eq!(
//! #     &[0x67, 0x02, 0xa4, 0x2c, 0x78, 0x56, 0x34, 0x12],
//! #     &raw_emitter.𝖼𝗈𝗇𝗍𝖾𝗇𝗍[0..raw_emitter.𝖼𝗈𝗇𝗍𝖾𝗇𝗍.len()]
//! # );
//! ```
//!
//! But that one wouldn't compile because you can't use 𝔞𝔥 and 𝔯15𝔡 in the same instruction.
//!
//! ```compile_fail,E0277
//! # #![allow(uncommon_codepoints)]
//! # #![allow(non_camel_case_types)]
//! # use yace::𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝘅𝟴𝟲::{𝔞𝔡𝔡𝔯𝔢𝔰𝔰_32ᵇⁱᵗ,
//! #                            𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯64_𝔡𝔞𝔱𝔞32,
//! #                            𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯64_𝔡𝔞𝔱𝔞32,
//! #                            𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓};
//! #
//! # struct 𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫 {
//! #     𝖼𝗈𝗇𝗍𝖾𝗇𝗍: Vec<u8>,
//! # }
//! #
//! # impl 𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫 {
//! #     pub const fn new() -> Self {
//! #         𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫 {
//! #             𝖼𝗈𝗇𝗍𝖾𝗇𝗍: Vec::<u8>::new(),
//! #         }
//! #     }
//! # }
//! #
//! # impl 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓 for 𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫 {
//! #     type 𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞 = ();
//! #     type 𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞 = ();
//! #
//! #     fn emit_u8(&mut self, value: u8) -> Result<(), ()> {
//! #         self.𝖼𝗈𝗇𝗍𝖾𝗇𝗍.push(value);
//! #         Ok(())
//! #     }
//! # }
//! #
//! # type 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_64ᵇⁱᵗ =
//! #     𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯64_𝔡𝔞𝔱𝔞32<'static, 𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫>;
//! # let mut raw_emitter = 𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫::new();
//! 𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯64_𝔡𝔞𝔱𝔞32::from(&mut raw_emitter)
//!     .add((
//!         𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_64ᵇⁱᵗ::𝔞𝔥,
//!         𝔞𝔡𝔡𝔯𝔢𝔰𝔰_32ᵇⁱᵗ
//!             .with_base(𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_64ᵇⁱᵗ::𝔢𝔰𝔭)
//!             .with_index(𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_64ᵇⁱᵗ::𝔯15𝔡)
//!             .with_disp(0x12345678),
//!     ))
//!     .expect("Testing assembler");
//! # assert_eq!(
//! #     &[0x67, 0x02, 0xa4, 0x3c, 0x78, 0x56, 0x34, 0x12],
//! #     &raw_emitter.𝖼𝗈𝗇𝗍𝖾𝗇𝗍[0..raw_emitter.𝖼𝗈𝗇𝗍𝖾𝗇𝗍.len()]
//! # );
//! ```

use yace_codegen::𝖋𝖎𝖑𝖙𝖊𝖗_𝖝𝟴𝟲_𝖒𝖆𝖗𝖐𝖊𝖗𝖘;

#[path = "basic_assembler.rs"]
mod 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿;

#[path = "implementation.rs"]
mod 𝗶𝗺𝗽𝗹𝗲𝗺𝗲𝗻𝘁𝗮𝘁𝗶𝗼𝗻;
use 𝗶𝗺𝗽𝗹𝗲𝗺𝗲𝗻𝘁𝗮𝘁𝗶𝗼𝗻::*;

#[macro_use]
#[path = "macros.rs"]
pub mod 𝗺𝗮𝗰𝗿𝗼𝘀;

#[cfg(feature = "std")]
#[path = "tests.rs"]
#[cfg(test)]
mod 𝘁𝗲𝘀𝘁𝘀;

𝖉𝖊𝖋𝖎𝖓𝖊_𝖆𝖘𝖘𝖊𝖒𝖇𝖑𝖊𝖗_𝖙𝖗𝖆𝖎𝖙! {
    [𝔞𝔡𝔡𝔯16 𝔡𝔞𝔱𝔞16]
    pub trait 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯16_𝔡𝔞𝔱𝔞16 {
    }

    [𝔞𝔡𝔡𝔯16 𝔡𝔞𝔱𝔞16 ₓ𝔦𝔷]
    pub trait 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯16_𝒘𝒊𝒕𝒉_𝒆𝒊𝒛_𝔡𝔞𝔱𝔞16 {
    }

    [𝔞𝔡𝔡𝔯16 𝔡𝔞𝔱𝔞32]
    pub trait 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯16_𝔡𝔞𝔱𝔞32 {
    }

    [𝔞𝔡𝔡𝔯16 𝔡𝔞𝔱𝔞32 ₓ𝔦𝔷]
    pub trait 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯16_𝒘𝒊𝒕𝒉_𝒆𝒊𝒛_𝔡𝔞𝔱𝔞32 {
    }

    [𝔞𝔡𝔡𝔯32 𝔡𝔞𝔱𝔞16]
    pub trait 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯32_𝔡𝔞𝔱𝔞16 {
    }

    [𝔞𝔡𝔡𝔯32 𝔡𝔞𝔱𝔞16 ₓ𝔦𝔷]
    pub trait 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯32_𝒘𝒊𝒕𝒉_𝒆𝒊𝒛_𝔡𝔞𝔱𝔞16 {
    }

    [𝔞𝔡𝔡𝔯32 𝔡𝔞𝔱𝔞32]
    pub trait 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯32_𝔡𝔞𝔱𝔞32 {
    }

    [𝔞𝔡𝔡𝔯32 𝔡𝔞𝔱𝔞32 ₓ𝔦𝔷]
    pub trait 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯32_𝒘𝒊𝒕𝒉_𝒆𝒊𝒛_𝔡𝔞𝔱𝔞32 {
    }

    [𝔞𝔡𝔡𝔯64 𝔡𝔞𝔱𝔞32]
    pub trait 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯64_𝔡𝔞𝔱𝔞32 {
    }

    [𝔞𝔡𝔡𝔯64 𝔡𝔞𝔱𝔞32 ₓ𝔦𝔷]
    pub trait 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯64_𝒘𝒊𝒕𝒉_𝒓𝒊𝒛_𝔡𝔞𝔱𝔞32 {
    }

    [𝔞𝔡𝔡𝔯64 𝔡𝔞𝔱𝔞32 ₐᵥₓ512]
    pub trait 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯64_𝔡𝔞𝔱𝔞32_ₐᵥₓ512 {
    }

    [𝔞𝔡𝔡𝔯64 𝔡𝔞𝔱𝔞32 ₓ𝔦𝔷 ₐᵥₓ512]
    pub trait 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯64_𝒘𝒊𝒕𝒉_𝒓𝒊𝒛_𝔡𝔞𝔱𝔞32_ₐᵥₓ512 {
    }
}

𝖉𝖊𝖋𝖎𝖓𝖊_𝖆𝖘𝖘𝖊𝖒𝖇𝖑𝖊𝖗_𝖘𝖙𝖗𝖚𝖈𝖙! {
    [𝔞𝔡𝔡𝔯16 𝔡𝔞𝔱𝔞16]
    pub struct 𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯16_𝔡𝔞𝔱𝔞16[<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ, 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>]
    as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯16_𝔡𝔞𝔱𝔞16 {
    }

    [𝔞𝔡𝔡𝔯16 𝔡𝔞𝔱𝔞16 ₓ𝔦𝔷]
    pub struct 𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯16_𝐰𝐢𝐭𝐡_𝐞𝐢𝐳_𝔡𝔞𝔱𝔞16[<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ, 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>]
    as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯16_𝔡𝔞𝔱𝔞16 {
    }

    [𝔞𝔡𝔡𝔯16 𝔡𝔞𝔱𝔞16 ₓ𝔦𝔷 ₓ𝔦𝔷]
    impl[<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ, 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>] 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯16_𝒘𝒊𝒕𝒉_𝒆𝒊𝒛_𝔡𝔞𝔱𝔞16
    for 𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯16_𝐰𝐢𝐭𝐡_𝐞𝐢𝐳_𝔡𝔞𝔱𝔞16[<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ, 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮>];

    [𝔞𝔡𝔡𝔯16 𝔡𝔞𝔱𝔞32]
    pub struct 𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯16_𝔡𝔞𝔱𝔞32[<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ, 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>]
    as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯16_𝔡𝔞𝔱𝔞32 {
    }

    [𝔞𝔡𝔡𝔯16 𝔡𝔞𝔱𝔞32 ₓ𝔦𝔷]
    pub struct 𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯16_𝐰𝐢𝐭𝐡_𝐞𝐢𝐳_𝔡𝔞𝔱𝔞32[<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ, 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>]
    as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯16_𝔡𝔞𝔱𝔞32 {
    }

    [𝔞𝔡𝔡𝔯16 𝔡𝔞𝔱𝔞32 ₓ𝔦𝔷 ₓ𝔦𝔷]
    impl[<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ, 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>] 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯16_𝒘𝒊𝒕𝒉_𝒆𝒊𝒛_𝔡𝔞𝔱𝔞32
    for 𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯16_𝐰𝐢𝐭𝐡_𝐞𝐢𝐳_𝔡𝔞𝔱𝔞32[<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ, 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮>];

    [𝔞𝔡𝔡𝔯32 𝔡𝔞𝔱𝔞16]
    pub struct 𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯32_𝔡𝔞𝔱𝔞16[<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ, 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>]
    as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯32_𝔡𝔞𝔱𝔞16 {
    }

    [𝔞𝔡𝔡𝔯32 𝔡𝔞𝔱𝔞16 ₓ𝔦𝔷]
    pub struct 𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯32_𝐰𝐢𝐭𝐡_𝐞𝐢𝐳_𝔡𝔞𝔱𝔞16[<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ, 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>]
    as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯32_𝔡𝔞𝔱𝔞16 {
    }

    [𝔞𝔡𝔡𝔯32 𝔡𝔞𝔱𝔞16 ₓ𝔦𝔷 ₓ𝔦𝔷]
    impl[<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ, 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>] 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯32_𝒘𝒊𝒕𝒉_𝒆𝒊𝒛_𝔡𝔞𝔱𝔞16
    for 𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯32_𝐰𝐢𝐭𝐡_𝐞𝐢𝐳_𝔡𝔞𝔱𝔞16[<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ, 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮>];

    [𝔞𝔡𝔡𝔯32 𝔡𝔞𝔱𝔞32]
    pub struct 𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯32_𝔡𝔞𝔱𝔞32[<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ, 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>]
    as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯32_𝔡𝔞𝔱𝔞32 {
    }

    [𝔞𝔡𝔡𝔯32 𝔡𝔞𝔱𝔞32 ₓ𝔦𝔷]
    pub struct 𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯32_𝐰𝐢𝐭𝐡_𝐞𝐢𝐳_𝔡𝔞𝔱𝔞32[<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ, 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>]
    as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯32_𝔡𝔞𝔱𝔞32 {
    }

    [𝔞𝔡𝔡𝔯32 𝔡𝔞𝔱𝔞32 ₓ𝔦𝔷 ₓ𝔦𝔷]
    impl[<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ, 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>] 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯32_𝒘𝒊𝒕𝒉_𝒆𝒊𝒛_𝔡𝔞𝔱𝔞32
    for 𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯32_𝐰𝐢𝐭𝐡_𝐞𝐢𝐳_𝔡𝔞𝔱𝔞32[<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ, 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮>];

    [𝔞𝔡𝔡𝔯64 𝔡𝔞𝔱𝔞32]
    pub struct 𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯64_𝔡𝔞𝔱𝔞32[<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ, 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>]
    as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯64_𝔡𝔞𝔱𝔞32 {
    }

    [𝔞𝔡𝔡𝔯64 𝔡𝔞𝔱𝔞32 ₐᵥₓ512]
    pub struct 𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯64_𝔡𝔞𝔱𝔞32_ₐᵥₓ512[<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ, 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>]
    as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯64_𝔡𝔞𝔱𝔞32_ₐᵥₓ512 {
    }

    [𝔞𝔡𝔡𝔯64 𝔡𝔞𝔱𝔞32 ₓ𝔦𝔷]
    pub struct 𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯64_𝐰𝐢𝐭𝐡_𝐫𝐢𝐳_𝔡𝔞𝔱𝔞32[<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ, 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>]
    as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯64_𝔡𝔞𝔱𝔞32 {
    }

    [𝔞𝔡𝔡𝔯64 𝔡𝔞𝔱𝔞32 ₓ𝔦𝔷 ₓ𝔦𝔷]
    impl[<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ, 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>] 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯64_𝒘𝒊𝒕𝒉_𝒓𝒊𝒛_𝔡𝔞𝔱𝔞32
    for 𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯64_𝐰𝐢𝐭𝐡_𝐫𝐢𝐳_𝔡𝔞𝔱𝔞32[<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ, 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮>];

    [𝔞𝔡𝔡𝔯64 𝔡𝔞𝔱𝔞32 ₓ𝔦𝔷 ₐᵥₓ512]
    pub struct 𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯64_𝐰𝐢𝐭𝐡_𝐫𝐢𝐳_𝔡𝔞𝔱𝔞32_ₐᵥₓ512[<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ, 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>]
    as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯64_𝔡𝔞𝔱𝔞32_ₐᵥₓ512 {
    }

    [𝔞𝔡𝔡𝔯64 𝔡𝔞𝔱𝔞32 ₓ𝔦𝔷 ₓ𝔦𝔷 ₐᵥₓ512]
    impl[<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ, 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>] 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯64_𝒘𝒊𝒕𝒉_𝒓𝒊𝒛_𝔡𝔞𝔱𝔞32_ₐᵥₓ512
    for 𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯64_𝐰𝐢𝐭𝐡_𝐫𝐢𝐳_𝔡𝔞𝔱𝔞32_ₐᵥₓ512[<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ, 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮>];
}

𝖉𝖊𝖋𝖎𝖓𝖊_𝖆𝖘𝖘𝖊𝖒𝖇𝖑𝖊𝖗_𝖘𝖚𝖕𝖕𝖑𝖊𝖒𝖊𝖓𝖙𝖆𝖗𝖞_𝖙𝖗𝖆𝖎𝖙𝖘! {
    [𝔞𝔡𝔡𝔯16 𝔡𝔞𝔱𝔞16]
    impl[<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ, 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>] for 𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯16_𝔡𝔞𝔱𝔞16
    as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯16_𝔡𝔞𝔱𝔞16;

    [𝔞𝔡𝔡𝔯16 𝔡𝔞𝔱𝔞16 ₓ𝔦𝔷]
    impl[<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ, 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>] for 𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯16_𝐰𝐢𝐭𝐡_𝐞𝐢𝐳_𝔡𝔞𝔱𝔞16
    as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯16_𝔡𝔞𝔱𝔞16;

    [𝔞𝔡𝔡𝔯16 𝔡𝔞𝔱𝔞32]
    impl[<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ, 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>] for 𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯16_𝔡𝔞𝔱𝔞32
    as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯16_𝔡𝔞𝔱𝔞32;

    [𝔞𝔡𝔡𝔯16 𝔡𝔞𝔱𝔞32 ₓ𝔦𝔷]
    impl[<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ, 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>] for 𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯16_𝐰𝐢𝐭𝐡_𝐞𝐢𝐳_𝔡𝔞𝔱𝔞32
    as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯16_𝔡𝔞𝔱𝔞32;

    [𝔞𝔡𝔡𝔯32 𝔡𝔞𝔱𝔞16]
    impl[<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ, 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>] for 𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯32_𝔡𝔞𝔱𝔞16
    as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯32_𝔡𝔞𝔱𝔞16;

    [𝔞𝔡𝔡𝔯32 𝔡𝔞𝔱𝔞16 ₓ𝔦𝔷]
    impl[<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ, 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>] for 𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯32_𝐰𝐢𝐭𝐡_𝐞𝐢𝐳_𝔡𝔞𝔱𝔞16
    as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯32_𝔡𝔞𝔱𝔞16;

    [𝔞𝔡𝔡𝔯32 𝔡𝔞𝔱𝔞32]
    impl[<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ, 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>] for 𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯32_𝔡𝔞𝔱𝔞32
    as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯32_𝔡𝔞𝔱𝔞32;

    [𝔞𝔡𝔡𝔯32 𝔡𝔞𝔱𝔞32 ₓ𝔦𝔷]
    impl[<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ, 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>] for 𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯32_𝐰𝐢𝐭𝐡_𝐞𝐢𝐳_𝔡𝔞𝔱𝔞32
    as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯32_𝔡𝔞𝔱𝔞32;

    [𝔞𝔡𝔡𝔯64 𝔡𝔞𝔱𝔞32]
    impl[<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ, 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>] for 𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯64_𝔡𝔞𝔱𝔞32
    as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯64_𝔡𝔞𝔱𝔞32;

    [𝔞𝔡𝔡𝔯64 𝔡𝔞𝔱𝔞32 ₐᵥₓ512]
    impl[<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ, 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>] for 𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯64_𝔡𝔞𝔱𝔞32_ₐᵥₓ512
    as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯64_𝔡𝔞𝔱𝔞32_ₐᵥₓ512;

    [𝔞𝔡𝔡𝔯64 𝔡𝔞𝔱𝔞32 ₓ𝔦𝔷]
    impl[<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ, 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>] for 𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯64_𝐰𝐢𝐭𝐡_𝐫𝐢𝐳_𝔡𝔞𝔱𝔞32
    as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯64_𝔡𝔞𝔱𝔞32;

    [𝔞𝔡𝔡𝔯64 𝔡𝔞𝔱𝔞32 ₓ𝔦𝔷 ₐᵥₓ512]
    impl[<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ, 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>] for 𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯64_𝐰𝐢𝐭𝐡_𝐫𝐢𝐳_𝔡𝔞𝔱𝔞32_ₐᵥₓ512
    as 𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝔞𝔡𝔡𝔯64_𝒘𝒊𝒕𝒉_𝒓𝒊𝒛_𝔡𝔞𝔱𝔞32_ₐᵥₓ512;
}

pub use 𝗶𝗺𝗽𝗹𝗲𝗺𝗲𝗻𝘁𝗮𝘁𝗶𝗼𝗻::𝔞𝔡𝔡𝔯𝔢𝔰𝔰_16ᵇⁱᵗ;
pub use 𝗶𝗺𝗽𝗹𝗲𝗺𝗲𝗻𝘁𝗮𝘁𝗶𝗼𝗻::𝔞𝔡𝔡𝔯𝔢𝔰𝔰_32ᵇⁱᵗ;
pub use 𝗶𝗺𝗽𝗹𝗲𝗺𝗲𝗻𝘁𝗮𝘁𝗶𝗼𝗻::𝔞𝔡𝔡𝔯𝔢𝔰𝔰_64ᵇⁱᵗ;

pub use 𝗶𝗺𝗽𝗹𝗲𝗺𝗲𝗻𝘁𝗮𝘁𝗶𝗼𝗻::𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086;
pub use 𝗶𝗺𝗽𝗹𝗲𝗺𝗲𝗻𝘁𝗮𝘁𝗶𝗼𝗻::𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86;
pub use 𝗶𝗺𝗽𝗹𝗲𝗺𝗲𝗻𝘁𝗮𝘁𝗶𝗼𝗻::𝒈𝒂𝒕𝒉𝒆𝒓_𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔;

pub use 𝗶𝗺𝗽𝗹𝗲𝗺𝗲𝗻𝘁𝗮𝘁𝗶𝗼𝗻::𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓;

mod 𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿_𝗶𝗻𝘀𝘁𝗿𝘂𝗰𝘁𝗶𝗼𝗻𝘀 {

// We are implementing two-level scheme with a set of ₓₓₓ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔 traits and a set of ₓₓₓ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏 traits.
//
// This allows us to avoid combinatiorial explosion: there are more than dozen of types which may represent just general purpose
// register argument and for three-argument instruction it would mean there are almost two thousand variants.
//
// ₓₓₓ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔 uses traits below to convert arguments to less diverse set and then ₓₓₓ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏 implement the remaining
// combinations.
//
// If all arguments would be handled identically, then of course, it wouldn't make much sense to even have these two levels.
// Instead some instructions use not 𝒈𝒆𝒏𝒆𝒓𝒊𝒄_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕 trait, but more specialized conversion traits.
//
// E.g. shift instructions use 𝒔𝒉𝒊𝒇𝒕_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒂𝒓𝒈𝒖𝒎𝒆𝒏𝒕 trait which only support i8 and 𝐜𝐨𝐮𝐧𝐭𝐞𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_8ᵇⁱᵗ arguments (and they
// just accept them without conversion).
//
// Thus way we both avoid the combinatorial explosion and guarantee that incorrect registers are excluded during the compilation
// time. Not only this makes debugging easier, this also means that we can still thinking about reporting these particular errors.
//
// Note: Even with this approach we have some extra variants to implement (e.g. add have separate version for accumulator and
// 𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫 and 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 because there are special version for 𝐚𝐜𝐜𝐮𝐦𝐮𝐥𝐚𝐭𝐨𝐫 and immediate, but overral it's less than 2x
// superfluous instructions.  This is considered acceptable.

use yace_codegen::𝖉𝖊𝖋𝖎𝖓𝖊_𝖆𝖘𝖘𝖊𝖒𝖇𝖑𝖊𝖗_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓𝖘;

use super::𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::*;

use super::𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86;
use super::𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086;
use super::𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓;

use super::𝗶𝗺𝗽𝗹𝗲𝗺𝗲𝗻𝘁𝗮𝘁𝗶𝗼𝗻::𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒐𝒑𝒆𝒓𝒂𝒏𝒅_𝒐𝒇_8ᵇⁱᵗ_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏;
use super::𝗶𝗺𝗽𝗹𝗲𝗺𝗲𝗻𝘁𝗮𝘁𝗶𝗼𝗻::𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒐𝒑𝒆𝒓𝒂𝒏𝒅_𝒔𝒆𝒑𝒂𝒓𝒂𝒕𝒆_𝒂𝒄𝒄𝒖𝒎𝒖𝒍𝒂𝒕𝒐𝒓;

use super::𝗶𝗺𝗽𝗹𝗲𝗺𝗲𝗻𝘁𝗮𝘁𝗶𝗼𝗻::𝐥𝐞𝐠𝐚𝐜𝐲_𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧;
use super::𝗶𝗺𝗽𝗹𝗲𝗺𝗲𝗻𝘁𝗮𝘁𝗶𝗼𝗻::𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞;
use super::𝗶𝗺𝗽𝗹𝗲𝗺𝗲𝗻𝘁𝗮𝘁𝗶𝗼𝗻::𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧_𝐛𝐲𝐭𝐞;

use super::𝗶𝗺𝗽𝗹𝗲𝗺𝗲𝗻𝘁𝗮𝘁𝗶𝗼𝗻::𝒆𝒎𝒊𝒕_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏;
use super::𝗶𝗺𝗽𝗹𝗲𝗺𝗲𝗻𝘁𝗮𝘁𝗶𝗼𝗻::𝒆𝒎𝒊𝒕_𝒎𝒆𝒎𝒐𝒓𝒚_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏;
use super::𝗶𝗺𝗽𝗹𝗲𝗺𝗲𝗻𝘁𝗮𝘁𝗶𝗼𝗻::𝒆𝒎𝒊𝒕_𝒑𝒓𝒆𝒇𝒊𝒙𝒆𝒔_𝒂𝒏𝒅_𝒐𝒑𝒄𝒐𝒅𝒆;

𝖉𝖊𝖋𝖎𝖓𝖊_𝖆𝖘𝖘𝖊𝖒𝖇𝖑𝖊𝖗_𝖎𝖓𝖘𝖙𝖗𝖚𝖈𝖙𝖎𝖔𝖓𝖘!{}

pub trait 𝒍𝒆𝒈𝒂𝒄𝒚_𝒎𝒐𝒅𝒆_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓 {
    type 𝐝𝐚𝐭𝐚_𝐩𝐫𝐞𝐟𝐢𝐱_16ᵇⁱᵗ;
    type 𝐝𝐚𝐭𝐚_𝐩𝐫𝐞𝐟𝐢𝐱_32ᵇⁱᵗ;
    type 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐩𝐫𝐞𝐟𝐢𝐱_16ᵇⁱᵗ;
    type 𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐩𝐫𝐞𝐟𝐢𝐱_32ᵇⁱᵗ;
}

}
use 𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿_𝗶𝗻𝘀𝘁𝗿𝘂𝗰𝘁𝗶𝗼𝗻𝘀::*;
