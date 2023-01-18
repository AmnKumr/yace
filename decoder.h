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

namespace decoder_riscv {

enum class GpRegister: std::int8_t {
  kX0 = 0,
  kX1 = 1,
  kX2 = 2,
  kX3 = 3,
  kX4 = 4,
  kX5 = 5,
  kX6 = 6,
  kX7 = 7,
  kX8 = 8,
  kX9 = 9,
  kX10 = 10,
  kX11 = 11,
  kX12 = 12,
  kX13 = 13,
  kX14 = 14,
  kX15 = 15,
  kX16 = 16,
  kX17 = 17,
  kX18 = 18,
  kX19 = 19,
  kX20 = 20,
  kX21 = 21,
  kX22 = 22,
  kX23 = 23,
  kX24 = 24,
  kX25 = 25,
  kX26 = 26,
  kX27 = 27,
  kX28 = 28,
  kX29 = 29,
  kX30 = 30,
  kX31 = 31,
};

enum class FpRegister: std::int8_t {
  kF0 = 0,
  kF1 = 1,
  kF2 = 2,
  kF3 = 3,
  kF4 = 4,
  kF5 = 5,
  kF6 = 6,
  kF7 = 7,
  kF8 = 8,
  kF9 = 9,
  kF10 = 10,
  kF11 = 11,
  kF12 = 12,
  kF13 = 13,
  kF14 = 14,
  kF15 = 15,
  kF16 = 16,
  kF17 = 17,
  kF18 = 18,
  kF19 = 19,
  kF20 = 20,
  kF21 = 21,
  kF22 = 22,
  kF23 = 23,
  kF24 = 24,
  kF25 = 25,
  kF26 = 26,
  kF27 = 27,
  kF28 = 28,
  kF29 = 29,
  kF30 = 30,
  kF31 = 31,
};

enum class RoundingMode: std::int8_t {
  kRne = 0,
  kRtz = 1,
  kRdn = 2,
  kRup = 3,
  kRmm = 4,
  kDyn = 7,
};

enum class Fence: std::int8_t {
  kW = 1,
  kR = 2,
  kRw = 3,
  kO = 4,
  kOw = 5,
  kOr = 6,
  kOrw = 7,
  kI = 8,
  kIw = 9,
  kIr = 10,
  kIrw = 11,
  kIo = 12,
  kIow = 13,
  kIor = 14,
  kIorw = 15,
};

enum class CsrRegister: std::int16_t {
  kFflags = 0x001,
  kFrm = 0x002,
  kFcsr = 0x003,
  kSstatus = 0x100,
  kSie = 0x104,
  kStvec = 0x105,
  kScounteren = 0x106,
  kSenvcfg = 0x10a,
  kSscratch = 0x140,
  kSepc = 0x141,
  kScause = 0x142,
  kStval = 0x143,
  kSip = 0x144,
  kSatp = 0x180,
  kVsstatus = 0x200,
  kVsie = 0x204,
  kVstvec = 0x205,
  kVsscratch = 0x240,
  kVsepc = 0x241,
  kVscause = 0x242,
  kVstval = 0x243,
  kVsip = 0x244,
  kVsatp = 0x280,
  kMstatus = 0x300,
  kMisa = 0x301,
  kMedeleg = 0x302,
  kMideleg = 0x303,
  kMie = 0x304,
  kMtvec = 0x305,
  kMcounteren = 0x306,
  kMenvcfg = 0x30a,
  kMcountinhibit = 0x320,
  kMhpmevent3 = 0x323,
  kMhpmevent4 = 0x324,
  kMhpmevent5 = 0x325,
  kMhpmevent6 = 0x326,
  kMhpmevent7 = 0x327,
  kMhpmevent8 = 0x328,
  kMhpmevent9 = 0x329,
  kMhpmevent10 = 0x32a,
  kMhpmevent11 = 0x32b,
  kMhpmevent12 = 0x32c,
  kMhpmevent13 = 0x32d,
  kMhpmevent14 = 0x32e,
  kMhpmevent15 = 0x32f,
  kMhpmevent16 = 0x330,
  kMhpmevent17 = 0x331,
  kMhpmevent18 = 0x332,
  kMhpmevent19 = 0x333,
  kMhpmevent20 = 0x334,
  kMhpmevent21 = 0x335,
  kMhpmevent22 = 0x336,
  kMhpmevent23 = 0x337,
  kMhpmevent24 = 0x338,
  kMhpmevent25 = 0x339,
  kMhpmevent26 = 0x33a,
  kMhpmevent27 = 0x33b,
  kMhpmevent28 = 0x33c,
  kMhpmevent29 = 0x33d,
  kMhpmevent30 = 0x33e,
  kMhpmevent31 = 0x33f,
  kMscratch = 0x340,
  kMepc = 0x341,
  kMcause = 0x342,
  kMtval = 0x343,
  kMip = 0x344,
  kMtinst = 0x34a,
  kMtval2 = 0x34b,
  kPmpcfg0 = 0x3a0,
  kPmpcfg2 = 0x3a2,
  kPmpcfg4 = 0x3a4,
  kPmpcfg6 = 0x3a6,
  kPmpcfg8 = 0x3a8,
  kPmpcfg10 = 0x3aa,
  kPmpcfg12 = 0x3ac,
  kPmpcfg14 = 0x3ae,
  kPmpaddr0 = 0x3b0,
  kPmpaddr1 = 0x3b1,
  kPmpaddr2 = 0x3b2,
  kPmpaddr3 = 0x3b3,
  kPmpaddr4 = 0x3b4,
  kPmpaddr5 = 0x3b5,
  kPmpaddr6 = 0x3b6,
  kPmpaddr7 = 0x3b7,
  kPmpaddr8 = 0x3b8,
  kPmpaddr9 = 0x3b9,
  kPmpaddr10 = 0x3ba,
  kPmpaddr11 = 0x3bb,
  kPmpaddr12 = 0x3bc,
  kPmpaddr13 = 0x3bd,
  kPmpaddr14 = 0x3be,
  kPmpaddr15 = 0x3bf,
  kPmpaddr16 = 0x3c0,
  kPmpaddr17 = 0x3c1,
  kPmpaddr18 = 0x3c2,
  kPmpaddr19 = 0x3c3,
  kPmpaddr20 = 0x3c4,
  kPmpaddr21 = 0x3c5,
  kPmpaddr22 = 0x3c6,
  kPmpaddr23 = 0x3c7,
  kPmpaddr24 = 0x3c8,
  kPmpaddr25 = 0x3c9,
  kPmpaddr26 = 0x3ca,
  kPmpaddr27 = 0x3cb,
  kPmpaddr28 = 0x3cc,
  kPmpaddr29 = 0x3cd,
  kPmpaddr30 = 0x3ce,
  kPmpaddr31 = 0x3cf,
  kPmpaddr32 = 0x3d0,
  kPmpaddr33 = 0x3d1,
  kPmpaddr34 = 0x3d2,
  kPmpaddr35 = 0x3d3,
  kPmpaddr36 = 0x3d4,
  kPmpaddr37 = 0x3d5,
  kPmpaddr38 = 0x3d6,
  kPmpaddr39 = 0x3d7,
  kPmpaddr40 = 0x3d8,
  kPmpaddr41 = 0x3d9,
  kPmpaddr42 = 0x3da,
  kPmpaddr43 = 0x3db,
  kPmpaddr44 = 0x3dc,
  kPmpaddr45 = 0x3dd,
  kPmpaddr46 = 0x3de,
  kPmpaddr47 = 0x3df,
  kPmpaddr48 = 0x3e0,
  kPmpaddr49 = 0x3e1,
  kPmpaddr50 = 0x3e2,
  kPmpaddr51 = 0x3e3,
  kPmpaddr52 = 0x3e4,
  kPmpaddr53 = 0x3e5,
  kPmpaddr54 = 0x3e6,
  kPmpaddr55 = 0x3e7,
  kPmpaddr56 = 0x3e8,
  kPmpaddr57 = 0x3e9,
  kPmpaddr58 = 0x3ea,
  kPmpaddr59 = 0x3eb,
  kPmpaddr60 = 0x3ec,
  kPmpaddr61 = 0x3ed,
  kPmpaddr62 = 0x3ee,
  kPmpaddr63 = 0x3ef,
  kScontext = 0x5a8,
  kHstatus = 0x600,
  kHedeleg = 0x602,
  kHideleg = 0x603,
  kHie = 0x604,
  kHtimedelta = 0x605,
  kHcounteren = 0x606,
  kHgeie = 0x607,
  kHenvcfg = 0x60a,
  kHtval = 0x643,
  kHip = 0x644,
  kHvip = 0x645,
  kHtinst = 0x64a,
  kMseccfg = 0x747,
  kTselect = 0x7a0,
  kTdata1 = 0x7a1,
  kTdata2 = 0x7a2,
  kTdata3 = 0x7a3,
  kMcontext = 0x7a8,
  kDcsr = 0x7b0,
  kDpc = 0x7b1,
  kDscratch0 = 0x7b2,
  kDscratch1 = 0x7b3,
  kMcycle = 0xb00,
  kMinstret = 0xb02,
  kMhpmcounter3 = 0xb03,
  kMhpmcounter4 = 0xb04,
  kMhpmcounter5 = 0xb05,
  kMhpmcounter6 = 0xb06,
  kMhpmcounter7 = 0xb07,
  kMhpmcounter8 = 0xb08,
  kMhpmcounter9 = 0xb09,
  kMhpmcounter10 = 0xb0a,
  kMhpmcounter11 = 0xb0b,
  kMhpmcounter12 = 0xb0c,
  kMhpmcounter13 = 0xb0d,
  kMhpmcounter14 = 0xb0e,
  kMhpmcounter15 = 0xb0f,
  kMhpmcounter16 = 0xb10,
  kMhpmcounter17 = 0xb11,
  kMhpmcounter18 = 0xb12,
  kMhpmcounter19 = 0xb13,
  kMhpmcounter20 = 0xb14,
  kMhpmcounter21 = 0xb15,
  kMhpmcounter22 = 0xb16,
  kMhpmcounter23 = 0xb17,
  kMhpmcounter24 = 0xb18,
  kMhpmcounter25 = 0xb19,
  kMhpmcounter26 = 0xb1a,
  kMhpmcounter27 = 0xb1b,
  kMhpmcounter28 = 0xb1c,
  kMhpmcounter29 = 0xb1d,
  kMhpmcounter30 = 0xb1e,
  kMhpmcounter31 = 0xb1f,
  kCycle = 0xc00,
  kTime = 0xc01,
  kInstret = 0xc02,
  kHpmcounter3 = 0xc03,
  kHpmcounter4 = 0xc04,
  kHpmcounter5 = 0xc05,
  kHpmcounter6 = 0xc06,
  kHpmcounter7 = 0xc07,
  kHpmcounter8 = 0xc08,
  kHpmcounter9 = 0xc09,
  kHpmcounter10 = 0xc0a,
  kHpmcounter11 = 0xc0b,
  kHpmcounter12 = 0xc0c,
  kHpmcounter13 = 0xc0d,
  kHpmcounter14 = 0xc0e,
  kHpmcounter15 = 0xc0f,
  kHpmcounter16 = 0xc10,
  kHpmcounter17 = 0xc11,
  kHpmcounter18 = 0xc12,
  kHpmcounter19 = 0xc13,
  kHpmcounter20 = 0xc14,
  kHpmcounter21 = 0xc15,
  kHpmcounter22 = 0xc16,
  kHpmcounter23 = 0xc17,
  kHpmcounter24 = 0xc18,
  kHpmcounter25 = 0xc19,
  kHpmcounter26 = 0xc1a,
  kHpmcounter27 = 0xc1b,
  kHpmcounter28 = 0xc1c,
  kHpmcounter29 = 0xc1d,
  kHpmcounter30 = 0xc1e,
  kHpmcounter31 = 0xc1f,
  kHgeip = 0xe12,
  kMvendorid = 0xf11,
  kMarchid = 0xf12,
  kMimpid = 0xf13,
  kMhartid = 0xf14,
  kMconfigptr = 0xf15,
};

enum class Instruction: std::int16_t {
  kAdd = 0x000,
  kAddi = 0x004,
  kAddiw = 0x009,
  kAddw = 0x00f,
  kAmoaddD = 0x014,
  kAmoaddDAq = 0x01d,
  kAmoaddDAqrl = 0x029,
  kAmoaddDRl = 0x037,
  kAmoaddW = 0x043,
  kAmoaddWAq = 0x04c,
  kAmoaddWAqrl = 0x058,
  kAmoaddWRl = 0x066,
  kAmoandD = 0x072,
  kAmoandDAq = 0x07b,
  kAmoandDAqrl = 0x087,
  kAmoandDRl = 0x095,
  kAmoandW = 0x0a1,
  kAmoandWAq = 0x0aa,
  kAmoandWAqrl = 0x0b6,
  kAmoandWRl = 0x0c4,
  kAmomaxD = 0x0d0,
  kAmomaxDAq = 0x0d9,
  kAmomaxDAqrl = 0x0e5,
  kAmomaxDRl = 0x0f3,
  kAmomaxW = 0x0ff,
  kAmomaxWAq = 0x108,
  kAmomaxWAqrl = 0x114,
  kAmomaxWRl = 0x122,
  kAmomaxuD = 0x12e,
  kAmomaxuDAq = 0x138,
  kAmomaxuDAqrl = 0x145,
  kAmomaxuDRl = 0x154,
  kAmomaxuW = 0x161,
  kAmomaxuWAq = 0x16b,
  kAmomaxuWAqrl = 0x178,
  kAmomaxuWRl = 0x187,
  kAmominD = 0x194,
  kAmominDAq = 0x19d,
  kAmominDAqrl = 0x1a9,
  kAmominDRl = 0x1b7,
  kAmominW = 0x1c3,
  kAmominWAq = 0x1cc,
  kAmominWAqrl = 0x1d8,
  kAmominWRl = 0x1e6,
  kAmominuD = 0x1f2,
  kAmominuDAq = 0x1fc,
  kAmominuDAqrl = 0x209,
  kAmominuDRl = 0x218,
  kAmominuW = 0x225,
  kAmominuWAq = 0x22f,
  kAmominuWAqrl = 0x23c,
  kAmominuWRl = 0x24b,
  kAmoorD = 0x258,
  kAmoorDAq = 0x260,
  kAmoorDAqrl = 0x26b,
  kAmoorDRl = 0x278,
  kAmoorW = 0x283,
  kAmoorWAq = 0x28b,
  kAmoorWAqrl = 0x296,
  kAmoorWRl = 0x2a3,
  kAmoswapD = 0x2ae,
  kAmoswapDAq = 0x2b8,
  kAmoswapDAqrl = 0x2c5,
  kAmoswapDRl = 0x2d4,
  kAmoswapW = 0x2e1,
  kAmoswapWAq = 0x2eb,
  kAmoswapWAqrl = 0x2f8,
  kAmoswapWRl = 0x307,
  kAmoxorD = 0x314,
  kAmoxorDAq = 0x31d,
  kAmoxorDAqrl = 0x329,
  kAmoxorDRl = 0x337,
  kAmoxorW = 0x343,
  kAmoxorWAq = 0x34c,
  kAmoxorWAqrl = 0x358,
  kAmoxorWRl = 0x366,
  kAnd = 0x372,
  kAndi = 0x376,
  kAuipc = 0x37b,
  kBeq = 0x381,
  kBge = 0x385,
  kBgeu = 0x389,
  kBlt = 0x38e,
  kBltu = 0x392,
  kBne = 0x397,
  kCsrrc = 0x39b,
  kCsrrci = 0x3a1,
  kCsrrs = 0x3a8,
  kCsrrsi = 0x3ae,
  kCsrrw = 0x3b5,
  kCsrrwi = 0x3bb,
  kDiv = 0x3c2,
  kDivu = 0x3c6,
  kDivuw = 0x3cb,
  kDivw = 0x3d1,
  kEbreak = 0x3d6,
  kEcall = 0x3dd,
  kFaddD = 0x3e3,
  kFaddQ = 0x3ea,
  kFaddS = 0x3f1,
  kFclassS = 0x3f8,
  kFcvtDL = 0x401,
  kFcvtDLu = 0x40a,
  kFcvtDQ = 0x414,
  kFcvtDS = 0x41d,
  kFcvtDW = 0x426,
  kFcvtDWu = 0x42f,
  kFcvtLD = 0x439,
  kFcvtLQ = 0x442,
  kFcvtLS = 0x44b,
  kFcvtLuD = 0x454,
  kFcvtLuQ = 0x45e,
  kFcvtLuS = 0x468,
  kFcvtQD = 0x472,
  kFcvtQL = 0x47b,
  kFcvtQLu = 0x484,
  kFcvtQS = 0x48e,
  kFcvtQW = 0x497,
  kFcvtQWu = 0x4a0,
  kFcvtSD = 0x4aa,
  kFcvtSL = 0x4b3,
  kFcvtSLu = 0x4bc,
  kFcvtSQ = 0x4c6,
  kFcvtSW = 0x4cf,
  kFcvtSWu = 0x4d8,
  kFcvtWD = 0x4e2,
  kFcvtWQ = 0x4eb,
  kFcvtWS = 0x4f4,
  kFcvtWuD = 0x4fd,
  kFcvtWuQ = 0x507,
  kFcvtWuS = 0x511,
  kFdivD = 0x51b,
  kFdivQ = 0x522,
  kFdivS = 0x529,
  kFence = 0x530,
  kFenceI = 0x536,
  kFenceTso = 0x53e,
  kFeqD = 0x548,
  kFeqQ = 0x54e,
  kFeqS = 0x554,
  kFld = 0x55a,
  kFleD = 0x55e,
  kFleQ = 0x564,
  kFleS = 0x56a,
  kFlq = 0x570,
  kFltD = 0x574,
  kFltQ = 0x57a,
  kFltS = 0x580,
  kFlw = 0x586,
  kFmaddD = 0x58a,
  kFmaddQ = 0x592,
  kFmaddS = 0x59a,
  kFmaxD = 0x5a2,
  kFmaxQ = 0x5a9,
  kFmaxS = 0x5b0,
  kFminD = 0x5b7,
  kFminQ = 0x5be,
  kFminS = 0x5c5,
  kFmsubD = 0x5cc,
  kFmsubQ = 0x5d4,
  kFmsubS = 0x5dc,
  kFmulD = 0x5e4,
  kFmulQ = 0x5eb,
  kFmulS = 0x5f2,
  kFmvDX = 0x5f9,
  kFmvWX = 0x601,
  kFmvXD = 0x609,
  kFmvXW = 0x611,
  kFnmaddD = 0x619,
  kFnmaddQ = 0x622,
  kFnmaddS = 0x62b,
  kFnmsubD = 0x634,
  kFnmsubQ = 0x63d,
  kFnmsubS = 0x646,
  kFsd = 0x64f,
  kFsgnjD = 0x653,
  kFsgnjQ = 0x65b,
  kFsgnjS = 0x663,
  kFsgnjnD = 0x66b,
  kFsgnjnQ = 0x674,
  kFsgnjnS = 0x67d,
  kFsgnjxD = 0x686,
  kFsgnjxQ = 0x68f,
  kFsgnjxS = 0x698,
  kFsq = 0x6a1,
  kFsqrtD = 0x6a5,
  kFsqrtQ = 0x6ad,
  kFsqrtS = 0x6b5,
  kFsubD = 0x6bd,
  kFsubQ = 0x6c4,
  kFsubS = 0x6cb,
  kFsw = 0x6d2,
  kJal = 0x6d6,
  kJalr = 0x6da,
  kLb = 0x6df,
  kLbu = 0x6e2,
  kLd = 0x6e6,
  kLh = 0x6e9,
  kLhu = 0x6ec,
  kLrD = 0x6f0,
  kLrDAq = 0x6f5,
  kLrDAqrl = 0x6fd,
  kLrDRl = 0x707,
  kLrW = 0x70f,
  kLrWAq = 0x714,
  kLrWAqrl = 0x71c,
  kLrWRl = 0x726,
  kLui = 0x72e,
  kLw = 0x732,
  kLwu = 0x735,
  kMul = 0x739,
  kMulh = 0x73d,
  kMulhsu = 0x742,
  kMulhu = 0x749,
  kMulw = 0x74f,
  kNop = 0x754,
  kOr = 0x758,
  kOri = 0x75b,
  kPause = 0x75f,
  kPrefetchI = 0x765,
  kPrefetchR = 0x770,
  kPrefetchW = 0x77b,
  kRem = 0x786,
  kRemu = 0x78a,
  kRemuw = 0x78f,
  kRemw = 0x795,
  kSb = 0x79a,
  kScD = 0x79d,
  kScDAq = 0x7a2,
  kScDAqrl = 0x7aa,
  kScDRl = 0x7b4,
  kScW = 0x7bc,
  kScWAq = 0x7c1,
  kScWAqrl = 0x7c9,
  kScWRl = 0x7d3,
  kSd = 0x7db,
  kSh = 0x7de,
  kSll = 0x7e1,
  kSlli = 0x7e5,
  kSlliw = 0x7ea,
  kSllw = 0x7f0,
  kSlt = 0x7f5,
  kSlti = 0x7f9,
  kSltiu = 0x7fe,
  kSltu = 0x804,
  kSra = 0x809,
  kSrai = 0x80d,
  kSraiw = 0x812,
  kSraw = 0x818,
  kSrl = 0x81d,
  kSrli = 0x821,
  kSrliw = 0x826,
  kSrlw = 0x82c,
  kSub = 0x831,
  kSubw = 0x835,
  kSw = 0x83a,
  kUnimp = 0x83d,
  kXor = 0x843,
  kXori = 0x847,
};

#define InstructionChecker(CheckerName, function_name)                                         \
    template<typename Consumer, typename Types, typename=void>                                 \
    inline constexpr auto CheckerName = false;                                                 \
    template<typename Consumer, typename... Types>                                             \
    inline constexpr auto CheckerName<Consumer, std::tuple<Types...>, decltype((               \
        static_cast<void (Consumer::*)(Types...)>(&Consumer::function_name), void()))> = true
InstructionChecker(kAddSupported, add);
InstructionChecker(kAddiSupported, addi);
InstructionChecker(kAddiwSupported, addiw);
InstructionChecker(kAddwSupported, addw);
InstructionChecker(kAmoaddDSupported, amoadd_d);
InstructionChecker(kAmoaddDAqSupported, amoadd_d_aq);
InstructionChecker(kAmoaddDAqrlSupported, amoadd_d_aqrl);
InstructionChecker(kAmoaddDRlSupported, amoadd_d_rl);
InstructionChecker(kAmoaddWSupported, amoadd_w);
InstructionChecker(kAmoaddWAqSupported, amoadd_w_aq);
InstructionChecker(kAmoaddWAqrlSupported, amoadd_w_aqrl);
InstructionChecker(kAmoaddWRlSupported, amoadd_w_rl);
InstructionChecker(kAmoandDSupported, amoand_d);
InstructionChecker(kAmoandDAqSupported, amoand_d_aq);
InstructionChecker(kAmoandDAqrlSupported, amoand_d_aqrl);
InstructionChecker(kAmoandDRlSupported, amoand_d_rl);
InstructionChecker(kAmoandWSupported, amoand_w);
InstructionChecker(kAmoandWAqSupported, amoand_w_aq);
InstructionChecker(kAmoandWAqrlSupported, amoand_w_aqrl);
InstructionChecker(kAmoandWRlSupported, amoand_w_rl);
InstructionChecker(kAmomaxDSupported, amomax_d);
InstructionChecker(kAmomaxDAqSupported, amomax_d_aq);
InstructionChecker(kAmomaxDAqrlSupported, amomax_d_aqrl);
InstructionChecker(kAmomaxDRlSupported, amomax_d_rl);
InstructionChecker(kAmomaxWSupported, amomax_w);
InstructionChecker(kAmomaxWAqSupported, amomax_w_aq);
InstructionChecker(kAmomaxWAqrlSupported, amomax_w_aqrl);
InstructionChecker(kAmomaxWRlSupported, amomax_w_rl);
InstructionChecker(kAmomaxuDSupported, amomaxu_d);
InstructionChecker(kAmomaxuDAqSupported, amomaxu_d_aq);
InstructionChecker(kAmomaxuDAqrlSupported, amomaxu_d_aqrl);
InstructionChecker(kAmomaxuDRlSupported, amomaxu_d_rl);
InstructionChecker(kAmomaxuWSupported, amomaxu_w);
InstructionChecker(kAmomaxuWAqSupported, amomaxu_w_aq);
InstructionChecker(kAmomaxuWAqrlSupported, amomaxu_w_aqrl);
InstructionChecker(kAmomaxuWRlSupported, amomaxu_w_rl);
InstructionChecker(kAmominDSupported, amomin_d);
InstructionChecker(kAmominDAqSupported, amomin_d_aq);
InstructionChecker(kAmominDAqrlSupported, amomin_d_aqrl);
InstructionChecker(kAmominDRlSupported, amomin_d_rl);
InstructionChecker(kAmominWSupported, amomin_w);
InstructionChecker(kAmominWAqSupported, amomin_w_aq);
InstructionChecker(kAmominWAqrlSupported, amomin_w_aqrl);
InstructionChecker(kAmominWRlSupported, amomin_w_rl);
InstructionChecker(kAmominuDSupported, amominu_d);
InstructionChecker(kAmominuDAqSupported, amominu_d_aq);
InstructionChecker(kAmominuDAqrlSupported, amominu_d_aqrl);
InstructionChecker(kAmominuDRlSupported, amominu_d_rl);
InstructionChecker(kAmominuWSupported, amominu_w);
InstructionChecker(kAmominuWAqSupported, amominu_w_aq);
InstructionChecker(kAmominuWAqrlSupported, amominu_w_aqrl);
InstructionChecker(kAmominuWRlSupported, amominu_w_rl);
InstructionChecker(kAmoorDSupported, amoor_d);
InstructionChecker(kAmoorDAqSupported, amoor_d_aq);
InstructionChecker(kAmoorDAqrlSupported, amoor_d_aqrl);
InstructionChecker(kAmoorDRlSupported, amoor_d_rl);
InstructionChecker(kAmoorWSupported, amoor_w);
InstructionChecker(kAmoorWAqSupported, amoor_w_aq);
InstructionChecker(kAmoorWAqrlSupported, amoor_w_aqrl);
InstructionChecker(kAmoorWRlSupported, amoor_w_rl);
InstructionChecker(kAmoswapDSupported, amoswap_d);
InstructionChecker(kAmoswapDAqSupported, amoswap_d_aq);
InstructionChecker(kAmoswapDAqrlSupported, amoswap_d_aqrl);
InstructionChecker(kAmoswapDRlSupported, amoswap_d_rl);
InstructionChecker(kAmoswapWSupported, amoswap_w);
InstructionChecker(kAmoswapWAqSupported, amoswap_w_aq);
InstructionChecker(kAmoswapWAqrlSupported, amoswap_w_aqrl);
InstructionChecker(kAmoswapWRlSupported, amoswap_w_rl);
InstructionChecker(kAmoxorDSupported, amoxor_d);
InstructionChecker(kAmoxorDAqSupported, amoxor_d_aq);
InstructionChecker(kAmoxorDAqrlSupported, amoxor_d_aqrl);
InstructionChecker(kAmoxorDRlSupported, amoxor_d_rl);
InstructionChecker(kAmoxorWSupported, amoxor_w);
InstructionChecker(kAmoxorWAqSupported, amoxor_w_aq);
InstructionChecker(kAmoxorWAqrlSupported, amoxor_w_aqrl);
InstructionChecker(kAmoxorWRlSupported, amoxor_w_rl);
InstructionChecker(kAndSupported, and_);
InstructionChecker(kAndiSupported, andi);
InstructionChecker(kAuipcSupported, auipc);
InstructionChecker(kBeqSupported, beq);
InstructionChecker(kBgeSupported, bge);
InstructionChecker(kBgeuSupported, bgeu);
InstructionChecker(kBltSupported, blt);
InstructionChecker(kBltuSupported, bltu);
InstructionChecker(kBneSupported, bne);
InstructionChecker(kCsrrcSupported, csrrc);
InstructionChecker(kCsrrciSupported, csrrci);
InstructionChecker(kCsrrsSupported, csrrs);
InstructionChecker(kCsrrsiSupported, csrrsi);
InstructionChecker(kCsrrwSupported, csrrw);
InstructionChecker(kCsrrwiSupported, csrrwi);
InstructionChecker(kDivSupported, div);
InstructionChecker(kDivuSupported, divu);
InstructionChecker(kDivuwSupported, divuw);
InstructionChecker(kDivwSupported, divw);
InstructionChecker(kEbreakSupported, ebreak);
InstructionChecker(kEcallSupported, ecall);
InstructionChecker(kFaddDSupported, fadd_d);
InstructionChecker(kFaddQSupported, fadd_q);
InstructionChecker(kFaddSSupported, fadd_s);
InstructionChecker(kFclassSSupported, fclass_s);
InstructionChecker(kFcvtDLSupported, fcvt_d_l);
InstructionChecker(kFcvtDLuSupported, fcvt_d_lu);
InstructionChecker(kFcvtDQSupported, fcvt_d_q);
InstructionChecker(kFcvtDSSupported, fcvt_d_s);
InstructionChecker(kFcvtDWSupported, fcvt_d_w);
InstructionChecker(kFcvtDWuSupported, fcvt_d_wu);
InstructionChecker(kFcvtLDSupported, fcvt_l_d);
InstructionChecker(kFcvtLQSupported, fcvt_l_q);
InstructionChecker(kFcvtLSSupported, fcvt_l_s);
InstructionChecker(kFcvtLuDSupported, fcvt_lu_d);
InstructionChecker(kFcvtLuQSupported, fcvt_lu_q);
InstructionChecker(kFcvtLuSSupported, fcvt_lu_s);
InstructionChecker(kFcvtQDSupported, fcvt_q_d);
InstructionChecker(kFcvtQLSupported, fcvt_q_l);
InstructionChecker(kFcvtQLuSupported, fcvt_q_lu);
InstructionChecker(kFcvtQSSupported, fcvt_q_s);
InstructionChecker(kFcvtQWSupported, fcvt_q_w);
InstructionChecker(kFcvtQWuSupported, fcvt_q_wu);
InstructionChecker(kFcvtSDSupported, fcvt_s_d);
InstructionChecker(kFcvtSLSupported, fcvt_s_l);
InstructionChecker(kFcvtSLuSupported, fcvt_s_lu);
InstructionChecker(kFcvtSQSupported, fcvt_s_q);
InstructionChecker(kFcvtSWSupported, fcvt_s_w);
InstructionChecker(kFcvtSWuSupported, fcvt_s_wu);
InstructionChecker(kFcvtWDSupported, fcvt_w_d);
InstructionChecker(kFcvtWQSupported, fcvt_w_q);
InstructionChecker(kFcvtWSSupported, fcvt_w_s);
InstructionChecker(kFcvtWuDSupported, fcvt_wu_d);
InstructionChecker(kFcvtWuQSupported, fcvt_wu_q);
InstructionChecker(kFcvtWuSSupported, fcvt_wu_s);
InstructionChecker(kFdivDSupported, fdiv_d);
InstructionChecker(kFdivQSupported, fdiv_q);
InstructionChecker(kFdivSSupported, fdiv_s);
InstructionChecker(kFenceSupported, fence);
InstructionChecker(kFenceISupported, fence_i);
InstructionChecker(kFenceTsoSupported, fence_tso);
InstructionChecker(kFeqDSupported, feq_d);
InstructionChecker(kFeqQSupported, feq_q);
InstructionChecker(kFeqSSupported, feq_s);
InstructionChecker(kFldSupported, fld);
InstructionChecker(kFleDSupported, fle_d);
InstructionChecker(kFleQSupported, fle_q);
InstructionChecker(kFleSSupported, fle_s);
InstructionChecker(kFlqSupported, flq);
InstructionChecker(kFltDSupported, flt_d);
InstructionChecker(kFltQSupported, flt_q);
InstructionChecker(kFltSSupported, flt_s);
InstructionChecker(kFlwSupported, flw);
InstructionChecker(kFmaddDSupported, fmadd_d);
InstructionChecker(kFmaddQSupported, fmadd_q);
InstructionChecker(kFmaddSSupported, fmadd_s);
InstructionChecker(kFmaxDSupported, fmax_d);
InstructionChecker(kFmaxQSupported, fmax_q);
InstructionChecker(kFmaxSSupported, fmax_s);
InstructionChecker(kFminDSupported, fmin_d);
InstructionChecker(kFminQSupported, fmin_q);
InstructionChecker(kFminSSupported, fmin_s);
InstructionChecker(kFmsubDSupported, fmsub_d);
InstructionChecker(kFmsubQSupported, fmsub_q);
InstructionChecker(kFmsubSSupported, fmsub_s);
InstructionChecker(kFmulDSupported, fmul_d);
InstructionChecker(kFmulQSupported, fmul_q);
InstructionChecker(kFmulSSupported, fmul_s);
InstructionChecker(kFmvDXSupported, fmv_d_x);
InstructionChecker(kFmvWXSupported, fmv_w_x);
InstructionChecker(kFmvXDSupported, fmv_x_d);
InstructionChecker(kFmvXWSupported, fmv_x_w);
InstructionChecker(kFnmaddDSupported, fnmadd_d);
InstructionChecker(kFnmaddQSupported, fnmadd_q);
InstructionChecker(kFnmaddSSupported, fnmadd_s);
InstructionChecker(kFnmsubDSupported, fnmsub_d);
InstructionChecker(kFnmsubQSupported, fnmsub_q);
InstructionChecker(kFnmsubSSupported, fnmsub_s);
InstructionChecker(kFsdSupported, fsd);
InstructionChecker(kFsgnjDSupported, fsgnj_d);
InstructionChecker(kFsgnjQSupported, fsgnj_q);
InstructionChecker(kFsgnjSSupported, fsgnj_s);
InstructionChecker(kFsgnjnDSupported, fsgnjn_d);
InstructionChecker(kFsgnjnQSupported, fsgnjn_q);
InstructionChecker(kFsgnjnSSupported, fsgnjn_s);
InstructionChecker(kFsgnjxDSupported, fsgnjx_d);
InstructionChecker(kFsgnjxQSupported, fsgnjx_q);
InstructionChecker(kFsgnjxSSupported, fsgnjx_s);
InstructionChecker(kFsqSupported, fsq);
InstructionChecker(kFsqrtDSupported, fsqrt_d);
InstructionChecker(kFsqrtQSupported, fsqrt_q);
InstructionChecker(kFsqrtSSupported, fsqrt_s);
InstructionChecker(kFsubDSupported, fsub_d);
InstructionChecker(kFsubQSupported, fsub_q);
InstructionChecker(kFsubSSupported, fsub_s);
InstructionChecker(kFswSupported, fsw);
InstructionChecker(kJalSupported, jal);
InstructionChecker(kJalrSupported, jalr);
InstructionChecker(kLbSupported, lb);
InstructionChecker(kLbuSupported, lbu);
InstructionChecker(kLdSupported, ld);
InstructionChecker(kLhSupported, lh);
InstructionChecker(kLhuSupported, lhu);
InstructionChecker(kLrDSupported, lr_d);
InstructionChecker(kLrDAqSupported, lr_d_aq);
InstructionChecker(kLrDAqrlSupported, lr_d_aqrl);
InstructionChecker(kLrDRlSupported, lr_d_rl);
InstructionChecker(kLrWSupported, lr_w);
InstructionChecker(kLrWAqSupported, lr_w_aq);
InstructionChecker(kLrWAqrlSupported, lr_w_aqrl);
InstructionChecker(kLrWRlSupported, lr_w_rl);
InstructionChecker(kLuiSupported, lui);
InstructionChecker(kLwSupported, lw);
InstructionChecker(kLwuSupported, lwu);
InstructionChecker(kMulSupported, mul);
InstructionChecker(kMulhSupported, mulh);
InstructionChecker(kMulhsuSupported, mulhsu);
InstructionChecker(kMulhuSupported, mulhu);
InstructionChecker(kMulwSupported, mulw);
InstructionChecker(kNopSupported, nop);
InstructionChecker(kOrSupported, or_);
InstructionChecker(kOriSupported, ori);
InstructionChecker(kPauseSupported, pause);
InstructionChecker(kPrefetchISupported, prefetch_i);
InstructionChecker(kPrefetchRSupported, prefetch_r);
InstructionChecker(kPrefetchWSupported, prefetch_w);
InstructionChecker(kRemSupported, rem);
InstructionChecker(kRemuSupported, remu);
InstructionChecker(kRemuwSupported, remuw);
InstructionChecker(kRemwSupported, remw);
InstructionChecker(kSbSupported, sb);
InstructionChecker(kScDSupported, sc_d);
InstructionChecker(kScDAqSupported, sc_d_aq);
InstructionChecker(kScDAqrlSupported, sc_d_aqrl);
InstructionChecker(kScDRlSupported, sc_d_rl);
InstructionChecker(kScWSupported, sc_w);
InstructionChecker(kScWAqSupported, sc_w_aq);
InstructionChecker(kScWAqrlSupported, sc_w_aqrl);
InstructionChecker(kScWRlSupported, sc_w_rl);
InstructionChecker(kSdSupported, sd);
InstructionChecker(kShSupported, sh);
InstructionChecker(kSllSupported, sll);
InstructionChecker(kSlliSupported, slli);
InstructionChecker(kSlliwSupported, slliw);
InstructionChecker(kSllwSupported, sllw);
InstructionChecker(kSltSupported, slt);
InstructionChecker(kSltiSupported, slti);
InstructionChecker(kSltiuSupported, sltiu);
InstructionChecker(kSltuSupported, sltu);
InstructionChecker(kSraSupported, sra);
InstructionChecker(kSraiSupported, srai);
InstructionChecker(kSraiwSupported, sraiw);
InstructionChecker(kSrawSupported, sraw);
InstructionChecker(kSrlSupported, srl);
InstructionChecker(kSrliSupported, srli);
InstructionChecker(kSrliwSupported, srliw);
InstructionChecker(kSrlwSupported, srlw);
InstructionChecker(kSubSupported, sub);
InstructionChecker(kSubwSupported, subw);
InstructionChecker(kSwSupported, sw);
InstructionChecker(kUnimpSupported, unimp);
InstructionChecker(kXorSupported, xor_);
InstructionChecker(kXoriSupported, xori);
#undef InstructionChecker

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
bool decoder(ParcelProvider* provider, InsnConsumer* insn_consumer) {
  std::optional<std::uint16_t> parcel0 = provider->get_parcel();
  if (!parcel0.has_value()) {
    return false;
  }

  std::uint16_t w16 = *parcel0;

  if ((w16 & 0b111'1'00000'00000'11) == 0b100'1'00000'00000'10) {
    GpRegister rd = static_cast<GpRegister>((w16 >> 7) & 0b11111);
    GpRegister rs2 = static_cast<GpRegister>((w16 >> 2) & 0b11111);
    if constexpr (kAddSupported<InsnConsumer, std::tuple<GpRegister, GpRegister, GpRegister>>) {
      insn_consumer->add(rd, rd, rs2);
    } else {
      insn_consumer->instruction(Instruction::kAdd, {Operand(rd), Operand(rd), Operand(rs2)});
    }
    return true;
  }

  if ((w16 & 0b11) != 0b11) {
    insn_consumer->unrecognized(w16);
    return true;
  }

  std::optional<std::uint16_t> parcel1 = provider->get_parcel();
  if (!parcel1.has_value()) {
    return false;
  }
  std::uint32_t w32 = (*parcel1 << 16) | w16;

  if ((w32 & 0b1111111'00000'00000'111'00000'1111111) == 0b0000000'00000'00000'000'00000'0110011) {
    GpRegister rd = static_cast<GpRegister>((w32 >> 7) & 0b11111);
    GpRegister rs1 = static_cast<GpRegister>((w32 >> 15) & 0b11111);
    GpRegister rs2 = static_cast<GpRegister>((w32 >> 20) & 0b11111);
    if constexpr (kAddSupported<InsnConsumer, std::tuple<GpRegister, GpRegister, GpRegister>>) {
      insn_consumer->add(rd, rs1, rs2);
    } else {
      insn_consumer->instruction(Instruction::kAdd, {Operand(rd), Operand(rs1), Operand(rs2)});
    }
    return true;
  }

  insn_consumer->unrecognized(w32);

  return true;
}

}  // namespace decoder_riscv

#endif  // DECODER_RISCV64_H_
