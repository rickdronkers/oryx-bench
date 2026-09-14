//! ZSA Moonlander Mark I — 72 matrix keys, 0 encoders, split layout with
//! 4-key thumb clusters (one 2u "big" key + three 1.5u-tall "piano" keys
//! per half).
//!
//! ## Position naming scheme
//!
//! Positions are named `<HAND>_<COL>_<ROW>` where:
//!
//! - `HAND` ∈ { `L`, `R` } — left or right half
//! - `COL` ∈ { `outer`, `pinky`, `ring`, `middle`, `index`, `inner`,
//!   `innermost` } — the physical column. `outer` is the outermost edge
//!   column (leftmost on the left half, rightmost on the right half),
//!   `inner` is the second column from the split gap, and `innermost`
//!   is the extra column directly at the split gap (the Moonlander has
//!   one more column per half than the Voyager).
//! - `ROW` ∈ { `num`, `top`, `home`, `bottom`, `mod` } — top to bottom.
//!   `mod` is the 5-key lowest row (outer..index columns only); `bottom`
//!   has no `innermost` key and `mod` has neither `inner` nor
//!   `innermost` keys.
//!
//! Thumb keys are named `L_thumb_big` (the 2u red key) and
//! `L_thumb_piano_1` / `_2` / `_3` (the three tall piano keys, numbered
//! from the one nearest the main key area outward to the tip of the
//! cluster), mirrored for the right half.
//!
//! ## Matrix indices
//!
//! Two index orderings are relevant: Oryx's `keys[]` serialization
//! order (which is what the canonical layout uses) and QMK's
//! `LAYOUT_moonlander(...)` macro positional argument order (which the
//! codegen layer permutes into via `Geometry::qmk_arg_order`).
//!
//! The serialization order Oryx uses is:
//!
//!   indices  0..32  → left half rows 0-4 (7 + 7 + 7 + 6 + 5 keys)
//!   indices 32..36  → LEFT THUMB cluster (big, piano 1..3)
//!   indices 36..68  → right half rows 0-4 (mirrored: innermost→outer)
//!   indices 68..72  → RIGHT THUMB cluster (big, piano 3..1)
//!
//! Verified by cross-matching the pinned ZSA QMK fork's
//! `keyboards/zsa/moonlander/keymaps/oryx/keymap.c` (whose `LAYOUT(...)`
//! arguments are in QMK arg order) against the same default layout
//! pulled from the Oryx GraphQL endpoint (whose `keys[]` array is in
//! canonical order) — every distinctive key (LSFT at matrix [3,0],
//! `LT(SYMB, KC_GRV)`, Hyper, Meh, both 2u thumb keys) lands at the
//! index this module assigns it. The fixture lives at
//! `examples/moonlander-default/pulled/revision.json`.

use super::{
    Geometry, GridLayout, GridRow, Hand, PhysicalKey, PhysicalLayout, ThumbCluster, ThumbKey,
    ThumbKeyWidth,
};

pub struct Moonlander;

impl Geometry for Moonlander {
    fn id(&self) -> &'static str {
        "moonlander"
    }

    fn display_name(&self) -> &'static str {
        "ZSA Moonlander Mark I"
    }

    fn matrix_key_count(&self) -> usize {
        72
    }

    fn encoder_count(&self) -> usize {
        0
    }

    fn position_to_index(&self, name: &str) -> Option<usize> {
        POSITION_TABLE
            .iter()
            .find(|(n, _)| *n == name)
            .map(|(_, i)| *i)
    }

    fn index_to_position(&self, index: usize) -> Option<&'static str> {
        POSITION_TABLE
            .iter()
            .find(|(_, i)| *i == index)
            .map(|(n, _)| *n)
    }

    fn matrix_to_index(&self, row: u8, col: u8) -> Option<usize> {
        MATRIX_TABLE
            .iter()
            .find(|(r, c, _)| *r == row && *c == col)
            .map(|(_, _, idx)| *idx)
    }

    fn ascii_layout(&self) -> &'static GridLayout {
        &GRID
    }

    fn physical_layout(&self) -> &'static PhysicalLayout {
        &PHYSICAL
    }

    fn qmk_keyboard(&self) -> &'static str {
        "zsa/moonlander"
    }

    fn layout_macro(&self) -> &'static str {
        // `keyboard.json` names the layout `LAYOUT` and declares
        // `LAYOUT_moonlander` as a `layout_aliases` entry; QMK generates
        // macros for both. We emit the alias because it is what the
        // wider Moonlander keymap ecosystem uses and it keeps the
        // generated keymap.c self-describing.
        "LAYOUT_moonlander"
    }

    fn qmk_arg_order(&self) -> &'static [usize] {
        &QMK_ARG_ORDER
    }

    fn hand(&self, index: usize) -> Option<Hand> {
        // Oryx serializes the keys[] array in this order:
        //   [0..32]  left half rows 0-4
        //   [32..36] left thumb cluster
        //   [36..68] right half rows 0-4
        //   [68..72] right thumb cluster
        if index < 36 {
            Some(Hand::Left)
        } else if index < 72 {
            Some(Hand::Right)
        } else {
            None
        }
    }

    fn usb_vendor_id(&self) -> &'static str {
        // ZSA Technology Labs USB vendor ID; matches the value in the
        // QMK fork's keyboards/zsa/moonlander/keyboard.json
        // (`usb.vid = 0x3297`, `usb.pid = 0x1969`).
        "0x3297"
    }

    fn flash_budget_bytes(&self) -> u64 {
        // The Moonlander's STM32F303CCT6 has 256KB of internal flash;
        // QMK's STM32F303xC link script exposes the full 256KB to the
        // application image (the stm32-dfu bootloader lives in system
        // memory, not application flash).
        256 * 1024
    }
}

