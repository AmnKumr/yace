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

// On 𝗿𝗶𝘀𝗰𝘃 we only have two main types of assembler operands: 𝒔𝒐𝒖𝒓𝒄𝒆_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒐𝒑𝒆𝒓𝒂𝒏𝒅 and 𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒐𝒑𝒆𝒓𝒂𝒏𝒅.
//
// The main difference is treatment of immediates: 𝒔𝒐𝒖𝒓𝒄𝒆_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒐𝒑𝒆𝒓𝒂𝒏𝒅 uses 𝐁_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞/𝐈_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞/𝐉_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞/𝐔_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞,
// while 𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒐𝒑𝒆𝒓𝒂𝒏𝒅 uses 𝐒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞.  We map all other immediates for them for convenience.
//
// Additionally there are one extra type: 𝒛𝒆𝒓𝒐_𝒐𝒇𝒇𝒔𝒆𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔. It's used mostly by atomic instructions and implies zero offset.
// We just accept addresses with 𝖽𝗂𝗌𝗉 set into 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞.

pub trait 𝒔𝒐𝒖𝒓𝒄𝒆_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒐𝒑𝒆𝒓𝒂𝒏𝒅<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮: ?Sized>: Into<Self::𝐭𝐚𝐫𝐠𝐞𝐭> {
    type 𝐭𝐚𝐫𝐠𝐞𝐭;
}

impl<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮: ?Sized> 𝒔𝒐𝒖𝒓𝒄𝒆_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒐𝒑𝒆𝒓𝒂𝒏𝒅<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮> for 𝐁_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    type 𝐭𝐚𝐫𝐠𝐞𝐭 = 𝐁_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞;
}

impl<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮: ?Sized> 𝒔𝒐𝒖𝒓𝒄𝒆_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒐𝒑𝒆𝒓𝒂𝒏𝒅<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮> for 𝐜𝐬𝐫_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    type 𝐭𝐚𝐫𝐠𝐞𝐭 = 𝐈_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞;
}

impl<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮: ?Sized> 𝒔𝒐𝒖𝒓𝒄𝒆_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒐𝒑𝒆𝒓𝒂𝒏𝒅<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮> for 𝐈_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    type 𝐭𝐚𝐫𝐠𝐞𝐭 = 𝐈_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞;
}

impl<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮: ?Sized> 𝒔𝒐𝒖𝒓𝒄𝒆_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒐𝒑𝒆𝒓𝒂𝒏𝒅<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮> for 𝐉_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    type 𝐭𝐚𝐫𝐠𝐞𝐭 = 𝐉_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞;
}

impl<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮: ?Sized> 𝒔𝒐𝒖𝒓𝒄𝒆_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒐𝒑𝒆𝒓𝒂𝒏𝒅<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮> for 𝐏_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    type 𝐭𝐚𝐫𝐠𝐞𝐭 = 𝐈_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞;
}

impl<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮: ?Sized> 𝒔𝒐𝒖𝒓𝒄𝒆_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒐𝒑𝒆𝒓𝒂𝒏𝒅<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮> for 𝐒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    type 𝐭𝐚𝐫𝐠𝐞𝐭 = 𝐈_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞;
}

impl<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮: ?Sized> 𝒔𝒐𝒖𝒓𝒄𝒆_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒐𝒑𝒆𝒓𝒂𝒏𝒅<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮> for 𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟑𝟐_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    type 𝐭𝐚𝐫𝐠𝐞𝐭 = 𝐈_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞;
}

impl<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮: ?Sized> 𝒔𝒐𝒖𝒓𝒄𝒆_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒐𝒑𝒆𝒓𝒂𝒏𝒅<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮> for 𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟔𝟒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    type 𝐭𝐚𝐫𝐠𝐞𝐭 = 𝐈_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞;
}

impl<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮: ?Sized> 𝒔𝒐𝒖𝒓𝒄𝒆_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒐𝒑𝒆𝒓𝒂𝒏𝒅<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮> for 𝐔_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    type 𝐭𝐚𝐫𝐠𝐞𝐭 = 𝐔_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞;
}

impl<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮: ?Sized, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮: Default, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮: Default>
    𝒔𝒐𝒖𝒓𝒄𝒆_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒐𝒑𝒆𝒓𝒂𝒏𝒅<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>
    for 𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮> {
    type 𝐭𝐚𝐫𝐠𝐞𝐭 = 𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮>;
}

impl<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮: ?Sized,
     𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮: Into<<𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮 as 𝒔𝒐𝒖𝒓𝒄𝒆_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭> +
                𝒔𝒐𝒖𝒓𝒄𝒆_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>,
     𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮: Into<<𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮 as 𝒔𝒐𝒖𝒓𝒄𝒆_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒅𝒊𝒔𝒑<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭> +
                𝒔𝒐𝒖𝒓𝒄𝒆_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒅𝒊𝒔𝒑<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>
    𝒔𝒐𝒖𝒓𝒄𝒆_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒐𝒑𝒆𝒓𝒂𝒏𝒅<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>
    for 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮> {
    type 𝐭𝐚𝐫𝐠𝐞𝐭 = 𝒂𝒅𝒅𝒓𝒆𝒔𝒔<<𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮 as 𝒔𝒐𝒖𝒓𝒄𝒆_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭,
                          <𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮 as 𝒔𝒐𝒖𝒓𝒄𝒆_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒅𝒊𝒔𝒑<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭>;
}

pub trait 𝒔𝒐𝒖𝒓𝒄𝒆_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮: ?Sized>: Into<Self::𝐭𝐚𝐫𝐠𝐞𝐭> {
    type 𝐭𝐚𝐫𝐠𝐞𝐭: Default;
}

pub trait 𝒔𝒐𝒖𝒓𝒄𝒆_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒅𝒊𝒔𝒑<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮: ?Sized>: Into<Self::𝐭𝐚𝐫𝐠𝐞𝐭> {
    type 𝐭𝐚𝐫𝐠𝐞𝐭: Default;
}

impl<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮: ?Sized> 𝒔𝒐𝒖𝒓𝒄𝒆_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒅𝒊𝒔𝒑<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮> for 𝐁_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    type 𝐭𝐚𝐫𝐠𝐞𝐭 = 𝐁_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞;
}

impl<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮: ?Sized> 𝒔𝒐𝒖𝒓𝒄𝒆_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒅𝒊𝒔𝒑<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮> for 𝐈_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    type 𝐭𝐚𝐫𝐠𝐞𝐭 = 𝐈_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞;
}

impl<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮: ?Sized> 𝒔𝒐𝒖𝒓𝒄𝒆_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒅𝒊𝒔𝒑<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮> for 𝐉_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    type 𝐭𝐚𝐫𝐠𝐞𝐭 = 𝐉_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞;
}

impl<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮: ?Sized> 𝒔𝒐𝒖𝒓𝒄𝒆_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒅𝒊𝒔𝒑<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮> for 𝐏_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    type 𝐭𝐚𝐫𝐠𝐞𝐭 = 𝐈_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞;
}

impl<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮: ?Sized> 𝒔𝒐𝒖𝒓𝒄𝒆_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒅𝒊𝒔𝒑<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮> for 𝐒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    type 𝐭𝐚𝐫𝐠𝐞𝐭 = 𝐈_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞;
}

impl<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮: ?Sized> 𝒔𝒐𝒖𝒓𝒄𝒆_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒅𝒊𝒔𝒑<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮> for 𝐔_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    type 𝐭𝐚𝐫𝐠𝐞𝐭 = 𝐔_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞;
}

pub trait 𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒐𝒑𝒆𝒓𝒂𝒏𝒅<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮: ?Sized>: Into<Self::𝐭𝐚𝐫𝐠𝐞𝐭> {
    type 𝐭𝐚𝐫𝐠𝐞𝐭;
}

impl<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮: ?Sized> 𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒐𝒑𝒆𝒓𝒂𝒏𝒅<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮> for 𝐁_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    type 𝐭𝐚𝐫𝐠𝐞𝐭 = 𝐁_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞;
}

impl<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮: ?Sized> 𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒐𝒑𝒆𝒓𝒂𝒏𝒅<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮> for 𝐜𝐬𝐫_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    type 𝐭𝐚𝐫𝐠𝐞𝐭 = 𝐒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞;
}

impl<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮: ?Sized> 𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒐𝒑𝒆𝒓𝒂𝒏𝒅<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮> for 𝐈_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    type 𝐭𝐚𝐫𝐠𝐞𝐭 = 𝐒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞;
}

impl<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮: ?Sized> 𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒐𝒑𝒆𝒓𝒂𝒏𝒅<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮> for 𝐉_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    type 𝐭𝐚𝐫𝐠𝐞𝐭 = 𝐉_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞;
}

impl<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮: ?Sized> 𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒐𝒑𝒆𝒓𝒂𝒏𝒅<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮> for 𝐏_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    type 𝐭𝐚𝐫𝐠𝐞𝐭 = 𝐒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞;
}

impl<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮: ?Sized> 𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒐𝒑𝒆𝒓𝒂𝒏𝒅<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮> for 𝐒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    type 𝐭𝐚𝐫𝐠𝐞𝐭 = 𝐒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞;
}

impl<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮: ?Sized> 𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒐𝒑𝒆𝒓𝒂𝒏𝒅<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮> for 𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟑𝟐_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    type 𝐭𝐚𝐫𝐠𝐞𝐭 = 𝐒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞;
}

impl<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮: ?Sized> 𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒐𝒑𝒆𝒓𝒂𝒏𝒅<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮> for 𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟔𝟒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    type 𝐭𝐚𝐫𝐠𝐞𝐭 = 𝐒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞;
}

impl<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮: ?Sized> 𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒐𝒑𝒆𝒓𝒂𝒏𝒅<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮> for 𝐔_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    type 𝐭𝐚𝐫𝐠𝐞𝐭 = 𝐔_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞;
}

impl<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮: ?Sized, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮: Default, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮: Default>
    𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒐𝒑𝒆𝒓𝒂𝒏𝒅<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>
    for 𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮> {
    type 𝐭𝐚𝐫𝐠𝐞𝐭 = 𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮>;
}

impl<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮: ?Sized,
     𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮: Into<<𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮 as 𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭> +
                𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>,
     𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮: Into<<𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮 as 𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒅𝒊𝒔𝒑<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭> +
                𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒅𝒊𝒔𝒑<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>
    𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒐𝒑𝒆𝒓𝒂𝒏𝒅<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>
    for 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮> {
    type 𝐭𝐚𝐫𝐠𝐞𝐭 = 𝒂𝒅𝒅𝒓𝒆𝒔𝒔<<𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮 as 𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭,
                          <𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮 as 𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒅𝒊𝒔𝒑<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭>;
}

