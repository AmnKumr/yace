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

use yace_codegen::𝖋𝖎𝖑𝖙𝖊𝖗_𝖗𝖎𝖘𝖈𝖛_𝖒𝖆𝖗𝖐𝖊𝖗𝖘;

𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖎𝖘𝖈𝖛_𝖆𝖘𝖘𝖊𝖒𝖇𝖑𝖊𝖗_𝖙𝖗𝖆𝖎𝖙! {
    [𝔯𝔳32𝔦]
    pub trait 𝑪𝑷𝑼 {
    }

    [𝔯𝔳32𝔦 𝔢𝔞𝔟𝔦]
    pub trait 𝑪𝑷𝑼_𝔢𝔞𝔟𝔦 {
    }
}

𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖎𝖘𝖈𝖛_𝖆𝖘𝖘𝖊𝖒𝖇𝖑𝖊𝖗_𝖘𝖙𝖗𝖚𝖈𝖙! {
    [𝔯𝔳32𝔦]
    pub struct 𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫[<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ, 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>]
    as 𝑪𝑷𝑼 {
    }

    [𝔯𝔳32𝔦 𝔢𝔞𝔟𝔦]
    impl[<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ, 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>] 𝑪𝑷𝑼_𝔢𝔞𝔟𝔦
    for 𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫[<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ, 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮>];
}

𝖉𝖊𝖋𝖎𝖓𝖊_𝖗𝖎𝖘𝖈𝖛_𝖆𝖘𝖘𝖊𝖒𝖇𝖑𝖊𝖗_𝖘𝖚𝖕𝖕𝖑𝖊𝖒𝖊𝖓𝖙𝖆𝖗𝖞_𝖙𝖗𝖆𝖎𝖙𝖘! {
    [𝔯𝔳32𝔦]
    impl[<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ, 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>] for 𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫
    as 𝑪𝑷𝑼;

    [𝔯𝔳32𝔦 𝔢𝔞𝔟𝔦]
    impl[<'ᵉᵐⁱᵗᵗᵉʳ_ˡⁱᶠᵉᵗⁱᵐᵉ, 𝓮𝓶𝓲𝓽𝓽𝓮𝓻_𝓽𝔂𝓹𝓮: 𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓>] for 𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫
    as 𝑪𝑷𝑼_𝔢𝔞𝔟𝔦;
}

pub use super::𝗿𝗶𝘀𝗰_𝘃::𝗼𝗽𝗲𝗿𝗮𝗻𝗱𝘀::𝔞𝔡𝔡𝔯𝔢𝔰𝔰;

use super::𝗿𝗶𝘀𝗰_𝘃;

use super::𝒃𝒚𝒕𝒆_𝒆𝒎𝒊𝒕𝒕𝒆𝒓;

use super::𝗿𝗶𝘀𝗰_𝘃::𝗯𝗮𝘀𝗶𝗰_𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿;

use super::𝗿𝗶𝘀𝗰_𝘃::𝗼𝗽𝗲𝗿𝗮𝗻𝗱𝘀::𝐮𝐧𝐟𝐢𝐥𝐥𝐞𝐝_𝐟𝐥𝐮𝐞𝐧𝐭_𝐯𝐚𝐥𝐮𝐞;

use super::𝗿𝗶𝘀𝗰_𝘃::𝗼𝗽𝗲𝗿𝗮𝗻𝗱𝘀::𝒔𝒐𝒖𝒓𝒄𝒆_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒐𝒑𝒆𝒓𝒂𝒏𝒅;
use super::𝗿𝗶𝘀𝗰_𝘃::𝗼𝗽𝗲𝗿𝗮𝗻𝗱𝘀::𝒔𝒐𝒖𝒓𝒄𝒆_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆;
use super::𝗿𝗶𝘀𝗰_𝘃::𝗼𝗽𝗲𝗿𝗮𝗻𝗱𝘀::𝒔𝒐𝒖𝒓𝒄𝒆_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒅𝒊𝒔𝒑;

use super::𝗿𝗶𝘀𝗰_𝘃::𝗼𝗽𝗲𝗿𝗮𝗻𝗱𝘀::𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒐𝒑𝒆𝒓𝒂𝒏𝒅;
use super::𝗿𝗶𝘀𝗰_𝘃::𝗼𝗽𝗲𝗿𝗮𝗻𝗱𝘀::𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆;
use super::𝗿𝗶𝘀𝗰_𝘃::𝗼𝗽𝗲𝗿𝗮𝗻𝗱𝘀::𝒅𝒆𝒔𝒕𝒊𝒏𝒂𝒕𝒊𝒐𝒏_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒅𝒊𝒔𝒑;

use super::𝗿𝗶𝘀𝗰_𝘃::𝗼𝗽𝗲𝗿𝗮𝗻𝗱𝘀::𝒄𝒔𝒓_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒐𝒑𝒆𝒓𝒂𝒏𝒅;

use super::𝗿𝗶𝘀𝗰_𝘃::𝗼𝗽𝗲𝗿𝗮𝗻𝗱𝘀::𝒑𝒓𝒆𝒇𝒆𝒕𝒄𝒉_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆;
use super::𝗿𝗶𝘀𝗰_𝘃::𝗼𝗽𝗲𝗿𝗮𝗻𝗱𝘀::𝒑𝒓𝒆𝒇𝒆𝒕𝒄𝒉_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒅𝒊𝒔𝒑;

use super::𝗿𝗶𝘀𝗰_𝘃::𝗼𝗽𝗲𝗿𝗮𝗻𝗱𝘀::𝒔𝒉𝒊𝒇𝒕_𝑹𝑽𝟑𝟐_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒐𝒑𝒆𝒓𝒂𝒏𝒅;

use super::𝗿𝗶𝘀𝗰_𝘃::𝗼𝗽𝗲𝗿𝗮𝗻𝗱𝘀::𝒛𝒆𝒓𝒐_𝒐𝒇𝒇𝒔𝒆𝒕_𝒂𝒅𝒅𝒓𝒆𝒔𝒔_𝒂𝒔𝒔𝒆𝒎𝒃𝒍𝒆𝒓_𝒃𝒂𝒔𝒆;