// =============================================================================
// Moonlander position table
// =============================================================================
//
// The Moonlander has, per half: three 7-key rows, one 6-key row, one
// 5-key row (32 main-grid keys) plus a 4-key thumb cluster = 36 keys,
// 72 total.
//
// Left half (indices 0..32):
//   Row 0 (num):     0  1  2  3  4  5  6
//   Row 1 (top):     7  8  9 10 11 12 13
//   Row 2 (home):   14 15 16 17 18 19 20
//   Row 3 (bottom): 21 22 23 24 25 26        (no innermost key)
//   Row 4 (mod):    27 28 29 30 31           (outer..index only)
//
// Left thumb: 32 (big 2u), 33 34 35 (piano 1..3)
//
// Right half (indices 36..68) mirrors the left, serialized
// innermost→outer within each row:
//   Row 0:          36 37 38 39 40 41 42
//   Row 1:          43 44 45 46 47 48 49
//   Row 2:          50 51 52 53 54 55 56
//   Row 3:          57 58 59 60 61 62        (inner..outer)
//   Row 4:          63 64 65 66 67           (index..outer)
//
// Right thumb: 68 (big 2u), 69 70 71 (piano 3..1)
//
// Serialization pinned against the fixture
// `examples/moonlander-default/pulled/revision.json` (the stock
// "Moonlander Default Layout"): idx 21 = hold-Shift (KC_LSFT in the
// QMK keymap), idx 27 = `LT(SYMB, KC_GRV)`, idx 32 = the left 2u
// `LALT_T(KC_APP)`, idx 33 = KC_SPACE (piano 1), idx 68 = the right
// 2u `RCTL_T(KC_ESC)`, idx 71 = KC_ENTER (right piano 1).
#[rustfmt::skip]
const POSITION_TABLE: &[(&str, usize)] = &[
    // ── Left half ────────────────────────────────────────────────────────────
    // row 0 (number row)
    ("L_outer_num",        0),
    ("L_pinky_num",        1),
    ("L_ring_num",         2),
    ("L_middle_num",       3),
    ("L_index_num",        4),
    ("L_inner_num",        5),
    ("L_innermost_num",    6),

    // row 1 (top letter row)
    ("L_outer_top",        7),
    ("L_pinky_top",        8),
    ("L_ring_top",         9),
    ("L_middle_top",      10),
    ("L_index_top",       11),
    ("L_inner_top",       12),
    ("L_innermost_top",   13),

    // row 2 (home row)
    ("L_outer_home",      14),
    ("L_pinky_home",      15),
    ("L_ring_home",       16),
    ("L_middle_home",     17),
    ("L_index_home",      18),
    ("L_inner_home",      19),
    ("L_innermost_home",  20),

    // row 3 (bottom letter row — no innermost key)
    ("L_outer_bottom",    21),
    ("L_pinky_bottom",    22),
    ("L_ring_bottom",     23),
    ("L_middle_bottom",   24),
    ("L_index_bottom",    25),
    ("L_inner_bottom",    26),

    // row 4 (mod row — outer..index only)
    ("L_outer_mod",       27),
    ("L_pinky_mod",       28),
    ("L_ring_mod",        29),
    ("L_middle_mod",      30),
    ("L_index_mod",       31),

    // ── Left thumb cluster ──────────────────────────────────────────────────
    ("L_thumb_big",       32),
    ("L_thumb_piano_1",   33),
    ("L_thumb_piano_2",   34),
    ("L_thumb_piano_3",   35),

    // ── Right half (mirrored: innermost is leftmost, outer is rightmost) ────
    // row 0
    ("R_innermost_num",   36),
    ("R_inner_num",       37),
    ("R_index_num",       38),
    ("R_middle_num",      39),
    ("R_ring_num",        40),
    ("R_pinky_num",       41),
    ("R_outer_num",       42),

    // row 1
    ("R_innermost_top",   43),
    ("R_inner_top",       44),
    ("R_index_top",       45),
    ("R_middle_top",      46),
    ("R_ring_top",        47),
    ("R_pinky_top",       48),
    ("R_outer_top",       49),

    // row 2 (home row)
    ("R_innermost_home",  50),
    ("R_inner_home",      51),
    ("R_index_home",      52),
    ("R_middle_home",     53),
    ("R_ring_home",       54),
    ("R_pinky_home",      55),
    ("R_outer_home",      56),

    // row 3 (bottom letter row — no innermost key)
    ("R_inner_bottom",    57),
    ("R_index_bottom",    58),
    ("R_middle_bottom",   59),
    ("R_ring_bottom",     60),
    ("R_pinky_bottom",    61),
    ("R_outer_bottom",    62),

    // row 4 (mod row — index..outer only)
    ("R_index_mod",       63),
    ("R_middle_mod",      64),
    ("R_ring_mod",        65),
    ("R_pinky_mod",       66),
    ("R_outer_mod",       67),

    // ── Right thumb cluster ─────────────────────────────────────────────────
    ("R_thumb_big",       68),
    ("R_thumb_piano_3",   69),
    ("R_thumb_piano_2",   70),
    ("R_thumb_piano_1",   71),
];

