use crate::CHUNK_LEN;
use std::arch::{asm, global_asm};

global_asm!(
    // --------------------------------------------------------------------------------------------
    // blake3_avx512_kernel_16
    //
    // zmm0-zmm15: state vectors
    // zmm16-zmm31: transposed message vectors
    //
    // This routine executes all 7 rounds of compression and performs the XOR of the upper half of
    // the state into the lower half (but not the feed-forward). The result is left in zmm0-zmm7.
    // --------------------------------------------------------------------------------------------
    "blake3_avx512_kernel_16:",
    // round 1
    "vpaddd  zmm0, zmm0, zmm16",
    "vpaddd  zmm1, zmm1, zmm18",
    "vpaddd  zmm2, zmm2, zmm20",
    "vpaddd  zmm3, zmm3, zmm22",
    "vpaddd  zmm0, zmm0, zmm4",
    "vpaddd  zmm1, zmm1, zmm5",
    "vpaddd  zmm2, zmm2, zmm6",
    "vpaddd  zmm3, zmm3, zmm7",
    "vpxord  zmm12, zmm12, zmm0",
    "vpxord  zmm13, zmm13, zmm1",
    "vpxord  zmm14, zmm14, zmm2",
    "vpxord  zmm15, zmm15, zmm3",
    "vprord  zmm12, zmm12, 16",
    "vprord  zmm13, zmm13, 16",
    "vprord  zmm14, zmm14, 16",
    "vprord  zmm15, zmm15, 16",
    "vpaddd  zmm8, zmm8, zmm12",
    "vpaddd  zmm9, zmm9, zmm13",
    "vpaddd  zmm10, zmm10, zmm14",
    "vpaddd  zmm11, zmm11, zmm15",
    "vpxord  zmm4, zmm4, zmm8",
    "vpxord  zmm5, zmm5, zmm9",
    "vpxord  zmm6, zmm6, zmm10",
    "vpxord  zmm7, zmm7, zmm11",
    "vprord  zmm4, zmm4, 12",
    "vprord  zmm5, zmm5, 12",
    "vprord  zmm6, zmm6, 12",
    "vprord  zmm7, zmm7, 12",
    "vpaddd  zmm0, zmm0, zmm17",
    "vpaddd  zmm1, zmm1, zmm19",
    "vpaddd  zmm2, zmm2, zmm21",
    "vpaddd  zmm3, zmm3, zmm23",
    "vpaddd  zmm0, zmm0, zmm4",
    "vpaddd  zmm1, zmm1, zmm5",
    "vpaddd  zmm2, zmm2, zmm6",
    "vpaddd  zmm3, zmm3, zmm7",
    "vpxord  zmm12, zmm12, zmm0",
    "vpxord  zmm13, zmm13, zmm1",
    "vpxord  zmm14, zmm14, zmm2",
    "vpxord  zmm15, zmm15, zmm3",
    "vprord  zmm12, zmm12, 8",
    "vprord  zmm13, zmm13, 8",
    "vprord  zmm14, zmm14, 8",
    "vprord  zmm15, zmm15, 8",
    "vpaddd  zmm8, zmm8, zmm12",
    "vpaddd  zmm9, zmm9, zmm13",
    "vpaddd  zmm10, zmm10, zmm14",
    "vpaddd  zmm11, zmm11, zmm15",
    "vpxord  zmm4, zmm4, zmm8",
    "vpxord  zmm5, zmm5, zmm9",
    "vpxord  zmm6, zmm6, zmm10",
    "vpxord  zmm7, zmm7, zmm11",
    "vprord  zmm4, zmm4, 7",
    "vprord  zmm5, zmm5, 7",
    "vprord  zmm6, zmm6, 7",
    "vprord  zmm7, zmm7, 7",
    "vpaddd  zmm0, zmm0, zmm24",
    "vpaddd  zmm1, zmm1, zmm26",
    "vpaddd  zmm2, zmm2, zmm28",
    "vpaddd  zmm3, zmm3, zmm30",
    "vpaddd  zmm0, zmm0, zmm5",
    "vpaddd  zmm1, zmm1, zmm6",
    "vpaddd  zmm2, zmm2, zmm7",
    "vpaddd  zmm3, zmm3, zmm4",
    "vpxord  zmm15, zmm15, zmm0",
    "vpxord  zmm12, zmm12, zmm1",
    "vpxord  zmm13, zmm13, zmm2",
    "vpxord  zmm14, zmm14, zmm3",
    "vprord  zmm15, zmm15, 16",
    "vprord  zmm12, zmm12, 16",
    "vprord  zmm13, zmm13, 16",
    "vprord  zmm14, zmm14, 16",
    "vpaddd  zmm10, zmm10, zmm15",
    "vpaddd  zmm11, zmm11, zmm12",
    "vpaddd  zmm8, zmm8, zmm13",
    "vpaddd  zmm9, zmm9, zmm14",
    "vpxord  zmm5, zmm5, zmm10",
    "vpxord  zmm6, zmm6, zmm11",
    "vpxord  zmm7, zmm7, zmm8",
    "vpxord  zmm4, zmm4, zmm9",
    "vprord  zmm5, zmm5, 12",
    "vprord  zmm6, zmm6, 12",
    "vprord  zmm7, zmm7, 12",
    "vprord  zmm4, zmm4, 12",
    "vpaddd  zmm0, zmm0, zmm25",
    "vpaddd  zmm1, zmm1, zmm27",
    "vpaddd  zmm2, zmm2, zmm29",
    "vpaddd  zmm3, zmm3, zmm31",
    "vpaddd  zmm0, zmm0, zmm5",
    "vpaddd  zmm1, zmm1, zmm6",
    "vpaddd  zmm2, zmm2, zmm7",
    "vpaddd  zmm3, zmm3, zmm4",
    "vpxord  zmm15, zmm15, zmm0",
    "vpxord  zmm12, zmm12, zmm1",
    "vpxord  zmm13, zmm13, zmm2",
    "vpxord  zmm14, zmm14, zmm3",
    "vprord  zmm15, zmm15, 8",
    "vprord  zmm12, zmm12, 8",
    "vprord  zmm13, zmm13, 8",
    "vprord  zmm14, zmm14, 8",
    "vpaddd  zmm10, zmm10, zmm15",
    "vpaddd  zmm11, zmm11, zmm12",
    "vpaddd  zmm8, zmm8, zmm13",
    "vpaddd  zmm9, zmm9, zmm14",
    "vpxord  zmm5, zmm5, zmm10",
    "vpxord  zmm6, zmm6, zmm11",
    "vpxord  zmm7, zmm7, zmm8",
    "vpxord  zmm4, zmm4, zmm9",
    "vprord  zmm5, zmm5, 7",
    "vprord  zmm6, zmm6, 7",
    "vprord  zmm7, zmm7, 7",
    "vprord  zmm4, zmm4, 7",
    // round 2
    "vpaddd  zmm0, zmm0, zmm18",
    "vpaddd  zmm1, zmm1, zmm19",
    "vpaddd  zmm2, zmm2, zmm23",
    "vpaddd  zmm3, zmm3, zmm20",
    "vpaddd  zmm0, zmm0, zmm4",
    "vpaddd  zmm1, zmm1, zmm5",
    "vpaddd  zmm2, zmm2, zmm6",
    "vpaddd  zmm3, zmm3, zmm7",
    "vpxord  zmm12, zmm12, zmm0",
    "vpxord  zmm13, zmm13, zmm1",
    "vpxord  zmm14, zmm14, zmm2",
    "vpxord  zmm15, zmm15, zmm3",
    "vprord  zmm12, zmm12, 16",
    "vprord  zmm13, zmm13, 16",
    "vprord  zmm14, zmm14, 16",
    "vprord  zmm15, zmm15, 16",
    "vpaddd  zmm8, zmm8, zmm12",
    "vpaddd  zmm9, zmm9, zmm13",
    "vpaddd  zmm10, zmm10, zmm14",
    "vpaddd  zmm11, zmm11, zmm15",
    "vpxord  zmm4, zmm4, zmm8",
    "vpxord  zmm5, zmm5, zmm9",
    "vpxord  zmm6, zmm6, zmm10",
    "vpxord  zmm7, zmm7, zmm11",
    "vprord  zmm4, zmm4, 12",
    "vprord  zmm5, zmm5, 12",
    "vprord  zmm6, zmm6, 12",
    "vprord  zmm7, zmm7, 12",
    "vpaddd  zmm0, zmm0, zmm22",
    "vpaddd  zmm1, zmm1, zmm26",
    "vpaddd  zmm2, zmm2, zmm16",
    "vpaddd  zmm3, zmm3, zmm29",
    "vpaddd  zmm0, zmm0, zmm4",
    "vpaddd  zmm1, zmm1, zmm5",
    "vpaddd  zmm2, zmm2, zmm6",
    "vpaddd  zmm3, zmm3, zmm7",
    "vpxord  zmm12, zmm12, zmm0",
    "vpxord  zmm13, zmm13, zmm1",
    "vpxord  zmm14, zmm14, zmm2",
    "vpxord  zmm15, zmm15, zmm3",
    "vprord  zmm12, zmm12, 8",
    "vprord  zmm13, zmm13, 8",
    "vprord  zmm14, zmm14, 8",
    "vprord  zmm15, zmm15, 8",
    "vpaddd  zmm8, zmm8, zmm12",
    "vpaddd  zmm9, zmm9, zmm13",
    "vpaddd  zmm10, zmm10, zmm14",
    "vpaddd  zmm11, zmm11, zmm15",
    "vpxord  zmm4, zmm4, zmm8",
    "vpxord  zmm5, zmm5, zmm9",
    "vpxord  zmm6, zmm6, zmm10",
    "vpxord  zmm7, zmm7, zmm11",
    "vprord  zmm4, zmm4, 7",
    "vprord  zmm5, zmm5, 7",
    "vprord  zmm6, zmm6, 7",
    "vprord  zmm7, zmm7, 7",
    "vpaddd  zmm0, zmm0, zmm17",
    "vpaddd  zmm1, zmm1, zmm28",
    "vpaddd  zmm2, zmm2, zmm25",
    "vpaddd  zmm3, zmm3, zmm31",
    "vpaddd  zmm0, zmm0, zmm5",
    "vpaddd  zmm1, zmm1, zmm6",
    "vpaddd  zmm2, zmm2, zmm7",
    "vpaddd  zmm3, zmm3, zmm4",
    "vpxord  zmm15, zmm15, zmm0",
    "vpxord  zmm12, zmm12, zmm1",
    "vpxord  zmm13, zmm13, zmm2",
    "vpxord  zmm14, zmm14, zmm3",
    "vprord  zmm15, zmm15, 16",
    "vprord  zmm12, zmm12, 16",
    "vprord  zmm13, zmm13, 16",
    "vprord  zmm14, zmm14, 16",
    "vpaddd  zmm10, zmm10, zmm15",
    "vpaddd  zmm11, zmm11, zmm12",
    "vpaddd  zmm8, zmm8, zmm13",
    "vpaddd  zmm9, zmm9, zmm14",
    "vpxord  zmm5, zmm5, zmm10",
    "vpxord  zmm6, zmm6, zmm11",
    "vpxord  zmm7, zmm7, zmm8",
    "vpxord  zmm4, zmm4, zmm9",
    "vprord  zmm5, zmm5, 12",
    "vprord  zmm6, zmm6, 12",
    "vprord  zmm7, zmm7, 12",
    "vprord  zmm4, zmm4, 12",
    "vpaddd  zmm0, zmm0, zmm27",
    "vpaddd  zmm1, zmm1, zmm21",
    "vpaddd  zmm2, zmm2, zmm30",
    "vpaddd  zmm3, zmm3, zmm24",
    "vpaddd  zmm0, zmm0, zmm5",
    "vpaddd  zmm1, zmm1, zmm6",
    "vpaddd  zmm2, zmm2, zmm7",
    "vpaddd  zmm3, zmm3, zmm4",
    "vpxord  zmm15, zmm15, zmm0",
    "vpxord  zmm12, zmm12, zmm1",
    "vpxord  zmm13, zmm13, zmm2",
    "vpxord  zmm14, zmm14, zmm3",
    "vprord  zmm15, zmm15, 8",
    "vprord  zmm12, zmm12, 8",
    "vprord  zmm13, zmm13, 8",
    "vprord  zmm14, zmm14, 8",
    "vpaddd  zmm10, zmm10, zmm15",
    "vpaddd  zmm11, zmm11, zmm12",
    "vpaddd  zmm8, zmm8, zmm13",
    "vpaddd  zmm9, zmm9, zmm14",
    "vpxord  zmm5, zmm5, zmm10",
    "vpxord  zmm6, zmm6, zmm11",
    "vpxord  zmm7, zmm7, zmm8",
    "vpxord  zmm4, zmm4, zmm9",
    "vprord  zmm5, zmm5, 7",
    "vprord  zmm6, zmm6, 7",
    "vprord  zmm7, zmm7, 7",
    "vprord  zmm4, zmm4, 7",
    // round 3
    "vpaddd  zmm0, zmm0, zmm19",
    "vpaddd  zmm1, zmm1, zmm26",
    "vpaddd  zmm2, zmm2, zmm29",
    "vpaddd  zmm3, zmm3, zmm23",
    "vpaddd  zmm0, zmm0, zmm4",
    "vpaddd  zmm1, zmm1, zmm5",
    "vpaddd  zmm2, zmm2, zmm6",
    "vpaddd  zmm3, zmm3, zmm7",
    "vpxord  zmm12, zmm12, zmm0",
    "vpxord  zmm13, zmm13, zmm1",
    "vpxord  zmm14, zmm14, zmm2",
    "vpxord  zmm15, zmm15, zmm3",
    "vprord  zmm12, zmm12, 16",
    "vprord  zmm13, zmm13, 16",
    "vprord  zmm14, zmm14, 16",
    "vprord  zmm15, zmm15, 16",
    "vpaddd  zmm8, zmm8, zmm12",
    "vpaddd  zmm9, zmm9, zmm13",
    "vpaddd  zmm10, zmm10, zmm14",
    "vpaddd  zmm11, zmm11, zmm15",
    "vpxord  zmm4, zmm4, zmm8",
    "vpxord  zmm5, zmm5, zmm9",
    "vpxord  zmm6, zmm6, zmm10",
    "vpxord  zmm7, zmm7, zmm11",
    "vprord  zmm4, zmm4, 12",
    "vprord  zmm5, zmm5, 12",
    "vprord  zmm6, zmm6, 12",
    "vprord  zmm7, zmm7, 12",
    "vpaddd  zmm0, zmm0, zmm20",
    "vpaddd  zmm1, zmm1, zmm28",
    "vpaddd  zmm2, zmm2, zmm18",
    "vpaddd  zmm3, zmm3, zmm30",
    "vpaddd  zmm0, zmm0, zmm4",
    "vpaddd  zmm1, zmm1, zmm5",
    "vpaddd  zmm2, zmm2, zmm6",
    "vpaddd  zmm3, zmm3, zmm7",
    "vpxord  zmm12, zmm12, zmm0",
    "vpxord  zmm13, zmm13, zmm1",
    "vpxord  zmm14, zmm14, zmm2",
    "vpxord  zmm15, zmm15, zmm3",
    "vprord  zmm12, zmm12, 8",
    "vprord  zmm13, zmm13, 8",
    "vprord  zmm14, zmm14, 8",
    "vprord  zmm15, zmm15, 8",
    "vpaddd  zmm8, zmm8, zmm12",
    "vpaddd  zmm9, zmm9, zmm13",
    "vpaddd  zmm10, zmm10, zmm14",
    "vpaddd  zmm11, zmm11, zmm15",
    "vpxord  zmm4, zmm4, zmm8",
    "vpxord  zmm5, zmm5, zmm9",
    "vpxord  zmm6, zmm6, zmm10",
    "vpxord  zmm7, zmm7, zmm11",
    "vprord  zmm4, zmm4, 7",
    "vprord  zmm5, zmm5, 7",
    "vprord  zmm6, zmm6, 7",
    "vprord  zmm7, zmm7, 7",
    "vpaddd  zmm0, zmm0, zmm22",
    "vpaddd  zmm1, zmm1, zmm25",
    "vpaddd  zmm2, zmm2, zmm27",
    "vpaddd  zmm3, zmm3, zmm24",
    "vpaddd  zmm0, zmm0, zmm5",
    "vpaddd  zmm1, zmm1, zmm6",
    "vpaddd  zmm2, zmm2, zmm7",
    "vpaddd  zmm3, zmm3, zmm4",
    "vpxord  zmm15, zmm15, zmm0",
    "vpxord  zmm12, zmm12, zmm1",
    "vpxord  zmm13, zmm13, zmm2",
    "vpxord  zmm14, zmm14, zmm3",
    "vprord  zmm15, zmm15, 16",
    "vprord  zmm12, zmm12, 16",
    "vprord  zmm13, zmm13, 16",
    "vprord  zmm14, zmm14, 16",
    "vpaddd  zmm10, zmm10, zmm15",
    "vpaddd  zmm11, zmm11, zmm12",
    "vpaddd  zmm8, zmm8, zmm13",
    "vpaddd  zmm9, zmm9, zmm14",
    "vpxord  zmm5, zmm5, zmm10",
    "vpxord  zmm6, zmm6, zmm11",
    "vpxord  zmm7, zmm7, zmm8",
    "vpxord  zmm4, zmm4, zmm9",
    "vprord  zmm5, zmm5, 12",
    "vprord  zmm6, zmm6, 12",
    "vprord  zmm7, zmm7, 12",
    "vprord  zmm4, zmm4, 12",
    "vpaddd  zmm0, zmm0, zmm21",
    "vpaddd  zmm1, zmm1, zmm16",
    "vpaddd  zmm2, zmm2, zmm31",
    "vpaddd  zmm3, zmm3, zmm17",
    "vpaddd  zmm0, zmm0, zmm5",
    "vpaddd  zmm1, zmm1, zmm6",
    "vpaddd  zmm2, zmm2, zmm7",
    "vpaddd  zmm3, zmm3, zmm4",
    "vpxord  zmm15, zmm15, zmm0",
    "vpxord  zmm12, zmm12, zmm1",
    "vpxord  zmm13, zmm13, zmm2",
    "vpxord  zmm14, zmm14, zmm3",
    "vprord  zmm15, zmm15, 8",
    "vprord  zmm12, zmm12, 8",
    "vprord  zmm13, zmm13, 8",
    "vprord  zmm14, zmm14, 8",
    "vpaddd  zmm10, zmm10, zmm15",
    "vpaddd  zmm11, zmm11, zmm12",
    "vpaddd  zmm8, zmm8, zmm13",
    "vpaddd  zmm9, zmm9, zmm14",
    "vpxord  zmm5, zmm5, zmm10",
    "vpxord  zmm6, zmm6, zmm11",
    "vpxord  zmm7, zmm7, zmm8",
    "vpxord  zmm4, zmm4, zmm9",
    "vprord  zmm5, zmm5, 7",
    "vprord  zmm6, zmm6, 7",
    "vprord  zmm7, zmm7, 7",
    "vprord  zmm4, zmm4, 7",
    // round 4
    "vpaddd  zmm0, zmm0, zmm26",
    "vpaddd  zmm1, zmm1, zmm28",
    "vpaddd  zmm2, zmm2, zmm30",
    "vpaddd  zmm3, zmm3, zmm29",
    "vpaddd  zmm0, zmm0, zmm4",
    "vpaddd  zmm1, zmm1, zmm5",
    "vpaddd  zmm2, zmm2, zmm6",
    "vpaddd  zmm3, zmm3, zmm7",
    "vpxord  zmm12, zmm12, zmm0",
    "vpxord  zmm13, zmm13, zmm1",
    "vpxord  zmm14, zmm14, zmm2",
    "vpxord  zmm15, zmm15, zmm3",
    "vprord  zmm12, zmm12, 16",
    "vprord  zmm13, zmm13, 16",
    "vprord  zmm14, zmm14, 16",
    "vprord  zmm15, zmm15, 16",
    "vpaddd  zmm8, zmm8, zmm12",
    "vpaddd  zmm9, zmm9, zmm13",
    "vpaddd  zmm10, zmm10, zmm14",
    "vpaddd  zmm11, zmm11, zmm15",
    "vpxord  zmm4, zmm4, zmm8",
    "vpxord  zmm5, zmm5, zmm9",
    "vpxord  zmm6, zmm6, zmm10",
    "vpxord  zmm7, zmm7, zmm11",
    "vprord  zmm4, zmm4, 12",
    "vprord  zmm5, zmm5, 12",
    "vprord  zmm6, zmm6, 12",
    "vprord  zmm7, zmm7, 12",
    "vpaddd  zmm0, zmm0, zmm23",
    "vpaddd  zmm1, zmm1, zmm25",
    "vpaddd  zmm2, zmm2, zmm19",
    "vpaddd  zmm3, zmm3, zmm31",
    "vpaddd  zmm0, zmm0, zmm4",
    "vpaddd  zmm1, zmm1, zmm5",
    "vpaddd  zmm2, zmm2, zmm6",
    "vpaddd  zmm3, zmm3, zmm7",
    "vpxord  zmm12, zmm12, zmm0",
    "vpxord  zmm13, zmm13, zmm1",
    "vpxord  zmm14, zmm14, zmm2",
    "vpxord  zmm15, zmm15, zmm3",
    "vprord  zmm12, zmm12, 8",
    "vprord  zmm13, zmm13, 8",
    "vprord  zmm14, zmm14, 8",
    "vprord  zmm15, zmm15, 8",
    "vpaddd  zmm8, zmm8, zmm12",
    "vpaddd  zmm9, zmm9, zmm13",
    "vpaddd  zmm10, zmm10, zmm14",
    "vpaddd  zmm11, zmm11, zmm15",
    "vpxord  zmm4, zmm4, zmm8",
    "vpxord  zmm5, zmm5, zmm9",
    "vpxord  zmm6, zmm6, zmm10",
    "vpxord  zmm7, zmm7, zmm11",
    "vprord  zmm4, zmm4, 7",
    "vprord  zmm5, zmm5, 7",
    "vprord  zmm6, zmm6, 7",
    "vprord  zmm7, zmm7, 7",
    "vpaddd  zmm0, zmm0, zmm20",
    "vpaddd  zmm1, zmm1, zmm27",
    "vpaddd  zmm2, zmm2, zmm21",
    "vpaddd  zmm3, zmm3, zmm17",
    "vpaddd  zmm0, zmm0, zmm5",
    "vpaddd  zmm1, zmm1, zmm6",
    "vpaddd  zmm2, zmm2, zmm7",
    "vpaddd  zmm3, zmm3, zmm4",
    "vpxord  zmm15, zmm15, zmm0",
    "vpxord  zmm12, zmm12, zmm1",
    "vpxord  zmm13, zmm13, zmm2",
    "vpxord  zmm14, zmm14, zmm3",
    "vprord  zmm15, zmm15, 16",
    "vprord  zmm12, zmm12, 16",
    "vprord  zmm13, zmm13, 16",
    "vprord  zmm14, zmm14, 16",
    "vpaddd  zmm10, zmm10, zmm15",
    "vpaddd  zmm11, zmm11, zmm12",
    "vpaddd  zmm8, zmm8, zmm13",
    "vpaddd  zmm9, zmm9, zmm14",
    "vpxord  zmm5, zmm5, zmm10",
    "vpxord  zmm6, zmm6, zmm11",
    "vpxord  zmm7, zmm7, zmm8",
    "vpxord  zmm4, zmm4, zmm9",
    "vprord  zmm5, zmm5, 12",
    "vprord  zmm6, zmm6, 12",
    "vprord  zmm7, zmm7, 12",
    "vprord  zmm4, zmm4, 12",
    "vpaddd  zmm0, zmm0, zmm16",
    "vpaddd  zmm1, zmm1, zmm18",
    "vpaddd  zmm2, zmm2, zmm24",
    "vpaddd  zmm3, zmm3, zmm22",
    "vpaddd  zmm0, zmm0, zmm5",
    "vpaddd  zmm1, zmm1, zmm6",
    "vpaddd  zmm2, zmm2, zmm7",
    "vpaddd  zmm3, zmm3, zmm4",
    "vpxord  zmm15, zmm15, zmm0",
    "vpxord  zmm12, zmm12, zmm1",
    "vpxord  zmm13, zmm13, zmm2",
    "vpxord  zmm14, zmm14, zmm3",
    "vprord  zmm15, zmm15, 8",
    "vprord  zmm12, zmm12, 8",
    "vprord  zmm13, zmm13, 8",
    "vprord  zmm14, zmm14, 8",
    "vpaddd  zmm10, zmm10, zmm15",
    "vpaddd  zmm11, zmm11, zmm12",
    "vpaddd  zmm8, zmm8, zmm13",
    "vpaddd  zmm9, zmm9, zmm14",
    "vpxord  zmm5, zmm5, zmm10",
    "vpxord  zmm6, zmm6, zmm11",
    "vpxord  zmm7, zmm7, zmm8",
    "vpxord  zmm4, zmm4, zmm9",
    "vprord  zmm5, zmm5, 7",
    "vprord  zmm6, zmm6, 7",
    "vprord  zmm7, zmm7, 7",
    "vprord  zmm4, zmm4, 7",
    // round 5
    "vpaddd  zmm0, zmm0, zmm28",
    "vpaddd  zmm1, zmm1, zmm25",
    "vpaddd  zmm2, zmm2, zmm31",
    "vpaddd  zmm3, zmm3, zmm30",
    "vpaddd  zmm0, zmm0, zmm4",
    "vpaddd  zmm1, zmm1, zmm5",
    "vpaddd  zmm2, zmm2, zmm6",
    "vpaddd  zmm3, zmm3, zmm7",
    "vpxord  zmm12, zmm12, zmm0",
    "vpxord  zmm13, zmm13, zmm1",
    "vpxord  zmm14, zmm14, zmm2",
    "vpxord  zmm15, zmm15, zmm3",
    "vprord  zmm12, zmm12, 16",
    "vprord  zmm13, zmm13, 16",
    "vprord  zmm14, zmm14, 16",
    "vprord  zmm15, zmm15, 16",
    "vpaddd  zmm8, zmm8, zmm12",
    "vpaddd  zmm9, zmm9, zmm13",
    "vpaddd  zmm10, zmm10, zmm14",
    "vpaddd  zmm11, zmm11, zmm15",
    "vpxord  zmm4, zmm4, zmm8",
    "vpxord  zmm5, zmm5, zmm9",
    "vpxord  zmm6, zmm6, zmm10",
    "vpxord  zmm7, zmm7, zmm11",
    "vprord  zmm4, zmm4, 12",
    "vprord  zmm5, zmm5, 12",
    "vprord  zmm6, zmm6, 12",
    "vprord  zmm7, zmm7, 12",
    "vpaddd  zmm0, zmm0, zmm29",
    "vpaddd  zmm1, zmm1, zmm27",
    "vpaddd  zmm2, zmm2, zmm26",
    "vpaddd  zmm3, zmm3, zmm24",
    "vpaddd  zmm0, zmm0, zmm4",
    "vpaddd  zmm1, zmm1, zmm5",
    "vpaddd  zmm2, zmm2, zmm6",
    "vpaddd  zmm3, zmm3, zmm7",
    "vpxord  zmm12, zmm12, zmm0",
    "vpxord  zmm13, zmm13, zmm1",
    "vpxord  zmm14, zmm14, zmm2",
    "vpxord  zmm15, zmm15, zmm3",
    "vprord  zmm12, zmm12, 8",
    "vprord  zmm13, zmm13, 8",
    "vprord  zmm14, zmm14, 8",
    "vprord  zmm15, zmm15, 8",
    "vpaddd  zmm8, zmm8, zmm12",
    "vpaddd  zmm9, zmm9, zmm13",
    "vpaddd  zmm10, zmm10, zmm14",
    "vpaddd  zmm11, zmm11, zmm15",
    "vpxord  zmm4, zmm4, zmm8",
    "vpxord  zmm5, zmm5, zmm9",
    "vpxord  zmm6, zmm6, zmm10",
    "vpxord  zmm7, zmm7, zmm11",
    "vprord  zmm4, zmm4, 7",
    "vprord  zmm5, zmm5, 7",
    "vprord  zmm6, zmm6, 7",
    "vprord  zmm7, zmm7, 7",
    "vpaddd  zmm0, zmm0, zmm23",
    "vpaddd  zmm1, zmm1, zmm21",
    "vpaddd  zmm2, zmm2, zmm16",
    "vpaddd  zmm3, zmm3, zmm22",
    "vpaddd  zmm0, zmm0, zmm5",
    "vpaddd  zmm1, zmm1, zmm6",
    "vpaddd  zmm2, zmm2, zmm7",
    "vpaddd  zmm3, zmm3, zmm4",
    "vpxord  zmm15, zmm15, zmm0",
    "vpxord  zmm12, zmm12, zmm1",
    "vpxord  zmm13, zmm13, zmm2",
    "vpxord  zmm14, zmm14, zmm3",
    "vprord  zmm15, zmm15, 16",
    "vprord  zmm12, zmm12, 16",
    "vprord  zmm13, zmm13, 16",
    "vprord  zmm14, zmm14, 16",
    "vpaddd  zmm10, zmm10, zmm15",
    "vpaddd  zmm11, zmm11, zmm12",
    "vpaddd  zmm8, zmm8, zmm13",
    "vpaddd  zmm9, zmm9, zmm14",
    "vpxord  zmm5, zmm5, zmm10",
    "vpxord  zmm6, zmm6, zmm11",
    "vpxord  zmm7, zmm7, zmm8",
    "vpxord  zmm4, zmm4, zmm9",
    "vprord  zmm5, zmm5, 12",
    "vprord  zmm6, zmm6, 12",
    "vprord  zmm7, zmm7, 12",
    "vprord  zmm4, zmm4, 12",
    "vpaddd  zmm0, zmm0, zmm18",
    "vpaddd  zmm1, zmm1, zmm19",
    "vpaddd  zmm2, zmm2, zmm17",
    "vpaddd  zmm3, zmm3, zmm20",
    "vpaddd  zmm0, zmm0, zmm5",
    "vpaddd  zmm1, zmm1, zmm6",
    "vpaddd  zmm2, zmm2, zmm7",
    "vpaddd  zmm3, zmm3, zmm4",
    "vpxord  zmm15, zmm15, zmm0",
    "vpxord  zmm12, zmm12, zmm1",
    "vpxord  zmm13, zmm13, zmm2",
    "vpxord  zmm14, zmm14, zmm3",
    "vprord  zmm15, zmm15, 8",
    "vprord  zmm12, zmm12, 8",
    "vprord  zmm13, zmm13, 8",
    "vprord  zmm14, zmm14, 8",
    "vpaddd  zmm10, zmm10, zmm15",
    "vpaddd  zmm11, zmm11, zmm12",
    "vpaddd  zmm8, zmm8, zmm13",
    "vpaddd  zmm9, zmm9, zmm14",
    "vpxord  zmm5, zmm5, zmm10",
    "vpxord  zmm6, zmm6, zmm11",
    "vpxord  zmm7, zmm7, zmm8",
    "vpxord  zmm4, zmm4, zmm9",
    "vprord  zmm5, zmm5, 7",
    "vprord  zmm6, zmm6, 7",
    "vprord  zmm7, zmm7, 7",
    "vprord  zmm4, zmm4, 7",
    // round 6
    "vpaddd  zmm0, zmm0, zmm25",
    "vpaddd  zmm1, zmm1, zmm27",
    "vpaddd  zmm2, zmm2, zmm24",
    "vpaddd  zmm3, zmm3, zmm31",
    "vpaddd  zmm0, zmm0, zmm4",
    "vpaddd  zmm1, zmm1, zmm5",
    "vpaddd  zmm2, zmm2, zmm6",
    "vpaddd  zmm3, zmm3, zmm7",
    "vpxord  zmm12, zmm12, zmm0",
    "vpxord  zmm13, zmm13, zmm1",
    "vpxord  zmm14, zmm14, zmm2",
    "vpxord  zmm15, zmm15, zmm3",
    "vprord  zmm12, zmm12, 16",
    "vprord  zmm13, zmm13, 16",
    "vprord  zmm14, zmm14, 16",
    "vprord  zmm15, zmm15, 16",
    "vpaddd  zmm8, zmm8, zmm12",
    "vpaddd  zmm9, zmm9, zmm13",
    "vpaddd  zmm10, zmm10, zmm14",
    "vpaddd  zmm11, zmm11, zmm15",
    "vpxord  zmm4, zmm4, zmm8",
    "vpxord  zmm5, zmm5, zmm9",
    "vpxord  zmm6, zmm6, zmm10",
    "vpxord  zmm7, zmm7, zmm11",
    "vprord  zmm4, zmm4, 12",
    "vprord  zmm5, zmm5, 12",
    "vprord  zmm6, zmm6, 12",
    "vprord  zmm7, zmm7, 12",
    "vpaddd  zmm0, zmm0, zmm30",
    "vpaddd  zmm1, zmm1, zmm21",
    "vpaddd  zmm2, zmm2, zmm28",
    "vpaddd  zmm3, zmm3, zmm17",
    "vpaddd  zmm0, zmm0, zmm4",
    "vpaddd  zmm1, zmm1, zmm5",
    "vpaddd  zmm2, zmm2, zmm6",
    "vpaddd  zmm3, zmm3, zmm7",
    "vpxord  zmm12, zmm12, zmm0",
    "vpxord  zmm13, zmm13, zmm1",
    "vpxord  zmm14, zmm14, zmm2",
    "vpxord  zmm15, zmm15, zmm3",
    "vprord  zmm12, zmm12, 8",
    "vprord  zmm13, zmm13, 8",
    "vprord  zmm14, zmm14, 8",
    "vprord  zmm15, zmm15, 8",
    "vpaddd  zmm8, zmm8, zmm12",
    "vpaddd  zmm9, zmm9, zmm13",
    "vpaddd  zmm10, zmm10, zmm14",
    "vpaddd  zmm11, zmm11, zmm15",
    "vpxord  zmm4, zmm4, zmm8",
    "vpxord  zmm5, zmm5, zmm9",
    "vpxord  zmm6, zmm6, zmm10",
    "vpxord  zmm7, zmm7, zmm11",
    "vprord  zmm4, zmm4, 7",
    "vprord  zmm5, zmm5, 7",
    "vprord  zmm6, zmm6, 7",
    "vprord  zmm7, zmm7, 7",
    "vpaddd  zmm0, zmm0, zmm29",
    "vpaddd  zmm1, zmm1, zmm16",
    "vpaddd  zmm2, zmm2, zmm18",
    "vpaddd  zmm3, zmm3, zmm20",
    "vpaddd  zmm0, zmm0, zmm5",
    "vpaddd  zmm1, zmm1, zmm6",
    "vpaddd  zmm2, zmm2, zmm7",
    "vpaddd  zmm3, zmm3, zmm4",
    "vpxord  zmm15, zmm15, zmm0",
    "vpxord  zmm12, zmm12, zmm1",
    "vpxord  zmm13, zmm13, zmm2",
    "vpxord  zmm14, zmm14, zmm3",
    "vprord  zmm15, zmm15, 16",
    "vprord  zmm12, zmm12, 16",
    "vprord  zmm13, zmm13, 16",
    "vprord  zmm14, zmm14, 16",
    "vpaddd  zmm10, zmm10, zmm15",
    "vpaddd  zmm11, zmm11, zmm12",
    "vpaddd  zmm8, zmm8, zmm13",
    "vpaddd  zmm9, zmm9, zmm14",
    "vpxord  zmm5, zmm5, zmm10",
    "vpxord  zmm6, zmm6, zmm11",
    "vpxord  zmm7, zmm7, zmm8",
    "vpxord  zmm4, zmm4, zmm9",
    "vprord  zmm5, zmm5, 12",
    "vprord  zmm6, zmm6, 12",
    "vprord  zmm7, zmm7, 12",
    "vprord  zmm4, zmm4, 12",
    "vpaddd  zmm0, zmm0, zmm19",
    "vpaddd  zmm1, zmm1, zmm26",
    "vpaddd  zmm2, zmm2, zmm22",
    "vpaddd  zmm3, zmm3, zmm23",
    "vpaddd  zmm0, zmm0, zmm5",
    "vpaddd  zmm1, zmm1, zmm6",
    "vpaddd  zmm2, zmm2, zmm7",
    "vpaddd  zmm3, zmm3, zmm4",
    "vpxord  zmm15, zmm15, zmm0",
    "vpxord  zmm12, zmm12, zmm1",
    "vpxord  zmm13, zmm13, zmm2",
    "vpxord  zmm14, zmm14, zmm3",
    "vprord  zmm15, zmm15, 8",
    "vprord  zmm12, zmm12, 8",
    "vprord  zmm13, zmm13, 8",
    "vprord  zmm14, zmm14, 8",
    "vpaddd  zmm10, zmm10, zmm15",
    "vpaddd  zmm11, zmm11, zmm12",
    "vpaddd  zmm8, zmm8, zmm13",
    "vpaddd  zmm9, zmm9, zmm14",
    "vpxord  zmm5, zmm5, zmm10",
    "vpxord  zmm6, zmm6, zmm11",
    "vpxord  zmm7, zmm7, zmm8",
    "vpxord  zmm4, zmm4, zmm9",
    "vprord  zmm5, zmm5, 7",
    "vprord  zmm6, zmm6, 7",
    "vprord  zmm7, zmm7, 7",
    "vprord  zmm4, zmm4, 7",
    // round 7
    "vpaddd  zmm0, zmm0, zmm27",
    "vpaddd  zmm1, zmm1, zmm21",
    "vpaddd  zmm2, zmm2, zmm17",
    "vpaddd  zmm3, zmm3, zmm24",
    "vpaddd  zmm0, zmm0, zmm4",
    "vpaddd  zmm1, zmm1, zmm5",
    "vpaddd  zmm2, zmm2, zmm6",
    "vpaddd  zmm3, zmm3, zmm7",
    "vpxord  zmm12, zmm12, zmm0",
    "vpxord  zmm13, zmm13, zmm1",
    "vpxord  zmm14, zmm14, zmm2",
    "vpxord  zmm15, zmm15, zmm3",
    "vprord  zmm12, zmm12, 16",
    "vprord  zmm13, zmm13, 16",
    "vprord  zmm14, zmm14, 16",
    "vprord  zmm15, zmm15, 16",
    "vpaddd  zmm8, zmm8, zmm12",
    "vpaddd  zmm9, zmm9, zmm13",
    "vpaddd  zmm10, zmm10, zmm14",
    "vpaddd  zmm11, zmm11, zmm15",
    "vpxord  zmm4, zmm4, zmm8",
    "vpxord  zmm5, zmm5, zmm9",
    "vpxord  zmm6, zmm6, zmm10",
    "vpxord  zmm7, zmm7, zmm11",
    "vprord  zmm4, zmm4, 12",
    "vprord  zmm5, zmm5, 12",
    "vprord  zmm6, zmm6, 12",
    "vprord  zmm7, zmm7, 12",
    "vpaddd  zmm0, zmm0, zmm31",
    "vpaddd  zmm1, zmm1, zmm16",
    "vpaddd  zmm2, zmm2, zmm25",
    "vpaddd  zmm3, zmm3, zmm22",
    "vpaddd  zmm0, zmm0, zmm4",
    "vpaddd  zmm1, zmm1, zmm5",
    "vpaddd  zmm2, zmm2, zmm6",
    "vpaddd  zmm3, zmm3, zmm7",
    "vpxord  zmm12, zmm12, zmm0",
    "vpxord  zmm13, zmm13, zmm1",
    "vpxord  zmm14, zmm14, zmm2",
    "vpxord  zmm15, zmm15, zmm3",
    "vprord  zmm12, zmm12, 8",
    "vprord  zmm13, zmm13, 8",
    "vprord  zmm14, zmm14, 8",
    "vprord  zmm15, zmm15, 8",
    "vpaddd  zmm8, zmm8, zmm12",
    "vpaddd  zmm9, zmm9, zmm13",
    "vpaddd  zmm10, zmm10, zmm14",
    "vpaddd  zmm11, zmm11, zmm15",
    "vpxord  zmm4, zmm4, zmm8",
    "vpxord  zmm5, zmm5, zmm9",
    "vpxord  zmm6, zmm6, zmm10",
    "vpxord  zmm7, zmm7, zmm11",
    "vprord  zmm4, zmm4, 7",
    "vprord  zmm5, zmm5, 7",
    "vprord  zmm6, zmm6, 7",
    "vprord  zmm7, zmm7, 7",
    "vpaddd  zmm0, zmm0, zmm30",
    "vpaddd  zmm1, zmm1, zmm18",
    "vpaddd  zmm2, zmm2, zmm19",
    "vpaddd  zmm3, zmm3, zmm23",
    "vpaddd  zmm0, zmm0, zmm5",
    "vpaddd  zmm1, zmm1, zmm6",
    "vpaddd  zmm2, zmm2, zmm7",
    "vpaddd  zmm3, zmm3, zmm4",
    "vpxord  zmm15, zmm15, zmm0",
    "vpxord  zmm12, zmm12, zmm1",
    "vpxord  zmm13, zmm13, zmm2",
    "vpxord  zmm14, zmm14, zmm3",
    "vprord  zmm15, zmm15, 16",
    "vprord  zmm12, zmm12, 16",
    "vprord  zmm13, zmm13, 16",
    "vprord  zmm14, zmm14, 16",
    "vpaddd  zmm10, zmm10, zmm15",
    "vpaddd  zmm11, zmm11, zmm12",
    "vpaddd  zmm8, zmm8, zmm13",
    "vpaddd  zmm9, zmm9, zmm14",
    "vpxord  zmm5, zmm5, zmm10",
    "vpxord  zmm6, zmm6, zmm11",
    "vpxord  zmm7, zmm7, zmm8",
    "vpxord  zmm4, zmm4, zmm9",
    "vprord  zmm5, zmm5, 12",
    "vprord  zmm6, zmm6, 12",
    "vprord  zmm7, zmm7, 12",
    "vprord  zmm4, zmm4, 12",
    "vpaddd  zmm0, zmm0, zmm26",
    "vpaddd  zmm1, zmm1, zmm28",
    "vpaddd  zmm2, zmm2, zmm20",
    "vpaddd  zmm3, zmm3, zmm29",
    "vpaddd  zmm0, zmm0, zmm5",
    "vpaddd  zmm1, zmm1, zmm6",
    "vpaddd  zmm2, zmm2, zmm7",
    "vpaddd  zmm3, zmm3, zmm4",
    "vpxord  zmm15, zmm15, zmm0",
    "vpxord  zmm12, zmm12, zmm1",
    "vpxord  zmm13, zmm13, zmm2",
    "vpxord  zmm14, zmm14, zmm3",
    "vprord  zmm15, zmm15, 8",
    "vprord  zmm12, zmm12, 8",
    "vprord  zmm13, zmm13, 8",
    "vprord  zmm14, zmm14, 8",
    "vpaddd  zmm10, zmm10, zmm15",
    "vpaddd  zmm11, zmm11, zmm12",
    "vpaddd  zmm8, zmm8, zmm13",
    "vpaddd  zmm9, zmm9, zmm14",
    "vpxord  zmm5, zmm5, zmm10",
    "vpxord  zmm6, zmm6, zmm11",
    "vpxord  zmm7, zmm7, zmm8",
    "vpxord  zmm4, zmm4, zmm9",
    "vprord  zmm5, zmm5, 7",
    "vprord  zmm6, zmm6, 7",
    "vprord  zmm7, zmm7, 7",
    "vprord  zmm4, zmm4, 7",
    // final xors
    "vpxord  zmm0, zmm0, zmm8",
    "vpxord  zmm1, zmm1, zmm9",
    "vpxord  zmm2, zmm2, zmm10",
    "vpxord  zmm3, zmm3, zmm11",
    "vpxord  zmm4, zmm4, zmm12",
    "vpxord  zmm5, zmm5, zmm13",
    "vpxord  zmm6, zmm6, zmm14",
    "vpxord  zmm7, zmm7, zmm15",
    "ret",
    //
    // --------------------------------------------------------------------------------------------
    // blake3_avx512_blocks_16
    //
    // zmm0-zmm7: incoming CV
    // rdi: pointer to first message block in rdi, subsequent blocks offset by 1024 bytes each
    // rsi: [unused]
    // rdx: pointer to two 64-byte aligned vectors, counter-low followed by counter-high
    // ecx: block len (always 64)
    // r8d: flags (other than CHUNK_START and CHUNK_END)
    //
    // This routine loads and transposes message words, populates the rest of the state registers,
    // and invokes blake3_avx512_kernel_16.
    // --------------------------------------------------------------------------------------------
    "blake3_avx512_blocks_16:",
    // Load the message blocks first (unaligned). See the comments immediately below for why we
    // choose these registers.
    "vmovdqu32 zmm24, zmmword ptr [rdi +  0 * 1024]",
    "vmovdqu32 zmm25, zmmword ptr [rdi +  1 * 1024]",
    "vmovdqu32 zmm26, zmmword ptr [rdi +  2 * 1024]",
    "vmovdqu32 zmm27, zmmword ptr [rdi +  3 * 1024]",
    "vmovdqu32 zmm28, zmmword ptr [rdi +  4 * 1024]",
    "vmovdqu32 zmm29, zmmword ptr [rdi +  5 * 1024]",
    "vmovdqu32 zmm30, zmmword ptr [rdi +  6 * 1024]",
    "vmovdqu32 zmm31, zmmword ptr [rdi +  7 * 1024]",
    "vmovdqu32  zmm8, zmmword ptr [rdi +  8 * 1024]",
    "vmovdqu32  zmm9, zmmword ptr [rdi +  9 * 1024]",
    "vmovdqu32 zmm10, zmmword ptr [rdi + 10 * 1024]",
    "vmovdqu32 zmm11, zmmword ptr [rdi + 11 * 1024]",
    "vmovdqu32 zmm12, zmmword ptr [rdi + 12 * 1024]",
    "vmovdqu32 zmm13, zmmword ptr [rdi + 13 * 1024]",
    "vmovdqu32 zmm14, zmmword ptr [rdi + 14 * 1024]",
    "vmovdqu32 zmm15, zmmword ptr [rdi + 15 * 1024]",
    // Transpose the message blocks. This requires a few different passes:
    // 1) interleave 32-bit lanes
    // 2) interleave 64-bit lanes
    // 3) interleave 128-bit lanes
    // 4) interleave 256-bit lanes (but there's no such instruction, so actually 128 bits again)
    // The last of these passes is easier to implement if we can make use of 8 scratch registers.
    // zmm0-zmm7 are holding the incoming CV and we don't want to touch those. But zmm8-zmm15
    // aren't holding anything important, and we can use those as long as we reinitialize them
    // before we run the kernel. For consistency, we'll use all 8 scratch registers for each pass
    // (even though the earlier passes would be fine using fewer), and we'll have each pass rotate
    // our 24 message+scratch vectors 8 places "to the left". Thus starting 8 places "to the right"
    // in the rotation lets us end up on target after 4 passes, and that's why we loaded the
    // message vectors in the order we did above.
    //
    // The first pass, interleaving 32-bit lanes. Here's the first vector before:
    // (zmm24) a0,  a1,  a2,  a3,  a4,  a5,  a8,  a7,  a8,  a9, a10, a11, a12, a13, a14, a15
    // And after:
    // (zmm16) a0,  b0,  a1,  b1,  a4,  b4,  a5,  b5,  a8,  b8,  a9,  b9, a12, b12, a13, b13
    "vpunpckldq zmm16, zmm24, zmm25",
    "vpunpckhdq zmm17, zmm24, zmm25",
    "vpunpckldq zmm18, zmm26, zmm27",
    "vpunpckhdq zmm19, zmm26, zmm27",
    "vpunpckldq zmm20, zmm28, zmm29",
    "vpunpckhdq zmm21, zmm28, zmm29",
    "vpunpckldq zmm22, zmm30, zmm31",
    "vpunpckhdq zmm23, zmm30, zmm31",
    "vpunpckldq zmm24,  zmm8,  zmm9",
    "vpunpckhdq zmm25,  zmm8,  zmm9",
    "vpunpckldq zmm26, zmm10, zmm11",
    "vpunpckhdq zmm27, zmm10, zmm11",
    "vpunpckldq zmm28, zmm12, zmm13",
    "vpunpckhdq zmm29, zmm12, zmm13",
    "vpunpckldq zmm30, zmm14, zmm15",
    "vpunpckhdq zmm31, zmm14, zmm15",
    // The second pass, interleaving 64-bit lanes. After this the first vector will be:
    // (zmm8)  a0,  b0,  c0,  d0,  a4,  b4,  c4,  d4,  a8,  b8,  c8,  d8, a12, b12, c12, d12
    "vpunpcklqdq  zmm8, zmm16, zmm18",
    "vpunpckhqdq  zmm9, zmm16, zmm18",
    "vpunpcklqdq zmm10, zmm17, zmm19",
    "vpunpckhqdq zmm11, zmm17, zmm19",
    "vpunpcklqdq zmm12, zmm20, zmm22",
    "vpunpckhqdq zmm13, zmm20, zmm22",
    "vpunpcklqdq zmm14, zmm21, zmm23",
    "vpunpckhqdq zmm15, zmm21, zmm23",
    "vpunpcklqdq zmm16, zmm24, zmm26",
    "vpunpckhqdq zmm17, zmm24, zmm26",
    "vpunpcklqdq zmm18, zmm25, zmm27",
    "vpunpckhqdq zmm19, zmm25, zmm27",
    "vpunpcklqdq zmm20, zmm28, zmm30",
    "vpunpckhqdq zmm21, zmm28, zmm30",
    "vpunpcklqdq zmm22, zmm29, zmm31",
    "vpunpckhqdq zmm23, zmm29, zmm31",
    // The third pass, interleaving 128-bit lanes. After this the first vector will be:
    // (zmm24) a0,  b0,  c0,  d0,  a8,  b8,  c8,  d8,  e0,  f0,  g0,  h0,  e8,  f8,  g8,  h8
    "vshufi32x4 zmm24,  zmm8, zmm12, 0x88", // 0b10001000: lo 128-bit lanes A0/A2/B0/B2
    "vshufi32x4 zmm25,  zmm9, zmm13, 0x88",
    "vshufi32x4 zmm26, zmm10, zmm14, 0x88",
    "vshufi32x4 zmm27, zmm11, zmm15, 0x88",
    "vshufi32x4 zmm28,  zmm8, zmm12, 0xdd", // 0b11011101: hi 128-bit lanes A1/A3/B1/B3
    "vshufi32x4 zmm29,  zmm9, zmm13, 0xdd",
    "vshufi32x4 zmm30, zmm10, zmm14, 0xdd",
    "vshufi32x4 zmm31, zmm11, zmm15, 0xdd",
    "vshufi32x4  zmm8, zmm16, zmm20, 0x88", // lo
    "vshufi32x4  zmm9, zmm17, zmm21, 0x88",
    "vshufi32x4 zmm10, zmm18, zmm22, 0x88",
    "vshufi32x4 zmm11, zmm19, zmm23, 0x88",
    "vshufi32x4 zmm12, zmm16, zmm20, 0xdd", // hi
    "vshufi32x4 zmm13, zmm17, zmm21, 0xdd",
    "vshufi32x4 zmm14, zmm18, zmm22, 0xdd",
    "vshufi32x4 zmm15, zmm19, zmm23, 0xdd",
    // The fourth and final pass, interleaving 128-bit lanes again. The first vector will be:
    // (zmm16) a0,  b0,  c0,  d0,  e0,  f0,  g0,  h0,  i0,  j0,  k0,  l0,  m0,  n0,  o0,  p0
    "vshufi32x4 zmm16, zmm24,  zmm8, 0x88", // lo
    "vshufi32x4 zmm17, zmm25,  zmm9, 0x88",
    "vshufi32x4 zmm18, zmm26, zmm10, 0x88",
    "vshufi32x4 zmm19, zmm27, zmm11, 0x88",
    "vshufi32x4 zmm20, zmm28, zmm12, 0x88",
    "vshufi32x4 zmm21, zmm29, zmm13, 0x88",
    "vshufi32x4 zmm22, zmm30, zmm14, 0x88",
    "vshufi32x4 zmm23, zmm31, zmm15, 0x88",
    "vshufi32x4 zmm24, zmm24,  zmm8, 0xdd", // hi
    "vshufi32x4 zmm25, zmm25,  zmm9, 0xdd",
    "vshufi32x4 zmm26, zmm26, zmm10, 0xdd",
    "vshufi32x4 zmm27, zmm27, zmm11, 0xdd",
    "vshufi32x4 zmm28, zmm28, zmm12, 0xdd",
    "vshufi32x4 zmm29, zmm29, zmm13, 0xdd",
    "vshufi32x4 zmm30, zmm30, zmm14, 0xdd",
    "vshufi32x4 zmm31, zmm31, zmm15, 0xdd",
    // Initialize the third and fourth rows of the state, which we just used as scratch space
    // during transposition.
    "vmovdqa32  zmm8, zmmword ptr [BLAKE3_IV0_16 + rip]", // IV constants
    "vmovdqa32  zmm9, zmmword ptr [BLAKE3_IV1_16 + rip]",
    "vmovdqa32 zmm10, zmmword ptr [BLAKE3_IV2_16 + rip]",
    "vmovdqa32 zmm11, zmmword ptr [BLAKE3_IV3_16 + rip]",
    "vmovdqa32 zmm12, zmmword ptr [rdx + 64 * 0]", // counter low
    "vmovdqa32 zmm13, zmmword ptr [rdx + 64 * 1]", // counter high
    "vpbroadcastd zmm14, ecx",                     // block length (always 64)
    "vpbroadcastd zmm15, r8d",                     // flags
    // Run the kernel and then exit.
    "call blake3_avx512_kernel_16",
    "ret",
    //
    // --------------------------------------------------------------------------------------------
    // blake3_avx512_chunks_16
    //
    // zmm0-zmm31: [clobbered]
    // rdi: pointer to 16 contiguous chunks of 1024 bytes each, unaligned
    // rsi: pointer to the 32-byte key, unaligned
    // rdx: pointer to two 64-byte aligned vectors, counter-low followed by counter-high
    // ecx: [clobbered]
    // r8d: flags (other than CHUNK_START and CHUNK_END)
    //  r9: out pointer to 8x64 bytes, 64-byte aligned
    //
    // This routine broadcasts the key and calls blake3_avx512_blocks_16 for each block, setting
    // CHUNK_START and CHUNK_END for the first and last blocks respectively. The final transposed
    // CVs in zmm0-zmm7 are written to the out pointer.
    // --------------------------------------------------------------------------------------------
    "blake3_avx512_chunks_16:",
    // Broadcast the key into zmm0-zmm7. Use ecx as scratch.
    "mov ecx, dword ptr [rsi + 0 * 4]",
    "vpbroadcastd zmm0, ecx",
    "mov ecx, dword ptr [rsi + 1 * 4]",
    "vpbroadcastd zmm1, ecx",
    "mov ecx, dword ptr [rsi + 2 * 4]",
    "vpbroadcastd zmm2, ecx",
    "mov ecx, dword ptr [rsi + 3 * 4]",
    "vpbroadcastd zmm3, ecx",
    "mov ecx, dword ptr [rsi + 4 * 4]",
    "vpbroadcastd zmm4, ecx",
    "mov ecx, dword ptr [rsi + 5 * 4]",
    "vpbroadcastd zmm5, ecx",
    "mov ecx, dword ptr [rsi + 6 * 4]",
    "vpbroadcastd zmm6, ecx",
    "mov ecx, dword ptr [rsi + 7 * 4]",
    "vpbroadcastd zmm7, ecx",
    // ecx is the block length arg to blake3_avx512_blocks_16. It is always 64.
    "mov ecx, 64",
    // Set the CHUNK_START flag.
    "or r8d, 1",
    // Compress the first block.
    "call blake3_avx512_blocks_16",
    // Clear the CHUNK_START flag.
    "and r8d, 0xFFFFFFFE",
    // Compress the middle fourteen blocks.
    "add rdi, 64",
    "call blake3_avx512_blocks_16",
    "add rdi, 64",
    "call blake3_avx512_blocks_16",
    "add rdi, 64",
    "call blake3_avx512_blocks_16",
    "add rdi, 64",
    "call blake3_avx512_blocks_16",
    "add rdi, 64",
    "call blake3_avx512_blocks_16",
    "add rdi, 64",
    "call blake3_avx512_blocks_16",
    "add rdi, 64",
    "call blake3_avx512_blocks_16",
    "add rdi, 64",
    "call blake3_avx512_blocks_16",
    "add rdi, 64",
    "call blake3_avx512_blocks_16",
    "add rdi, 64",
    "call blake3_avx512_blocks_16",
    "add rdi, 64",
    "call blake3_avx512_blocks_16",
    "add rdi, 64",
    "call blake3_avx512_blocks_16",
    "add rdi, 64",
    "call blake3_avx512_blocks_16",
    "add rdi, 64",
    "call blake3_avx512_blocks_16",
    // Set the CHUNK_END flag.
    "or r8d, 2",
    // Compress the last block.
    "add rdi, 64",
    "call blake3_avx512_blocks_16",
    // Write the output and exit.
    "vmovdqa32 zmmword ptr [r9 + 0 * 64], zmm0",
    "vmovdqa32 zmmword ptr [r9 + 1 * 64], zmm1",
    "vmovdqa32 zmmword ptr [r9 + 2 * 64], zmm2",
    "vmovdqa32 zmmword ptr [r9 + 3 * 64], zmm3",
    "vmovdqa32 zmmword ptr [r9 + 4 * 64], zmm4",
    "vmovdqa32 zmmword ptr [r9 + 5 * 64], zmm5",
    "vmovdqa32 zmmword ptr [r9 + 6 * 64], zmm6",
    "vmovdqa32 zmmword ptr [r9 + 7 * 64], zmm7",
    "vzeroupper",
    "ret",
);