pub trait 𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮: ?Sized>: Into<Self::𝐭𝐚𝐫𝐠𝐞𝐭> {
    type 𝐭𝐚𝐫𝐠𝐞𝐭: Default;
}

pub trait 𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒅𝒊𝒔𝒑<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮: ?Sized>: Into<Self::𝐭𝐚𝐫𝐠𝐞𝐭> {
    type 𝐭𝐚𝐫𝐠𝐞𝐭: Default;
}

impl<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮: ?Sized> 𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒅𝒊𝒔𝒑<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮> for 𝐁_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    type 𝐭𝐚𝐫𝐠𝐞𝐭 = 𝐁_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞;
}

impl<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮: ?Sized> 𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒅𝒊𝒔𝒑<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮> for 𝐈_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    type 𝐭𝐚𝐫𝐠𝐞𝐭 = 𝐒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞;
}

impl<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮: ?Sized> 𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒅𝒊𝒔𝒑<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮> for 𝐉_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    type 𝐭𝐚𝐫𝐠𝐞𝐭 = 𝐉_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞;
}

impl<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮: ?Sized> 𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒅𝒊𝒔𝒑<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮> for 𝐏_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    type 𝐭𝐚𝐫𝐠𝐞𝐭 = 𝐒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞;
}

impl<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮: ?Sized> 𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒅𝒊𝒔𝒑<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮> for 𝐒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    type 𝐭𝐚𝐫𝐠𝐞𝐭 = 𝐒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞;
}

impl<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮: ?Sized> 𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒅𝒊𝒔𝒑<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮> for 𝐔_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    type 𝐭𝐚𝐫𝐠𝐞𝐭 = 𝐔_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞;
}

pub trait 𝒄𝒔𝒓_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒐𝒑𝒆𝒓𝒂𝒏𝒅<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮: ?Sized>: Into<Self::𝐭𝐚𝐫𝐠𝐞𝐭> {
    type 𝐭𝐚𝐫𝐠𝐞𝐭;
}

impl<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮: ?Sized> 𝒄𝒔𝒓_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒐𝒑𝒆𝒓𝒂𝒏𝒅<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮> for 𝐜𝐬𝐫_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    type 𝐭𝐚𝐫𝐠𝐞𝐭 = 𝐜𝐬𝐫_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞;
}

pub trait 𝒑𝒓𝒆𝒇𝒆𝒕𝒄𝒉_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒐𝒑𝒆𝒓𝒂𝒏𝒅<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮: ?Sized>: Into<Self::𝐭𝐚𝐫𝐠𝐞𝐭> {
    type 𝐭𝐚𝐫𝐠𝐞𝐭;
}

impl<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮: ?Sized, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮: Default, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮: Default>
    𝒑𝒓𝒆𝒇𝒆𝒕𝒄𝒉_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒐𝒑𝒆𝒓𝒂𝒏𝒅<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>
    for 𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮> {
    type 𝐭𝐚𝐫𝐠𝐞𝐭 = 𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮>;
}

impl<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮: ?Sized,
     𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮: Into<<𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮 as 𝒑𝒓𝒆𝒇𝒆𝒕𝒄𝒉_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭> +
                𝒑𝒓𝒆𝒇𝒆𝒕𝒄𝒉_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>,
     𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮: Into<<𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮 as 𝒑𝒓𝒆𝒇𝒆𝒕𝒄𝒉_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒅𝒊𝒔𝒑<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭> +
                𝒑𝒓𝒆𝒇𝒆𝒕𝒄𝒉_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒅𝒊𝒔𝒑<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>
    𝒑𝒓𝒆𝒇𝒆𝒕𝒄𝒉_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒐𝒑𝒆𝒓𝒂𝒏𝒅<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>
    for 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮> {
    type 𝐭𝐚𝐫𝐠𝐞𝐭 = 𝒂𝒅𝒅𝒓𝒆𝒔𝒔<<𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮 as 𝒑𝒓𝒆𝒇𝒆𝒕𝒄𝒉_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭,
                          <𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮 as 𝒑𝒓𝒆𝒇𝒆𝒕𝒄𝒉_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒅𝒊𝒔𝒑<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭>;
}

pub trait 𝒑𝒓𝒆𝒇𝒆𝒕𝒄𝒉_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮: ?Sized>: Into<Self::𝐭𝐚𝐫𝐠𝐞𝐭> {
    type 𝐭𝐚𝐫𝐠𝐞𝐭: Default;
}

pub trait 𝒑𝒓𝒆𝒇𝒆𝒕𝒄𝒉_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒅𝒊𝒔𝒑<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮: ?Sized>: Into<Self::𝐭𝐚𝐫𝐠𝐞𝐭> {
    type 𝐭𝐚𝐫𝐠𝐞𝐭: Default;
}

impl<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮: ?Sized> 𝒑𝒓𝒆𝒇𝒆𝒕𝒄𝒉_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒅𝒊𝒔𝒑<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮> for 𝐏_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    type 𝐭𝐚𝐫𝐠𝐞𝐭 = 𝐏_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞;
}

pub trait 𝒔𝒉𝒊𝒇𝒕_𝑹𝑽𝟑𝟐_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒐𝒑𝒆𝒓𝒂𝒏𝒅<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮: ?Sized>: Into<Self::𝐭𝐚𝐫𝐠𝐞𝐭> {
    type 𝐭𝐚𝐫𝐠𝐞𝐭;
}

impl<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮: ?Sized> 𝒔𝒉𝒊𝒇𝒕_𝑹𝑽𝟑𝟐_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒐𝒑𝒆𝒓𝒂𝒏𝒅<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮> for 𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟑𝟐_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    type 𝐭𝐚𝐫𝐠𝐞𝐭 = 𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟑𝟐_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞;
}

pub trait 𝒔𝒉𝒊𝒇𝒕_𝑹𝑽𝟔𝟒_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒐𝒑𝒆𝒓𝒂𝒏𝒅<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮: ?Sized>: Into<Self::𝐭𝐚𝐫𝐠𝐞𝐭> {
    type 𝐭𝐚𝐫𝐠𝐞𝐭;
}

impl<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮: ?Sized> 𝒔𝒉𝒊𝒇𝒕_𝑹𝑽𝟔𝟒_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒐𝒑𝒆𝒓𝒂𝒏𝒅<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮> for 𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟔𝟒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    type 𝐭𝐚𝐫𝐠𝐞𝐭 = 𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟔𝟒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞;
}

pub trait 𝒛𝒆𝒓𝒐_𝒐𝒇𝒇𝒔𝒆𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_𝒐𝒑𝒆𝒓𝒂𝒏𝒅<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮: ?Sized>: Into<Self::𝐭𝐚𝐫𝐠𝐞𝐭> {
    type 𝐭𝐚𝐫𝐠𝐞𝐭;
}

impl<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮: ?Sized, 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮: Default>
    𝒛𝒆𝒓𝒐_𝒐𝒇𝒇𝒔𝒆𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_𝒐𝒑𝒆𝒓𝒂𝒏𝒅<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>
    for 𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞> {
    type 𝐭𝐚𝐫𝐠𝐞𝐭 = 𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞>;
}

impl<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮: ?Sized,
     𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮: Into<<𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮 as 𝒛𝒆𝒓𝒐_𝒐𝒇𝒇𝒔𝒆𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭> +
                𝒛𝒆𝒓𝒐_𝒐𝒇𝒇𝒔𝒆𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>
    𝒛𝒆𝒓𝒐_𝒐𝒇𝒇𝒔𝒆𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_𝒐𝒑𝒆𝒓𝒂𝒏𝒅<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>
    for 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞> {
    type 𝐭𝐚𝐫𝐠𝐞𝐭 = 𝒂𝒅𝒅𝒓𝒆𝒔𝒔<<𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮 as 𝒛𝒆𝒓𝒐_𝒐𝒇𝒇𝒔𝒆𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮>>::𝐭𝐚𝐫𝐠𝐞𝐭, 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞>;
}

pub trait 𝒛𝒆𝒓𝒐_𝒐𝒇𝒇𝒔𝒆𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮: ?Sized>: Into<Self::𝐭𝐚𝐫𝐠𝐞𝐭> {
    type 𝐭𝐚𝐫𝐠𝐞𝐭: Default;
}

// Address includes some values which are optional and can be unfilled. We use empty type to mark these.
// Note: we can not use just an empty tuple because then we couldn't define From trait for it.
// Note2: register and displacement must be obtainable from 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞.  Integer types are obtainable automatically.
#[derive(Clone, Copy, Default, Debug)]
pub struct 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞 {}

#[derive(Clone, Copy, Default, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct 𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮: Default, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮: Default> {
    pub 𝖻𝖺𝗌𝖾: 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮,
    pub 𝖽𝗂𝗌𝗉: 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮,
}

impl<
        𝓽𝓪𝓻𝓰𝓮𝓽_𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮: Default,
        𝓽𝓪𝓻𝓰𝓮𝓽_𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮: Default,
        𝓼𝓸𝓾𝓻𝓬𝓮_𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮: Into<𝓽𝓪𝓻𝓰𝓮𝓽_𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮>,
        𝓼𝓸𝓾𝓻𝓬𝓮_𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮: Into<𝓽𝓪𝓻𝓰𝓮𝓽_𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮>,
    > From<𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓼𝓸𝓾𝓻𝓬𝓮_𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓼𝓸𝓾𝓻𝓬𝓮_𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮>>
for 𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓽𝓪𝓻𝓰𝓮𝓽_𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓽𝓪𝓻𝓰𝓮𝓽_𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮>
{
    #[inline(always)]
    fn from(
        new_address: 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓼𝓸𝓾𝓻𝓬𝓮_𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓼𝓸𝓾𝓻𝓬𝓮_𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮>
    ) -> Self {
        Self {
            𝖻𝖺𝗌𝖾: new_address.𝖻𝖺𝗌𝖾.into(),
            𝖽𝗂𝗌𝗉: new_address.𝖽𝗂𝗌𝗉.into(),
        }
    }
}

#[allow(non_upper_case_globals)]
pub const 𝔞𝔡𝔡𝔯𝔢𝔰𝔰: 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞, 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞> = 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔 {
    𝖻𝖺𝗌𝖾: 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞 {},
    𝖽𝗂𝗌𝗉: 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞 {},
};

