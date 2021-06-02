// DO NOT EDIT THIS FILE. IT WAS AUTOMATICALLY GENERATED BY:
//
//   yeslogic-ucd-generate canonical-combining-class --rust-enum ../../ucd-13.0.0
//
// Unicode version: 13.0.0.
//
// ucd-generate 0.4.2 is available on crates.io.

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum CanonicalCombiningClass {
    NotReordered = 0,
    Overlay = 1,
    HanReading = 6,
    Nukta = 7,
    KanaVoicing = 8,
    Virama = 9,
    CCC10 = 10,
    CCC11 = 11,
    CCC12 = 12,
    CCC13 = 13,
    CCC14 = 14,
    CCC15 = 15,
    CCC16 = 16,
    CCC17 = 17,
    CCC18 = 18,
    CCC19 = 19,
    CCC20 = 20,
    CCC21 = 21,
    CCC22 = 22,
    CCC23 = 23,
    CCC24 = 24,
    CCC25 = 25,
    CCC26 = 26,
    CCC27 = 27,
    CCC28 = 28,
    CCC29 = 29,
    CCC30 = 30,
    CCC31 = 31,
    CCC32 = 32,
    CCC33 = 33,
    CCC34 = 34,
    CCC35 = 35,
    CCC36 = 36,
    CCC84 = 84,
    CCC91 = 91,
    CCC103 = 103,
    CCC107 = 107,
    CCC118 = 118,
    CCC122 = 122,
    CCC129 = 129,
    CCC130 = 130,
    CCC132 = 132,
    AttachedBelow = 202,
    AttachedAbove = 214,
    AttachedAboveRight = 216,
    BelowLeft = 218,
    Below = 220,
    BelowRight = 222,
    Left = 224,
    Right = 226,
    AboveLeft = 228,
    Above = 230,
    AboveRight = 232,
    DoubleBelow = 233,
    DoubleAbove = 234,
    IotaSubscript = 240,
}