#[repr(C, align(64))]
#[derive(Copy, Clone, Debug)]
pub struct Words16([u32; 16]);

#[no_mangle]
static BLAKE3_IV0_16: Words16 = Words16([crate::IV[0]; 16]);
#[no_mangle]
static BLAKE3_IV1_16: Words16 = Words16([crate::IV[1]; 16]);
#[no_mangle]
static BLAKE3_IV2_16: Words16 = Words16([crate::IV[2]; 16]);
#[no_mangle]
static BLAKE3_IV3_16: Words16 = Words16([crate::IV[3]; 16]);

pub unsafe fn chunks16(
    message: &[u8; 16 * CHUNK_LEN],
    key: &[u32; 8],
    counter: u64,
    flags: u32,
    out_ptr: *mut [Words16; 8],
) {
    // Prepare the counter vectors, the low words and high words.
    let mut counter_vectors = [Words16([0; 16]); 2];
    for i in 0..16 {
        counter_vectors[0].0[i] = (counter + i as u64) as u32;
        counter_vectors[1].0[i] = ((counter + i as u64) >> 32) as u32;
    }
    asm!(
        "call blake3_avx512_chunks_16",
        inout("rdi") message => _,
        inout("rsi") key => _,
        inout("rdx") &counter_vectors => _,
        out("ecx") _,
        inout("r8d") flags => _,
        inout("r9") out_ptr => _,
        out("zmm0") _, out("zmm1") _, out("zmm2") _, out("zmm3") _,
        out("zmm4") _, out("zmm5") _, out("zmm6") _, out("zmm7") _,
        out("zmm8") _, out("zmm9") _, out("zmm10") _, out("zmm11") _,
        out("zmm12") _, out("zmm13") _, out("zmm14") _, out("zmm15") _,
        out("zmm16") _, out("zmm17") _, out("zmm18") _, out("zmm19") _,
        out("zmm20") _, out("zmm21") _, out("zmm22") _, out("zmm23") _,
        out("zmm24") _, out("zmm25") _, out("zmm26") _, out("zmm27") _,
        out("zmm28") _, out("zmm29") _, out("zmm30") _, out("zmm31") _,
    );
}