// =============================================================================
// Electrical matrix → canonical index
// =============================================================================
//
// Verbatim transcription of the pinned ZSA fork's
// `keyboards/zsa/moonlander/keyboard.json` `layouts.LAYOUT.layout[]`:
// each entry's `matrix: [row, col]`, paired with the canonical index
// via `QMK_ARG_ORDER` below (the Moonlander's keyboard.json carries no
// `label` fields, so the pairing is established by cross-matching the
// fork's Oryx-exported `keymaps/oryx/keymap.c` against the same layout
// pulled from the Oryx GraphQL endpoint — see the module docs). The
// firmware raw HID `KEYDOWN` / `KEYUP` events carry the same
// (row, col) pair that QMK stores in `keyrecord_t.event.key`, so this
// table is the lookup every UI / stats consumer goes through.
//
// The Moonlander's scan matrix is 12 rows × 7 cols (rows 0-5 left,
// 6-11 right); only the entries below are populated (the rest are
// matrix holes).
#[rustfmt::skip]
const MATRIX_TABLE: &[(u8, u8, usize)] = &[
    // ── Left half rows ──────────────────────────────────────────────
    (0, 0,  0), (0, 1,  1), (0, 2,  2), (0, 3,  3), (0, 4,  4), (0, 5,  5), (0, 6,  6),
    (1, 0,  7), (1, 1,  8), (1, 2,  9), (1, 3, 10), (1, 4, 11), (1, 5, 12), (1, 6, 13),
    (2, 0, 14), (2, 1, 15), (2, 2, 16), (2, 3, 17), (2, 4, 18), (2, 5, 19), (2, 6, 20),
    (3, 0, 21), (3, 1, 22), (3, 2, 23), (3, 3, 24), (3, 4, 25), (3, 5, 26),
    (4, 0, 27), (4, 1, 28), (4, 2, 29), (4, 3, 30), (4, 4, 31),

    // ── Left thumb cluster (big key, then piano 1..3) ───────────────
    (5, 3, 32), (5, 0, 33), (5, 1, 34), (5, 2, 35),

    // ── Right half rows ─────────────────────────────────────────────
    (6, 0, 36), (6, 1, 37), (6, 2, 38), (6, 3, 39), (6, 4, 40), (6, 5, 41), (6, 6, 42),
    (7, 0, 43), (7, 1, 44), (7, 2, 45), (7, 3, 46), (7, 4, 47), (7, 5, 48), (7, 6, 49),
    (8, 0, 50), (8, 1, 51), (8, 2, 52), (8, 3, 53), (8, 4, 54), (8, 5, 55), (8, 6, 56),
    (9, 1, 57), (9, 2, 58), (9, 3, 59), (9, 4, 60), (9, 5, 61), (9, 6, 62),
    (10, 2, 63), (10, 3, 64), (10, 4, 65), (10, 5, 66), (10, 6, 67),

    // ── Right thumb cluster (big key, then piano 3..1) ──────────────
    (11, 3, 68), (11, 4, 69), (11, 5, 70), (11, 6, 71),
];