// Fluent interface requires the ability to find out type from arguments: Rust doesn't try to do complex reasoning when you have
// something like 𝔞𝔡𝔡𝔯𝔢𝔰𝔰.with_base(…).with_disp(…).
//
// Collect all arguments into 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔 instead and then provide conversions into proper addess.
#[derive(Clone, Copy, Default, Debug, Eq, PartialEq)]
pub struct 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮> {
    pub 𝖻𝖺𝗌𝖾: 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮,
    pub 𝖽𝗂𝗌𝗉: 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮,
}

impl<𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮> 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮> {
    #[inline(always)]
    pub fn with_base<𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮>(self, new_base: 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮) -> 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮> {
        𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔::<𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮> {
            𝖻𝖺𝗌𝖾: new_base,
            𝖽𝗂𝗌𝗉: self.𝖽𝗂𝗌𝗉,
        }
    }
}

impl<𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮> 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞> {
    #[inline(always)]
    pub fn with_disp<𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮>(self, new_disp: 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮) -> 𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮> {
        𝒇𝒍𝒖𝒆𝒏𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔::<𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮> {
            𝖻𝖺𝗌𝖾: self.𝖻𝖺𝗌𝖾,
            𝖽𝗂𝗌𝗉: new_disp,
        }
    }
}

macro_rules! 𝖉𝖊𝖋𝖎𝖓𝖊_𝖋𝖑𝖚𝖊𝖓𝖙_𝖒𝖊𝖒𝖇𝖊𝖗_𝖋𝖗𝖔𝖒 {
    ($($𝓲𝓶𝓶𝓮𝓭𝓲𝓪𝓽𝓮_𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮:ty),*) => {
        $(
            impl From<𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞> for $𝓲𝓶𝓶𝓮𝓭𝓲𝓪𝓽𝓮_𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮 {
                #[inline(always)]
                fn from(_: 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞) -> $𝓲𝓶𝓶𝓮𝓭𝓲𝓪𝓽𝓮_𝓽𝔂𝓹𝓮_𝓷𝓪𝓶𝓮 {
                    Default::default()
                }
            }
         )*
    }
}

𝖉𝖊𝖋𝖎𝖓𝖊_𝖋𝖑𝖚𝖊𝖓𝖙_𝖒𝖊𝖒𝖇𝖊𝖗_𝖋𝖗𝖔𝖒!(
    super::𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜_𝐫𝐯𝟑𝟐𝐞,
    super::𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜,
    𝐁_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞,
    𝐈_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞,
    𝐉_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞,
    𝐏_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞,
    𝐔_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞,
    𝐒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞
);

#[derive(Clone, Copy, Default, Debug, Eq, PartialEq)]
pub struct 𝐁_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞(pub(crate) i32);

#[derive(Clone, Copy, Default, Debug, Eq, PartialEq)]
pub struct 𝐜𝐬𝐫_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞(pub(crate) i32);

#[derive(Clone, Copy, Default, Debug, Eq, PartialEq)]
pub struct 𝐈_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞(pub(crate) i32);

#[derive(Clone, Copy, Default, Debug, Eq, PartialEq)]
pub struct 𝐉_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞(pub(crate) i32);

#[derive(Clone, Copy, Default, Debug, Eq, PartialEq)]
pub struct 𝐏_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞(pub(crate) i32);

#[derive(Clone, Copy, Default, Debug, Eq, PartialEq)]
pub struct 𝐒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞(pub(crate) i32);

#[derive(Clone, Copy, Default, Debug, Eq, PartialEq)]
pub struct 𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟑𝟐_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞(pub(crate) i32);

#[derive(Clone, Copy, Default, Debug, Eq, PartialEq)]
pub struct 𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟔𝟒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞(pub(crate) i32);

#[derive(Clone, Copy, Default, Debug, Eq, PartialEq)]
pub struct 𝐔_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞(pub(crate) i32);

impl 𝐁_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    #[inline(always)]
    pub const fn new_from_instruction(encoding: i32) -> 𝐁_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
        𝐁_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞(encoding & -0x1fff080)
    }
    #[inline(always)]
    pub const unsafe fn new_unchecked(imm: i32) -> 𝐁_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
        𝐁_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞((imm & -0x80000000) | (imm & 0x000007e0) << 20 | (imm & 0x0000001f) << 7 | (imm & 0x00000800) >> 4)
    }
    #[inline(always)]
    pub const fn new_const(imm: i32) -> Option<𝐁_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞> {
        if let 0x00000000 | -0x00001000 = imm & -0x00000fff {
            Some(𝐁_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞((imm & -0x80000000) | (imm & 0x000007e0) << 20 | (imm & 0x0000001f) << 7 | (imm & 0x00000800) >> 4))
        } else {
            None
        }
    }
    #[inline(always)]
    pub fn new(imm: impl 𝑩_𝒊𝒎𝒎𝒆𝒅𝒊𝒂𝒕𝒆_𝒔𝒐𝒖𝒓𝒄𝒆) -> Option<𝐁_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞> {
        imm.try_into().ok()
    }
}
pub trait 𝑩_𝒊𝒎𝒎𝒆𝒅𝒊𝒂𝒕𝒆_𝒔𝒐𝒖𝒓𝒄𝒆: TryInto<𝐁_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞> {}

impl From<𝐁_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞> for i32 {
    #[inline(always)]
    fn from(imm: 𝐁_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞) -> i32 {
        (imm.0 & -0x80000000) >> 19 | (imm.0 & 0x7e000000) >> 20 | (imm.0 & 0x00000f00) >> 7 | (imm.0 & 0x00000080) << 4
    }
}

impl TryFrom<i8> for 𝐁_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    type Error = 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫;
    #[inline(always)]
    fn try_from(imm: i8) -> Result<𝐁_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞, 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫> {
        if let 0x00 = imm & 0x01 {
            Ok(𝐁_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞((imm as i32 & -0x08000000) |
                           (imm as i32 & 0x60) << 20 |
                           (imm as i32 & 0x1f) << 7 |
                           (imm as i32 & 0x80)))
        } else {
            Err(𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫(()))
        }
    }
}
impl 𝑩_𝒊𝒎𝒎𝒆𝒅𝒊𝒂𝒕𝒆_𝒔𝒐𝒖𝒓𝒄𝒆 for i8 {}

impl TryFrom<u8> for 𝐁_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    type Error = 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫;
    #[inline(always)]
    fn try_from(imm: u8) -> Result<𝐁_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞, 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫> {
        if let 0x0000 = imm & 0x01 {
            Ok(𝐁_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞((imm as i32 & 0xe0) << 20 | (imm as i32 & 0x1f) << 7))
        } else {
            Err(𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫(()))
        }
    }
}
impl 𝑩_𝒊𝒎𝒎𝒆𝒅𝒊𝒂𝒕𝒆_𝒔𝒐𝒖𝒓𝒄𝒆 for u8 {}

impl TryFrom<i16> for 𝐁_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    type Error = 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫;
    #[inline(always)]
    fn try_from(imm: i16) -> Result<𝐁_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞, 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫> {
        if let 0x0000 | -0x1000 = imm & -0x0fff {
            Ok(𝐁_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞((imm as i32 & -0x80000000) |
                           (imm as i32 & 0x07e0) << 20 |
                           (imm as i32 & 0x001f) << 7 |
                           (imm as i32 & 0x0800) >> 4))
        } else {
            Err(𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫(()))
        }
    }
}
impl 𝑩_𝒊𝒎𝒎𝒆𝒅𝒊𝒂𝒕𝒆_𝒔𝒐𝒖𝒓𝒄𝒆 for i16 {}

impl TryFrom<u16> for 𝐁_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    type Error = 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫;
    #[inline(always)]
    fn try_from(imm: u16) -> Result<𝐁_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞, 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫> {
        if let 0x0000 = imm & 0xf001 {
            Ok(𝐁_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞((imm as i32 & 0x07e0) << 20 | (imm as i32 & 0x001f) << 7 | (imm as i32 & 0x0800) >> 4))
        } else {
            Err(𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫(()))
        }
    }
}
impl 𝑩_𝒊𝒎𝒎𝒆𝒅𝒊𝒂𝒕𝒆_𝒔𝒐𝒖𝒓𝒄𝒆 for u16 {}

impl TryFrom<i32> for 𝐁_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    type Error = 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫;
    #[inline(always)]
    fn try_from(imm: i32) -> Result<𝐁_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞, 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫> {
        if let 0x00000000 | -0x00001000 = imm & -0x00000fff {
            Ok(𝐁_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞((imm & -0x80000000) | (imm & 0x000007e0) << 20 | (imm & 0x0000001f) << 7 | (imm & 0x00000800) >> 4))
        } else {
            Err(𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫(()))
        }
    }
}
impl 𝑩_𝒊𝒎𝒎𝒆𝒅𝒊𝒂𝒕𝒆_𝒔𝒐𝒖𝒓𝒄𝒆 for i32 {}

impl TryFrom<u32> for 𝐁_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    type Error = 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫;
    #[inline(always)]
    fn try_from(imm: u32) -> Result<𝐁_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞, 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫> {
        if let 0x00000000 = imm & 0xfffff001 {
            Ok(𝐁_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞((imm as i32 & 0x000007e0) << 20 | (imm as i32 & 0x0000001f) << 7 | (imm as i32 & 0x00000800) >> 4))
        } else {
            Err(𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫(()))
        }
    }
}
impl 𝑩_𝒊𝒎𝒎𝒆𝒅𝒊𝒂𝒕𝒆_𝒔𝒐𝒖𝒓𝒄𝒆 for u32 {}

impl TryFrom<i64> for 𝐁_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    type Error = 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫;
    #[inline(always)]
    fn try_from(imm: i64) -> Result<𝐁_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞, 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫> {
        if let 0x0000000000000000 | -0x0000000000001000 = imm & -0x0000000000000fff {
            Ok(𝐁_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞((imm as i32 & -0x0000000080000000) |
                           (imm as i32 & 0x00000000000007e0) << 20 |
                           (imm as i32 & 0x000000000000001f) << 7 |
                           (imm as i32 & 0x0000000000000800) >> 4))
        } else {
            Err(𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫(()))
        }
    }
}
impl 𝑩_𝒊𝒎𝒎𝒆𝒅𝒊𝒂𝒕𝒆_𝒔𝒐𝒖𝒓𝒄𝒆 for i64 {}

