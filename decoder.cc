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

class TestProvider {
 public:
  TestProvider(std::vector<std::uint16_t>&& code): code_(code), current_(std::begin(code_)) {}
  std::optional<std::uint16_t> get_parcel() {
    if (current_ == std::end(code_)) {
      return std::optional<std::uint16_t>();
    }
    return std::optional(*current_++);
  }
 private:
  std::vector<std::uint16_t> code_;
  std::vector<std::uint16_t>::iterator current_;
};

class TestConsumer {
 public:
  void instruction(decoder_riscv::Instruction, std::initializer_list<decoder_riscv::Operand>) {
    std::cout << "instruction called" << std::endl;
  }
  void unrecognized(std::uint32_t) {
    std::cout << "unrecognized called" << std::endl;
  }
};

class TestConsumerWithAdd {
 public:
  void add(decoder_riscv::GpRegister, decoder_riscv::GpRegister, decoder_riscv::GpRegister) {
    std::cout << "add called" << std::endl;
  }

  void instruction(decoder_riscv::Instruction, std::initializer_list<decoder_riscv::Operand>) {
    std::cout << "instruction called" << std::endl;
  }
  void unrecognized(std::uint32_t) {
    std::cout << "unrecognized called" << std::endl;
  }
};

int main() {
    // Compressed add x4, x4, x2 instruction.
    TestProvider provider1(std::vector<std::uint16_t>{0x920a});
    TestConsumer consumer1;
    if (decoder_riscv::decoder(&provider1, &consumer1)) {
      std::cout << "processed successfully" << std::endl;
    }  else {
      std::cout << "processed unsuccessfully" << std::endl;
    }
    // Compressed add x4, x4, x2 instruction.
    TestProvider provider2(std::vector<std::uint16_t>{0x920a});
    TestConsumerWithAdd consumer2;
    if (decoder_riscv::decoder(&provider2, &consumer2)) {
      std::cout << "processed successfully" << std::endl;
    }  else {
      std::cout << "processed unsuccessfully" << std::endl;
    }
    // Add x4, x4, x2 instruction.
    TestProvider provider3(std::vector<std::uint16_t>{0x0233, 0x0022});
    TestConsumerWithAdd consumer3;
    if (decoder_riscv::decoder(&provider3, &consumer3)) {
      std::cout << "processed successfully" << std::endl;
    }  else {
      std::cout << "processed unsuccessfully" << std::endl;
    }
    // Two bytes of long instruction.
    TestProvider provider4(std::vector<std::uint16_t>{0x0233});
    TestConsumerWithAdd consumer4;
    if (decoder_riscv::decoder(&provider4, &consumer4)) {
      std::cout << "processed successfully" << std::endl;
    }  else {
      std::cout << "processed unsuccessfully" << std::endl;
    }
}