// =============================================================================
// QMK LAYOUT_moonlander argument order
// =============================================================================
//
// Maps each QMK LAYOUT positional argument index (0..72) to the
// corresponding Oryx canonical-layout key index. Derived from
// `keyboards/zsa/moonlander/keyboard.json` `layouts.LAYOUT.layout[]`
// in the pinned ZSA QMK fork, cross-checked against the fork's
// Oryx-exported `keymaps/oryx/keymap.c` and the Oryx GraphQL response
// for the same layout (see module docs).
//
// Pattern: QMK interleaves left/right per row, then emits the two 2u
// thumb keys, the right mod row, and finally both piano triples:
//   QMK   0..7   = L row 0        (canonical  0..7)
//   QMK   7..14  = R row 0        (canonical 36..43)
//   QMK  14..21  = L row 1        (canonical  7..14)
//   QMK  21..28  = R row 1        (canonical 43..50)
//   QMK  28..35  = L row 2        (canonical 14..21)
//   QMK  35..42  = R row 2        (canonical 50..57)
//   QMK  42..48  = L row 3        (canonical 21..27)
//   QMK  48..54  = R row 3        (canonical 57..63)
//   QMK  54..59  = L row 4 (mod)  (canonical 27..32)
//   QMK  59      = L thumb big    (canonical 32)
//   QMK  60      = R thumb big    (canonical 68)
//   QMK  61..66  = R row 4 (mod)  (canonical 63..68)
//   QMK  66..69  = L piano 1..3   (canonical 33..36)
//   QMK  69..72  = R piano 3..1   (canonical 69..72)
//
// **If this permutation is wrong, the firmware is physically
// scrambled.** The `qmk_arg_order_is_a_complete_permutation` and
// fixture-pinning tests below, plus the structural round-trip test in
// `tests/codegen_roundtrip.rs`, guard it.
#[rustfmt::skip]
const QMK_ARG_ORDER: [usize; 72] = [
    // L row 0
    0, 1, 2, 3, 4, 5, 6,
    // R row 0
    36, 37, 38, 39, 40, 41, 42,
    // L row 1
    7, 8, 9, 10, 11, 12, 13,
    // R row 1
    43, 44, 45, 46, 47, 48, 49,
    // L row 2
    14, 15, 16, 17, 18, 19, 20,
    // R row 2
    50, 51, 52, 53, 54, 55, 56,
    // L row 3
    21, 22, 23, 24, 25, 26,
    // R row 3
    57, 58, 59, 60, 61, 62,
    // L row 4 (mod)
    27, 28, 29, 30, 31,
    // L thumb big, R thumb big
    32, 68,
    // R row 4 (mod)
    63, 64, 65, 66, 67,
    // L piano 1..3
    33, 34, 35,
    // R piano 3..1
    69, 70, 71,
];

// =============================================================================
// ASCII grid
// =============================================================================
//
// Rows 3 and 4 are shorter than the full 7 columns; `None` entries pad
// them so every column stays visually aligned: row 3 is missing the
// innermost key (padded on the split-gap side), row 4 is missing both
// inner columns.

#[rustfmt::skip]
const ROW0_L: &[Option<usize>] = &[Some(0), Some(1), Some(2), Some(3), Some(4), Some(5), Some(6)];
#[rustfmt::skip]
const ROW0_R: &[Option<usize>] = &[Some(36), Some(37), Some(38), Some(39), Some(40), Some(41), Some(42)];

#[rustfmt::skip]
const ROW1_L: &[Option<usize>] = &[Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13)];
#[rustfmt::skip]
const ROW1_R: &[Option<usize>] = &[Some(43), Some(44), Some(45), Some(46), Some(47), Some(48), Some(49)];

#[rustfmt::skip]
const ROW2_L: &[Option<usize>] = &[Some(14), Some(15), Some(16), Some(17), Some(18), Some(19), Some(20)];
#[rustfmt::skip]
const ROW2_R: &[Option<usize>] = &[Some(50), Some(51), Some(52), Some(53), Some(54), Some(55), Some(56)];