impl TryFrom<u64> for 𝐁_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    type Error = 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫;
    #[inline(always)]
    fn try_from(imm: u64) -> Result<𝐁_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞, 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫> {
        if let 0x0000000000000000 = imm & 0xfffffffffffff001 {
            Ok(𝐁_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞((imm as i32 & 0x00000000000007e0) << 20 |
                           (imm as i32 & 0x000000000000001f) << 7 |
                           (imm as i32 & 0x0000000000000800) >> 4))
        } else {
            Err(𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫(()))
        }
    }
}
impl 𝑩_𝒊𝒎𝒎𝒆𝒅𝒊𝒂𝒕𝒆_𝒔𝒐𝒖𝒓𝒄𝒆 for u64 {}

impl 𝐜𝐬𝐫_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    #[inline(always)]
    pub const fn new_from_instruction(encoding: i32) -> 𝐜𝐬𝐫_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
        𝐜𝐬𝐫_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞(encoding & 0x000f80000)
    }
    #[inline(always)]
    pub const unsafe fn new_unchecked(imm: i32) -> 𝐜𝐬𝐫_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
        𝐜𝐬𝐫_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞(imm << 15)
    }
    #[inline(always)]
    pub const fn new_const(imm: i32) -> Option<𝐜𝐬𝐫_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞> {
        if let 0x00000000 = imm & -0x00000020 {
            Some(𝐜𝐬𝐫_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞(imm << 15))
        } else {
            None
        }
    }
    #[inline(always)]
    pub fn new(imm: impl 𝒄𝒔𝒓_𝒊𝒎𝒎𝒆𝒅𝒊𝒂𝒕𝒆_𝒔𝒐𝒖𝒓𝒄𝒆) -> Option<𝐜𝐬𝐫_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞> {
        imm.try_into().ok()
    }
}
pub trait 𝒄𝒔𝒓_𝒊𝒎𝒎𝒆𝒅𝒊𝒂𝒕𝒆_𝒔𝒐𝒖𝒓𝒄𝒆: TryInto<𝐜𝐬𝐫_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞> {}

impl From<𝐜𝐬𝐫_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞> for i32 {
    #[inline(always)]
    fn from(imm: 𝐜𝐬𝐫_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞) -> i32 {
        imm.0 >> 15
    }
}

impl TryFrom<i8> for 𝐜𝐬𝐫_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    type Error = 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫;
    #[inline(always)]
    fn try_from(imm: i8) -> Result<𝐜𝐬𝐫_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞, 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫> {
        if let 0x00 = imm & -0x20 {
            Ok(𝐜𝐬𝐫_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞((imm as i32) << 15))
        } else {
            Err(𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫(()))
        }
    }
}
impl 𝒄𝒔𝒓_𝒊𝒎𝒎𝒆𝒅𝒊𝒂𝒕𝒆_𝒔𝒐𝒖𝒓𝒄𝒆 for i8 {}

impl TryFrom<u8> for 𝐜𝐬𝐫_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    type Error = 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫;
    #[inline(always)]
    fn try_from(imm: u8) -> Result<𝐜𝐬𝐫_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞, 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫> {
        if let 0x00 = imm & 0xe0 {
            Ok(𝐜𝐬𝐫_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞((imm as i32) << 15))
        } else {
            Err(𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫(()))
        }
    }
}
impl 𝒄𝒔𝒓_𝒊𝒎𝒎𝒆𝒅𝒊𝒂𝒕𝒆_𝒔𝒐𝒖𝒓𝒄𝒆 for u8 {}

impl TryFrom<i16> for 𝐜𝐬𝐫_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    type Error = 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫;
    #[inline(always)]
    fn try_from(imm: i16) -> Result<𝐜𝐬𝐫_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞, 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫> {
        if let 0x0000 = imm & -0x0020 {
            Ok(𝐜𝐬𝐫_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞((imm as i32) << 15))
        } else {
            Err(𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫(()))
        }
    }
}
impl 𝒄𝒔𝒓_𝒊𝒎𝒎𝒆𝒅𝒊𝒂𝒕𝒆_𝒔𝒐𝒖𝒓𝒄𝒆 for i16 {}

impl TryFrom<u16> for 𝐜𝐬𝐫_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    type Error = 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫;
    #[inline(always)]
    fn try_from(imm: u16) -> Result<𝐜𝐬𝐫_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞, 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫> {
        if let 0x0000 = imm & 0xffe0 {
            Ok(𝐜𝐬𝐫_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞((imm as i32) << 15))
        } else {
            Err(𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫(()))
        }
    }
}
impl 𝒄𝒔𝒓_𝒊𝒎𝒎𝒆𝒅𝒊𝒂𝒕𝒆_𝒔𝒐𝒖𝒓𝒄𝒆 for u16 {}

impl TryFrom<i32> for 𝐜𝐬𝐫_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    type Error = 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫;
    #[inline(always)]
    fn try_from(imm: i32) -> Result<𝐜𝐬𝐫_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞, 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫> {
        if let 0x00000000 = imm & -0x00000020 {
            Ok(𝐜𝐬𝐫_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞(imm << 15))
        } else {
            Err(𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫(()))
        }
    }
}
impl 𝒄𝒔𝒓_𝒊𝒎𝒎𝒆𝒅𝒊𝒂𝒕𝒆_𝒔𝒐𝒖𝒓𝒄𝒆 for i32 {}

impl TryFrom<u32> for 𝐜𝐬𝐫_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    type Error = 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫;
    #[inline(always)]
    fn try_from(imm: u32) -> Result<𝐜𝐬𝐫_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞, 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫> {
        if let 0x00000000 = imm & 0xffffffe0 {
            Ok(𝐜𝐬𝐫_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞((imm as i32) << 15))
        } else {
            Err(𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫(()))
        }
    }
}
impl 𝒄𝒔𝒓_𝒊𝒎𝒎𝒆𝒅𝒊𝒂𝒕𝒆_𝒔𝒐𝒖𝒓𝒄𝒆 for u32 {}

impl TryFrom<i64> for 𝐜𝐬𝐫_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    type Error = 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫;
    #[inline(always)]
    fn try_from(imm: i64) -> Result<𝐜𝐬𝐫_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞, 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫> {
        if let 0x0000000000000000 = imm & -0x0000000000000020 {
            Ok(𝐜𝐬𝐫_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞((imm as i32) << 15))
        } else {
            Err(𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫(()))
        }
    }
}
impl 𝒄𝒔𝒓_𝒊𝒎𝒎𝒆𝒅𝒊𝒂𝒕𝒆_𝒔𝒐𝒖𝒓𝒄𝒆 for i64 {}

impl TryFrom<u64> for 𝐜𝐬𝐫_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    type Error = 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫;
    #[inline(always)]
    fn try_from(imm: u64) -> Result<𝐜𝐬𝐫_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞, 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫> {
        if let 0x0000000000000000 = imm & 0xffffffffffffffe0 {
            Ok(𝐜𝐬𝐫_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞((imm as i32) << 15))
        } else {
            Err(𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫(()))
        }
    }
}
impl 𝒄𝒔𝒓_𝒊𝒎𝒎𝒆𝒅𝒊𝒂𝒕𝒆_𝒔𝒐𝒖𝒓𝒄𝒆 for u64 {}

impl 𝐈_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    #[inline(always)]
    pub const fn new_from_instruction(encoding: i32) -> 𝐈_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
        𝐈_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞(encoding & -0x00100000)
    }
    #[inline(always)]
    pub const unsafe fn new_unchecked(imm: i32) -> 𝐈_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
        𝐈_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞(imm << 20)
    }
    #[inline(always)]
    pub const fn new_const(imm: i32) -> Option<𝐈_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞> {
        if let 0x00000000 | -0x00000800 = imm & -0x00000800 {
            Some(𝐈_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞(imm << 20))
        } else {
            None
        }
    }
    #[inline(always)]
    pub fn new(imm: impl 𝑰_𝒊𝒎𝒎𝒆𝒅𝒊𝒂𝒕𝒆_𝒔𝒐𝒖𝒓𝒄𝒆) -> Option<𝐈_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞> {
        imm.try_into().ok()
    }
}
pub trait 𝑰_𝒊𝒎𝒎𝒆𝒅𝒊𝒂𝒕𝒆_𝒔𝒐𝒖𝒓𝒄𝒆: TryInto<𝐈_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞> {}

impl From<𝐈_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞> for i32 {
    #[inline(always)]
    fn from(imm: 𝐈_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞) -> i32 {
        imm.0 >> 20
    }
}

impl From<𝐏_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞> for 𝐈_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    #[inline(always)]
    fn from(imm: 𝐏_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞) -> 𝐈_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
        𝐈_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞(imm.0)
    }
}

impl From<𝐒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞> for 𝐈_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    #[inline(always)]
    fn from(imm: 𝐒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞) -> 𝐈_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
        𝐈_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞((imm.0 & -0x02000000) | (imm.0 & 0x01f00000) >> 13)
    }
}

impl From<𝐜𝐬𝐫_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞> for 𝐈_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    #[inline(always)]
    fn from(imm: 𝐜𝐬𝐫_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞) -> 𝐈_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
        𝐈_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞(imm.0 << 5)
    }
}

impl From<𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟑𝟐_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞> for 𝐈_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    #[inline(always)]
    fn from(imm: 𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟑𝟐_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞) -> 𝐈_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
        𝐈_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞(imm.0)
    }
}

impl From<𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟔𝟒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞> for 𝐈_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    #[inline(always)]
    fn from(imm: 𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟔𝟒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞) -> 𝐈_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
        𝐈_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞(imm.0)
    }
}

impl From<i8> for 𝐈_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    #[inline(always)]
    fn from(imm: i8) -> 𝐈_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
        𝐈_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞((imm as i32) << 20)
    }
}
impl 𝑰_𝒊𝒎𝒎𝒆𝒅𝒊𝒂𝒕𝒆_𝒔𝒐𝒖𝒓𝒄𝒆 for i8 {}

impl From<u8> for 𝐈_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    #[inline(always)]
    fn from(imm: u8) -> 𝐈_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
        𝐈_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞((imm as i32) << 20)
    }
}
impl 𝑰_𝒊𝒎𝒎𝒆𝒅𝒊𝒂𝒕𝒆_𝒔𝒐𝒖𝒓𝒄𝒆 for u8 {}

impl TryFrom<i16> for 𝐈_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    type Error = 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫;
    #[inline(always)]
    fn try_from(imm: i16) -> Result<𝐈_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞, 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫> {
        if let 0x0000 | -0x0800 = imm & -0x0800 {
            Ok(𝐈_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞((imm as i32) << 20))
        } else {
            Err(𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫(()))
        }
    }
}
impl 𝑰_𝒊𝒎𝒎𝒆𝒅𝒊𝒂𝒕𝒆_𝒔𝒐𝒖𝒓𝒄𝒆 for i16 {}

