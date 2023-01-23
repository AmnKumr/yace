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

macro_rules! 𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖎𝖘𝖈𝖛_𝖉𝖎𝖘𝖆𝖘𝖘𝖊𝖒𝖇𝖑𝖊𝖗_𝖙𝖗𝖆𝖎𝖙 {
    (   $𝓭𝓲𝓼𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼:tt
        pub trait $𝓭𝓲𝓼𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮:ident {
           $($𝓮𝔁𝓽𝓻𝓪_𝓭𝓮𝓯𝓲𝓷𝓮𝓼:tt)*
        }
      ) => {
        𝖋𝖎𝖑𝖙𝖊𝖗_𝖗𝖎𝖘𝖈𝖛_𝖒𝖆𝖗𝖐𝖊𝖗𝖘! {
            𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖎𝖘𝖈𝖛_𝖉𝖎𝖘𝖆𝖘𝖘𝖊𝖒𝖇𝖑𝖊𝖗_𝖙𝖗𝖆𝖎𝖙! {
                $𝓭𝓲𝓼𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼
                pub trait $𝓭𝓲𝓼𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮
                where
                    super::super::𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝗿𝗶𝘀𝗰_𝘃::𝗼𝗽𝗲𝗿𝗮𝗻𝗱𝘀::𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞:
                        𝒔𝒐𝒖𝒓𝒄𝒆_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆<Self::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮, 𝐭𝐚𝐫𝐠𝐞𝐭 = <<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜> +
                        𝒔𝒐𝒖𝒓𝒄𝒆_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒅𝒊𝒔𝒑<Self::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮, 𝐭𝐚𝐫𝐠𝐞𝐭 = <<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞> +
                        𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆<Self::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮, 𝐭𝐚𝐫𝐠𝐞𝐭 = <<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜> +
                        𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒅𝒊𝒔𝒑<Self::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮, 𝐭𝐚𝐫𝐠𝐞𝐭 = <<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝐝𝐞𝐬𝐭𝐢𝐧𝐚𝐭𝐢𝐨𝐧_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞> +
                        𝒑𝒓𝒆𝒇𝒆𝒕𝒄𝒉_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆<Self::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮, 𝐭𝐚𝐫𝐠𝐞𝐭 = <<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜> +
                        𝒑𝒓𝒆𝒇𝒆𝒕𝒄𝒉_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒅𝒊𝒔𝒑<Self::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮, 𝐭𝐚𝐫𝐠𝐞𝐭 = <<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝐩𝐫𝐞𝐟𝐞𝐭𝐜𝐡_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞> +
                        𝒛𝒆𝒓𝒐_𝒐𝒇𝒇𝒔𝒆𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆<Self::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮, 𝐭𝐚𝐫𝐠𝐞𝐭 = <<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜>,
                    u32:
                        Into<<<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝐧𝐨𝐳𝐞𝐫𝐨_𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜> +
                        Into<<<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝐧𝐨𝐳𝐞𝐫𝐨_𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐚𝐛𝐢> +
                        Into<<<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜> +
                        Into<<<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐚𝐛𝐢> +
                        Into<<<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝐟𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜> +
                        Into<<<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝐟𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐚𝐛𝐢> +
                        Into<<<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝐜𝐬𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫> +
                        Into<<<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝐫𝐨𝐮𝐧𝐝𝐢𝐧𝐠_𝐦𝐨𝐝𝐞> +
                        Into<<<Self as 𝒊𝒏𝒔𝒕𝒓𝒖𝒄𝒕𝒊𝒐𝒏𝒔_𝒄𝒐𝒏𝒔𝒖𝒎𝒆𝒓>::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮 as 𝑪𝑷𝑼>::𝐟𝐞𝐧𝐜𝐞> +
                     
                {
                    type 𝓒𝓟𝓤_𝓽𝔂𝓹𝓮: 𝑪𝑷𝑼;
                    type 𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞;
                    type 𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞;

                    fn instruction(&mut self, instruction: 𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧_𝐤𝐢𝐧𝐝, operands: &[𝐨𝐩𝐞𝐫𝐚𝐧𝐝<Self::𝓒𝓟𝓤_𝓽𝔂𝓹𝓮>])
                        -> Result<Self::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞>;
                    fn unimplemented_16bit_instruction(&mut self, instruction: u16) -> Result<Self::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞>;
                    fn unimplemented_32bit_instruction(&mut self, instruction: u32) -> Result<Self::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞, Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞>;

                    fn decode<𝓫𝔂𝓽𝓮_𝓹𝓻𝓸𝓭𝓾𝓬𝓮𝓻_𝓽𝔂𝓹𝓮: 𝒃𝒚𝒕𝒆_𝒑𝒓𝒐𝒅𝒖𝒄𝒆𝒓<𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞 = Self::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞>>(&mut self, producer: &mut 𝓫𝔂𝓽𝓮_𝓹𝓻𝓸𝓭𝓾𝓬𝓮𝓻_𝓽𝔂𝓹𝓮)
                        -> Result<Self::𝐫𝐞𝐬𝐮𝐥𝐭_𝐭𝐲𝐩𝐞, 𝓫𝔂𝓽𝓮_𝓹𝓻𝓸𝓭𝓾𝓬𝓮𝓻_𝓽𝔂𝓹𝓮::𝐞𝐫𝐫𝐨𝐫_𝐭𝐲𝐩𝐞>
                    {
                        // Long instructions use bits 12-14 and 2-6 for opcode but short instruction use at least bits 15-13 and
                        // 1-0 for it.
                        //
                        // Different instruction have differently sized opcodes, longest ones are 𝔠.𝔞𝔫𝔡, 𝔠.𝔬𝔯, 𝔠.𝔰𝔲𝔟, 𝔠.𝔵𝔬𝔯 where
                        // bits 15-10, 6-5, and 1-0 are used as opcode.
                        //
                        // Since bits 2-6 are also used as immediates in short instruction we split low 16 bits in the following
                        // parts: bits 15-10 and 1-0 are compressed_instruction_opcode while bits 6-2 are full_instruction_opcode
                        // and 6-9 becomes rd_field (although it includes only 3 bits, but you get the remaining two during
                        // processing of the compressed_instruction_opcode).

                        let parcel0: u16 = producer.get_u16()?;
                        let compressed_instruction_opcode = (((parcel0 >> 8) & 0xfc) | (parcel0 & 0x3)) as usize;
                        let full_instruction_opcode = ((parcel0 >> 2) & 0x1f) as usize;
                        let rd_bits = ((parcel0 >> 7) & 0x07) as usize;

                        let compressed_instruction_step =
                            super::𝗿𝗶𝘀𝗰_𝘃::𝘁𝗮𝗯𝗹𝗲𝘀::Ξ𝔯𝔳32[𝔠𝔬𝔪𝔭𝔯𝔢𝔰𝔢𝔡_𝔰𝔱𝔢𝔭_𝔡𝔦𝔰𝔭𝔞𝔱𝔠𝔥_𝐫𝐯𝟑𝟐]Ξ𝔯𝔳64[𝔠𝔬𝔪𝔭𝔯𝔢𝔰𝔢𝔡_𝔰𝔱𝔢𝔭_𝔡𝔦𝔰𝔭𝔞𝔱𝔠𝔥_𝐫𝐯𝟔𝟒][compressed_instruction_opcode];

                        type 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩 = super::𝗿𝗶𝘀𝗰_𝘃::𝘁𝗮𝗯𝗹𝗲𝘀::Ξ𝔯𝔳32[𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟑𝟐]Ξ𝔯𝔳64[𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩_𝐫𝐯𝟔𝟒];

                        let instruction_bits = if compressed_instruction_step.1 <= 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩::𝔩𝔬𝔫𝔤_𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔣𝔲𝔫𝔠3_7 {
                           let parcel1: u16 = producer.get_u16()?;
                           (parcel1 as u32) << 16 | (parcel0 as u32)
                        } else {
                            0
                        };

                        match compressed_instruction_step.1 {
                            𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩::𝔩𝔬𝔫𝔤_𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔣𝔲𝔫𝔠3_0
                            | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩::𝔩𝔬𝔫𝔤_𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔣𝔲𝔫𝔠3_1
                            | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩::𝔩𝔬𝔫𝔤_𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔣𝔲𝔫𝔠3_2
                            | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩::𝔩𝔬𝔫𝔤_𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔣𝔲𝔫𝔠3_3
                            | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩::𝔩𝔬𝔫𝔤_𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔣𝔲𝔫𝔠3_4
                            | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩::𝔩𝔬𝔫𝔤_𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔣𝔲𝔫𝔠3_5
                            | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩::𝔩𝔬𝔫𝔤_𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔣𝔲𝔫𝔠3_6
                            | 𝐜𝐨𝐦𝐩𝐫𝐞𝐬𝐞𝐝_𝐬𝐭𝐞𝐩::𝔩𝔬𝔫𝔤_𝔦𝔫𝔰𝔱𝔯𝔲𝔠𝔱𝔦𝔬𝔫_𝔣𝔲𝔫𝔠3_7 => {
                                self.unimplemented_32bit_instruction(instruction_bits)
                            }
                            _ => self.unimplemented_16bit_instruction(parcel0)
                        }
                    }

                    𝔻𝕚𝕤𝕒𝕤𝕤𝕖𝕞𝕓𝕝𝕖𝕣𝕀𝕟𝕤𝕥𝕣𝕦𝕔𝕥𝕚𝕠𝕟𝕤

                    $($𝓮𝔁𝓽𝓻𝓪_𝓭𝓮𝓯𝓲𝓷𝓮𝓼)*
                }
            }
        }
    };
    ($( $𝓭𝓲𝓼𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼:tt
        pub trait $𝓭𝓲𝓼𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮:ident {
           $($𝓮𝔁𝓽𝓻𝓪_𝓭𝓮𝓯𝓲𝓷𝓮𝓼:tt)*
        }
      )*) => {
        $(  𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖎𝖘𝖈𝖛_𝖆𝖘𝖘𝖊𝖒𝖇𝖑𝖊𝖗_𝖙𝖗𝖆𝖎𝖙! {
                $𝓭𝓲𝓼𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼
                pub trait $𝓭𝓲𝓼𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮 {
                    $($𝓮𝔁𝓽𝓻𝓪_𝓭𝓮𝓯𝓲𝓷𝓮𝓼)*
                }
            }
         )*
    };
    (   $𝓭𝓲𝓼𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼:tt
        pub trait $𝓭𝓲𝓼𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮:ident
        $($𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼_𝓪𝓷𝓭_𝓭𝓮𝓯𝓲𝓷𝓮𝓼:tt)*
      ) => {
        #[allow(non_upper_case_globals)]
        pub trait $𝓭𝓲𝓼𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝓻𝓪𝓲𝓽_𝓷𝓪𝓶𝓮
        $($𝓻𝓮𝓼𝓽𝓻𝓲𝓬𝓽𝓲𝓸𝓷𝓼_𝓪𝓷𝓭_𝓭𝓮𝓯𝓲𝓷𝓮𝓼)*
    };
}