#[test]
fn test_chunks16() {
    let mut message = [0; 16 * CHUNK_LEN];
    crate::test::paint_test_input(&mut message);

    let mut chunk_refs: Vec<&[u8; CHUNK_LEN]> = Vec::new();
    for i in 0..16 {
        chunk_refs.push(message[i * CHUNK_LEN..][..CHUNK_LEN].try_into().unwrap());
    }
    let mut expected_out = [0; 32 * 16];
    unsafe {
        crate::avx512::hash_many(
            chunk_refs[..].try_into().unwrap(),
            crate::IV,
            0,
            crate::IncrementCounter::Yes,
            0,
            crate::CHUNK_START,
            crate::CHUNK_END,
            &mut expected_out,
        );
    }

    let mut found_out = [Words16([0; 16]); 8];
    unsafe {
        chunks16(&message, crate::IV, 0, 0, &mut found_out);
    }
    let mut found_out_transposed = [0; 16 * 8 * 4];
    for vector_i in 0..8 {
        for element_i in 0..16 {
            let word = found_out[vector_i].0[element_i];
            let word_start = 32 * element_i + 4 * vector_i;
            found_out_transposed[word_start..][..4].copy_from_slice(&word.to_le_bytes());
        }
    }

    assert_eq!(expected_out, found_out_transposed);
}