impl TryFrom<u16> for 𝐈_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    type Error = 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫;
    #[inline(always)]
    fn try_from(imm: u16) -> Result<𝐈_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞, 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫> {
        if let 0x0000 = imm & 0xf800 {
            Ok(𝐈_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞((imm as i32) << 20))
        } else {
            Err(𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫(()))
        }
    }
}
impl 𝑰_𝒊𝒎𝒎𝒆𝒅𝒊𝒂𝒕𝒆_𝒔𝒐𝒖𝒓𝒄𝒆 for u16 {}

impl TryFrom<i32> for 𝐈_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    type Error = 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫;
    #[inline(always)]
    fn try_from(imm: i32) -> Result<𝐈_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞, 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫> {
        if let 0x00000000 | -0x00000800 = imm & -0x00000800 {
            Ok(𝐈_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞(imm << 20))
        } else {
            Err(𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫(()))
        }
    }
}
impl 𝑰_𝒊𝒎𝒎𝒆𝒅𝒊𝒂𝒕𝒆_𝒔𝒐𝒖𝒓𝒄𝒆 for i32 {}

impl TryFrom<u32> for 𝐈_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    type Error = 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫;
    #[inline(always)]
    fn try_from(imm: u32) -> Result<𝐈_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞, 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫> {
        if let 0x00000000 = imm & 0xfffff800 {
            Ok(𝐈_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞((imm as i32) << 20))
        } else {
            Err(𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫(()))
        }
    }
}
impl 𝑰_𝒊𝒎𝒎𝒆𝒅𝒊𝒂𝒕𝒆_𝒔𝒐𝒖𝒓𝒄𝒆 for u32 {}

impl TryFrom<i64> for 𝐈_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    type Error = 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫;
    #[inline(always)]
    fn try_from(imm: i64) -> Result<𝐈_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞, 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫> {
        if let 0x0000000000000000 | -0x0000000000000800 = imm & -0x0000000000000800 {
            Ok(𝐈_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞((imm as i32) << 20))
        } else {
            Err(𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫(()))
        }
    }
}
impl 𝑰_𝒊𝒎𝒎𝒆𝒅𝒊𝒂𝒕𝒆_𝒔𝒐𝒖𝒓𝒄𝒆 for i64 {}

impl TryFrom<u64> for 𝐈_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    type Error = 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫;
    #[inline(always)]
    fn try_from(imm: u64) -> Result<𝐈_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞, 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫> {
        if let 0x0000000000000000 = imm & 0xfffffffffffff800 {
            Ok(𝐈_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞((imm as i32) << 20))
        } else {
            Err(𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫(()))
        }
    }
}
impl 𝑰_𝒊𝒎𝒎𝒆𝒅𝒊𝒂𝒕𝒆_𝒔𝒐𝒖𝒓𝒄𝒆 for u64 {}


impl 𝐉_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    #[inline(always)]
    pub const fn new_from_instruction(encoding: i32) -> 𝐉_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
        𝐉_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞(encoding & -0x00001000)
    }
    #[inline(always)]
    pub const unsafe fn new_unchecked(imm: i32) -> 𝐉_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
        𝐉_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞((imm & -0x80000000) | (imm & 0x000007e0) << 20 | (imm & 0x0000001f) << 7 | (imm & 0x00000800) >> 4)
    }
    #[inline(always)]
    pub const fn new_const(imm: i32) -> Option<𝐉_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞> {
        if let 0x00000000 | -0x00100000 = imm & -0x000fffff {
            Some(𝐉_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞((imm & -0x80000000) | (imm & 0x0000007ff) << 20 | (imm & 0x00000800) << 9 | (imm & 0x000ff000)))
        } else {
            None
        }
    }
    #[inline(always)]
    pub fn new(imm: impl 𝑱_𝒊𝒎𝒎𝒆𝒅𝒊𝒂𝒕𝒆_𝒔𝒐𝒖𝒓𝒄𝒆) -> Option<𝐉_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞> {
        imm.try_into().ok()
    }
}
pub trait 𝑱_𝒊𝒎𝒎𝒆𝒅𝒊𝒂𝒕𝒆_𝒔𝒐𝒖𝒓𝒄𝒆: TryInto<𝐉_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞> {}

impl From<𝐉_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞> for i32 {
    #[inline(always)]
    fn from(imm: 𝐉_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞) -> i32 {
        (imm.0 & -0x80000000) >> 11 | (imm.0 & 0x7fe00000) >> 20 | (imm.0 & 0x00100000) >> 9 | (imm.0 & 0x000ff000)
    }
}

impl TryFrom<i8> for 𝐉_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    type Error = 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫;
    #[inline(always)]
    fn try_from(imm: i8) -> Result<𝐉_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞, 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫> {
        if let 0x00 = imm & 0x01 {
            Ok(𝐉_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞((imm as i32 & -0x07e01000) | (imm as i32 & 0xff) << 20))
        } else {
            Err(𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫(()))
        }
    }
}
impl 𝑱_𝒊𝒎𝒎𝒆𝒅𝒊𝒂𝒕𝒆_𝒔𝒐𝒖𝒓𝒄𝒆 for i8 {}

impl TryFrom<u8> for 𝐉_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    type Error = 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫;
    #[inline(always)]
    fn try_from(imm: u8) -> Result<𝐉_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞, 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫> {
        if let 0x00 = imm & 0x01 {
            Ok(𝐉_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞((imm as i32 & 0xff) << 20))
        } else {
            Err(𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫(()))
        }
    }
}
impl 𝑱_𝒊𝒎𝒎𝒆𝒅𝒊𝒂𝒕𝒆_𝒔𝒐𝒖𝒓𝒄𝒆 for u8 {}

impl TryFrom<i16> for 𝐉_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    type Error = 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫;
    #[inline(always)]
    fn try_from(imm: i16) -> Result<𝐉_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞, 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫> {
        if let 0x0000 = imm & 0x0001 {
            Ok(𝐉_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞((imm as i32 & -0x7ff10000) |
                           (imm as i32 & 0x07ff) << 20 |
                           (imm as i32 & 0x0800) << 9 |
                           (imm as i32 & 0xf000)))
        } else {
            Err(𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫(()))
        }
    }
}
impl 𝑱_𝒊𝒎𝒎𝒆𝒅𝒊𝒂𝒕𝒆_𝒔𝒐𝒖𝒓𝒄𝒆 for i16 {}

impl TryFrom<u16> for 𝐉_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    type Error = 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫;
    #[inline(always)]
    fn try_from(imm: u16) -> Result<𝐉_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞, 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫> {
        if let 0x0000 = imm & 0x0001 {
            Ok(𝐉_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞((imm as i32 & 0x07ff) << 20 | (imm as i32 & 0x0800) << 9 | (imm as i32 & 0xf000)))
        } else {
            Err(𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫(()))
        }
    }
}
impl 𝑱_𝒊𝒎𝒎𝒆𝒅𝒊𝒂𝒕𝒆_𝒔𝒐𝒖𝒓𝒄𝒆 for u16 {}

impl TryFrom<i32> for 𝐉_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    type Error = 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫;
    #[inline(always)]
    fn try_from(imm: i32) -> Result<𝐉_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞, 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫> {
        if let 0x00000000 | -0x00100000 = imm & -0x000fffff {
            Ok(𝐉_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞((imm & -0x80000000) | (imm & 0x0000007ff) << 20 | (imm & 0x00000800) << 9 | (imm & 0x000ff000)))
        } else {
            Err(𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫(()))
        }
    }
}
impl 𝑱_𝒊𝒎𝒎𝒆𝒅𝒊𝒂𝒕𝒆_𝒔𝒐𝒖𝒓𝒄𝒆 for i32 {}

impl TryFrom<u32> for 𝐉_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    type Error = 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫;
    #[inline(always)]
    fn try_from(imm: u32) -> Result<𝐉_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞, 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫> {
        if let 0x00000000 = imm & 0xfff00001 {
            Ok(𝐉_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞((imm as i32 & 0x0000007ff) << 20 |
                           (imm as i32 & 0x00000800) << 9 |
                           (imm as i32 & 0x000ff000)))
        } else {
            Err(𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫(()))
        }
    }
}
impl 𝑱_𝒊𝒎𝒎𝒆𝒅𝒊𝒂𝒕𝒆_𝒔𝒐𝒖𝒓𝒄𝒆 for u32 {}

impl TryFrom<i64> for 𝐉_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    type Error = 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫;
    #[inline(always)]
    fn try_from(imm: i64) -> Result<𝐉_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞, 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫> {
        if let 0x0000000000000000 | -0x0000000000100000 = imm & -0x00000000000fffff {
            Ok(𝐉_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞((imm as i32 & -0x0000000080000000) |
                           (imm as i32 & 0x000000000000007ff) << 20 |
                           (imm as i32 & 0x0000000000000800) << 9 |
                           (imm as i32 & 0x00000000000ff000)))
        } else {
            Err(𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫(()))
        }
    }
}
impl 𝑱_𝒊𝒎𝒎𝒆𝒅𝒊𝒂𝒕𝒆_𝒔𝒐𝒖𝒓𝒄𝒆 for i64 {}

impl TryFrom<u64> for 𝐉_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    type Error = 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫;
    #[inline(always)]
    fn try_from(imm: u64) -> Result<𝐉_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞, 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫> {
        if let 0x0000000000000000 = imm & 0xfffffffffff00001 {
            Ok(𝐉_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞((imm as i32 & 0x000000000000007ff) << 20 |
                           (imm as i32 & 0x0000000000000800) << 9 |
                           (imm as i32 & 0x00000000000ff000)))
        } else {
            Err(𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫(()))
        }
    }
}
impl 𝑱_𝒊𝒎𝒎𝒆𝒅𝒊𝒂𝒕𝒆_𝒔𝒐𝒖𝒓𝒄𝒆 for u64 {}

