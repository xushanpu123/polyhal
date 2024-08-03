// //! Font 6x8 ASCII library.
// //! Thanks https://github.com/jdmorise/TTF2BMH.
// //! ASCII 32 - 126, font with 6, height 8
// pub(super) const BIT_FONTS: [[u8; 6]; 95] = [
//     [0, 0, 0, 0, 0, 0],
//     [0, 0, 184, 0, 0, 0],
//     [0, 12, 12, 0, 0, 0],
//     [64, 248, 208, 120, 0, 0],
//     [136, 152, 254, 232, 0, 0],
//     [0, 68, 44, 124, 92, 96],
//     [0, 216, 168, 168, 224, 32],
//     [0, 0, 12, 0, 0, 0],
//     [0, 60, 194, 0, 0, 0],
//     [0, 128, 66, 60, 0, 0],
//     [0, 96, 48, 96, 0, 0],
//     [0, 32, 112, 32, 0, 0],
//     [0, 0, 192, 0, 0, 0],
//     [0, 32, 32, 32, 0, 0],
//     [0, 0, 64, 0, 0, 0],
//     [128, 96, 24, 6, 0, 0],
//     [0, 112, 136, 168, 112, 0],
//     [0, 136, 248, 128, 0, 0],
//     [128, 136, 200, 184, 0, 0],
//     [0, 136, 168, 216, 0, 0],
//     [32, 56, 36, 112, 32, 0],
//     [0, 152, 152, 104, 0, 0],
//     [0, 112, 152, 152, 96, 0],
//     [0, 136, 104, 24, 0, 0],
//     [0, 208, 168, 168, 216, 0],
//     [0, 48, 72, 200, 112, 0],
//     [0, 0, 144, 0, 0, 0],
//     [0, 0, 208, 0, 0, 0],
//     [0, 16, 40, 72, 0, 0],
//     [0, 96, 96, 96, 0, 0],
//     [0, 72, 40, 16, 0, 0],
//     [0, 8, 168, 24, 0, 0],
//     [0, 200, 40, 232, 8, 240],
//     [128, 112, 88, 224, 0, 0],
//     [0, 248, 168, 216, 192, 0],
//     [0, 112, 136, 136, 136, 0],
//     [0, 248, 136, 136, 112, 0],
//     [0, 248, 168, 168, 128, 0],
//     [0, 248, 40, 40, 0, 0],
//     [0, 112, 136, 168, 232, 0],
//     [0, 248, 32, 32, 248, 0],
//     [0, 136, 248, 136, 0, 0],
//     [128, 136, 136, 120, 0, 0],
//     [0, 248, 32, 80, 136, 0],
//     [0, 248, 128, 128, 128, 0],
//     [0, 248, 112, 48, 248, 0],
//     [0, 248, 56, 192, 248, 0],
//     [0, 112, 136, 136, 112, 0],
//     [0, 248, 40, 40, 24, 0],
//     [0, 112, 136, 136, 112, 0],
//     [0, 248, 40, 104, 152, 0],
//     [0, 152, 168, 168, 200, 0],
//     [8, 8, 248, 8, 8, 0],
//     [0, 248, 128, 128, 120, 0],
//     [24, 96, 192, 56, 0, 0],
//     [56, 224, 112, 224, 24, 0],
//     [128, 88, 48, 200, 0, 0],
//     [8, 48, 224, 16, 8, 0],
//     [0, 200, 168, 152, 8, 0],
//     [0, 252, 4, 0, 0, 0],
//     [0, 12, 48, 64, 0, 0],
//     [0, 0, 4, 252, 0, 0],
//     [0, 12, 6, 8, 0, 0],
//     [0, 0, 0, 0, 0, 0],
//     [0, 0, 4, 0, 0, 0],
//     [0, 192, 176, 240, 0, 0],
//     [0, 252, 144, 144, 112, 0],
//     [0, 96, 144, 144, 144, 0],
//     [0, 224, 144, 144, 252, 0],
//     [0, 96, 176, 176, 32, 0],
//     [0, 16, 252, 20, 4, 0],
//     [0, 88, 104, 88, 128, 0],
//     [0, 252, 16, 240, 0, 0],
//     [0, 144, 244, 128, 0, 0],
//     [0, 16, 20, 240, 0, 0],
//     [0, 252, 32, 80, 128, 0],
//     [0, 4, 252, 128, 0, 0],
//     [0, 240, 16, 240, 16, 240],
//     [0, 240, 16, 240, 0, 0],
//     [0, 96, 144, 144, 96, 0],
//     [0, 240, 144, 144, 240, 0],
//     [0, 224, 144, 144, 240, 0],
//     [16, 240, 16, 16, 0, 0],
//     [0, 176, 208, 208, 0, 0],
//     [0, 0, 248, 128, 128, 0],
//     [0, 240, 128, 240, 0, 0],
//     [16, 96, 192, 48, 0, 0],
//     [112, 224, 96, 224, 16, 0],
//     [0, 208, 96, 144, 0, 0],
//     [16, 96, 192, 48, 0, 0],
//     [0, 144, 240, 144, 0, 0],
//     [0, 32, 220, 2, 0, 0],
//     [0, 0, 252, 0, 0, 0],
//     [0, 2, 220, 32, 0, 0],
//     [64, 32, 64, 64, 0, 0],
// ];

pub(super) const BIT_FONTS: &[u8] = include_bytes!("iso-8x16.font");