#[rustfmt::skip]
const ROW3_L: &[Option<usize>] = &[Some(21), Some(22), Some(23), Some(24), Some(25), Some(26), None];
#[rustfmt::skip]
const ROW3_R: &[Option<usize>] = &[None, Some(57), Some(58), Some(59), Some(60), Some(61), Some(62)];

#[rustfmt::skip]
const ROW4_L: &[Option<usize>] = &[Some(27), Some(28), Some(29), Some(30), Some(31), None, None];
#[rustfmt::skip]
const ROW4_R: &[Option<usize>] = &[None, None, Some(63), Some(64), Some(65), Some(66), Some(67)];

const ROWS: &[GridRow] = &[
    GridRow {
        left: ROW0_L,
        right: ROW0_R,
    },
    GridRow {
        left: ROW1_L,
        right: ROW1_R,
    },
    GridRow {
        left: ROW2_L,
        right: ROW2_R,
    },
    GridRow {
        left: ROW3_L,
        right: ROW3_R,
    },
    GridRow {
        left: ROW4_L,
        right: ROW4_R,
    },
];

// Thumb clusters are listed big-key-first, then piano keys walking
// outward to the tip of the cluster. The ASCII renderer draws the
// right cluster in list order and mirrors the left one, so this
// ordering puts each 2u key adjacent to the split gap and fans the
// piano keys outward on both halves — the same split-gap symmetry the
// Voyager clusters render with.
const LEFT_THUMB: &[ThumbKey] = &[
    ThumbKey {
        index: 32,
        width: ThumbKeyWidth::Wide,
    },
    ThumbKey {
        index: 33,
        width: ThumbKeyWidth::Standard,
    },
    ThumbKey {
        index: 34,
        width: ThumbKeyWidth::Standard,
    },
    ThumbKey {
        index: 35,
        width: ThumbKeyWidth::Standard,
    },
];
const RIGHT_THUMB: &[ThumbKey] = &[
    ThumbKey {
        index: 68,
        width: ThumbKeyWidth::Wide,
    },
    ThumbKey {
        index: 71,
        width: ThumbKeyWidth::Standard,
    },
    ThumbKey {
        index: 70,
        width: ThumbKeyWidth::Standard,
    },
    ThumbKey {
        index: 69,
        width: ThumbKeyWidth::Standard,
    },
];

const THUMBS: &[ThumbCluster] = &[
    ThumbCluster {
        hand: Hand::Left,
        keys: LEFT_THUMB,
    },
    ThumbCluster {
        hand: Hand::Right,
        keys: RIGHT_THUMB,
    },
];

const GRID: GridLayout = GridLayout {
    halves: 2,
    rows: ROWS,
    thumb_clusters: THUMBS,
};

// =============================================================================
// Physical layout (pixel-accurate GUI)
// =============================================================================
//
// Per-key (x, y, w, h) values are transcribed verbatim from
// `keyboards/zsa/moonlander/keyboard.json` `layouts.LAYOUT.layout[]` —
// the same array we used to build `MATRIX_TABLE` above. The JSON does
// **not** include thumb-cluster rotation (same cosmetic omission as the
// Voyager's), so we add it here: each 4-key cluster rotates outward
// around the top corner of its 2u key nearest the main grid, matching
// the angle ZSA's own renders of the Moonlander use.

const THUMB_ROT_DEG: f32 = 30.0;

