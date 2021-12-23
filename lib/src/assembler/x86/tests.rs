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
use super::*;

struct 𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫 {
    𝖼𝗈𝗇𝗍𝖾𝗇𝗍: Vec<u8>,
}

impl 𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫 {
    pub const fn new() -> Self {
        𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫 {
            𝖼𝗈𝗇𝗍𝖾𝗇𝗍: Vec::<u8>::new(),
        }
    }
}

impl 𝗶𝗺𝗽𝗹𝗲𝗺𝗲𝗻𝘁𝗮𝘁𝗶𝗼𝗻::𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓 for 𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫 {
    type 𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞 = ();
    type 𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞 = ();

    fn emit_u8(&mut self, value: u8) -> Result<(), ()> {
        self.𝖼𝗈𝗇𝗍𝖾𝗇𝗍.push(value);
        Ok(())
    }
}

#[test]
fn test_add_𝔞𝔩_𝔞𝔩() {
    type 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_16ᵇⁱᵗ =
        𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯16_𝔡𝔞𝔱𝔞16<'static, 𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫>;
    let mut raw_emitter = 𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫::new();
    𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯16_𝔡𝔞𝔱𝔞16::from(&mut raw_emitter)
        .add((𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_16ᵇⁱᵗ::𝔞𝔩, 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_16ᵇⁱᵗ::𝔞𝔩))
        .expect("Testing assembler");
    assert_eq!(&[0x00, 0xc0], &raw_emitter.𝖼𝗈𝗇𝗍𝖾𝗇𝗍[0..raw_emitter.𝖼𝗈𝗇𝗍𝖾𝗇𝗍.len()]);
}

#[test]
fn test_add_𝔞𝔩_𝔞𝔡𝔡𝔯𝔢𝔰𝔰_16ᵇⁱᵗ_𝔟𝔵_𝔰𝔦() {
    type 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_16ᵇⁱᵗ =
        𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯16_𝔡𝔞𝔱𝔞16<'static, 𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫>;
    let mut raw_emitter = 𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫::new();
    𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯16_𝔡𝔞𝔱𝔞16::from(&mut raw_emitter)
        .add((
            𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_16ᵇⁱᵗ::𝔞𝔩,
            𝔞𝔡𝔡𝔯𝔢𝔰𝔰_16ᵇⁱᵗ
                .with_base(𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_16ᵇⁱᵗ::𝔟𝔵)
                .with_index(𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_16ᵇⁱᵗ::𝔰𝔦)
                .with_disp(0x1234i16),
        ))
        .expect("Testing assembler");
    assert_eq!(
        &[0x00, 0x80, 0x34, 0x12],
        &raw_emitter.𝖼𝗈𝗇𝗍𝖾𝗇𝗍[0..raw_emitter.𝖼𝗈𝗇𝗍𝖾𝗇𝗍.len()]
    );
}

#[test]
fn test_add_𝔞𝔩_𝔞𝔡𝔡𝔯𝔢𝔰𝔰_32ᵇⁱᵗ_𝔢𝔰𝔭_𝔢𝔟𝔭() {
    type 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_16ᵇⁱᵗ =
        𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯16_𝔡𝔞𝔱𝔞16<'static, 𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫>;
    let mut raw_emitter = 𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫::new();
    𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯16_𝔡𝔞𝔱𝔞16::from(&mut raw_emitter)
        .add((
            𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_16ᵇⁱᵗ::𝔞𝔩,
            𝔞𝔡𝔡𝔯𝔢𝔰𝔰_32ᵇⁱᵗ
                .with_base(𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_16ᵇⁱᵗ::𝔢𝔰𝔭)
                .with_index(𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_16ᵇⁱᵗ::𝔢𝔟𝔭)
                .with_disp(0x12345678),
        ))
        .expect("Testing assembler");
    assert_eq!(
        &[0x67, 0x00, 0x84, 0x2c, 0x78, 0x56, 0x34, 0x12],
        &raw_emitter.𝖼𝗈𝗇𝗍𝖾𝗇𝗍[0..raw_emitter.𝖼𝗈𝗇𝗍𝖾𝗇𝗍.len()]
    );
}

#[test]
fn test_add_𝔞𝔩_𝔞𝔡𝔡𝔯𝔢𝔰𝔰_64ᵇⁱᵗ_𝔢𝔰𝔭_𝔢𝔟𝔭() {
    type 𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_64ᵇⁱᵗ =
        𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯64_𝔡𝔞𝔱𝔞32<'static, 𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫>;
    let mut raw_emitter = 𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫::new();
    𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_𝔞𝔡𝔡𝔯64_𝔡𝔞𝔱𝔞32::from(&mut raw_emitter)
        .add((
            𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_64ᵇⁱᵗ::𝔞𝔩,
            𝔞𝔡𝔡𝔯𝔢𝔰𝔰_32ᵇⁱᵗ
                .with_base(𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_64ᵇⁱᵗ::𝔢𝔰𝔭)
                .with_index(𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫_64ᵇⁱᵗ::𝔢𝔟𝔭)
                .with_disp(0x12345678),
        ))
        .expect("Testing assembler");
    assert_eq!(
        &[0x67, 0x00, 0x84, 0x2c, 0x78, 0x56, 0x34, 0x12],
        &raw_emitter.𝖼𝗈𝗇𝗍𝖾𝗇𝗍[0..raw_emitter.𝖼𝗈𝗇𝗍𝖾𝗇𝗍.len()]
    );
}