#[allow(dead_code)]
pub const CANONICAL_COMBINING_CLASS: &'static [(u32, u32, CanonicalCombiningClass)] = &[
    (0, 767, CanonicalCombiningClass::NotReordered),
    (768, 788, CanonicalCombiningClass::Above),
    (789, 789, CanonicalCombiningClass::AboveRight),
    (790, 793, CanonicalCombiningClass::Below),
    (794, 794, CanonicalCombiningClass::AboveRight),
    (795, 795, CanonicalCombiningClass::AttachedAboveRight),
    (796, 800, CanonicalCombiningClass::Below),
    (801, 802, CanonicalCombiningClass::AttachedBelow),
    (803, 806, CanonicalCombiningClass::Below),
    (807, 808, CanonicalCombiningClass::AttachedBelow),
    (809, 819, CanonicalCombiningClass::Below),
    (820, 824, CanonicalCombiningClass::Overlay),
    (825, 828, CanonicalCombiningClass::Below),
    (829, 836, CanonicalCombiningClass::Above),
    (837, 837, CanonicalCombiningClass::IotaSubscript),
    (838, 838, CanonicalCombiningClass::Above),
    (839, 841, CanonicalCombiningClass::Below),
    (842, 844, CanonicalCombiningClass::Above),
    (845, 846, CanonicalCombiningClass::Below),
    (847, 847, CanonicalCombiningClass::NotReordered),
    (848, 850, CanonicalCombiningClass::Above),
    (851, 854, CanonicalCombiningClass::Below),
    (855, 855, CanonicalCombiningClass::Above),
    (856, 856, CanonicalCombiningClass::AboveRight),
    (857, 858, CanonicalCombiningClass::Below),
    (859, 859, CanonicalCombiningClass::Above),
    (860, 860, CanonicalCombiningClass::DoubleBelow),
    (861, 862, CanonicalCombiningClass::DoubleAbove),
    (863, 863, CanonicalCombiningClass::DoubleBelow),
    (864, 865, CanonicalCombiningClass::DoubleAbove),
    (866, 866, CanonicalCombiningClass::DoubleBelow),
    (867, 879, CanonicalCombiningClass::Above),
    (880, 1154, CanonicalCombiningClass::NotReordered),
    (1155, 1159, CanonicalCombiningClass::Above),
    (1160, 1424, CanonicalCombiningClass::NotReordered),
    (1425, 1425, CanonicalCombiningClass::Below),
    (1426, 1429, CanonicalCombiningClass::Above),
    (1430, 1430, CanonicalCombiningClass::Below),
    (1431, 1433, CanonicalCombiningClass::Above),
    (1434, 1434, CanonicalCombiningClass::BelowRight),
    (1435, 1435, CanonicalCombiningClass::Below),
    (1436, 1441, CanonicalCombiningClass::Above),
    (1442, 1447, CanonicalCombiningClass::Below),
    (1448, 1449, CanonicalCombiningClass::Above),
    (1450, 1450, CanonicalCombiningClass::Below),
    (1451, 1452, CanonicalCombiningClass::Above),
    (1453, 1453, CanonicalCombiningClass::BelowRight),
    (1454, 1454, CanonicalCombiningClass::AboveLeft),
    (1455, 1455, CanonicalCombiningClass::Above),
    (1456, 1456, CanonicalCombiningClass::CCC10),
    (1457, 1457, CanonicalCombiningClass::CCC11),
    (1458, 1458, CanonicalCombiningClass::CCC12),
    (1459, 1459, CanonicalCombiningClass::CCC13),
    (1460, 1460, CanonicalCombiningClass::CCC14),
    (1461, 1461, CanonicalCombiningClass::CCC15),
    (1462, 1462, CanonicalCombiningClass::CCC16),
    (1463, 1463, CanonicalCombiningClass::CCC17),
    (1464, 1464, CanonicalCombiningClass::CCC18),
    (1465, 1466, CanonicalCombiningClass::CCC19),
    (1467, 1467, CanonicalCombiningClass::CCC20),
    (1468, 1468, CanonicalCombiningClass::CCC21),
    (1469, 1469, CanonicalCombiningClass::CCC22),
    (1470, 1470, CanonicalCombiningClass::NotReordered),
    (1471, 1471, CanonicalCombiningClass::CCC23),
    (1472, 1472, CanonicalCombiningClass::NotReordered),
    (1473, 1473, CanonicalCombiningClass::CCC24),
    (1474, 1474, CanonicalCombiningClass::CCC25),
    (1475, 1475, CanonicalCombiningClass::NotReordered),
    (1476, 1476, CanonicalCombiningClass::Above),
    (1477, 1477, CanonicalCombiningClass::Below),
    (1478, 1478, CanonicalCombiningClass::NotReordered),
    (1479, 1479, CanonicalCombiningClass::CCC18),
    (1480, 1551, CanonicalCombiningClass::NotReordered),
    (1552, 1559, CanonicalCombiningClass::Above),
    (1560, 1560, CanonicalCombiningClass::CCC30),
    (1561, 1561, CanonicalCombiningClass::CCC31),
    (1562, 1562, CanonicalCombiningClass::CCC32),
    (1563, 1610, CanonicalCombiningClass::NotReordered),
    (1611, 1611, CanonicalCombiningClass::CCC27),
    (1612, 1612, CanonicalCombiningClass::CCC28),
    (1613, 1613, CanonicalCombiningClass::CCC29),
    (1614, 1614, CanonicalCombiningClass::CCC30),
    (1615, 1615, CanonicalCombiningClass::CCC31),
    (1616, 1616, CanonicalCombiningClass::CCC32),
    (1617, 1617, CanonicalCombiningClass::CCC33),
    (1618, 1618, CanonicalCombiningClass::CCC34),
    (1619, 1620, CanonicalCombiningClass::Above),
    (1621, 1622, CanonicalCombiningClass::Below),
    (1623, 1627, CanonicalCombiningClass::Above),
    (1628, 1628, CanonicalCombiningClass::Below),
    (1629, 1630, CanonicalCombiningClass::Above),
    (1631, 1631, CanonicalCombiningClass::Below),
    (1632, 1647, CanonicalCombiningClass::NotReordered),
    (1648, 1648, CanonicalCombiningClass::CCC35),
    (1649, 1749, CanonicalCombiningClass::NotReordered),
    (1750, 1756, CanonicalCombiningClass::Above),
    (1757, 1758, CanonicalCombiningClass::NotReordered),
    (1759, 1762, CanonicalCombiningClass::Above),
    (1763, 1763, CanonicalCombiningClass::Below),
    (1764, 1764, CanonicalCombiningClass::Above),
    (1765, 1766, CanonicalCombiningClass::NotReordered),
    (1767, 1768, CanonicalCombiningClass::Above),
    (1769, 1769, CanonicalCombiningClass::NotReordered),
    (1770, 1770, CanonicalCombiningClass::Below),
    (1771, 1772, CanonicalCombiningClass::Above),
    (1773, 1773, CanonicalCombiningClass::Below),
    (1774, 1808, CanonicalCombiningClass::NotReordered),
    (1809, 1809, CanonicalCombiningClass::CCC36),
    (1810, 1839, CanonicalCombiningClass::NotReordered),
    (1840, 1840, CanonicalCombiningClass::Above),
    (1841, 1841, CanonicalCombiningClass::Below),
    (1842, 1843, CanonicalCombiningClass::Above),
    (1844, 1844, CanonicalCombiningClass::Below),
    (1845, 1846, CanonicalCombiningClass::Above),
    (1847, 1849, CanonicalCombiningClass::Below),
    (1850, 1850, CanonicalCombiningClass::Above),
    (1851, 1852, CanonicalCombiningClass::Below),
    (1853, 1853, CanonicalCombiningClass::Above),
    (1854, 1854, CanonicalCombiningClass::Below),
    (1855, 1857, CanonicalCombiningClass::Above),
    (1858, 1858, CanonicalCombiningClass::Below),
    (1859, 1859, CanonicalCombiningClass::Above),
    (1860, 1860, CanonicalCombiningClass::Below),
    (1861, 1861, CanonicalCombiningClass::Above),
    (1862, 1862, CanonicalCombiningClass::Below),
    (1863, 1863, CanonicalCombiningClass::Above),
    (1864, 1864, CanonicalCombiningClass::Below),
    (1865, 1866, CanonicalCombiningClass::Above),
    (1867, 2026, CanonicalCombiningClass::NotReordered),
    (2027, 2033, CanonicalCombiningClass::Above),
    (2034, 2034, CanonicalCombiningClass::Below),
    (2035, 2035, CanonicalCombiningClass::Above),
    (2036, 2044, CanonicalCombiningClass::NotReordered),
    (2045, 2045, CanonicalCombiningClass::Below),
    (2046, 2069, CanonicalCombiningClass::NotReordered),
    (2070, 2073, CanonicalCombiningClass::Above),
    (2074, 2074, CanonicalCombiningClass::NotReordered),
    (2075, 2083, CanonicalCombiningClass::Above),
    (2084, 2084, CanonicalCombiningClass::NotReordered),
    (2085, 2087, CanonicalCombiningClass::Above),
    (2088, 2088, CanonicalCombiningClass::NotReordered),
    (2089, 2093, CanonicalCombiningClass::Above),
    (2094, 2136, CanonicalCombiningClass::NotReordered),
    (2137, 2139, CanonicalCombiningClass::Below),
    (2140, 2258, CanonicalCombiningClass::NotReordered),
    (2259, 2259, CanonicalCombiningClass::Below),
    (2260, 2273, CanonicalCombiningClass::Above),
    (2274, 2274, CanonicalCombiningClass::NotReordered),
    (2275, 2275, CanonicalCombiningClass::Below),
    (2276, 2277, CanonicalCombiningClass::Above),
    (2278, 2278, CanonicalCombiningClass::Below),
    (2279, 2280, CanonicalCombiningClass::Above),
    (2281, 2281, CanonicalCombiningClass::Below),
    (2282, 2284, CanonicalCombiningClass::Above),
    (2285, 2287, CanonicalCombiningClass::Below),
    (2288, 2288, CanonicalCombiningClass::CCC27),
    (2289, 2289, CanonicalCombiningClass::CCC28),
    (2290, 2290, CanonicalCombiningClass::CCC29),
    (2291, 2293, CanonicalCombiningClass::Above),
    (2294, 2294, CanonicalCombiningClass::Below),
    (2295, 2296, CanonicalCombiningClass::Above),
    (2297, 2298, CanonicalCombiningClass::Below),
    (2299, 2303, CanonicalCombiningClass::Above),
    (2304, 2363, CanonicalCombiningClass::NotReordered),
    (2364, 2364, CanonicalCombiningClass::Nukta),
    (2365, 2380, CanonicalCombiningClass::NotReordered),
    (2381, 2381, CanonicalCombiningClass::Virama),
    (2382, 2384, CanonicalCombiningClass::NotReordered),
    (2385, 2385, CanonicalCombiningClass::Above),
    (2386, 2386, CanonicalCombiningClass::Below),
    (2387, 2388, CanonicalCombiningClass::Above),
    (2389, 2491, CanonicalCombiningClass::NotReordered),
    (2492, 2492, CanonicalCombiningClass::Nukta),
    (2493, 2508, CanonicalCombiningClass::NotReordered),
    (2509, 2509, CanonicalCombiningClass::Virama),
    (2510, 2557, CanonicalCombiningClass::NotReordered),
    (2558, 2558, CanonicalCombiningClass::Above),
    (2559, 2619, CanonicalCombiningClass::NotReordered),
    (2620, 2620, CanonicalCombiningClass::Nukta),
    (2621, 2636, CanonicalCombiningClass::NotReordered),
    (2637, 2637, CanonicalCombiningClass::Virama),
    (2638, 2747, CanonicalCombiningClass::NotReordered),
    (2748, 2748, CanonicalCombiningClass::Nukta),
    (2749, 2764, CanonicalCombiningClass::NotReordered),
    (2765, 2765, CanonicalCombiningClass::Virama),
    (2766, 2875, CanonicalCombiningClass::NotReordered),
    (2876, 2876, CanonicalCombiningClass::Nukta),
    (2877, 2892, CanonicalCombiningClass::NotReordered),
    (2893, 2893, CanonicalCombiningClass::Virama),
    (2894, 3020, CanonicalCombiningClass::NotReordered),
    (3021, 3021, CanonicalCombiningClass::Virama),
    (3022, 3148, CanonicalCombiningClass::NotReordered),
    (3149, 3149, CanonicalCombiningClass::Virama),
    (3150, 3156, CanonicalCombiningClass::NotReordered),
    (3157, 3157, CanonicalCombiningClass::CCC84),
    (3158, 3158, CanonicalCombiningClass::CCC91),
    (3159, 3259, CanonicalCombiningClass::NotReordered),
    (3260, 3260, CanonicalCombiningClass::Nukta),
    (3261, 3276, CanonicalCombiningClass::NotReordered),
    (3277, 3277, CanonicalCombiningClass::Virama),
    (3278, 3386, CanonicalCombiningClass::NotReordered),
    (3387, 3388, CanonicalCombiningClass::Virama),
    (3389, 3404, CanonicalCombiningClass::NotReordered),
    (3405, 3405, CanonicalCombiningClass::Virama),
    (3406, 3529, CanonicalCombiningClass::NotReordered),
    (3530, 3530, CanonicalCombiningClass::Virama),
    (3531, 3639, CanonicalCombiningClass::NotReordered),
    (3640, 3641, CanonicalCombiningClass::CCC103),
    (3642, 3642, CanonicalCombiningClass::Virama),
    (3643, 3655, CanonicalCombiningClass::NotReordered),
    (3656, 3659, CanonicalCombiningClass::CCC107),
    (3660, 3767, CanonicalCombiningClass::NotReordered),
    (3768, 3769, CanonicalCombiningClass::CCC118),
    (3770, 3770, CanonicalCombiningClass::Virama),
    (3771, 3783, CanonicalCombiningClass::NotReordered),
    (3784, 3787, CanonicalCombiningClass::CCC122),
    (3788, 3863, CanonicalCombiningClass::NotReordered),
    (3864, 3865, CanonicalCombiningClass::Below),
    (3866, 3892, CanonicalCombiningClass::NotReordered),
    (3893, 3893, CanonicalCombiningClass::Below),
    (3894, 3894, CanonicalCombiningClass::NotReordered),
    (3895, 3895, CanonicalCombiningClass::Below),
    (3896, 3896, CanonicalCombiningClass::NotReordered),
    (3897, 3897, CanonicalCombiningClass::AttachedAboveRight),
    (3898, 3952, CanonicalCombiningClass::NotReordered),
    (3953, 3953, CanonicalCombiningClass::CCC129),
    (3954, 3954, CanonicalCombiningClass::CCC130),
    (3955, 3955, CanonicalCombiningClass::NotReordered),
    (3956, 3956, CanonicalCombiningClass::CCC132),
    (3957, 3961, CanonicalCombiningClass::NotReordered),
    (3962, 3965, CanonicalCombiningClass::CCC130),
    (3966, 3967, CanonicalCombiningClass::NotReordered),
    (3968, 3968, CanonicalCombiningClass::CCC130),
    (3969, 3969, CanonicalCombiningClass::NotReordered),
    (3970, 3971, CanonicalCombiningClass::Above),
    (3972, 3972, CanonicalCombiningClass::Virama),
    (3973, 3973, CanonicalCombiningClass::NotReordered),
    (3974, 3975, CanonicalCombiningClass::Above),
    (3976, 4037, CanonicalCombiningClass::NotReordered),
    (4038, 4038, CanonicalCombiningClass::Below),
    (4039, 4150, CanonicalCombiningClass::NotReordered),
    (4151, 4151, CanonicalCombiningClass::Nukta),
    (4152, 4152, CanonicalCombiningClass::NotReordered),
    (4153, 4154, CanonicalCombiningClass::Virama),
    (4155, 4236, CanonicalCombiningClass::NotReordered),
    (4237, 4237, CanonicalCombiningClass::Below),
    (4238, 4956, CanonicalCombiningClass::NotReordered),
    (4957, 4959, CanonicalCombiningClass::Above),
    (4960, 5907, CanonicalCombiningClass::NotReordered),
    (5908, 5908, CanonicalCombiningClass::Virama),
    (5909, 5939, CanonicalCombiningClass::NotReordered),
    (5940, 5940, CanonicalCombiningClass::Virama),
    (5941, 6097, CanonicalCombiningClass::NotReordered),
    (6098, 6098, CanonicalCombiningClass::Virama),
    (6099, 6108, CanonicalCombiningClass::NotReordered),
    (6109, 6109, CanonicalCombiningClass::Above),
    (6110, 6312, CanonicalCombiningClass::NotReordered),
    (6313, 6313, CanonicalCombiningClass::AboveLeft),
    (6314, 6456, CanonicalCombiningClass::NotReordered),
    (6457, 6457, CanonicalCombiningClass::BelowRight),
    (6458, 6458, CanonicalCombiningClass::Above),
    (6459, 6459, CanonicalCombiningClass::Below),
    (6460, 6678, CanonicalCombiningClass::NotReordered),
    (6679, 6679, CanonicalCombiningClass::Above),
    (6680, 6680, CanonicalCombiningClass::Below),
    (6681, 6751, CanonicalCombiningClass::NotReordered),
    (6752, 6752, CanonicalCombiningClass::Virama),
    (6753, 6772, CanonicalCombiningClass::NotReordered),
    (6773, 6780, CanonicalCombiningClass::Above),
    (6781, 6782, CanonicalCombiningClass::NotReordered),
    (6783, 6783, CanonicalCombiningClass::Below),
    (6784, 6831, CanonicalCombiningClass::NotReordered),
    (6832, 6836, CanonicalCombiningClass::Above),
    (6837, 6842, CanonicalCombiningClass::Below),
    (6843, 6844, CanonicalCombiningClass::Above),
    (6845, 6845, CanonicalCombiningClass::Below),
    (6846, 6846, CanonicalCombiningClass::NotReordered),
    (6847, 6848, CanonicalCombiningClass::Below),
    (6849, 6963, CanonicalCombiningClass::NotReordered),
    (6964, 6964, CanonicalCombiningClass::Nukta),
    (6965, 6979, CanonicalCombiningClass::NotReordered),
    (6980, 6980, CanonicalCombiningClass::Virama),
    (6981, 7018, CanonicalCombiningClass::NotReordered),
    (7019, 7019, CanonicalCombiningClass::Above),
    (7020, 7020, CanonicalCombiningClass::Below),
    (7021, 7027, CanonicalCombiningClass::Above),
    (7028, 7081, CanonicalCombiningClass::NotReordered),
    (7082, 7083, CanonicalCombiningClass::Virama),
    (7084, 7141, CanonicalCombiningClass::NotReordered),
    (7142, 7142, CanonicalCombiningClass::Nukta),
    (7143, 7153, CanonicalCombiningClass::NotReordered),
    (7154, 7155, CanonicalCombiningClass::Virama),
    (7156, 7222, CanonicalCombiningClass::NotReordered),
    (7223, 7223, CanonicalCombiningClass::Nukta),
    (7224, 7375, CanonicalCombiningClass::NotReordered),
    (7376, 7378, CanonicalCombiningClass::Above),
    (7379, 7379, CanonicalCombiningClass::NotReordered),
    (7380, 7380, CanonicalCombiningClass::Overlay),
    (7381, 7385, CanonicalCombiningClass::Below),
    (7386, 7387, CanonicalCombiningClass::Above),
    (7388, 7391, CanonicalCombiningClass::Below),
    (7392, 7392, CanonicalCombiningClass::Above),
    (7393, 7393, CanonicalCombiningClass::NotReordered),
    (7394, 7400, CanonicalCombiningClass::Overlay),
    (7401, 7404, CanonicalCombiningClass::NotReordered),
    (7405, 7405, CanonicalCombiningClass::Below),
    (7406, 7411, CanonicalCombiningClass::NotReordered),
    (7412, 7412, CanonicalCombiningClass::Above),
    (7413, 7415, CanonicalCombiningClass::NotReordered),
    (7416, 7417, CanonicalCombiningClass::Above),
    (7418, 7615, CanonicalCombiningClass::NotReordered),
    (7616, 7617, CanonicalCombiningClass::Above),
    (7618, 7618, CanonicalCombiningClass::Below),
    (7619, 7625, CanonicalCombiningClass::Above),
    (7626, 7626, CanonicalCombiningClass::Below),
    (7627, 7628, CanonicalCombiningClass::Above),
    (7629, 7629, CanonicalCombiningClass::DoubleAbove),
    (7630, 7630, CanonicalCombiningClass::AttachedAbove),
    (7631, 7631, CanonicalCombiningClass::Below),
    (7632, 7632, CanonicalCombiningClass::AttachedBelow),
    (7633, 7669, CanonicalCombiningClass::Above),
    (7670, 7670, CanonicalCombiningClass::AboveRight),
    (7671, 7672, CanonicalCombiningClass::AboveLeft),
    (7673, 7673, CanonicalCombiningClass::Below),
    (7674, 7674, CanonicalCombiningClass::NotReordered),
    (7675, 7675, CanonicalCombiningClass::Above),
    (7676, 7676, CanonicalCombiningClass::DoubleBelow),
    (7677, 7677, CanonicalCombiningClass::Below),
    (7678, 7678, CanonicalCombiningClass::Above),
    (7679, 7679, CanonicalCombiningClass::Below),
    (7680, 8399, CanonicalCombiningClass::NotReordered),
    (8400, 8401, CanonicalCombiningClass::Above),
    (8402, 8403, CanonicalCombiningClass::Overlay),
    (8404, 8407, CanonicalCombiningClass::Above),
    (8408, 8410, CanonicalCombiningClass::Overlay),
    (8411, 8412, CanonicalCombiningClass::Above),
    (8413, 8416, CanonicalCombiningClass::NotReordered),
    (8417, 8417, CanonicalCombiningClass::Above),
    (8418, 8420, CanonicalCombiningClass::NotReordered),
    (8421, 8422, CanonicalCombiningClass::Overlay),
    (8423, 8423, CanonicalCombiningClass::Above),
    (8424, 8424, CanonicalCombiningClass::Below),
    (8425, 8425, CanonicalCombiningClass::Above),
    (8426, 8427, CanonicalCombiningClass::Overlay),
    (8428, 8431, CanonicalCombiningClass::Below),
    (8432, 8432, CanonicalCombiningClass::Above),
    (8433, 11502, CanonicalCombiningClass::NotReordered),
    (11503, 11505, CanonicalCombiningClass::Above),
    (11506, 11646, CanonicalCombiningClass::NotReordered),
    (11647, 11647, CanonicalCombiningClass::Virama),
    (11648, 11743, CanonicalCombiningClass::NotReordered),
    (11744, 11775, CanonicalCombiningClass::Above),
    (11776, 12329, CanonicalCombiningClass::NotReordered),
    (12330, 12330, CanonicalCombiningClass::BelowLeft),
    (12331, 12331, CanonicalCombiningClass::AboveLeft),
    (12332, 12332, CanonicalCombiningClass::AboveRight),
    (12333, 12333, CanonicalCombiningClass::BelowRight),
    (12334, 12335, CanonicalCombiningClass::Left),
    (12336, 12440, CanonicalCombiningClass::NotReordered),
    (12441, 12442, CanonicalCombiningClass::KanaVoicing),
    (12443, 42606, CanonicalCombiningClass::NotReordered),
    (42607, 42607, CanonicalCombiningClass::Above),
    (42608, 42611, CanonicalCombiningClass::NotReordered),
    (42612, 42621, CanonicalCombiningClass::Above),
    (42622, 42653, CanonicalCombiningClass::NotReordered),
    (42654, 42655, CanonicalCombiningClass::Above),
    (42656, 42735, CanonicalCombiningClass::NotReordered),
    (42736, 42737, CanonicalCombiningClass::Above),
    (42738, 43013, CanonicalCombiningClass::NotReordered),
    (43014, 43014, CanonicalCombiningClass::Virama),
    (43015, 43051, CanonicalCombiningClass::NotReordered),
    (43052, 43052, CanonicalCombiningClass::Virama),
    (43053, 43203, CanonicalCombiningClass::NotReordered),
    (43204, 43204, CanonicalCombiningClass::Virama),
    (43205, 43231, CanonicalCombiningClass::NotReordered),
    (43232, 43249, CanonicalCombiningClass::Above),
    (43250, 43306, CanonicalCombiningClass::NotReordered),
    (43307, 43309, CanonicalCombiningClass::Below),
    (43310, 43346, CanonicalCombiningClass::NotReordered),
    (43347, 43347, CanonicalCombiningClass::Virama),
    (43348, 43442, CanonicalCombiningClass::NotReordered),
    (43443, 43443, CanonicalCombiningClass::Nukta),
    (43444, 43455, CanonicalCombiningClass::NotReordered),
    (43456, 43456, CanonicalCombiningClass::Virama),
    (43457, 43695, CanonicalCombiningClass::NotReordered),
    (43696, 43696, CanonicalCombiningClass::Above),
    (43697, 43697, CanonicalCombiningClass::NotReordered),
    (43698, 43699, CanonicalCombiningClass::Above),
    (43700, 43700, CanonicalCombiningClass::Below),
    (43701, 43702, CanonicalCombiningClass::NotReordered),
    (43703, 43704, CanonicalCombiningClass::Above),
    (43705, 43709, CanonicalCombiningClass::NotReordered),
    (43710, 43711, CanonicalCombiningClass::Above),
    (43712, 43712, CanonicalCombiningClass::NotReordered),
    (43713, 43713, CanonicalCombiningClass::Above),
    (43714, 43765, CanonicalCombiningClass::NotReordered),
    (43766, 43766, CanonicalCombiningClass::Virama),
    (43767, 44012, CanonicalCombiningClass::NotReordered),
    (44013, 44013, CanonicalCombiningClass::Virama),
    (44014, 64285, CanonicalCombiningClass::NotReordered),
    (64286, 64286, CanonicalCombiningClass::CCC26),
    (64287, 65055, CanonicalCombiningClass::NotReordered),
    (65056, 65062, CanonicalCombiningClass::Above),
    (65063, 65069, CanonicalCombiningClass::Below),
    (65070, 65071, CanonicalCombiningClass::Above),
    (65072, 66044, CanonicalCombiningClass::NotReordered),
    (66045, 66045, CanonicalCombiningClass::Below),
    (66046, 66271, CanonicalCombiningClass::NotReordered),
    (66272, 66272, CanonicalCombiningClass::Below),
    (66273, 66421, CanonicalCombiningClass::NotReordered),
    (66422, 66426, CanonicalCombiningClass::Above),
    (66427, 68108, CanonicalCombiningClass::NotReordered),
    (68109, 68109, CanonicalCombiningClass::Below),
    (68110, 68110, CanonicalCombiningClass::NotReordered),
    (68111, 68111, CanonicalCombiningClass::Above),
    (68112, 68151, CanonicalCombiningClass::NotReordered),
    (68152, 68152, CanonicalCombiningClass::Above),
    (68153, 68153, CanonicalCombiningClass::Overlay),
    (68154, 68154, CanonicalCombiningClass::Below),
    (68155, 68158, CanonicalCombiningClass::NotReordered),
    (68159, 68159, CanonicalCombiningClass::Virama),
    (68160, 68324, CanonicalCombiningClass::NotReordered),
    (68325, 68325, CanonicalCombiningClass::Above),
    (68326, 68326, CanonicalCombiningClass::Below),
    (68327, 68899, CanonicalCombiningClass::NotReordered),
    (68900, 68903, CanonicalCombiningClass::Above),
    (68904, 69290, CanonicalCombiningClass::NotReordered),
    (69291, 69292, CanonicalCombiningClass::Above),
    (69293, 69445, CanonicalCombiningClass::NotReordered),
    (69446, 69447, CanonicalCombiningClass::Below),
    (69448, 69450, CanonicalCombiningClass::Above),
    (69451, 69451, CanonicalCombiningClass::Below),
    (69452, 69452, CanonicalCombiningClass::Above),
    (69453, 69456, CanonicalCombiningClass::Below),
    (69457, 69701, CanonicalCombiningClass::NotReordered),
    (69702, 69702, CanonicalCombiningClass::Virama),
    (69703, 69758, CanonicalCombiningClass::NotReordered),
    (69759, 69759, CanonicalCombiningClass::Virama),
    (69760, 69816, CanonicalCombiningClass::NotReordered),
    (69817, 69817, CanonicalCombiningClass::Virama),
    (69818, 69818, CanonicalCombiningClass::Nukta),
    (69819, 69887, CanonicalCombiningClass::NotReordered),
    (69888, 69890, CanonicalCombiningClass::Above),
    (69891, 69938, CanonicalCombiningClass::NotReordered),
    (69939, 69940, CanonicalCombiningClass::Virama),
    (69941, 70002, CanonicalCombiningClass::NotReordered),
    (70003, 70003, CanonicalCombiningClass::Nukta),
    (70004, 70079, CanonicalCombiningClass::NotReordered),
    (70080, 70080, CanonicalCombiningClass::Virama),
    (70081, 70089, CanonicalCombiningClass::NotReordered),
    (70090, 70090, CanonicalCombiningClass::Nukta),
    (70091, 70196, CanonicalCombiningClass::NotReordered),
    (70197, 70197, CanonicalCombiningClass::Virama),
    (70198, 70198, CanonicalCombiningClass::Nukta),
    (70199, 70376, CanonicalCombiningClass::NotReordered),
    (70377, 70377, CanonicalCombiningClass::Nukta),
    (70378, 70378, CanonicalCombiningClass::Virama),
    (70379, 70458, CanonicalCombiningClass::NotReordered),
    (70459, 70460, CanonicalCombiningClass::Nukta),
    (70461, 70476, CanonicalCombiningClass::NotReordered),
    (70477, 70477, CanonicalCombiningClass::Virama),
    (70478, 70501, CanonicalCombiningClass::NotReordered),
    (70502, 70508, CanonicalCombiningClass::Above),
    (70509, 70511, CanonicalCombiningClass::NotReordered),
    (70512, 70516, CanonicalCombiningClass::Above),
    (70517, 70721, CanonicalCombiningClass::NotReordered),
    (70722, 70722, CanonicalCombiningClass::Virama),
    (70723, 70725, CanonicalCombiningClass::NotReordered),
    (70726, 70726, CanonicalCombiningClass::Nukta),
    (70727, 70749, CanonicalCombiningClass::NotReordered),
    (70750, 70750, CanonicalCombiningClass::Above),
    (70751, 70849, CanonicalCombiningClass::NotReordered),
    (70850, 70850, CanonicalCombiningClass::Virama),
    (70851, 70851, CanonicalCombiningClass::Nukta),
    (70852, 71102, CanonicalCombiningClass::NotReordered),
    (71103, 71103, CanonicalCombiningClass::Virama),
    (71104, 71104, CanonicalCombiningClass::Nukta),
    (71105, 71230, CanonicalCombiningClass::NotReordered),
    (71231, 71231, CanonicalCombiningClass::Virama),
    (71232, 71349, CanonicalCombiningClass::NotReordered),
    (71350, 71350, CanonicalCombiningClass::Virama),
    (71351, 71351, CanonicalCombiningClass::Nukta),
    (71352, 71466, CanonicalCombiningClass::NotReordered),
    (71467, 71467, CanonicalCombiningClass::Virama),
    (71468, 71736, CanonicalCombiningClass::NotReordered),
    (71737, 71737, CanonicalCombiningClass::Virama),
    (71738, 71738, CanonicalCombiningClass::Nukta),
    (71739, 71996, CanonicalCombiningClass::NotReordered),
    (71997, 71998, CanonicalCombiningClass::Virama),
    (71999, 72002, CanonicalCombiningClass::NotReordered),
    (72003, 72003, CanonicalCombiningClass::Nukta),
    (72004, 72159, CanonicalCombiningClass::NotReordered),
    (72160, 72160, CanonicalCombiningClass::Virama),
    (72161, 72243, CanonicalCombiningClass::NotReordered),
    (72244, 72244, CanonicalCombiningClass::Virama),
    (72245, 72262, CanonicalCombiningClass::NotReordered),
    (72263, 72263, CanonicalCombiningClass::Virama),
    (72264, 72344, CanonicalCombiningClass::NotReordered),
    (72345, 72345, CanonicalCombiningClass::Virama),
    (72346, 72766, CanonicalCombiningClass::NotReordered),
    (72767, 72767, CanonicalCombiningClass::Virama),
    (72768, 73025, CanonicalCombiningClass::NotReordered),
    (73026, 73026, CanonicalCombiningClass::Nukta),
    (73027, 73027, CanonicalCombiningClass::NotReordered),
    (73028, 73029, CanonicalCombiningClass::Virama),
    (73030, 73110, CanonicalCombiningClass::NotReordered),
    (73111, 73111, CanonicalCombiningClass::Virama),
    (73112, 92911, CanonicalCombiningClass::NotReordered),
    (92912, 92916, CanonicalCombiningClass::Overlay),
    (92917, 92975, CanonicalCombiningClass::NotReordered),
    (92976, 92982, CanonicalCombiningClass::Above),
    (92983, 94191, CanonicalCombiningClass::NotReordered),
    (94192, 94193, CanonicalCombiningClass::HanReading),
    (94194, 113821, CanonicalCombiningClass::NotReordered),
    (113822, 113822, CanonicalCombiningClass::Overlay),
    (113823, 119140, CanonicalCombiningClass::NotReordered),
    (119141, 119142, CanonicalCombiningClass::AttachedAboveRight),
    (119143, 119145, CanonicalCombiningClass::Overlay),
    (119146, 119148, CanonicalCombiningClass::NotReordered),
    (119149, 119149, CanonicalCombiningClass::Right),
    (119150, 119154, CanonicalCombiningClass::AttachedAboveRight),
    (119155, 119162, CanonicalCombiningClass::NotReordered),
    (119163, 119170, CanonicalCombiningClass::Below),
    (119171, 119172, CanonicalCombiningClass::NotReordered),
    (119173, 119177, CanonicalCombiningClass::Above),
    (119178, 119179, CanonicalCombiningClass::Below),
    (119180, 119209, CanonicalCombiningClass::NotReordered),
    (119210, 119213, CanonicalCombiningClass::Above),
    (119214, 119361, CanonicalCombiningClass::NotReordered),
    (119362, 119364, CanonicalCombiningClass::Above),
    (119365, 122879, CanonicalCombiningClass::NotReordered),
    (122880, 122886, CanonicalCombiningClass::Above),
    (122887, 122887, CanonicalCombiningClass::NotReordered),
    (122888, 122904, CanonicalCombiningClass::Above),
    (122905, 122906, CanonicalCombiningClass::NotReordered),
    (122907, 122913, CanonicalCombiningClass::Above),
    (122914, 122914, CanonicalCombiningClass::NotReordered),
    (122915, 122916, CanonicalCombiningClass::Above),
    (122917, 122917, CanonicalCombiningClass::NotReordered),
    (122918, 122922, CanonicalCombiningClass::Above),
    (122923, 123183, CanonicalCombiningClass::NotReordered),
    (123184, 123190, CanonicalCombiningClass::Above),
    (123191, 123627, CanonicalCombiningClass::NotReordered),
    (123628, 123631, CanonicalCombiningClass::Above),
    (123632, 125135, CanonicalCombiningClass::NotReordered),
    (125136, 125142, CanonicalCombiningClass::Below),
    (125143, 125251, CanonicalCombiningClass::NotReordered),
    (125252, 125257, CanonicalCombiningClass::Above),
    (125258, 125258, CanonicalCombiningClass::Nukta),
    (125259, 1114111, CanonicalCombiningClass::NotReordered),
];
