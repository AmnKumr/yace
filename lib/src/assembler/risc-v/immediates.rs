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
        𝐜𝐬𝐫_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞(encoding & 0x0000f8000)
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
        𝐉_𝐢𝐦𝐦𝐞𝐝𝐢𝐚𝐭𝐞((imm & -0x80000000) | (imm & 0x0000007ff) << 20 | (imm & 0x00000800) << 9 | (imm & 0x000ff000))
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
        if let 0x00 = imm {
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