impl 𝐏_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    #[inline(always)]
    pub const fn new_from_instruction(encoding: i32) -> 𝐏_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
        𝐏_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞(encoding & -0x02000000)
    }
    #[inline(always)]
    pub const unsafe fn new_unchecked(imm: i32) -> 𝐏_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
        𝐏_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞((imm & 0x00000fe0) << 20)
    }
    #[inline(always)]
    pub const fn new_const(imm: i32) -> Option<𝐏_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞> {
        if let 0x00000000 | -0x00000800 = imm & -0x000007e1 {
            Some(𝐏_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞(imm << 20))
        } else {
            None
        }
    }
    #[inline(always)]
    pub fn new(imm: impl 𝑷_𝒊𝒎𝒎𝒆𝒅𝒊𝒂𝒕𝒆_𝒔𝒐𝒖𝒓𝒄𝒆) -> Option<𝐏_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞> {
        imm.try_into().ok()
    }
}
pub trait 𝑷_𝒊𝒎𝒎𝒆𝒅𝒊𝒂𝒕𝒆_𝒔𝒐𝒖𝒓𝒄𝒆: TryInto<𝐏_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞> {}

impl From<𝐏_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞> for i32 {
    #[inline(always)]
    fn from(imm: 𝐏_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞) -> i32 {
        imm.0 >> 20
    }
}

impl TryFrom<i8> for 𝐏_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    type Error = 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫;
    #[inline(always)]
    fn try_from(imm: i8) -> Result<𝐏_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞, 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫> {
        if let 0x00 | -0x80 = imm & -0x61 {
            Ok(𝐏_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞((imm as i32) << 20))
        } else {
            Err(𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫(()))
        }
    }
}
impl 𝑷_𝒊𝒎𝒎𝒆𝒅𝒊𝒂𝒕𝒆_𝒔𝒐𝒖𝒓𝒄𝒆 for i8 {}

impl TryFrom<u8> for 𝐏_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    type Error = 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫;
    #[inline(always)]
    fn try_from(imm: u8) -> Result<𝐏_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞, 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫> {
        if let 0x00 = imm & 0x1f {
            Ok(𝐏_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞((imm as i32) << 20))
        } else {
            Err(𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫(()))
        }
    }
}
impl 𝑷_𝒊𝒎𝒎𝒆𝒅𝒊𝒂𝒕𝒆_𝒔𝒐𝒖𝒓𝒄𝒆 for u8 {}

impl TryFrom<i16> for 𝐏_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    type Error = 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫;
    #[inline(always)]
    fn try_from(imm: i16) -> Result<𝐏_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞, 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫> {
        if let 0x0000 | -0x0800 = imm & -0x07e1 {
            Ok(𝐏_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞((imm as i32) << 20))
        } else {
            Err(𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫(()))
        }
    }
}
impl 𝑷_𝒊𝒎𝒎𝒆𝒅𝒊𝒂𝒕𝒆_𝒔𝒐𝒖𝒓𝒄𝒆 for i16 {}

impl TryFrom<u16> for 𝐏_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    type Error = 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫;
    #[inline(always)]
    fn try_from(imm: u16) -> Result<𝐏_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞, 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫> {
        if let 0x0000 = imm & 0xf81f {
            Ok(𝐏_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞((imm as i32) << 20))
        } else {
            Err(𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫(()))
        }
    }
}
impl 𝑷_𝒊𝒎𝒎𝒆𝒅𝒊𝒂𝒕𝒆_𝒔𝒐𝒖𝒓𝒄𝒆 for u16 {}

impl TryFrom<i32> for 𝐏_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    type Error = 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫;
    #[inline(always)]
    fn try_from(imm: i32) -> Result<𝐏_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞, 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫> {
        if let 0x00000000 | -0x00000800 = imm & -0x000007e1 {
            Ok(𝐏_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞(imm << 20))
        } else {
            Err(𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫(()))
        }
    }
}
impl 𝑷_𝒊𝒎𝒎𝒆𝒅𝒊𝒂𝒕𝒆_𝒔𝒐𝒖𝒓𝒄𝒆 for i32 {}

impl TryFrom<u32> for 𝐏_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    type Error = 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫;
    #[inline(always)]
    fn try_from(imm: u32) -> Result<𝐏_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞, 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫> {
        if let 0x00000000 = imm & 0xfffff81f {
            Ok(𝐏_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞((imm as i32) << 20))
        } else {
            Err(𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫(()))
        }
    }
}
impl 𝑷_𝒊𝒎𝒎𝒆𝒅𝒊𝒂𝒕𝒆_𝒔𝒐𝒖𝒓𝒄𝒆 for u32 {}

impl TryFrom<i64> for 𝐏_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    type Error = 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫;
    #[inline(always)]
    fn try_from(imm: i64) -> Result<𝐏_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞, 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫> {
        if let 0x0000000000000000 | -0x0000000000000800 = imm & -0x00000000000007e1 {
            Ok(𝐏_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞((imm as i32) << 20))
        } else {
            Err(𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫(()))
        }
    }
}
impl 𝑷_𝒊𝒎𝒎𝒆𝒅𝒊𝒂𝒕𝒆_𝒔𝒐𝒖𝒓𝒄𝒆 for i64 {}

impl TryFrom<u64> for 𝐏_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    type Error = 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫;
    #[inline(always)]
    fn try_from(imm: u64) -> Result<𝐏_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞, 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫> {
        if let 0x0000000000000000 = imm & 0xfffffffffffff81f {
            Ok(𝐏_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞((imm as i32) << 20))
        } else {
            Err(𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫(()))
        }
    }
}
impl 𝑷_𝒊𝒎𝒎𝒆𝒅𝒊𝒂𝒕𝒆_𝒔𝒐𝒖𝒓𝒄𝒆 for u64 {}

impl 𝐒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    #[inline(always)]
    pub const fn new_from_instruction(encoding: i32) -> 𝐒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
        𝐒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞(encoding & -0x01fff080)
    }
    #[inline(always)]
    pub const unsafe fn new_unchecked(imm: i32) -> 𝐒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
        𝐒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞((imm & 0x00000fe0) << 20 | (imm & 0x0000001f) << 7)
    }
    #[inline(always)]
    pub const fn new_const(imm: i32) -> Option<𝐒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞> {
        if let 0x00000000 | -0x00000800 = imm & -0x00000800 {
            Some(𝐒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞((imm & 0x00000fe0) << 20 | (imm & 0x0000001f) << 7))
        } else {
            None
        }
    }
    #[inline(always)]
    pub fn new(imm: impl 𝑺_𝒊𝒎𝒎𝒆𝒅𝒊𝒂𝒕𝒆_𝒔𝒐𝒖𝒓𝒄𝒆) -> Option<𝐒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞> {
        imm.try_into().ok()
    }
}
pub trait 𝑺_𝒊𝒎𝒎𝒆𝒅𝒊𝒂𝒕𝒆_𝒔𝒐𝒖𝒓𝒄𝒆: TryInto<𝐒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞> {}

impl From<𝐒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞> for i32 {
    #[inline(always)]
    fn from(imm: 𝐒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞) -> i32 {
        imm.0 >> 20 | ((imm.0 & 0x00000f80) >> 7)
    }
}

impl From<𝐈_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞> for 𝐒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    #[inline(always)]
    fn from(imm: 𝐈_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞) -> 𝐒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
        𝐒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞((imm.0 & -0x02000000) | (imm.0 & 0x00000f80) << 13)
    }
}

impl From<𝐏_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞> for 𝐒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    #[inline(always)]
    fn from(imm: 𝐏_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞) -> 𝐒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
        𝐒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞(imm.0)
    }
}

impl From<𝐜𝐬𝐫_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞> for 𝐒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    #[inline(always)]
    fn from(imm: 𝐜𝐬𝐫_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞) -> 𝐒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
        𝐒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞(imm.0 >> 8)
    }
}

impl From<𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟑𝟐_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞> for 𝐒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    #[inline(always)]
    fn from(imm: 𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟑𝟐_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞) -> 𝐒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
        𝐒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞((imm.0 & 0x00000f80) << 13)
    }
}

impl From<𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟔𝟒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞> for 𝐒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    #[inline(always)]
    fn from(imm: 𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟔𝟒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞) -> 𝐒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
        𝐒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞((imm.0 & -0x02000000) | (imm.0 & 0x00000f80) << 13)
    }
}

impl From<i8> for 𝐒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    #[inline(always)]
    fn from(imm: i8) -> 𝐒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
        𝐒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞((imm as i32 & 0x0fe0) << 20 | (imm as i32 & 0x001f) << 7)
    }
}
impl 𝑺_𝒊𝒎𝒎𝒆𝒅𝒊𝒂𝒕𝒆_𝒔𝒐𝒖𝒓𝒄𝒆 for i8 {}

impl From<u8> for 𝐒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    #[inline(always)]
    fn from(imm: u8) -> 𝐒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
        𝐒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞((imm as i32 & 0x0fe0) << 20 | (imm as i32 & 0x001f) << 7)
    }
}
impl 𝑺_𝒊𝒎𝒎𝒆𝒅𝒊𝒂𝒕𝒆_𝒔𝒐𝒖𝒓𝒄𝒆 for u8 {}

impl TryFrom<i16> for 𝐒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    type Error = 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫;
    #[inline(always)]
    fn try_from(imm: i16) -> Result<𝐒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞, 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫> {
        if let 0x0000 | -0x0800 = imm & -0x0800 {
            Ok(𝐒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞((imm as i32 & 0x0fe0) << 20 | (imm as i32 & 0x001f) << 7))
        } else {
            Err(𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫(()))
        }
    }
}
impl 𝑺_𝒊𝒎𝒎𝒆𝒅𝒊𝒂𝒕𝒆_𝒔𝒐𝒖𝒓𝒄𝒆 for i16 {}

impl TryFrom<u16> for 𝐒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    type Error = 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫;
    #[inline(always)]
    fn try_from(imm: u16) -> Result<𝐒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞, 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫> {
        if let 0x0000 = imm & 0xf800 {
            Ok(𝐒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞((imm as i32 & 0x0fe0) << 20 | (imm as i32 & 0x001f) << 7))
        } else {
            Err(𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫(()))
        }
    }
}
impl 𝑺_𝒊𝒎𝒎𝒆𝒅𝒊𝒂𝒕𝒆_𝒔𝒐𝒖𝒓𝒄𝒆 for u16 {}

impl TryFrom<i32> for 𝐒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    type Error = 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫;
    #[inline(always)]
    fn try_from(imm: i32) -> Result<𝐒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞, 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫> {
        if let 0x00000000 | -0x00000800 = imm & -0x00000800 {
            Ok(𝐒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞((imm & 0x00000fe0) << 20 | (imm & 0x0000001f) << 7))
        } else {
            Err(𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫(()))
        }
    }
}
impl 𝑺_𝒊𝒎𝒎𝒆𝒅𝒊𝒂𝒕𝒆_𝒔𝒐𝒖𝒓𝒄𝒆 for i32 {}