#[rustfmt::skip]
const PHYSICAL_KEYS: &[PhysicalKey] = &[
    // ── Left half, rows 0..4 ────────────────────────────────────────
    pk(0,  0.0, 0.375), pk(1,  1.0, 0.375), pk(2,  2.0, 0.125), pk(3,  3.0, 0.000), pk(4,  4.0, 0.125), pk(5,  5.0, 0.250), pk(6,  6.0, 0.250),
    pk(7,  0.0, 1.375), pk(8,  1.0, 1.375), pk(9,  2.0, 1.125), pk(10, 3.0, 1.000), pk(11, 4.0, 1.125), pk(12, 5.0, 1.250), pk(13, 6.0, 1.250),
    pk(14, 0.0, 2.375), pk(15, 1.0, 2.375), pk(16, 2.0, 2.125), pk(17, 3.0, 2.000), pk(18, 4.0, 2.125), pk(19, 5.0, 2.250), pk(20, 6.0, 2.250),
    pk(21, 0.0, 3.375), pk(22, 1.0, 3.375), pk(23, 2.0, 3.125), pk(24, 3.0, 3.000), pk(25, 4.0, 3.125), pk(26, 5.0, 3.250),
    pk(27, 0.0, 4.375), pk(28, 1.0, 4.375), pk(29, 2.0, 4.125), pk(30, 3.0, 4.000), pk(31, 4.0, 4.125),

    // ── Left thumb cluster (rotated outward around (5, 4.5)) ────────
    pk_big(32,   5.0, 4.5,  THUMB_ROT_DEG, 5.0, 4.5),
    pk_piano(33, 5.0, 5.5,  THUMB_ROT_DEG, 5.0, 4.5),
    pk_piano(34, 6.0, 5.5,  THUMB_ROT_DEG, 5.0, 4.5),
    pk_piano(35, 7.0, 5.5,  THUMB_ROT_DEG, 5.0, 4.5),

    // ── Right half, rows 0..4 ───────────────────────────────────────
    pk(36, 10.0, 0.250), pk(37, 11.0, 0.250), pk(38, 12.0, 0.125), pk(39, 13.0, 0.000), pk(40, 14.0, 0.125), pk(41, 15.0, 0.375), pk(42, 16.0, 0.375),
    pk(43, 10.0, 1.250), pk(44, 11.0, 1.250), pk(45, 12.0, 1.125), pk(46, 13.0, 1.000), pk(47, 14.0, 1.125), pk(48, 15.0, 1.375), pk(49, 16.0, 1.375),
    pk(50, 10.0, 2.250), pk(51, 11.0, 2.250), pk(52, 12.0, 2.125), pk(53, 13.0, 2.000), pk(54, 14.0, 2.125), pk(55, 15.0, 2.375), pk(56, 16.0, 2.375),
    pk(57, 11.0, 3.250), pk(58, 12.0, 3.125), pk(59, 13.0, 3.000), pk(60, 14.0, 3.125), pk(61, 15.0, 3.375), pk(62, 16.0, 3.375),
    pk(63, 12.0, 4.125), pk(64, 13.0, 4.000), pk(65, 14.0, 4.125), pk(66, 15.0, 4.375), pk(67, 16.0, 4.375),

    // ── Right thumb cluster (rotated outward around (12, 4.5)) ──────
    pk_big(68,   10.0, 4.5, -THUMB_ROT_DEG, 12.0, 4.5),
    pk_piano(69,  9.0, 5.5, -THUMB_ROT_DEG, 12.0, 4.5),
    pk_piano(70, 10.0, 5.5, -THUMB_ROT_DEG, 12.0, 4.5),
    pk_piano(71, 11.0, 5.5, -THUMB_ROT_DEG, 12.0, 4.5),
];

const fn pk(index: usize, x: f32, y: f32) -> PhysicalKey {
    PhysicalKey {
        index,
        x,
        y,
        w: 1.0,
        h: 1.0,
        rot_deg: 0.0,
        rot_origin_x: 0.0,
        rot_origin_y: 0.0,
    }
}

/// A 2u-wide thumb "big" key (the red key), rotated with its cluster.
const fn pk_big(index: usize, x: f32, y: f32, rot_deg: f32, rx: f32, ry: f32) -> PhysicalKey {
    PhysicalKey {
        index,
        x,
        y,
        w: 2.0,
        h: 1.0,
        rot_deg,
        rot_origin_x: rx,
        rot_origin_y: ry,
    }
}

/// A 1u × 1.5u thumb piano key, rotated with its cluster.
const fn pk_piano(index: usize, x: f32, y: f32, rot_deg: f32, rx: f32, ry: f32) -> PhysicalKey {
    PhysicalKey {
        index,
        x,
        y,
        w: 1.0,
        h: 1.5,
        rot_deg,
        rot_origin_x: rx,
        rot_origin_y: ry,
    }
}

