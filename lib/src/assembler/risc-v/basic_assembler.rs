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

// To ensure safety we provice separate types for different classes of registers.
// But Rust compiler currently is not advanced enough to produce e.g. effective conversion from 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜 to
// 𝐧𝐨𝐳𝐞𝐫𝐨_𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐞𝐚𝐛𝐢: while you just need to copy value without doing anything compiler doesn't always optimize
// checks away.

// To make sure this woulnd't happen we are providing that conversion only for tests: 𝒘𝒓𝒂𝒑𝒑𝒆𝒓_𝒇𝒐𝒓_𝒕𝒆𝒔𝒕<𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜>
// would be converted to 𝒘𝒓𝒂𝒑𝒑𝒆𝒓_𝒇𝒐𝒓_𝒕𝒆𝒔𝒕<𝐧𝐨𝐳𝐞𝐫𝐨_𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐞𝐚𝐛𝐢> using safe code and matching values.

// The actual, production, conversion is compared to that one on the full range of 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜.

// This way we can guarantee that our tests are enough to ensure safety.
// Note: since mistakes here may trigger undefined behavior tests have to be run with “cargo +nightly miri test”.

𝖉𝖊𝖋𝖎𝖓𝖊_𝖊𝖓𝖚𝖒𝖘! {
    [{|value| !(1..=31).contains(&value)}]
    [   𝐧𝐨𝐳𝐞𝐫𝐨_𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐞𝐚𝐛𝐢,
        𝐧𝐨𝐳𝐞𝐫𝐨_𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐮𝐚𝐛𝐢
    ]
    [   𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜 {|value| value == 0},
        𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐞𝐚𝐛𝐢 {|value| value == 0},
        𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐮𝐚𝐛𝐢 {|value| value == 0}
    ]
    pub enum(i8, u8) 𝐧𝐨𝐳𝐞𝐫𝐨_𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜 {
        𝔵1 = 1,
        𝔵2 = 2,
        𝔵3 = 3,
        𝔵4 = 4,
        𝔵5 = 5,
        𝔵6 = 6,
        𝔵7 = 7,
        𝔵8 = 8,
        𝔵9 = 9,
        𝔵10 = 10,
        𝔵11 = 11,
        𝔵12 = 12,
        𝔵13 = 13,
        𝔵14 = 14,
        𝔵15 = 15,
        𝔵16 = 16,
        𝔵17 = 17,
        𝔵18 = 18,
        𝔵19 = 19,
        𝔵20 = 20,
        𝔵21 = 21,
        𝔵22 = 22,
        𝔵23 = 23,
        𝔵24 = 24,
        𝔵25 = 25,
        𝔵26 = 26,
        𝔵27 = 27,
        𝔵28 = 28,
        𝔵29 = 29,
        𝔵30 = 30,
        𝔵31 = 31
    }

    [{|value| !(1..=31).contains(&value)}]
    [   𝐧𝐨𝐳𝐞𝐫𝐨_𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜,
        𝐧𝐨𝐳𝐞𝐫𝐨_𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐮𝐚𝐛𝐢
    ]
    [   𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜 {|value| value == 0},
        𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐞𝐚𝐛𝐢 {|value| value == 0},
        𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐮𝐚𝐛𝐢 {|value| value == 0}
    ]
    pub enum(i8, u8) 𝐧𝐨𝐳𝐞𝐫𝐨_𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐞𝐚𝐛𝐢 {
        𝔯𝔞 = 1,
        𝔰𝔭 = 2,
        𝔤𝔭 = 3,
        𝔱𝔭 = 4,
        𝔱0 = 5,
        𝔰3 = 6,
        𝔰4 = 7,
        𝔰0 = 8,
        𝔰1 = 9,
        𝔞0 = 10,
        𝔞1 = 11,
        𝔞2 = 12,
        𝔞3 = 13,
        𝔰2 = 14,
        𝔱1 = 15,
        𝔰5 = 16,
        𝔰6 = 17,
        𝔰7 = 18,
        𝔰8 = 19,
        𝔰9 = 20,
        𝔰10 = 21,
        𝔰11 = 22,
        𝔰12 = 23,
        𝔰13 = 24,
        𝔰14 = 25,
        𝔰15 = 26,
        𝔰16 = 27,
        𝔰17 = 28,
        𝔰18 = 29,
        𝔰19 = 30,
        𝔰20 = 31
    }

    [{|value| !(1..=31).contains(&value)}]
    [   𝐧𝐨𝐳𝐞𝐫𝐨_𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜,
        𝐧𝐨𝐳𝐞𝐫𝐨_𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐞𝐚𝐛𝐢
    ]
    [   𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜 {|value| value == 0},
        𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐞𝐚𝐛𝐢 {|value| value == 0},
        𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐮𝐚𝐛𝐢 {|value| value == 0}
    ]
    pub enum(i8, u8) 𝐧𝐨𝐳𝐞𝐫𝐨_𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐮𝐚𝐛𝐢 {
        𝔯𝔞 = 1,
        𝔰𝔭 = 2,
        𝔤𝔭 = 3,
        𝔱𝔭 = 4,
        𝔱0 = 5,
        𝔱1 = 6,
        𝔱2 = 7,
        𝔰0 = 8,
        𝔰1 = 9,
        𝔞0 = 10,
        𝔞1 = 11,
        𝔞2 = 12,
        𝔞3 = 13,
        𝔞4 = 14,
        𝔞5 = 15,
        𝔞6 = 16,
        𝔞7 = 17,
        𝔰2 = 18,
        𝔰3 = 19,
        𝔰4 = 20,
        𝔰5 = 21,
        𝔰6 = 22,
        𝔰7 = 23,
        𝔰8 = 24,
        𝔰9 = 25,
        𝔰10 = 26,
        𝔰11 = 27,
        𝔱3 = 28,
        𝔱4 = 29,
        𝔱5 = 30,
        𝔱6 = 31
    }

    [{|value| !(0..=31).contains(&value)}]
    [   𝐧𝐨𝐳𝐞𝐫𝐨_𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜,
        𝐧𝐨𝐳𝐞𝐫𝐨_𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐞𝐚𝐛𝐢,
        𝐧𝐨𝐳𝐞𝐫𝐨_𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐮𝐚𝐛𝐢,
        𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐞𝐚𝐛𝐢,
        𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐮𝐚𝐛𝐢
    ]
    []
    pub enum(i8, u8) 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜 {
        𝔵0 = 0,
        𝔵1 = 1,
        𝔵2 = 2,
        𝔵3 = 3,
        𝔵4 = 4,
        𝔵5 = 5,
        𝔵6 = 6,
        𝔵7 = 7,
        𝔵8 = 8,
        𝔵9 = 9,
        𝔵10 = 10,
        𝔵11 = 11,
        𝔵12 = 12,
        𝔵13 = 13,
        𝔵14 = 14,
        𝔵15 = 15,
        𝔵16 = 16,
        𝔵17 = 17,
        𝔵18 = 18,
        𝔵19 = 19,
        𝔵20 = 20,
        𝔵21 = 21,
        𝔵22 = 22,
        𝔵23 = 23,
        𝔵24 = 24,
        𝔵25 = 25,
        𝔵26 = 26,
        𝔵27 = 27,
        𝔵28 = 28,
        𝔵29 = 29,
        𝔵30 = 30,
        𝔵31 = 31
    }

    [{|value| !(0..=31).contains(&value)}]
    [   𝐧𝐨𝐳𝐞𝐫𝐨_𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜,
        𝐧𝐨𝐳𝐞𝐫𝐨_𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐞𝐚𝐛𝐢,
        𝐧𝐨𝐳𝐞𝐫𝐨_𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐮𝐚𝐛𝐢,
        𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜,
        𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐮𝐚𝐛𝐢
    ]
    []
    pub enum(i8, u8) 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐞𝐚𝐛𝐢 {
        𝔷𝔢𝔯𝔬 = 0,
        𝔯𝔞 = 1,
        𝔰𝔭 = 2,
        𝔤𝔭 = 3,
        𝔱𝔭 = 4,
        𝔱0 = 5,
        𝔰3 = 6,
        𝔰4 = 7,
        𝔰0 = 8,
        𝔰1 = 9,
        𝔞0 = 10,
        𝔞1 = 11,
        𝔞2 = 12,
        𝔞3 = 13,
        𝔰2 = 14,
        𝔱1 = 15,
        𝔰5 = 16,
        𝔰6 = 17,
        𝔰7 = 18,
        𝔰8 = 19,
        𝔰9 = 20,
        𝔰10 = 21,
        𝔰11 = 22,
        𝔰12 = 23,
        𝔰13 = 24,
        𝔰14 = 25,
        𝔰15 = 26,
        𝔰16 = 27,
        𝔰17 = 28,
        𝔰18 = 29,
        𝔰19 = 30,
        𝔰20 = 31
    }

    [{|value| !(0..=31).contains(&value)}]
    [   𝐧𝐨𝐳𝐞𝐫𝐨_𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜,
        𝐧𝐨𝐳𝐞𝐫𝐨_𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐞𝐚𝐛𝐢,
        𝐧𝐨𝐳𝐞𝐫𝐨_𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐮𝐚𝐛𝐢,
        𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜,
        𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐞𝐚𝐛𝐢
    ]
    []
    pub enum(i8, u8) 𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐮𝐚𝐛𝐢 {
        𝔷𝔢𝔯𝔬 = 0,
        𝔯𝔞 = 1,
        𝔰𝔭 = 2,
        𝔤𝔭 = 3,
        𝔱𝔭 = 4,
        𝔱0 = 5,
        𝔱1 = 6,
        𝔱2 = 7,
        𝔰0 = 8,
        𝔰1 = 9,
        𝔞0 = 10,
        𝔞1 = 11,
        𝔞2 = 12,
        𝔞3 = 13,
        𝔞4 = 14,
        𝔞5 = 15,
        𝔞6 = 16,
        𝔞7 = 17,
        𝔰2 = 18,
        𝔰3 = 19,
        𝔰4 = 20,
        𝔰5 = 21,
        𝔰6 = 22,
        𝔰7 = 23,
        𝔰8 = 24,
        𝔰9 = 25,
        𝔰10 = 26,
        𝔰11 = 27,
        𝔱3 = 28,
        𝔱4 = 29,
        𝔱5 = 30,
        𝔱6 = 31
    }

    [{|value| !(0..=31).contains(&value)}]
    [𝐟𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐚𝐛𝐢]
    []
    pub enum(i8, u8) 𝐟𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜 {
        𝔣0 = 0,
        𝔣1 = 1,
        𝔣2 = 2,
        𝔣3 = 3,
        𝔣4 = 4,
        𝔣5 = 5,
        𝔣6 = 6,
        𝔣7 = 7,
        𝔣8 = 8,
        𝔣9 = 9,
        𝔣10 = 10,
        𝔣11 = 11,
        𝔣12 = 12,
        𝔣13 = 13,
        𝔣14 = 14,
        𝔣15 = 15,
        𝔣16 = 16,
        𝔣17 = 17,
        𝔣18 = 18,
        𝔣19 = 19,
        𝔣20 = 20,
        𝔣21 = 21,
        𝔣22 = 22,
        𝔣23 = 23,
        𝔣24 = 24,
        𝔣25 = 25,
        𝔣26 = 26,
        𝔣27 = 27,
        𝔣28 = 28,
        𝔣29 = 29,
        𝔣30 = 30,
        𝔣31 = 31
    }

    [{|value| !(0..=31).contains(&value)}]
    [𝐟𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜]
    []
    pub enum(i8, u8) 𝐟𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐚𝐛𝐢 {
        𝔣𝔱0 = 0,
        𝔣𝔱1 = 1,
        𝔣𝔱2 = 2,
        𝔣𝔱3 = 3,
        𝔣𝔱4 = 4,
        𝔣𝔱5 = 5,
        𝔣𝔱6 = 6,
        𝔣𝔱7 = 7,
        𝔣𝔰0 = 8,
        𝔣𝔰1 = 9,
        𝔣𝔞0 = 10,
        𝔣𝔞1 = 11,
        𝔣𝔞2 = 12,
        𝔣𝔞3 = 13,
        𝔣𝔞4 = 14,
        𝔣𝔞5 = 15,
        𝔣𝔞6 = 16,
        𝔣𝔞7 = 17,
        𝔣𝔰2 = 18,
        𝔣𝔰3 = 19,
        𝔣𝔰4 = 20,
        𝔣𝔰5 = 21,
        𝔣𝔰6 = 22,
        𝔣𝔰7 = 23,
        𝔣𝔰8 = 24,
        𝔣𝔰9 = 25,
        𝔣𝔰10 = 26,
        𝔣𝔰11 = 27,
        𝔣𝔱8 = 28,
        𝔣𝔱9 = 29,
        𝔣𝔱10 = 30,
        𝔣𝔱11 = 31
    }

    [{|value| !(0x001..=0x003).contains(&value) && !(0xc00..=0xc02).contains(&value) && !(0xc80..=0xc82).contains(&value)}]
    [𝐜𝐬𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐫𝐯𝟔𝟒]
    []
    pub enum(i16, u16) 𝐜𝐬𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐫𝐯𝟑𝟐 {
        𝔣𝔣𝔩𝔞𝔤𝔰 = 0x001,
        𝔣𝔯𝔪 = 0x002,
        𝔣𝔠𝔰𝔯 = 0x003,
        𝔠𝔶𝔠𝔩𝔢 = 0xc00,
        𝔱𝔦𝔪𝔢 = 0xc01,
        𝔦𝔫𝔰𝔱𝔯𝔢𝔱 = 0xc02,
        𝔠𝔶𝔠𝔩𝔢𝔥 = 0xc80,
        𝔱𝔦𝔪𝔢𝔥 = 0xc81,
        𝔦𝔫𝔰𝔱𝔯𝔢𝔱𝔥 = 0xc82
    }

    [{|value| !(0x001..=0x003).contains(&value) && !(0xc00..=0xc02).contains(&value)}]
    []
    [   𝐜𝐬𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐫𝐯𝟑𝟐 {|value| value >= 0xc80}]
    pub enum(i16, u16) 𝐜𝐬𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐫𝐯𝟔𝟒 {
        𝔣𝔣𝔩𝔞𝔤𝔰 = 0x001,
        𝔣𝔯𝔪 = 0x002,
        𝔣𝔠𝔰𝔯 = 0x003,
        𝔠𝔶𝔠𝔩𝔢 = 0xc00,
        𝔱𝔦𝔪𝔢 = 0xc01,
        𝔦𝔫𝔰𝔱𝔯𝔢𝔱 = 0xc02
    }
}