impl TryFrom<u32> for 𝐒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    type Error = 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫;
    #[inline(always)]
    fn try_from(imm: u32) -> Result<𝐒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞, 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫> {
        if let 0x00000000 = imm & 0xfffff800 {
            Ok(𝐒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞((imm as i32 & 0x00000fe0) << 20 | (imm as i32 & 0x0000001f) << 7))
        } else {
            Err(𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫(()))
        }
    }
}
impl 𝑺_𝒊𝒎𝒎𝒆𝒅𝒊𝒂𝒕𝒆_𝒔𝒐𝒖𝒓𝒄𝒆 for u32 {}

impl TryFrom<i64> for 𝐒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    type Error = 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫;
    #[inline(always)]
    fn try_from(imm: i64) -> Result<𝐒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞, 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫> {
        if let 0x0000000000000000 | -0x0000000000000800 = imm & -0x0000000000000800 {
            Ok(𝐒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞((imm as i32 & 0x0000000000000fe0) << 20 | (imm as i32 & 0x000000000000001f) << 7))
        } else {
            Err(𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫(()))
        }
    }
}
impl 𝑺_𝒊𝒎𝒎𝒆𝒅𝒊𝒂𝒕𝒆_𝒔𝒐𝒖𝒓𝒄𝒆 for i64 {}

impl TryFrom<u64> for 𝐒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    type Error = 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫;
    #[inline(always)]
    fn try_from(imm: u64) -> Result<𝐒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞, 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫> {
        if let 0x0000000000000000 = imm & 0xfffffffffffff800 {
            Ok(𝐒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞((imm as i32 & 0x0000000000000fe0) << 20 | (imm as i32 & 0x000000000000001f) << 7))
        } else {
            Err(𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫(()))
        }
    }
}
impl 𝑺_𝒊𝒎𝒎𝒆𝒅𝒊𝒂𝒕𝒆_𝒔𝒐𝒖𝒓𝒄𝒆 for u64 {}

impl 𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟑𝟐_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    #[inline(always)]
    pub const fn new_from_instruction(encoding: i32) -> 𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟑𝟐_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
        𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟑𝟐_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞(encoding & 0x01f00000)
    }
    #[inline(always)]
    pub const unsafe fn new_unchecked(imm: i32) -> 𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟑𝟐_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
        𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟑𝟐_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞(imm << 20)
    }
    #[inline(always)]
    pub const fn new_const(imm: i32) -> Option<𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟑𝟐_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞> {
        if let 0x00000000 = imm & -0x00000020 {
            Some(𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟑𝟐_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞(imm << 20))
        } else {
            None
        }
    }
    #[inline(always)]
    pub fn new(imm: impl 𝒔𝒉𝒊𝒇𝒕_𝑹𝑽𝟑𝟐_𝒊𝒎𝒎𝒆𝒅𝒊𝒂𝒕𝒆_𝒔𝒐𝒖𝒓𝒄𝒆) -> Option<𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟑𝟐_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞> {
        imm.try_into().ok()
    }
}
pub trait 𝒔𝒉𝒊𝒇𝒕_𝑹𝑽𝟑𝟐_𝒊𝒎𝒎𝒆𝒅𝒊𝒂𝒕𝒆_𝒔𝒐𝒖𝒓𝒄𝒆: TryInto<𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟑𝟐_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞> {}

impl From<𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟑𝟐_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞> for i32 {
    #[inline(always)]
    fn from(imm: 𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟑𝟐_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞) -> i32 {
        imm.0 >> 20
    }
}

impl TryFrom<i8> for 𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟑𝟐_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    type Error = 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫;
    #[inline(always)]
    fn try_from(imm: i8) -> Result<𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟑𝟐_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞, 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫> {
        if let 0x00 = imm & -0x20 {
            Ok(𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟑𝟐_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞((imm as i32) << 20))
        } else {
            Err(𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫(()))
        }
    }
}
impl 𝒔𝒉𝒊𝒇𝒕_𝑹𝑽𝟑𝟐_𝒊𝒎𝒎𝒆𝒅𝒊𝒂𝒕𝒆_𝒔𝒐𝒖𝒓𝒄𝒆 for i8 {}

impl TryFrom<u8> for 𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟑𝟐_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    type Error = 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫;
    #[inline(always)]
    fn try_from(imm: u8) -> Result<𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟑𝟐_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞, 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫> {
        if let 0x00 = imm & 0xe0 {
            Ok(𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟑𝟐_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞((imm as i32) << 20))
        } else {
            Err(𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫(()))
        }
    }
}
impl 𝒔𝒉𝒊𝒇𝒕_𝑹𝑽𝟑𝟐_𝒊𝒎𝒎𝒆𝒅𝒊𝒂𝒕𝒆_𝒔𝒐𝒖𝒓𝒄𝒆 for u8 {}

impl TryFrom<i16> for 𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟑𝟐_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    type Error = 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫;
    #[inline(always)]
    fn try_from(imm: i16) -> Result<𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟑𝟐_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞, 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫> {
        if let 0x0000 = imm & -0x0020 {
            Ok(𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟑𝟐_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞((imm as i32) << 20))
        } else {
            Err(𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫(()))
        }
    }
}
impl 𝒔𝒉𝒊𝒇𝒕_𝑹𝑽𝟑𝟐_𝒊𝒎𝒎𝒆𝒅𝒊𝒂𝒕𝒆_𝒔𝒐𝒖𝒓𝒄𝒆 for i16 {}

impl TryFrom<u16> for 𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟑𝟐_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    type Error = 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫;
    #[inline(always)]
    fn try_from(imm: u16) -> Result<𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟑𝟐_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞, 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫> {
        if let 0x0000 = imm & 0xffe0 {
            Ok(𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟑𝟐_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞((imm as i32) << 20))
        } else {
            Err(𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫(()))
        }
    }
}
impl 𝒔𝒉𝒊𝒇𝒕_𝑹𝑽𝟑𝟐_𝒊𝒎𝒎𝒆𝒅𝒊𝒂𝒕𝒆_𝒔𝒐𝒖𝒓𝒄𝒆 for u16 {}

impl TryFrom<i32> for 𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟑𝟐_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    type Error = 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫;
    #[inline(always)]
    fn try_from(imm: i32) -> Result<𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟑𝟐_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞, 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫> {
        if let 0x00000000 = imm & -0x00000020 {
            Ok(𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟑𝟐_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞(imm << 20))
        } else {
            Err(𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫(()))
        }
    }
}
impl 𝒔𝒉𝒊𝒇𝒕_𝑹𝑽𝟑𝟐_𝒊𝒎𝒎𝒆𝒅𝒊𝒂𝒕𝒆_𝒔𝒐𝒖𝒓𝒄𝒆 for i32 {}

impl TryFrom<u32> for 𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟑𝟐_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    type Error = 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫;
    #[inline(always)]
    fn try_from(imm: u32) -> Result<𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟑𝟐_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞, 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫> {
        if let 0x00000000 = imm & 0xffffffe0 {
            Ok(𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟑𝟐_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞((imm as i32) << 20))
        } else {
            Err(𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫(()))
        }
    }
}
impl 𝒔𝒉𝒊𝒇𝒕_𝑹𝑽𝟑𝟐_𝒊𝒎𝒎𝒆𝒅𝒊𝒂𝒕𝒆_𝒔𝒐𝒖𝒓𝒄𝒆 for u32 {}

impl TryFrom<i64> for 𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟑𝟐_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    type Error = 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫;
    #[inline(always)]
    fn try_from(imm: i64) -> Result<𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟑𝟐_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞, 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫> {
        if let 0x0000000000000000 = imm & -0x0000000000000020 {
            Ok(𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟑𝟐_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞((imm as i32) << 20))
        } else {
            Err(𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫(()))
        }
    }
}
impl 𝒔𝒉𝒊𝒇𝒕_𝑹𝑽𝟑𝟐_𝒊𝒎𝒎𝒆𝒅𝒊𝒂𝒕𝒆_𝒔𝒐𝒖𝒓𝒄𝒆 for i64 {}

impl TryFrom<u64> for 𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟑𝟐_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    type Error = 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫;
    #[inline(always)]
    fn try_from(imm: u64) -> Result<𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟑𝟐_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞, 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫> {
        if let 0x0000000000000000 = imm & 0xffffffffffffffe0 {
            Ok(𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟑𝟐_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞((imm as i32) << 20))
        } else {
            Err(𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫(()))
        }
    }
}
impl 𝒔𝒉𝒊𝒇𝒕_𝑹𝑽𝟑𝟐_𝒊𝒎𝒎𝒆𝒅𝒊𝒂𝒕𝒆_𝒔𝒐𝒖𝒓𝒄𝒆 for u64 {}

impl 𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟔𝟒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    #[inline(always)]
    pub const fn new_from_instruction(encoding: i32) -> 𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟔𝟒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
        𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟔𝟒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞(encoding & 0x03f00000)
    }
    #[inline(always)]
    pub const unsafe fn new_unchecked(imm: i32) -> 𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟔𝟒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
        𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟔𝟒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞(imm << 20)
    }
    #[inline(always)]
    pub const fn new_const(imm: i32) -> Option<𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟔𝟒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞> {
        if let 0x00000000 = imm & -0x00000040 {
            Some(𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟔𝟒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞(imm << 20))
        } else {
            None
        }
    }
    #[inline(always)]
    pub fn new(imm: impl 𝒔𝒉𝒊𝒇𝒕_𝑹𝑽𝟔𝟒_𝒊𝒎𝒎𝒆𝒅𝒊𝒂𝒕𝒆_𝒔𝒐𝒖𝒓𝒄𝒆) -> Option<𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟔𝟒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞> {
        imm.try_into().ok()
    }
}
pub trait 𝒔𝒉𝒊𝒇𝒕_𝑹𝑽𝟔𝟒_𝒊𝒎𝒎𝒆𝒅𝒊𝒂𝒕𝒆_𝒔𝒐𝒖𝒓𝒄𝒆: TryInto<𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟔𝟒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞> {}

impl From<𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟔𝟒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞> for i32 {
    #[inline(always)]
    fn from(imm: 𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟔𝟒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞) -> i32 {
        imm.0 >> 20
    }
}

impl TryFrom<i8> for 𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟔𝟒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    type Error = 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫;
    #[inline(always)]
    fn try_from(imm: i8) -> Result<𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟔𝟒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞, 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫> {
        if let 0x00 = imm & -0x40 {
            Ok(𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟔𝟒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞((imm as i32) << 20))
        } else {
            Err(𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫(()))
        }
    }
}
impl 𝒔𝒉𝒊𝒇𝒕_𝑹𝑽𝟔𝟒_𝒊𝒎𝒎𝒆𝒅𝒊𝒂𝒕𝒆_𝒔𝒐𝒖𝒓𝒄𝒆 for i8 {}