// This takes takes few second without miri but few hours when miri is used.
// And there are no unsafe code in it so just skip it on miri.
#[cfg(not(miri))]
#[test]
fn test_emit_legacy_instruction_sanity() {
    use 𝗶𝗺𝗽𝗹𝗲𝗺𝗲𝗻𝘁𝗮𝘁𝗶𝗼𝗻::𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓;

    macro_rules! 𝖊𝖓𝖚𝖒_𝖑𝖎𝖘𝖙 {
        ($𝓮𝓷𝓾𝓶_𝓽𝔂𝓹𝓮:ty[𝔫𝔬𝔫𝔢, $($𝓮𝓷𝓾𝓶_𝓿𝓪𝓵𝓾𝓮:ident),*]) => {
            [None, $(Some(<$𝓮𝓷𝓾𝓶_𝓽𝔂𝓹𝓮>::$𝓮𝓷𝓾𝓶_𝓿𝓪𝓵𝓾𝓮)),*]
        };
        ($𝓮𝓷𝓾𝓶_𝓽𝔂𝓹𝓮:ty[$($𝓮𝓷𝓾𝓶_𝓿𝓪𝓵𝓾𝓮:ident),*]) => {
            [$(<$𝓮𝓷𝓾𝓶_𝓽𝔂𝓹𝓮>::$𝓮𝓷𝓾𝓶_𝓿𝓪𝓵𝓾𝓮),*]
        }
    }

    macro_rules! 𝖛𝖊𝖗𝖎𝖋𝖞_𝖊𝖒𝖎𝖙𝖙𝖊𝖗 {
        ($𝓫𝓪𝓼𝓮_𝓯𝓾𝓷𝓬𝓽𝓲𝓸𝓷:ident,
         [$($𝓭𝓮𝓻𝓲𝓿𝓮𝓭_𝓯𝓾𝓷𝓬𝓽𝓲𝓸𝓷:ident [$($𝓮𝔁𝓽𝓻𝓪_𝓮𝓶𝓲𝓽:tt)*] [$($𝓮𝔁𝓽𝓻𝓪_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽𝓼:tt)*]),*],
         $𝓻𝓮𝓰_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽:ident, $𝓻𝓶_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽:ident
        ) => {
            $(
                let emitter1 = &mut 𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫::new();
                let emitter2 = &mut 𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫::new();
                <𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫 as 𝗶𝗺𝗽𝗹𝗲𝗺𝗲𝗻𝘁𝗮𝘁𝗶𝗼𝗻::𝒆𝒎𝒊𝒕_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<
                    <𝗶𝗺𝗽𝗹𝗲𝗺𝗲𝗻𝘁𝗮𝘁𝗶𝗼𝗻::𝐥𝐞𝐠𝐚𝐜𝐲_𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧 as 𝗶𝗺𝗽𝗹𝗲𝗺𝗲𝗻𝘁𝗮𝘁𝗶𝗼𝗻::𝒂𝒅𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<0x00>>::𝐭𝐚𝐫𝐠𝐞𝐭>>::
                    $𝓫𝓪𝓼𝓮_𝓯𝓾𝓷𝓬𝓽𝓲𝓸𝓷(
                        emitter1,
                            $𝓻𝓮𝓰_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽,
                            $𝓻𝓶_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽)
                    .expect("Testing assembler");
                emitter1.$($𝓮𝔁𝓽𝓻𝓪_𝓮𝓶𝓲𝓽)*.expect("Testing assembler");
                <𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫 as 𝗶𝗺𝗽𝗹𝗲𝗺𝗲𝗻𝘁𝗮𝘁𝗶𝗼𝗻::𝒆𝒎𝒊𝒕_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<
                    <𝗶𝗺𝗽𝗹𝗲𝗺𝗲𝗻𝘁𝗮𝘁𝗶𝗼𝗻::𝐥𝐞𝐠𝐚𝐜𝐲_𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧 as 𝗶𝗺𝗽𝗹𝗲𝗺𝗲𝗻𝘁𝗮𝘁𝗶𝗼𝗻::𝒂𝒅𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<0x00>>::𝐭𝐚𝐫𝐠𝐞𝐭>>::
                    $𝓭𝓮𝓻𝓲𝓿𝓮𝓭_𝓯𝓾𝓷𝓬𝓽𝓲𝓸𝓷(
                        emitter2,
                            $𝓻𝓮𝓰_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽,
                            $𝓻𝓶_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽,
                            $($𝓮𝔁𝓽𝓻𝓪_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽𝓼)*)
                    .expect("Testing assembler");
                assert_eq!(
                    &emitter1.𝖼𝗈𝗇𝗍𝖾𝗇𝗍[0..emitter1.𝖼𝗈𝗇𝗍𝖾𝗇𝗍.len()],
                    &emitter2.𝖼𝗈𝗇𝗍𝖾𝗇𝗍[0..emitter2.𝖼𝗈𝗇𝗍𝖾𝗇𝗍.len()]
                );
             )*
        };
        ($𝓫𝓪𝓼𝓮_𝓯𝓾𝓷𝓬𝓽𝓲𝓸𝓷:ident,
         [$($𝓭𝓮𝓻𝓲𝓿𝓮𝓭_𝓯𝓾𝓷𝓬𝓽𝓲𝓸𝓷:ident [$($𝓮𝔁𝓽𝓻𝓪_𝓮𝓶𝓲𝓽:tt)*] [$($𝓮𝔁𝓽𝓻𝓪_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽𝓼:tt)*]),*], $𝓻𝓮𝓰_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽:ident,
         𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<$𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽:ident $𝓫𝓪𝓼𝓮_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽:ident $𝓲𝓷𝓭𝓮𝔁_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽:ident $𝓭𝓲𝓼𝓹_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽:ident>
        ) => {
            $(
                let emitter1 = &mut 𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫::new();
                let emitter2 = &mut 𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫::new();
                <𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫 as 𝗶𝗺𝗽𝗹𝗲𝗺𝗲𝗻𝘁𝗮𝘁𝗶𝗼𝗻::𝒆𝒎𝒊𝒕_𝒎𝒆𝒎𝒐𝒓𝒚_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<
                    <𝗶𝗺𝗽𝗹𝗲𝗺𝗲𝗻𝘁𝗮𝘁𝗶𝗼𝗻::𝐥𝐞𝐠𝐚𝐜𝐲_𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧 as 𝗶𝗺𝗽𝗹𝗲𝗺𝗲𝗻𝘁𝗮𝘁𝗶𝗼𝗻::𝒂𝒅𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<0x00>>::𝐭𝐚𝐫𝐠𝐞𝐭>>::
                    $𝓫𝓪𝓼𝓮_𝓯𝓾𝓷𝓬𝓽𝓲𝓸𝓷(
                        emitter1,
                            $𝓻𝓮𝓰_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽,
                            𝗶𝗺𝗽𝗹𝗲𝗺𝗲𝗻𝘁𝗮𝘁𝗶𝗼𝗻::𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086::<𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,
                                                           𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
                                                           𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
                                                           i16,
                                                           0> {
                                𝗌𝖾𝗀𝗆𝖾𝗇𝗍: $𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽,
                                𝖻𝖺𝗌𝖾: $𝓫𝓪𝓼𝓮_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽,
                                𝗂𝗇𝖽𝖾𝗑: $𝓲𝓷𝓭𝓮𝔁_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽,
                                𝖽𝗂𝗌𝗉: $𝓭𝓲𝓼𝓹_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽
                            })
                    .expect("Testing assembler");
                emitter1.$($𝓮𝔁𝓽𝓻𝓪_𝓮𝓶𝓲𝓽)*.expect("Testing assembler");
                <𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫 as 𝗶𝗺𝗽𝗹𝗲𝗺𝗲𝗻𝘁𝗮𝘁𝗶𝗼𝗻::𝒆𝒎𝒊𝒕_𝒎𝒆𝒎𝒐𝒓𝒚_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<
                    <𝗶𝗺𝗽𝗹𝗲𝗺𝗲𝗻𝘁𝗮𝘁𝗶𝗼𝗻::𝐥𝐞𝐠𝐚𝐜𝐲_𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧 as 𝗶𝗺𝗽𝗹𝗲𝗺𝗲𝗻𝘁𝗮𝘁𝗶𝗼𝗻::𝒂𝒅𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<0x00>>::𝐭𝐚𝐫𝐠𝐞𝐭>>::
                    $𝓭𝓮𝓻𝓲𝓿𝓮𝓭_𝓯𝓾𝓷𝓬𝓽𝓲𝓸𝓷(
                        emitter2,
                            $𝓻𝓮𝓰_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽,
                            𝗶𝗺𝗽𝗹𝗲𝗺𝗲𝗻𝘁𝗮𝘁𝗶𝗼𝗻::𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086::<𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,
                                                           𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
                                                           𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ,
                                                           i16,
                                                           0> {
                                𝗌𝖾𝗀𝗆𝖾𝗇𝗍: $𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽,
                                𝖻𝖺𝗌𝖾: $𝓫𝓪𝓼𝓮_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽,
                                𝗂𝗇𝖽𝖾𝗑: $𝓲𝓷𝓭𝓮𝔁_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽,
                                𝖽𝗂𝗌𝗉: $𝓭𝓲𝓼𝓹_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽
                            },
                            $($𝓮𝔁𝓽𝓻𝓪_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽𝓼)*)
                    .expect("Testing assembler");
                assert_eq!(
                    &emitter1.𝖼𝗈𝗇𝗍𝖾𝗇𝗍[0..emitter1.𝖼𝗈𝗇𝗍𝖾𝗇𝗍.len()],
                    &emitter2.𝖼𝗈𝗇𝗍𝖾𝗇𝗍[0..emitter2.𝖼𝗈𝗇𝗍𝖾𝗇𝗍.len()]
                );
             )*
        };
        ($𝓫𝓪𝓼𝓮_𝓯𝓾𝓷𝓬𝓽𝓲𝓸𝓷:ident,
         [$($𝓭𝓮𝓻𝓲𝓿𝓮𝓭_𝓯𝓾𝓷𝓬𝓽𝓲𝓸𝓷:ident [$($𝓮𝔁𝓽𝓻𝓪_𝓮𝓶𝓲𝓽:tt)*] [$($𝓮𝔁𝓽𝓻𝓪_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽𝓼:tt)*]),*],
         $𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮:ty, $𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮:ty, $𝓻𝓮𝓰_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽:ident,
         𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<$𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽:ident $𝓫𝓪𝓼𝓮_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽:ident $𝓲𝓷𝓭𝓮𝔁_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽:ident $𝓼𝓬𝓪𝓵𝓮_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽:ident $𝓭𝓲𝓼𝓹_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽:ident>
        ) => {
            $(
                let emitter1 = &mut 𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫::new();
                let emitter2 = &mut 𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫::new();
                <𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫 as 𝗶𝗺𝗽𝗹𝗲𝗺𝗲𝗻𝘁𝗮𝘁𝗶𝗼𝗻::𝒆𝒎𝒊𝒕_𝒎𝒆𝒎𝒐𝒓𝒚_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<
                    <𝗶𝗺𝗽𝗹𝗲𝗺𝗲𝗻𝘁𝗮𝘁𝗶𝗼𝗻::𝐥𝐞𝐠𝐚𝐜𝐲_𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧 as 𝗶𝗺𝗽𝗹𝗲𝗺𝗲𝗻𝘁𝗮𝘁𝗶𝗼𝗻::𝒂𝒅𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<0x00>>::𝐭𝐚𝐫𝐠𝐞𝐭>>::
                    $𝓫𝓪𝓼𝓮_𝓯𝓾𝓷𝓬𝓽𝓲𝓸𝓷(
                        emitter1,
                            $𝓻𝓮𝓰_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽,
                            𝗶𝗺𝗽𝗹𝗲𝗺𝗲𝗻𝘁𝗮𝘁𝗶𝗼𝗻::𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86::<𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,
                                                          $𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮,
                                                          $𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮,
                                                          𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,
                                                          i32,
                                                          0> {
                                𝗌𝖾𝗀𝗆𝖾𝗇𝗍: $𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽,
                                𝖻𝖺𝗌𝖾: $𝓫𝓪𝓼𝓮_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽,
                                𝗂𝗇𝖽𝖾𝗑: $𝓲𝓷𝓭𝓮𝔁_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽,
                                𝗌𝖼𝖺𝗅𝖾: $𝓼𝓬𝓪𝓵𝓮_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽,
                                𝖽𝗂𝗌𝗉: $𝓭𝓲𝓼𝓹_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽
                            })
                    .expect("Testing assembler");
                emitter1.$($𝓮𝔁𝓽𝓻𝓪_𝓮𝓶𝓲𝓽)*.expect("Testing assembler");
                <𝐭𝐞𝐬𝐭_𝐞𝐦𝐢𝐭𝐭𝐞𝐫 as 𝗶𝗺𝗽𝗹𝗲𝗺𝗲𝗻𝘁𝗮𝘁𝗶𝗼𝗻::𝒆𝒎𝒊𝒕_𝒎𝒆𝒎𝒐𝒓𝒚_𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏<
                    <𝗶𝗺𝗽𝗹𝗲𝗺𝗲𝗻𝘁𝗮𝘁𝗶𝗼𝗻::𝐥𝐞𝐠𝐚𝐜𝐲_𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧 as 𝗶𝗺𝗽𝗹𝗲𝗺𝗲𝗻𝘁𝗮𝘁𝗶𝗼𝗻::𝒂𝒅𝒅_𝒐𝒑𝒄𝒐𝒅𝒆<0x00>>::𝐭𝐚𝐫𝐠𝐞𝐭>>::
                    $𝓭𝓮𝓻𝓲𝓿𝓮𝓭_𝓯𝓾𝓷𝓬𝓽𝓲𝓸𝓷(
                        emitter2,
                            $𝓻𝓮𝓰_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽,
                            𝗶𝗺𝗽𝗹𝗲𝗺𝗲𝗻𝘁𝗮𝘁𝗶𝗼𝗻::𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86::<𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫,
                                                          $𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮,
                                                          $𝓲𝓷𝓭𝓮𝔁_𝓽𝔂𝓹𝓮,
                                                          𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞,
                                                          i32,
                                                          0> {
                                𝗌𝖾𝗀𝗆𝖾𝗇𝗍: $𝓼𝓮𝓰𝓶𝓮𝓷𝓽_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽,
                                𝖻𝖺𝗌𝖾: $𝓫𝓪𝓼𝓮_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽,
                                𝗂𝗇𝖽𝖾𝗑: $𝓲𝓷𝓭𝓮𝔁_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽,
                                𝗌𝖼𝖺𝗅𝖾: $𝓼𝓬𝓪𝓵𝓮_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽,
                                𝖽𝗂𝗌𝗉: $𝓭𝓲𝓼𝓹_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽
                            },
                            $($𝓮𝔁𝓽𝓻𝓪_𝓪𝓻𝓰𝓾𝓶𝓮𝓷𝓽𝓼)*)
                    .expect("Testing assembler");
                assert_eq!(
                    &emitter1.𝖼𝗈𝗇𝗍𝖾𝗇𝗍[0..emitter1.𝖼𝗈𝗇𝗍𝖾𝗇𝗍.len()],
                    &emitter2.𝖼𝗈𝗇𝗍𝖾𝗇𝗍[0..emitter2.𝖼𝗈𝗇𝗍𝖾𝗇𝗍.len()]
                );
             )*
        }
    }

    for 𝗋𝖾𝗀 in 𝖊𝖓𝖚𝖒_𝖑𝖎𝖘𝖙!(𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64
            [𝔢𝔞𝔵, 𝔢𝔠𝔵, 𝔢𝔡𝔵, 𝔢𝔟𝔵, 𝔢𝔰𝔭, 𝔢𝔟𝔭, 𝔢𝔰𝔦, 𝔢𝔡𝔦, 𝔯8𝔡, 𝔯9𝔡, 𝔯10𝔡, 𝔯11𝔡, 𝔯12𝔡, 𝔯13𝔡, 𝔯14𝔡, 𝔯15𝔡]) {
        for 𝗋𝗆 in 𝖊𝖓𝖚𝖒_𝖑𝖎𝖘𝖙!(𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64
                [𝔢𝔞𝔵, 𝔢𝔠𝔵, 𝔢𝔡𝔵, 𝔢𝔟𝔵, 𝔢𝔰𝔭, 𝔢𝔟𝔭, 𝔢𝔰𝔦, 𝔢𝔡𝔦, 𝔯8𝔡, 𝔯9𝔡, 𝔯10𝔡, 𝔯11𝔡, 𝔯12𝔡, 𝔯13𝔡, 𝔯14𝔡, 𝔯15𝔡]) {
            𝖛𝖊𝖗𝖎𝖋𝖞_𝖊𝖒𝖎𝖙𝖙𝖊𝖗!(
                emit_legacy_reg_rm_instruction,
                [   emit_legacy_reg_rm_instruction_with_u8 [emit_u8(0xf1)] [0xf1],
                    emit_legacy_reg_rm_instruction_with_i8 [emit_u8(0xf1)] [-15],
                    emit_legacy_reg_rm_instruction_with_2ₓu8 [emit_u16(0xf2f1)] [0xf1, 0xf2],
                    emit_legacy_reg_rm_instruction_with_2ₓi8 [emit_u16(0xf2f1)] [-15, -14],
                    emit_legacy_reg_rm_instruction_with_u16 [emit_u16(0xf2f1)] [0xf2f1],
                    emit_legacy_reg_rm_instruction_with_i16 [emit_u16(0xf2f1)] [-3343],
                    emit_legacy_reg_rm_instruction_with_4ₓu8 [emit_u32(0xf4f3f2f1)] [0xf1, 0xf2, 0xf3, 0xf4],
                    emit_legacy_reg_rm_instruction_with_4ₓi8 [emit_u32(0xf4f3f2f1)] [-15, -14, -13, -12],
                    emit_legacy_reg_rm_instruction_with_u32 [emit_u32(0xf4f3f2f1)] [0xf4f3f2f1],
                    emit_legacy_reg_rm_instruction_with_i32 [emit_u32(0xf4f3f2f1)] [-185339151]],
                𝗋𝖾𝗀,
                𝗋𝗆
            );
            𝖛𝖊𝖗𝖎𝖋𝖞_𝖊𝖒𝖎𝖙𝖙𝖊𝖗!(
                emit_legacy_reg_rm_instruction_with_rex8,
                [   emit_legacy_reg_rm_instruction_with_rex8_and_u8 [emit_u8(0xf1)] [0xf1],
                    emit_legacy_reg_rm_instruction_with_rex8_and_i8 [emit_u8(0xf1)] [-15],
                    emit_legacy_reg_rm_instruction_with_rex8_and_2ₓu8 [emit_u16(0xf2f1)] [0xf1, 0xf2],
                    emit_legacy_reg_rm_instruction_with_rex8_and_2ₓi8 [emit_u16(0xf2f1)] [-15, -14],
                    emit_legacy_reg_rm_instruction_with_rex8_and_u16 [emit_u16(0xf2f1)] [0xf2f1],
                    emit_legacy_reg_rm_instruction_with_rex8_and_i16 [emit_u16(0xf2f1)] [-3343],
                    emit_legacy_reg_rm_instruction_with_rex8_and_4ₓu8 [emit_u32(0xf4f3f2f1)] [0xf1, 0xf2, 0xf3, 0xf4],
                    emit_legacy_reg_rm_instruction_with_rex8_and_4ₓi8 [emit_u32(0xf4f3f2f1)] [-15, -14, -13, -12],
                    emit_legacy_reg_rm_instruction_with_rex8_and_u32 [emit_u32(0xf4f3f2f1)] [0xf4f3f2f1],
                    emit_legacy_reg_rm_instruction_with_rex8_and_i32 [emit_u32(0xf4f3f2f1)] [-185339151]],
                𝗋𝖾𝗀,
                𝗋𝗆
            );
            𝖛𝖊𝖗𝖎𝖋𝖞_𝖊𝖒𝖎𝖙𝖙𝖊𝖗!(
                emit_legacy_reg_rm_instruction_with_rexw,
                [   emit_legacy_reg_rm_instruction_with_rexw_and_u8 [emit_u8(0xf1)] [0xf1],
                    emit_legacy_reg_rm_instruction_with_rexw_and_i8 [emit_u8(0xf1)] [-15],
                    emit_legacy_reg_rm_instruction_with_rexw_and_2ₓu8 [emit_u16(0xf2f1)] [0xf1, 0xf2],
                    emit_legacy_reg_rm_instruction_with_rexw_and_2ₓi8 [emit_u16(0xf2f1)] [-15, -14],
                    emit_legacy_reg_rm_instruction_with_rexw_and_u16 [emit_u16(0xf2f1)] [0xf2f1],
                    emit_legacy_reg_rm_instruction_with_rexw_and_i16 [emit_u16(0xf2f1)] [-3343],
                    emit_legacy_reg_rm_instruction_with_rexw_and_4ₓu8 [emit_u32(0xf4f3f2f1)] [0xf1, 0xf2, 0xf3, 0xf4],
                    emit_legacy_reg_rm_instruction_with_rexw_and_4ₓi8 [emit_u32(0xf4f3f2f1)] [-15, -14, -13, -12],
                    emit_legacy_reg_rm_instruction_with_rexw_and_u32 [emit_u32(0xf4f3f2f1)] [0xf4f3f2f1],
                    emit_legacy_reg_rm_instruction_with_rexw_and_i32 [emit_u32(0xf4f3f2f1)] [-185339151]],
                𝗋𝖾𝗀,
                𝗋𝗆
            );
        }
    }

    for 𝗋𝖾𝗀 in 𝖊𝖓𝖚𝖒_𝖑𝖎𝖘𝖙!(𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386 [𝔢𝔞𝔵, 𝔢𝔠𝔵, 𝔢𝔡𝔵, 𝔢𝔟𝔵, 𝔢𝔰𝔭, 𝔢𝔟𝔭, 𝔢𝔰𝔦, 𝔢𝔡𝔦]) {
        for 𝗌𝖾𝗀𝗆𝖾𝗇𝗍 in 𝖊𝖓𝖚𝖒_𝖑𝖎𝖘𝖙!(𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 [𝔫𝔬𝔫𝔢, 𝔢𝔰, 𝔠𝔰, 𝔰𝔰, 𝔡𝔰, 𝔣𝔰, 𝔤𝔰]) {
            for 𝖻𝖺𝗌𝖾 in 𝖊𝖓𝖚𝖒_𝖑𝖎𝖘𝖙!(𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐛𝐚𝐬𝐞_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ [𝔫𝔬𝔫𝔢, 𝔟𝔵, 𝔟𝔭]) {
                for 𝗂𝗇𝖽𝖾𝗑 in 𝖊𝖓𝖚𝖒_𝖑𝖎𝖘𝖙!(𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_16ᵇⁱᵗ [𝔫𝔬𝔫𝔢, 𝔰𝔦, 𝔡𝔦]) {
                    for 𝖽𝗂𝗌𝗉 in [0, 0x11, 0x1234] {
                        𝖛𝖊𝖗𝖎𝖋𝖞_𝖊𝖒𝖎𝖙𝖙𝖊𝖗!(
                            emit_legacy_reg_address_8086_memory_instruction,
                            [   emit_legacy_reg_address_8086_memory_instruction_with_u8 [emit_u8(0xf1)] [0xf1],
                                emit_legacy_reg_address_8086_memory_instruction_with_i8 [emit_u8(0xf1)] [-15],
                                emit_legacy_reg_address_8086_memory_instruction_with_2ₓu8 [emit_u16(0xf2f1)] [0xf1, 0xf2],
                                emit_legacy_reg_address_8086_memory_instruction_with_2ₓi8 [emit_u16(0xf2f1)] [-15, -14],
                                emit_legacy_reg_address_8086_memory_instruction_with_u16 [emit_u16(0xf2f1)] [0xf2f1],
                                emit_legacy_reg_address_8086_memory_instruction_with_i16 [emit_u16(0xf2f1)] [-3343],
                                emit_legacy_reg_address_8086_memory_instruction_with_4ₓu8
                                    [emit_u32(0xf4f3f2f1)] [0xf1, 0xf2, 0xf3, 0xf4],
                                emit_legacy_reg_address_8086_memory_instruction_with_4ₓi8
                                    [emit_u32(0xf4f3f2f1)] [-15, -14, -13, -12],
                                emit_legacy_reg_address_8086_memory_instruction_with_u32 [emit_u32(0xf4f3f2f1)] [0xf4f3f2f1],
                                emit_legacy_reg_address_8086_memory_instruction_with_i32 [emit_u32(0xf4f3f2f1)] [-185339151]],
                            𝗋𝖾𝗀,
                            𝒂𝒅𝒅𝒓𝒆𝒔𝒔_8086<𝗌𝖾𝗀𝗆𝖾𝗇𝗍 𝖻𝖺𝗌𝖾 𝗂𝗇𝖽𝖾𝗑 𝖽𝗂𝗌𝗉>);
                    }
                }
            }
        }
    }

    for 𝗋𝖾𝗀 in 𝖊𝖓𝖚𝖒_𝖑𝖎𝖘𝖙!(𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386 [𝔢𝔞𝔵, 𝔢𝔠𝔵, 𝔢𝔡𝔵, 𝔢𝔟𝔵, 𝔢𝔰𝔭, 𝔢𝔟𝔭, 𝔢𝔰𝔦, 𝔢𝔡𝔦]) {
        for 𝗌𝖾𝗀𝗆𝖾𝗇𝗍 in 𝖊𝖓𝖚𝖒_𝖑𝖎𝖘𝖙!(𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 [𝔫𝔬𝔫𝔢, 𝔢𝔰, 𝔠𝔰, 𝔰𝔰, 𝔡𝔰, 𝔣𝔰, 𝔤𝔰]) {
            for 𝖻𝖺𝗌𝖾 in 𝖊𝖓𝖚𝖒_𝖑𝖎𝖘𝖙!(𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386 [𝔫𝔬𝔫𝔢, 𝔢𝔞𝔵, 𝔢𝔠𝔵, 𝔢𝔡𝔵, 𝔢𝔟𝔵, 𝔢𝔰𝔭, 𝔢𝔟𝔭, 𝔢𝔰𝔦, 𝔢𝔡𝔦]) {
                for 𝗂𝗇𝖽𝖾𝗑 in 𝖊𝖓𝖚𝖒_𝖑𝖎𝖘𝖙!(𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386
                        [𝔫𝔬𝔫𝔢, 𝔢𝔞𝔵, 𝔢𝔠𝔵, 𝔢𝔡𝔵, 𝔢𝔟𝔵, 𝔢𝔦𝔷, 𝔢𝔟𝔭, 𝔢𝔰𝔦, 𝔢𝔡𝔦]) {
                    for 𝗌𝖼𝖺𝗅𝖾 in 𝖊𝖓𝖚𝖒_𝖑𝖎𝖘𝖙!(𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞 [𝔵1, 𝔵2, 𝔵4, 𝔵8]) {
                        for 𝖽𝗂𝗌𝗉 in [0, 0x11, 0x1234, 0x12345678] {
                            𝖛𝖊𝖗𝖎𝖋𝖞_𝖊𝖒𝖎𝖙𝖙𝖊𝖗!(
                                emit_legacy_reg_address_80386_memory_instruction,
                                [   emit_legacy_reg_address_80386_memory_instruction_with_u8 [emit_u8(0xf1)] [0xf1],
                                    emit_legacy_reg_address_80386_memory_instruction_with_i8 [emit_u8(0xf1)] [-15],
                                    emit_legacy_reg_address_80386_memory_instruction_with_2ₓu8 [emit_u16(0xf2f1)] [0xf1, 0xf2],
                                    emit_legacy_reg_address_80386_memory_instruction_with_2ₓi8 [emit_u16(0xf2f1)] [-15, -14],
                                    emit_legacy_reg_address_80386_memory_instruction_with_u16 [emit_u16(0xf2f1)] [0xf2f1],
                                    emit_legacy_reg_address_80386_memory_instruction_with_i16 [emit_u16(0xf2f1)] [-3343],
                                    emit_legacy_reg_address_80386_memory_instruction_with_4ₓu8
                                        [emit_u32(0xf4f3f2f1)] [0xf1, 0xf2, 0xf3, 0xf4],
                                    emit_legacy_reg_address_80386_memory_instruction_with_4ₓi8
                                        [emit_u32(0xf4f3f2f1)] [-15, -14, -13, -12],
                                    emit_legacy_reg_address_80386_memory_instruction_with_u32 [emit_u32(0xf4f3f2f1)] [0xf4f3f2f1],
                                    emit_legacy_reg_address_80386_memory_instruction_with_i32 [emit_u32(0xf4f3f2f1)] [-185339151]],
                                𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,
                                𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_80386,
                                𝗋𝖾𝗀,
                                𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝗌𝖾𝗀𝗆𝖾𝗇𝗍 𝖻𝖺𝗌𝖾 𝗂𝗇𝖽𝖾𝗑 𝗌𝖼𝖺𝗅𝖾 𝖽𝗂𝗌𝗉>);
                        }
                    }
                }
            }
        }
    }

    for 𝗋𝖾𝗀 in 𝖊𝖓𝖚𝖒_𝖑𝖎𝖘𝖙!(𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64
            [𝔢𝔞𝔵, 𝔢𝔠𝔵, 𝔢𝔡𝔵, 𝔢𝔟𝔵, 𝔢𝔰𝔭, 𝔢𝔟𝔭, 𝔢𝔰𝔦, 𝔢𝔡𝔦, 𝔯8𝔡, 𝔯9𝔡, 𝔯10𝔡, 𝔯11𝔡, 𝔯12𝔡, 𝔯13𝔡, 𝔯14𝔡, 𝔯15𝔡]) {
        for 𝗌𝖾𝗀𝗆𝖾𝗇𝗍 in 𝖊𝖓𝖚𝖒_𝖑𝖎𝖘𝖙!(𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐬𝐞𝐠𝐦𝐞𝐧𝐭_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫 [𝔫𝔬𝔫𝔢, 𝔢𝔰, 𝔠𝔰, 𝔰𝔰, 𝔡𝔰, 𝔣𝔰, 𝔤𝔰]) {
            for 𝖻𝖺𝗌𝖾 in 𝖊𝖓𝖚𝖒_𝖑𝖎𝖘𝖙!(𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64
                    [𝔫𝔬𝔫𝔢, 𝔢𝔞𝔵, 𝔢𝔠𝔵, 𝔢𝔡𝔵, 𝔢𝔟𝔵, 𝔢𝔰𝔭, 𝔢𝔟𝔭, 𝔢𝔰𝔦, 𝔢𝔡𝔦, 𝔯8𝔡, 𝔯9𝔡, 𝔯10𝔡, 𝔯11𝔡, 𝔯12𝔡, 𝔯13𝔡, 𝔯14𝔡, 𝔯15𝔡]) {
                for 𝗂𝗇𝖽𝖾𝗑 in 𝖊𝖓𝖚𝖒_𝖑𝖎𝖘𝖙!(𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64 [
                        𝔫𝔬𝔫𝔢, 𝔢𝔞𝔵, 𝔢𝔠𝔵, 𝔢𝔡𝔵, 𝔢𝔟𝔵, 𝔢𝔦𝔷, 𝔢𝔟𝔭, 𝔢𝔰𝔦, 𝔢𝔡𝔦, 𝔯8𝔡, 𝔯9𝔡, 𝔯10𝔡, 𝔯11𝔡, 𝔯12𝔡, 𝔯13𝔡, 𝔯14𝔡, 𝔯15𝔡]) {
                    for 𝗌𝖼𝖺𝗅𝖾 in 𝖊𝖓𝖚𝖒_𝖑𝖎𝖘𝖙!(𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐢𝐧𝐝𝐞𝐱_𝐬𝐜𝐚𝐥𝐞 [𝔵1, 𝔵2, 𝔵4, 𝔵8]) {
                        for 𝖽𝗂𝗌𝗉 in [0, 0x11, 0x1234, 0x12345678] {
                            𝖛𝖊𝖗𝖎𝖋𝖞_𝖊𝖒𝖎𝖙𝖙𝖊𝖗!(
                                emit_legacy_reg_address_ₓ86_64_memory_instruction,
                                [   emit_legacy_reg_address_ₓ86_64_memory_instruction_with_u8 [emit_u8(0xf1)] [0xf1],
                                    emit_legacy_reg_address_ₓ86_64_memory_instruction_with_i8 [emit_u8(0xf1)] [-15],
                                    emit_legacy_reg_address_ₓ86_64_memory_instruction_with_2ₓu8 [emit_u16(0xf2f1)] [0xf1, 0xf2],
                                    emit_legacy_reg_address_ₓ86_64_memory_instruction_with_2ₓi8 [emit_u16(0xf2f1)] [-15, -14],
                                    emit_legacy_reg_address_ₓ86_64_memory_instruction_with_u16 [emit_u16(0xf2f1)] [0xf2f1],
                                    emit_legacy_reg_address_ₓ86_64_memory_instruction_with_i16 [emit_u16(0xf2f1)] [-3343],
                                    emit_legacy_reg_address_ₓ86_64_memory_instruction_with_4ₓu8
                                        [emit_u32(0xf4f3f2f1)] [0xf1, 0xf2, 0xf3, 0xf4],
                                    emit_legacy_reg_address_ₓ86_64_memory_instruction_with_4ₓi8
                                        [emit_u32(0xf4f3f2f1)] [-15, -14, -13, -12],
                                    emit_legacy_reg_address_ₓ86_64_memory_instruction_with_u32 [emit_u32(0xf4f3f2f1)] [0xf4f3f2f1],
                                    emit_legacy_reg_address_ₓ86_64_memory_instruction_with_i32 [emit_u32(0xf4f3f2f1)] [-185339151]],
                                𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,
                                𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,
                                𝗋𝖾𝗀,
                                𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝗌𝖾𝗀𝗆𝖾𝗇𝗍 𝖻𝖺𝗌𝖾 𝗂𝗇𝖽𝖾𝗑 𝗌𝖼𝖺𝗅𝖾 𝖽𝗂𝗌𝗉>);
                            𝖛𝖊𝖗𝖎𝖋𝖞_𝖊𝖒𝖎𝖙𝖙𝖊𝖗!(
                                emit_legacy_reg_address_ₓ86_64_memory_instruction_with_rex8,
                                [   emit_legacy_reg_address_ₓ86_64_memory_instruction_with_rex8_and_u8 [emit_u8(0xf1)] [0xf1],
                                    emit_legacy_reg_address_ₓ86_64_memory_instruction_with_rex8_and_i8 [emit_u8(0xf1)] [-15],
                                    emit_legacy_reg_address_ₓ86_64_memory_instruction_with_rex8_and_2ₓu8 [emit_u16(0xf2f1)] [0xf1, 0xf2],
                                    emit_legacy_reg_address_ₓ86_64_memory_instruction_with_rex8_and_2ₓi8 [emit_u16(0xf2f1)] [-15, -14],
                                    emit_legacy_reg_address_ₓ86_64_memory_instruction_with_rex8_and_u16 [emit_u16(0xf2f1)] [0xf2f1],
                                    emit_legacy_reg_address_ₓ86_64_memory_instruction_with_rex8_and_i16 [emit_u16(0xf2f1)] [-3343],
                                    emit_legacy_reg_address_ₓ86_64_memory_instruction_with_rex8_and_4ₓu8
                                        [emit_u32(0xf4f3f2f1)] [0xf1, 0xf2, 0xf3, 0xf4],
                                    emit_legacy_reg_address_ₓ86_64_memory_instruction_with_rex8_and_4ₓi8
                                        [emit_u32(0xf4f3f2f1)] [-15, -14, -13, -12],
                                    emit_legacy_reg_address_ₓ86_64_memory_instruction_with_rex8_and_u32 [emit_u32(0xf4f3f2f1)] [0xf4f3f2f1],
                                    emit_legacy_reg_address_ₓ86_64_memory_instruction_with_rex8_and_i32 [emit_u32(0xf4f3f2f1)] [-185339151]],
                                𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,
                                𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,
                                𝗋𝖾𝗀,
                                𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝗌𝖾𝗀𝗆𝖾𝗇𝗍 𝖻𝖺𝗌𝖾 𝗂𝗇𝖽𝖾𝗑 𝗌𝖼𝖺𝗅𝖾 𝖽𝗂𝗌𝗉>);
                            𝖛𝖊𝖗𝖎𝖋𝖞_𝖊𝖒𝖎𝖙𝖙𝖊𝖗!(
                                emit_legacy_reg_address_ₓ86_64_memory_instruction_with_rexw,
                                [   emit_legacy_reg_address_ₓ86_64_memory_instruction_with_rexw_and_u8 [emit_u8(0xf1)] [0xf1],
                                    emit_legacy_reg_address_ₓ86_64_memory_instruction_with_rexw_and_i8 [emit_u8(0xf1)] [-15],
                                    emit_legacy_reg_address_ₓ86_64_memory_instruction_with_rexw_and_2ₓu8 [emit_u16(0xf2f1)] [0xf1, 0xf2],
                                    emit_legacy_reg_address_ₓ86_64_memory_instruction_with_rexw_and_2ₓi8 [emit_u16(0xf2f1)] [-15, -14],
                                    emit_legacy_reg_address_ₓ86_64_memory_instruction_with_rexw_and_u16 [emit_u16(0xf2f1)] [0xf2f1],
                                    emit_legacy_reg_address_ₓ86_64_memory_instruction_with_rexw_and_i16 [emit_u16(0xf2f1)] [-3343],
                                    emit_legacy_reg_address_ₓ86_64_memory_instruction_with_rexw_and_4ₓu8
                                        [emit_u32(0xf4f3f2f1)] [0xf1, 0xf2, 0xf3, 0xf4],
                                    emit_legacy_reg_address_ₓ86_64_memory_instruction_with_rexw_and_4ₓi8
                                        [emit_u32(0xf4f3f2f1)] [-15, -14, -13, -12],
                                    emit_legacy_reg_address_ₓ86_64_memory_instruction_with_rexw_and_u32 [emit_u32(0xf4f3f2f1)] [0xf4f3f2f1],
                                    emit_legacy_reg_address_ₓ86_64_memory_instruction_with_rexw_and_i32 [emit_u32(0xf4f3f2f1)] [-185339151]],
                                𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,
                                𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐚𝐝𝐝𝐫𝐞𝐬𝐬_𝐢𝐧𝐝𝐞𝐱_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐨𝐫_𝐞𝐢𝐳_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_32ᵇⁱᵗ_ₓ86_64,
                                𝗋𝖾𝗀,
                                𝒂𝒅𝒅𝒓𝒆𝒔𝒔_ₓ86<𝗌𝖾𝗀𝗆𝖾𝗇𝗍 𝖻𝖺𝗌𝖾 𝗂𝗇𝖽𝖾𝗑 𝗌𝖼𝖺𝗅𝖾 𝖽𝗂𝗌𝗉>);
                        }
                    }
                }
            }
        }
    }
}
