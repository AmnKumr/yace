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

pub trait 𝒔𝒉𝒊𝒇𝒕_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒐𝒑𝒆𝒓𝒂𝒏𝒅<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮: ?Sized>: Into<Self::𝐭𝐚𝐫𝐠𝐞𝐭> {
    type 𝐭𝐚𝐫𝐠𝐞𝐭;
}

impl<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮: ?Sized> 𝒔𝒉𝒊𝒇𝒕_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒐𝒑𝒆𝒓𝒂𝒏𝒅<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮> for 𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟑𝟐_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
    type 𝐭𝐚𝐫𝐠𝐞𝐭 = 𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟑𝟐_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞;
}

impl<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮: ?Sized> 𝒔𝒉𝒊𝒇𝒕_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒐𝒑𝒆𝒓𝒂𝒏𝒅<𝓪𝓼𝓼𝓮𝓶𝓫𝓵𝓮𝓻_𝓽𝔂𝓹𝓮> for 𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟔𝟒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞 {
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
#[derive(Clone, Copy, Default, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞 {}

#[cfg(feature = "std")]
impl std::fmt::Display for 𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞 {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct 𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮> {
    pub 𝖻𝖺𝗌𝖾: 𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮,
    pub 𝖽𝗂𝗌𝗉: 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮,
}

#[cfg(feature = "std")]
impl<𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮: std::fmt::Display, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮: std::fmt::Display> std::fmt::Display for 𝒂𝒅𝒅𝒓𝒆𝒔𝒔<𝓫𝓪𝓼𝓮_𝓽𝔂𝓹𝓮, 𝓭𝓲𝓼𝓹_𝓽𝔂𝓹𝓮> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}({})", self.𝖽𝗂𝗌𝗉, self.𝖻𝖺𝗌𝖾)
    }
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
    𝐜𝐬𝐫_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞,
    𝐈_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞,
    𝐉_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞,
    𝐏_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞,
    𝐒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞,
    𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟑𝟐_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞,
    𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟔𝟒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞,
    𝐔_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞
);

use super::𝗶𝗺𝗺𝗲𝗱𝗶𝗮𝘁𝗲𝘀::𝐁_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞;
use super::𝗶𝗺𝗺𝗲𝗱𝗶𝗮𝘁𝗲𝘀::𝐜𝐬𝐫_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞;
use super::𝗶𝗺𝗺𝗲𝗱𝗶𝗮𝘁𝗲𝘀::𝐈_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞;
use super::𝗶𝗺𝗺𝗲𝗱𝗶𝗮𝘁𝗲𝘀::𝐉_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞;
use super::𝗶𝗺𝗺𝗲𝗱𝗶𝗮𝘁𝗲𝘀::𝐏_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞;
use super::𝗶𝗺𝗺𝗲𝗱𝗶𝗮𝘁𝗲𝘀::𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟑𝟐_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞;
use super::𝗶𝗺𝗺𝗲𝗱𝗶𝗮𝘁𝗲𝘀::𝐬𝐡𝐢𝐟𝐭_𝐑𝐕𝟔𝟒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞;
use super::𝗶𝗺𝗺𝗲𝗱𝗶𝗮𝘁𝗲𝘀::𝐒_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞;
use super::𝗶𝗺𝗺𝗲𝗱𝗶𝗮𝘁𝗲𝘀::𝐔_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞;