// The bbox is generous by ~1.5u below the piano keys so the rotated
// clusters don't clip the viewport — a rotation of 30° around (5, 4.5)
// pushes the far corner of the piano triple ((8, 7) unrotated) down to
// ≈y 8.2.
const PHYSICAL: PhysicalLayout = PhysicalLayout {
    keys: PHYSICAL_KEYS,
    width: 17.0,
    height: 8.5,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn has_72_matrix_keys() {
        assert_eq!(Moonlander.matrix_key_count(), 72);
        assert_eq!(POSITION_TABLE.len(), 72);
    }

    #[test]
    fn every_position_in_range_and_unique() {
        let mut seen = [false; 72];
        for (name, idx) in POSITION_TABLE.iter() {
            assert!(
                *idx < Moonlander.matrix_key_count(),
                "position {name} has out-of-range index {idx}"
            );
            assert!(!seen[*idx], "index {idx} appears twice in position table");
            seen[*idx] = true;
        }
        assert!(seen.iter().all(|&b| b), "position table misses an index");
    }

    #[test]
    fn position_name_round_trip() {
        let samples = [
            "L_pinky_home",
            "L_innermost_num",
            "L_thumb_big",
            "L_thumb_piano_1",
            "R_innermost_top",
            "R_outer_mod",
            "R_thumb_big",
            "R_thumb_piano_3",
        ];
        for name in samples {
            let idx = Moonlander.position_to_index(name).expect(name);
            let back = Moonlander.index_to_position(idx).expect("idx in table");
            assert_eq!(back, name);
        }
    }

    #[test]
    fn hand_classification() {
        // Left half rows
        assert_eq!(Moonlander.hand(0), Some(Hand::Left));
        assert_eq!(Moonlander.hand(31), Some(Hand::Left));
        // Left thumb cluster
        assert_eq!(Moonlander.hand(32), Some(Hand::Left));
        assert_eq!(Moonlander.hand(35), Some(Hand::Left));
        // Right half rows
        assert_eq!(Moonlander.hand(36), Some(Hand::Right));
        assert_eq!(Moonlander.hand(67), Some(Hand::Right));
        // Right thumb cluster
        assert_eq!(Moonlander.hand(68), Some(Hand::Right));
        assert_eq!(Moonlander.hand(71), Some(Hand::Right));
        // Out of range
        assert_eq!(Moonlander.hand(72), None);
    }

    #[test]
    fn qmk_arg_order_is_a_complete_permutation() {
        // Every canonical index 0..72 must appear exactly once in the
        // QMK arg order, otherwise the codegen permutation drops or
        // duplicates a key.
        let mut seen = [false; 72];
        for &idx in QMK_ARG_ORDER.iter() {
            assert!(idx < 72, "qmk arg index {idx} out of range");
            assert!(!seen[idx], "canonical index {idx} appears twice");
            seen[idx] = true;
        }
        assert!(
            seen.iter().all(|&b| b),
            "qmk arg order is missing some canonical index"
        );
        assert_eq!(QMK_ARG_ORDER.len(), 72);
        assert_eq!(QMK_ARG_ORDER.len(), Moonlander.matrix_key_count());
    }

    #[test]
    fn qmk_arg_order_pins_known_positions() {
        // Every pin below is verified against the pinned ZSA fork's
        // keymaps/oryx/keymap.c ↔ Oryx GraphQL cross-match (module docs).
        // L row 0 starts the array (canonical 0..7 → QMK 0..7).
        assert_eq!(QMK_ARG_ORDER[0], 0);
        assert_eq!(QMK_ARG_ORDER[6], 6);
        // R row 0 follows immediately (QMK 7..14 = canonical 36..43).
        assert_eq!(QMK_ARG_ORDER[7], 36);
        assert_eq!(QMK_ARG_ORDER[13], 42);
        // L row 3 starts at QMK 42 (KC_LSFT in the oryx keymap).
        assert_eq!(QMK_ARG_ORDER[42], 21);
        // L row 4 starts at QMK 54 (LT(SYMB, KC_GRV) in the oryx keymap).
        assert_eq!(QMK_ARG_ORDER[54], 27);
        // The two 2u thumb keys (LALT_T(KC_APP) / RCTL_T(KC_ESC)).
        assert_eq!(QMK_ARG_ORDER[59], 32);
        assert_eq!(QMK_ARG_ORDER[60], 68);
        // R row 4 (QMK 61..66 = canonical 63..68).
        assert_eq!(QMK_ARG_ORDER[61], 63);
        assert_eq!(QMK_ARG_ORDER[65], 67);
        // L piano keys (KC_SPC, KC_BSPC, KC_LGUI in the oryx keymap).
        assert_eq!(QMK_ARG_ORDER[66], 33);
        assert_eq!(QMK_ARG_ORDER[68], 35);
        // R piano keys end the array (KC_LALT, KC_TAB, KC_ENT).
        assert_eq!(QMK_ARG_ORDER[69], 69);
        assert_eq!(QMK_ARG_ORDER[71], 71);
    }

    #[test]
    fn matrix_table_covers_every_canonical_index() {
        // The firmware HID KEYDOWN path depends on this being a total
        // map: every canonical index 0..72 must be reachable from
        // exactly one (row, col) pair, otherwise a press on the
        // unmapped key silently goes un-highlighted.
        let mut seen = [false; 72];
        for (_, _, idx) in MATRIX_TABLE.iter() {
            assert!(*idx < 72, "matrix entry has out-of-range index {idx}");
            assert!(!seen[*idx], "canonical index {idx} appears twice");
            seen[*idx] = true;
        }
        assert!(
            seen.iter().all(|&b| b),
            "matrix table missing some canonical index"
        );
        assert_eq!(MATRIX_TABLE.len(), Moonlander.matrix_key_count());
    }

    #[test]
    fn matrix_to_index_pins_known_coords() {
        // keyboard.json: top-left key on the left half is matrix [0,0].
        assert_eq!(Moonlander.matrix_to_index(0, 0), Some(0));
        // Thumb clusters — big keys sit at col 3 of their thumb row.
        assert_eq!(Moonlander.matrix_to_index(5, 3), Some(32));
        assert_eq!(Moonlander.matrix_to_index(5, 0), Some(33));
        assert_eq!(Moonlander.matrix_to_index(11, 3), Some(68));
        assert_eq!(Moonlander.matrix_to_index(11, 6), Some(71));
        // Matrix holes — row 3 has no col-6 key on the left, row 9 has
        // no col-0 key on the right; the thumb rows skip cols too.
        assert_eq!(Moonlander.matrix_to_index(3, 6), None);
        assert_eq!(Moonlander.matrix_to_index(9, 0), None);
        assert_eq!(Moonlander.matrix_to_index(5, 4), None);
        assert_eq!(Moonlander.matrix_to_index(11, 0), None);
        // Out-of-matrix coordinates.
        assert_eq!(Moonlander.matrix_to_index(12, 0), None);
        assert_eq!(Moonlander.matrix_to_index(0, 7), None);
    }

    #[test]
    fn physical_layout_covers_every_canonical_index() {
        let mut seen = [false; 72];
        for k in PHYSICAL_KEYS.iter() {
            assert!(k.index < 72);
            assert!(!seen[k.index], "index {} appears twice", k.index);
            seen[k.index] = true;
        }
        assert!(seen.iter().all(|&b| b));
        assert_eq!(PHYSICAL_KEYS.len(), 72);
    }

    #[test]
    fn ascii_grid_covers_every_canonical_index() {
        let mut seen = [false; 72];
        let mut mark = |idx: usize| {
            assert!(idx < 72);
            assert!(!seen[idx], "index {idx} appears twice in grid");
            seen[idx] = true;
        };
        for row in GRID.rows {
            for cell in row.left.iter().chain(row.right.iter()).flatten() {
                mark(*cell);
            }
        }
        for cluster in GRID.thumb_clusters {
            for key in cluster.keys {
                mark(key.index);
            }
        }
        assert!(seen.iter().all(|&b| b), "grid misses a canonical index");
    }

    /// Pin the position table against the real Oryx fixture so a future
    /// off-by-N doesn't sneak past the snapshot tests.
    #[test]
    fn matches_oryx_serialization_order_in_fixture() {
        // From examples/moonlander-default/pulled/revision.json layer 0:
        // idx 0=KC_EQUAL, idx 6=KC_LEFT (innermost num), idx 21=hold
        // LEFT_SHIFT (bottom-row outer), idx 27=KC_GRAVE/MO (mod-row
        // outer), idx 32=KC_APPLICATION/LEFT_ALT (left 2u), idx
        // 33=KC_SPACE (left piano 1), idx 68=KC_ESCAPE/LEFT_CTRL (right
        // 2u), idx 71=KC_ENTER (right piano 1).
        assert_eq!(Moonlander.index_to_position(0), Some("L_outer_num"));
        assert_eq!(Moonlander.index_to_position(6), Some("L_innermost_num"));
        assert_eq!(Moonlander.index_to_position(21), Some("L_outer_bottom"));
        assert_eq!(Moonlander.index_to_position(27), Some("L_outer_mod"));
        assert_eq!(Moonlander.index_to_position(32), Some("L_thumb_big"));
        assert_eq!(Moonlander.index_to_position(33), Some("L_thumb_piano_1"));
        assert_eq!(Moonlander.index_to_position(36), Some("R_innermost_num"));
        assert_eq!(Moonlander.index_to_position(62), Some("R_outer_bottom"));
        assert_eq!(Moonlander.index_to_position(68), Some("R_thumb_big"));
        assert_eq!(Moonlander.index_to_position(71), Some("R_thumb_piano_1"));
    }
}
