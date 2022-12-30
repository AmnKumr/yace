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

#[path = "basic_assembler.rs"]
pub(crate) mod 𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿;

#[macro_use]
#[path = "macros.rs"]
mod 𝗺𝗮𝗰𝗿𝗼𝘀;

#[path = "operands.rs"]
pub(crate) mod 𝗼𝗽𝗲𝗿𝗮𝗻𝗱𝘀;

pub use 𝗼𝗽𝗲𝗿𝗮𝗻𝗱𝘀::𝐁_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 as 𝐛𝐫𝐚𝐧𝐜𝐡_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞;
pub use 𝗼𝗽𝗲𝗿𝗮𝗻𝗱𝘀::𝐉_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 as 𝐣𝐮𝐦𝐩_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞;
pub use 𝗼𝗽𝗲𝗿𝗮𝗻𝗱𝘀::𝐈_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 as 𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞;
pub use 𝗼𝗽𝗲𝗿𝗮𝗻𝗱𝘀::𝐏_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 as 𝐩𝐫𝐞𝐟𝐞𝐭𝐜𝐡_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞;
pub use 𝗼𝗽𝗲𝗿𝗮𝗻𝗱𝘀::𝐔_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 as 𝐮𝐩𝐩𝐞𝐫_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞;
