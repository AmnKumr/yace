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

#[path = "emitter.rs"]
pub mod 𝗲𝗺𝗶𝘁𝘁𝗲𝗿;

pub use 𝗲𝗺𝗶𝘁𝘁𝗲𝗿::𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓;
pub use 𝗲𝗺𝗶𝘁𝘁𝗲𝗿::𝒃𝒚𝒕𝒆_𝒂𝒓𝒓𝒂𝒚_𝒆𝒎𝒊𝒕𝒕𝒆𝒓;
pub use 𝗲𝗺𝗶𝘁𝘁𝗲𝗿::𝒓𝒆𝒃𝒊𝒏𝒅_𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓;
// This type can not be created, it's intended to be used with 𝒓𝒆𝒃𝒊𝒏𝒅_𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓 only.
pub use 𝗲𝗺𝗶𝘁𝘁𝗲𝗿::𝐝𝐮𝐦𝐦𝐲_𝐞𝐦𝐢𝐭𝐭𝐞𝐫;

#[macro_use]
#[path = "enums.rs"]
pub mod 𝗲𝗻𝘂𝗺𝘀;

pub use 𝗲𝗻𝘂𝗺𝘀::𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫;
pub use 𝗲𝗻𝘂𝗺𝘀::𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐞𝐫𝐫𝐨𝐫;

#[cfg(test)]
pub(crate) use 𝗲𝗻𝘂𝗺𝘀::𝒘𝒓𝒂𝒑𝒑𝒆𝒓_𝒇𝒐𝒓_𝒕𝒆𝒔𝒕;

#[macro_use]
#[path = "risc-v/mod.rs"]
pub mod 𝗿𝗶𝘀𝗰_𝘃;

#[path = "rv32e.rs"]
pub mod 𝗿𝘃𝟯𝟮𝗲;

#[path = "rv32i.rs"]
pub mod 𝗿𝘃𝟯𝟮𝗶;

#[path = "rv64i.rs"]
pub mod 𝗿𝘃𝟲𝟰𝗶;

#[macro_use]
#[path = "x86/mod.rs"]
pub mod 𝘅𝟴𝟲;

#[path = "ia32.rs"]
pub mod 𝗶𝗮𝟯𝟮;

#[path = "x86-64.rs"]
pub mod 𝘅𝟴𝟲_𝟲𝟰;
