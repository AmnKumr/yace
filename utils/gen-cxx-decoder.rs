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
#![allow(uncommon_codepoints)]
#![allow(non_camel_case_types)]
#![allow(confusable_idents)]

use convert_case::Case;
use convert_case::Casing as _;

use clap::{Parser, ValueEnum};

use indoc::printdoc;

use yace::𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝗿𝘃𝟲𝟰𝗶::𝐛𝐚𝐬𝐢𝐜_𝐚𝐬𝐬𝐞𝐦𝐛𝐥𝐞𝐫 as 𝐚𝐬𝐬𝐦𝐛𝐥𝐞𝐫_𝐫𝐯𝟔𝟒𝐢;
use yace::𝗱𝗶𝘀𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝗿𝘃𝟲𝟰𝗶::𝑪𝑷𝑼 as 𝑪𝑷𝑼_𝗿𝘃𝟲𝟰𝗶;

use yace::𝗱𝗶𝘀𝗮𝘀𝘀𝗲𝗺𝗯𝗹𝗲𝗿::𝗿𝘃𝟲𝟰𝗶::𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧_𝐤𝐢𝐧𝐝 as 𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧_𝐤𝐢𝐧𝐝_𝗿𝘃𝟲𝟰𝗶;

fn main() {
    let options = 𝐨𝐩𝐭𝐢𝐨𝐧𝐬::parse();

    match options.𝗆𝗈𝖽𝖾 {
        𝐦𝐨𝐝𝐞::Header => {
            generate_header();
        }
        𝐦𝐨𝐝𝐞::Test => {
            generate_test();
        }
    }
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct 𝐨𝐩𝐭𝐢𝐨𝐧𝐬 {
    /// What mode to run the program in
    #[arg(value_enum)]
    𝗆𝗈𝖽𝖾: 𝐦𝐨𝐝𝐞,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum 𝐦𝐨𝐝𝐞 {
    /// Generate header file
    Header,
    /// Generate C++ test
    Test,
}

fn generate_header() {
    printdoc!(
        r##"
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

        #include <cstdint>
        #include <initializer_list>
        #include <optional>
        #include <variant>

        #ifndef DECODER_RISCV64_H_
        #define DECODER_RISCV64_H_

        namespace decoder_riscv {{

        "##
    );
    generate_enums();
    generate_instruction_checkers::<𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧_𝐤𝐢𝐧𝐝_𝗿𝘃𝟲𝟰𝗶>();
    generate_decoder();
    printdoc!(
        r##"
        }}  // namespace decoder_riscv

        #endif  // DECODER_RISCV64_H_
        "##
    );
}

fn generate_enums() {
    generate_enum::<<𝐚𝐬𝐬𝐦𝐛𝐥𝐞𝐫_𝐫𝐯𝟔𝟒𝐢 as 𝑪𝑷𝑼_𝗿𝘃𝟲𝟰𝗶>::𝐠𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜>("GpRegister");
    generate_enum::<<𝐚𝐬𝐬𝐦𝐛𝐥𝐞𝐫_𝐫𝐯𝟔𝟒𝐢 as 𝑪𝑷𝑼_𝗿𝘃𝟲𝟰𝗶>::𝐟𝐩_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫_𝐧𝐮𝐦𝐞𝐫𝐢𝐜>("FpRegister");
    generate_enum::<<𝐚𝐬𝐬𝐦𝐛𝐥𝐞𝐫_𝐫𝐯𝟔𝟒𝐢 as 𝑪𝑷𝑼_𝗿𝘃𝟲𝟰𝗶>::𝐫𝐨𝐮𝐧𝐝𝐢𝐧𝐠_𝐦𝐨𝐝𝐞>("RoundingMode");
    generate_enum::<<𝐚𝐬𝐬𝐦𝐛𝐥𝐞𝐫_𝐫𝐯𝟔𝟒𝐢 as 𝑪𝑷𝑼_𝗿𝘃𝟲𝟰𝗶>::𝐟𝐞𝐧𝐜𝐞>("Fence");
    generate_enum_hex::<<𝐚𝐬𝐬𝐦𝐛𝐥𝐞𝐫_𝐫𝐯𝟔𝟒𝐢 as 𝑪𝑷𝑼_𝗿𝘃𝟲𝟰𝗶>::𝐜𝐬𝐫_𝐫𝐞𝐠𝐢𝐬𝐭𝐞𝐫>("CsrRegister");
    generate_enum_hex::<𝐢𝐧𝐬𝐭𝐫𝐮𝐜𝐭𝐢𝐨𝐧_𝐤𝐢𝐧𝐝_𝗿𝘃𝟲𝟰𝗶>("Instruction");
}

fn generate_enum<𝓮𝓷𝓾𝓶_𝓽𝔂𝓹𝓮: std::fmt::Display>(cxx_enum_name: &str)
where i8: TryInto<𝓮𝓷𝓾𝓶_𝓽𝔂𝓹𝓮> {
    println!("enum class {cxx_enum_name}: std::int8_t {{");
    for int in i8::MIN..=i8::MAX {
        if let Ok(value) = TryInto::<𝓮𝓷𝓾𝓶_𝓽𝔂𝓹𝓮>::try_into(int) {
            println!("  k{} = {},", value.to_string().to_case(Case::UpperCamel), int);
        }
    }
    println!("}};\n");
}

fn generate_enum_hex<𝓮𝓷𝓾𝓶_𝓽𝔂𝓹𝓮: std::fmt::Display>(cxx_enum_name: &str)
where i16: TryInto<𝓮𝓷𝓾𝓶_𝓽𝔂𝓹𝓮> {
    println!("enum class {cxx_enum_name}: std::int16_t {{");
    for int in i16::MIN..=i16::MAX {
        if let Ok(value) = TryInto::<𝓮𝓷𝓾𝓶_𝓽𝔂𝓹𝓮>::try_into(int) {
            println!("  k{} = 0x{:03x},", value.to_string().to_case(Case::UpperCamel), int);
        }
    }
    println!("}};\n");
}

fn generate_instruction_checkers<𝓮𝓷𝓾𝓶_𝓽𝔂𝓹𝓮: std::fmt::Display>()
where i16: TryInto<𝓮𝓷𝓾𝓶_𝓽𝔂𝓹𝓮> {
    printdoc!(
        r##"
        #define InstructionChecker(CheckerName, function_name)                                         \
            template<typename Consumer, typename Types, typename=void>                                 \
            inline constexpr auto CheckerName = false;                                                 \
            template<typename Consumer, typename... Types>                                             \
            inline constexpr auto CheckerName<Consumer, std::tuple<Types...>, decltype((               \
                static_cast<void (Consumer::*)(Types...)>(&Consumer::function_name), void()))> = true
        "##
    );
    for int in i16::MIN..=i16::MAX {
        if let Ok(value) = TryInto::<𝓮𝓷𝓾𝓶_𝓽𝔂𝓹𝓮>::try_into(int) {
            let s_original = value.to_string();
            let s_adjusted = match s_original.as_ref() {
                "and" => "and_", 
                "or" => "or_", 
                "xor" => "xor_",
                ref s => s,
            };
            println!("InstructionChecker(k{}Supported, {});", s_original.to_case(Case::UpperCamel), s_adjusted);
        }
    }
    println!("#undef InstructionChecker\n");
}

fn generate_decoder() {
    printdoc!(
        r#"
        using Operand = std::variant<GpRegister, FpRegister, RoundingMode, CsrRegister, Fence>;
        using Operands = std::initializer_list<Operand>;

        // ParcelProvider:
        //   Must have one function get_parcel which provides std::optional<std::uint16_t>.
        // InsnConsumer:
        //   Must have one function called "instruction" which would accept Instruction and Operands.
        //   Must have one function called "unrecognized" which would accept unrecognized std::uint32_t.
        //   Optionally may include specialized functions which would accept values without them being
        //   transformed into Operands, e.g.:
        //       void InsnConsumer::add(GpRegister, GpRegister, GpRegister);
        //
        // Returns false if ParcelProvider haven't provided parcel.
        template <typename ParcelProvider, typename InsnConsumer>
        bool decoder(ParcelProvider* provider, InsnConsumer* insn_consumer) {{
          std::optional<std::uint16_t> parcel0 = provider->get_parcel();
          if (!parcel0.has_value()) {{
            return false;
          }}

          std::uint16_t w16 = *parcel0;

          if ((w16 & 0b111'1'00000'00000'11) == 0b100'1'00000'00000'10) {{
            GpRegister rd = static_cast<GpRegister>((w16 >> 7) & 0b11111);
            GpRegister rs2 = static_cast<GpRegister>((w16 >> 2) & 0b11111);
            if constexpr (kAddSupported<InsnConsumer, std::tuple<GpRegister, GpRegister, GpRegister>>) {{
              insn_consumer->add(rd, rd, rs2);
            }} else {{
              insn_consumer->instruction(Instruction::kAdd, {{Operand(rd), Operand(rd), Operand(rs2)}});
            }}
            return true;
          }}

          if ((w16 & 0b11) != 0b11) {{
            insn_consumer->unrecognized(w16);
            return true;
          }}

          std::optional<std::uint16_t> parcel1 = provider->get_parcel();
          if (!parcel1.has_value()) {{
            return false;
          }}
          std::uint32_t w32 = (*parcel1 << 16) | w16;

          if ((w32 & 0b1111111'00000'00000'111'00000'1111111) == 0b0000000'00000'00000'000'00000'0110011) {{
            GpRegister rd = static_cast<GpRegister>((w32 >> 7) & 0b11111);
            GpRegister rs1 = static_cast<GpRegister>((w32 >> 15) & 0b11111);
            GpRegister rs2 = static_cast<GpRegister>((w32 >> 20) & 0b11111);
            if constexpr (kAddSupported<InsnConsumer, std::tuple<GpRegister, GpRegister, GpRegister>>) {{
              insn_consumer->add(rd, rs1, rs2);
            }} else {{
              insn_consumer->instruction(Instruction::kAdd, {{Operand(rd), Operand(rs1), Operand(rs2)}});
            }}
            return true;
          }}

          insn_consumer->unrecognized(w32);

          return true;
        }}

        "#);
}

fn generate_test() {
    printdoc!(
        r##"
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
        #include "decoder.h"

        #include <cstdint>
        #include <iostream>
        #include <vector>

        class TestProvider {{
         public:
          TestProvider(std::vector<std::uint16_t>&& code): code_(code), current_(std::begin(code_)) {{}}
          std::optional<std::uint16_t> get_parcel() {{
            if (current_ == std::end(code_)) {{
              return std::optional<std::uint16_t>();
            }}
            return std::optional(*current_++);
          }}
         private:
          std::vector<std::uint16_t> code_;
          std::vector<std::uint16_t>::iterator current_;
        }};

        class TestConsumer {{
         public:
          void instruction(decoder_riscv::Instruction, std::initializer_list<decoder_riscv::Operand>) {{
            std::cout << "instruction called" << std::endl;
          }}
          void unrecognized(std::uint32_t) {{
            std::cout << "unrecognized called" << std::endl;
          }}
        }};

        class TestConsumerWithAdd {{
         public:
          void add(decoder_riscv::GpRegister, decoder_riscv::GpRegister, decoder_riscv::GpRegister) {{
            std::cout << "add called" << std::endl;
          }}

          void instruction(decoder_riscv::Instruction, std::initializer_list<decoder_riscv::Operand>) {{
            std::cout << "instruction called" << std::endl;
          }}
          void unrecognized(std::uint32_t) {{
            std::cout << "unrecognized called" << std::endl;
          }}
        }};

        int main() {{
            // Compressed add x4, x4, x2 instruction.
            TestProvider provider1(std::vector<std::uint16_t>{{0x920a}});
            TestConsumer consumer1;
            if (decoder_riscv::decoder(&provider1, &consumer1)) {{
              std::cout << "processed successfully" << std::endl;
            }}  else {{
              std::cout << "processed unsuccessfully" << std::endl;
            }}
            // Compressed add x4, x4, x2 instruction.
            TestProvider provider2(std::vector<std::uint16_t>{{0x920a}});
            TestConsumerWithAdd consumer2;
            if (decoder_riscv::decoder(&provider2, &consumer2)) {{
              std::cout << "processed successfully" << std::endl;
            }}  else {{
              std::cout << "processed unsuccessfully" << std::endl;
            }}
            // Add x4, x4, x2 instruction.
            TestProvider provider3(std::vector<std::uint16_t>{{0x0233, 0x0022}});
            TestConsumerWithAdd consumer3;
            if (decoder_riscv::decoder(&provider3, &consumer3)) {{
              std::cout << "processed successfully" << std::endl;
            }}  else {{
              std::cout << "processed unsuccessfully" << std::endl;
            }}
            // Two bytes of long instruction.
            TestProvider provider4(std::vector<std::uint16_t>{{0x0233}});
            TestConsumerWithAdd consumer4;
            if (decoder_riscv::decoder(&provider4, &consumer4)) {{
              std::cout << "processed successfully" << std::endl;
            }}  else {{
              std::cout << "processed unsuccessfully" << std::endl;
            }}
        }}
        "##);
}