impl TryFrom<u8> for 𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟔𝟒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    type Error = 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫;
    #[inline(always)]
    fn try_from(imm: u8) -> Result<𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟔𝟒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞, 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫> {
        if let 0x00 = imm & 0xc0 {
            Ok(𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟔𝟒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞((imm as i32) << 20))
        } else {
            Err(𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫(()))
        }
    }
}
impl 𝒔𝒉𝒊𝒇𝒕_𝑹𝑽𝟔𝟒_𝒊𝒎𝒎𝒆𝒅𝒊𝒂𝒕𝒆_𝒔𝒐𝒖𝒓𝒄𝒆 for u8 {}

impl TryFrom<i16> for 𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟔𝟒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    type Error = 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫;
    #[inline(always)]
    fn try_from(imm: i16) -> Result<𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟔𝟒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞, 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫> {
        if let 0x0000 = imm & -0x0040 {
            Ok(𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟔𝟒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞((imm as i32) << 20))
        } else {
            Err(𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫(()))
        }
    }
}
impl 𝒔𝒉𝒊𝒇𝒕_𝑹𝑽𝟔𝟒_𝒊𝒎𝒎𝒆𝒅𝒊𝒂𝒕𝒆_𝒔𝒐𝒖𝒓𝒄𝒆 for i16 {}

impl TryFrom<u16> for 𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟔𝟒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    type Error = 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫;
    #[inline(always)]
    fn try_from(imm: u16) -> Result<𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟔𝟒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞, 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫> {
        if let 0x0000 = imm & 0xffc0 {
            Ok(𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟔𝟒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞((imm as i32) << 20))
        } else {
            Err(𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫(()))
        }
    }
}
impl 𝒔𝒉𝒊𝒇𝒕_𝑹𝑽𝟔𝟒_𝒊𝒎𝒎𝒆𝒅𝒊𝒂𝒕𝒆_𝒔𝒐𝒖𝒓𝒄𝒆 for u16 {}

impl TryFrom<i32> for 𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟔𝟒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    type Error = 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫;
    #[inline(always)]
    fn try_from(imm: i32) -> Result<𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟔𝟒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞, 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫> {
        if let 0x00000000 = imm & -0x00000040 {
            Ok(𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟔𝟒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞(imm << 20))
        } else {
            Err(𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫(()))
        }
    }
}
impl 𝒔𝒉𝒊𝒇𝒕_𝑹𝑽𝟔𝟒_𝒊𝒎𝒎𝒆𝒅𝒊𝒂𝒕𝒆_𝒔𝒐𝒖𝒓𝒄𝒆 for i32 {}

impl TryFrom<u32> for 𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟔𝟒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    type Error = 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫;
    #[inline(always)]
    fn try_from(imm: u32) -> Result<𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟔𝟒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞, 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫> {
        if let 0x00000000 = imm & 0xffffffc0 {
            Ok(𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟔𝟒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞((imm as i32) << 20))
        } else {
            Err(𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫(()))
        }
    }
}
impl 𝒔𝒉𝒊𝒇𝒕_𝑹𝑽𝟔𝟒_𝒊𝒎𝒎𝒆𝒅𝒊𝒂𝒕𝒆_𝒔𝒐𝒖𝒓𝒄𝒆 for u32 {}

impl TryFrom<i64> for 𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟔𝟒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    type Error = 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫;
    #[inline(always)]
    fn try_from(imm: i64) -> Result<𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟔𝟒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞, 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫> {
        if let 0x0000000000000000 = imm & -0x0000000000000040 {
            Ok(𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟔𝟒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞((imm as i32) << 20))
        } else {
            Err(𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫(()))
        }
    }
}
impl 𝒔𝒉𝒊𝒇𝒕_𝑹𝑽𝟔𝟒_𝒊𝒎𝒎𝒆𝒅𝒊𝒂𝒕𝒆_𝒔𝒐𝒖𝒓𝒄𝒆 for i64 {}

impl TryFrom<u64> for 𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟔𝟒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    type Error = 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫;
    #[inline(always)]
    fn try_from(imm: u64) -> Result<𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟔𝟒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞, 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫> {
        if let 0x0000000000000000 = imm & 0xffffffffffffffc0 {
            Ok(𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟔𝟒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞((imm as i32) << 20))
        } else {
            Err(𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫(()))
        }
    }
}
impl 𝒔𝒉𝒊𝒇𝒕_𝑹𝑽𝟔𝟒_𝒊𝒎𝒎𝒆𝒅𝒊𝒂𝒕𝒆_𝒔𝒐𝒖𝒓𝒄𝒆 for u64 {}

impl 𝐔_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    #[inline(always)]
    pub const fn new_from_instruction(encoding: i32) -> 𝐔_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
        𝐔_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞(encoding & -0x00001000)
    }
    #[inline(always)]
    pub const unsafe fn new_unchecked(imm: i32) -> 𝐔_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
        𝐔_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞(imm)
    }
    #[inline(always)]
    pub const fn new_const(imm: i32) -> Option<𝐔_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞> {
        if let 0x00000000 = imm & 0x00000fff {
            Some(𝐔_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞(imm))
        } else {
            None
        }
    }
    #[inline(always)]
    pub fn new(imm: impl 𝑼_𝒊𝒎𝒎𝒆𝒅𝒊𝒂𝒕𝒆_𝒔𝒐𝒖𝒓𝒄𝒆) -> Option<𝐔_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞> {
        imm.try_into().ok()
    }
}
pub trait 𝑼_𝒊𝒎𝒎𝒆𝒅𝒊𝒂𝒕𝒆_𝒔𝒐𝒖𝒓𝒄𝒆: TryInto<𝐔_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞> {}

impl From<𝐔_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞> for i32 {
    #[inline(always)]
    fn from(imm: 𝐔_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞) -> i32 {
        imm.0
    }
}

impl TryFrom<i8> for 𝐔_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    type Error = 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫;
    #[inline(always)]
    fn try_from(imm: i8) -> Result<𝐔_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞, 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫> {
        if let 0x00 = imm & 0x7f {
            Ok(𝐔_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞(imm as i32 & -0x00001000))
        } else {
            Err(𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫(()))
        }
    }
}
impl 𝑼_𝒊𝒎𝒎𝒆𝒅𝒊𝒂𝒕𝒆_𝒔𝒐𝒖𝒓𝒄𝒆 for i8 {}

impl TryFrom<u8> for 𝐔_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    type Error = 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫;
    #[inline(always)]
    fn try_from(imm: u8) -> Result<𝐔_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞, 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫> {
        if let 0x00 = imm {
            Ok(𝐔_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞(0))
        } else {
            Err(𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫(()))
        }
    }
}
impl 𝑼_𝒊𝒎𝒎𝒆𝒅𝒊𝒂𝒕𝒆_𝒔𝒐𝒖𝒓𝒄𝒆 for u8 {}

impl TryFrom<i16> for 𝐔_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    type Error = 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫;
    #[inline(always)]
    fn try_from(imm: i16) -> Result<𝐔_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞, 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫> {
        if let 0x0000 = imm & 0x0fff {
            Ok(𝐔_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞(imm as i32))
        } else {
            Err(𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫(()))
        }
    }
}
impl 𝑼_𝒊𝒎𝒎𝒆𝒅𝒊𝒂𝒕𝒆_𝒔𝒐𝒖𝒓𝒄𝒆 for i16 {}

impl TryFrom<u16> for 𝐔_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    type Error = 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫;
    #[inline(always)]
    fn try_from(imm: u16) -> Result<𝐔_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞, 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫> {
        if let 0x0000 = imm & 0x0fff {
            Ok(𝐔_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞(imm as i32))
        } else {
            Err(𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫(()))
        }
    }
}
impl 𝑼_𝒊𝒎𝒎𝒆𝒅𝒊𝒂𝒕𝒆_𝒔𝒐𝒖𝒓𝒄𝒆 for u16 {}

impl TryFrom<i32> for 𝐔_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    type Error = 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫;
    #[inline(always)]
    fn try_from(imm: i32) -> Result<𝐔_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞, 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫> {
        if let 0x00000000 = imm & 0x00000fff {
            Ok(𝐔_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞(imm))
        } else {
            Err(𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫(()))
        }
    }
}
impl 𝑼_𝒊𝒎𝒎𝒆𝒅𝒊𝒂𝒕𝒆_𝒔𝒐𝒖𝒓𝒄𝒆 for i32 {}

impl TryFrom<u32> for 𝐔_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    type Error = 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫;
    #[inline(always)]
    fn try_from(imm: u32) -> Result<𝐔_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞, 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫> {
        if let 0x00000000 = imm & 0x80000fff {
            Ok(𝐔_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞(imm as i32))
        } else {
            Err(𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫(()))
        }
    }
}
impl 𝑼_𝒊𝒎𝒎𝒆𝒅𝒊𝒂𝒕𝒆_𝒔𝒐𝒖𝒓𝒄𝒆 for u32 {}

impl TryFrom<i64> for 𝐔_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    type Error = 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫;
    #[inline(always)]
    fn try_from(imm: i64) -> Result<𝐔_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞, 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫> {
        if let 0x0000000000000000 | -0x0000000080000000 = imm & -0x000000007ffff001 {
            Ok(𝐔_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞(imm as i32))
        } else {
            Err(𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫(()))
        }
    }
}
impl 𝑼_𝒊𝒎𝒎𝒆𝒅𝒊𝒂𝒕𝒆_𝒔𝒐𝒖𝒓𝒄𝒆 for i64 {}

impl TryFrom<u64> for 𝐔_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    type Error = 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫;
    #[inline(always)]
    fn try_from(imm: u64) -> Result<𝐔_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞, 𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫> {
        if let 0x0000000000000000 = imm & 0xffffffff80000fff {
            Ok(𝐔_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞(imm as i32))
        } else {
            Err(𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫(()))
        }
    }
}
impl 𝑼_𝒊𝒎𝒎𝒆𝒅𝒊𝒂𝒕𝒆_𝒔𝒐𝒖𝒓𝒄𝒆 for u64 {}

use super::super::𝐭𝐫𝐲_𝐟𝐫𝐨𝐦_𝐢𝐧𝐭_𝐞𝐫𝐫𝐨𝐫;
