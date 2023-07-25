#[cfg(test)]
use crate::hiragana::convert_number;

#[test]
pub fn hiragana_tests() {
    assert_eq!(convert_number("0"), "ゼロ");
    assert_eq!(convert_number("1"), "いち");
    assert_eq!(convert_number("2"), "に");
    assert_eq!(convert_number("3"), "さん");
    assert_eq!(convert_number("4"), "よん");
    assert_eq!(convert_number("5"), "ご");
    assert_eq!(convert_number("6"), "ろく");
    assert_eq!(convert_number("7"), "なな");
    assert_eq!(convert_number("8"), "はち");
    assert_eq!(convert_number("9"), "きゅう");
    assert_eq!(convert_number("10"), "じゅう");
    assert_eq!(convert_number("20"), "にじゅう");
    assert_eq!(convert_number("22"), "にじゅうに");
    assert_eq!(convert_number("30"), "さんじゅう");
    assert_eq!(convert_number("33"), "さんじゅうさん");
    assert_eq!(convert_number("40"), "よんじゅう");
    assert_eq!(convert_number("44"), "よんじゅうよん");
    assert_eq!(convert_number("50"), "ごじゅう");
    assert_eq!(convert_number("55"), "ごじゅうご");
    assert_eq!(convert_number("60"), "ろくじゅう");
    assert_eq!(convert_number("66"), "ろくじゅうろく");
    assert_eq!(convert_number("70"), "ななじゅう");
    assert_eq!(convert_number("77"), "ななじゅうなな");
    assert_eq!(convert_number("80"), "はちじゅう");
    assert_eq!(convert_number("88"), "はちじゅうはち");
    assert_eq!(convert_number("90"), "きゅうじゅう");
    assert_eq!(convert_number("99"), "きゅうじゅうきゅう");
    assert_eq!(convert_number("100"), "ひゃく");
    assert_eq!(convert_number("111"), "ひゃくじゅういち");
    assert_eq!(convert_number("200"), "にひゃく");
    assert_eq!(convert_number("222"), "にひゃくにじゅうに");
    assert_eq!(convert_number("300"), "さんびゃく");
    assert_eq!(convert_number("333"), "さんびゃくさんじゅうさん");
    assert_eq!(convert_number("400"), "よんひゃく");
    assert_eq!(convert_number("444"), "よんひゃくよんじゅうよん");
    assert_eq!(convert_number("500"), "ごひゃく");
    assert_eq!(convert_number("555"), "ごひゃくごじゅうご");
    assert_eq!(convert_number("600"), "ろっぴゃく");
    assert_eq!(convert_number("666"), "ろっぴゃくろくじゅうろく");
    assert_eq!(convert_number("700"), "ななひゃく");
    assert_eq!(convert_number("777"), "ななひゃくななじゅうなな");
    assert_eq!(convert_number("800"), "はっぴゃく");
    assert_eq!(convert_number("888"), "はっぴゃくはちじゅうはち");
    assert_eq!(convert_number("900"), "きゅうひゃく");
    assert_eq!(convert_number("999"), "きゅうひゃくきゅうじゅうきゅう");
    assert_eq!(convert_number("1000"), "せん");
    assert_eq!(convert_number("1010"), "せんじゅう");
    assert_eq!(convert_number("1111"), "せんひゃくじゅういち");
    assert_eq!(convert_number("2000"), "にせん");
    assert_eq!(convert_number("2020"), "にせんにじゅう");
    assert_eq!(convert_number("2222"), "にせんにひゃくにじゅうに");
    assert_eq!(convert_number("3000"), "さんぜん");
    assert_eq!(convert_number("3030"), "さんぜんさんじゅう");
    assert_eq!(convert_number("3333"), "さんぜんさんびゃくさんじゅうさん");
    assert_eq!(convert_number("4000"), "よんせん");
    assert_eq!(convert_number("4040"), "よんせんよんじゅう");
    assert_eq!(convert_number("4444"), "よんせんよんひゃくよんじゅうよん");
    assert_eq!(convert_number("5000"), "ごせん");
    assert_eq!(convert_number("5050"), "ごせんごじゅう");
    assert_eq!(convert_number("5555"), "ごせんごひゃくごじゅうご");
    assert_eq!(convert_number("6000"), "ろくせん");
    assert_eq!(convert_number("6060"), "ろくせんろくじゅう");
    assert_eq!(convert_number("6666"), "ろくせんろっぴゃくろくじゅうろく");
    assert_eq!(convert_number("7000"), "ななせん");
    assert_eq!(convert_number("7070"), "ななせんななじゅう");
    assert_eq!(convert_number("7777"), "ななせんななひゃくななじゅうなな");
    assert_eq!(convert_number("8000"), "はっせん");
    assert_eq!(convert_number("8080"), "はっせんはちじゅう");
    assert_eq!(convert_number("8888"), "はっせんはっぴゃくはちじゅうはち");
    assert_eq!(convert_number("9000"), "きゅうせん");
    assert_eq!(convert_number("9090"), "きゅうせんきゅうじゅう");
    assert_eq!(convert_number("9999"), "きゅうせんきゅうひゃくきゅうじゅうきゅう");
    assert_eq!(convert_number("10000"), "いちまん");
    assert_eq!(convert_number("11111"), "いちまんいっせんひゃくじゅういち");
    assert_eq!(convert_number("20000"), "にまん");
    assert_eq!(convert_number("22222"), "にまんにせんにひゃくにじゅうに");
    assert_eq!(convert_number("30000"), "さんまん");
    assert_eq!(convert_number("33333"), "さんまんさんぜんさんびゃくさんじゅうさん");
    assert_eq!(convert_number("40000"), "よんまん");
    assert_eq!(convert_number("44444"), "よんまんよんせんよんひゃくよんじゅうよん");
    assert_eq!(convert_number("50000"), "ごまん");
    assert_eq!(convert_number("55555"), "ごまんごせんごひゃくごじゅうご");
    assert_eq!(convert_number("60000"), "ろくまん");
    assert_eq!(convert_number("66666"), "ろくまんろくせんろっぴゃくろくじゅうろく");
    assert_eq!(convert_number("70000"), "ななまん");
    assert_eq!(convert_number("77777"), "ななまんななせんななひゃくななじゅうなな");
    assert_eq!(convert_number("80000"), "はちまん");
    assert_eq!(convert_number("88888"), "はちまんはっせんはっぴゃくはちじゅうはち");
    assert_eq!(convert_number("90000"), "きゅうまん");
    assert_eq!(convert_number("99999"), "きゅうまんきゅうせんきゅうひゃくきゅうじゅうきゅう");
    assert_eq!(convert_number("100000"), "じゅうまん");
    assert_eq!(convert_number("100001"), "じゅうまんいち");
    assert_eq!(convert_number("100100"), "じゅうまんひゃく");
    assert_eq!(convert_number("101010"), "じゅうまんいっせんじゅう");
    assert_eq!(convert_number("111111"), "じゅういちまんいっせんひゃくじゅういち");
    assert_eq!(convert_number("200000"), "にじゅうまん");
    assert_eq!(convert_number("200002"), "にじゅうまんに");
    assert_eq!(convert_number("200200"), "にじゅうまんにひゃく");
    assert_eq!(convert_number("202020"), "にじゅうまんにせんにじゅう");
    assert_eq!(convert_number("222222"), "にじゅうにまんにせんにひゃくにじゅうに");
    assert_eq!(convert_number("300000"), "さんじゅうまん");
    assert_eq!(convert_number("300003"), "さんじゅうまんさん");
    assert_eq!(convert_number("300300"), "さんじゅうまんさんびゃく");
    assert_eq!(convert_number("303030"), "さんじゅうまんさんぜんさんじゅう");
    assert_eq!(convert_number("333333"), "さんじゅうさんまんさんぜんさんびゃくさんじゅうさん");
    assert_eq!(convert_number("400000"), "よんじゅうまん");
    assert_eq!(convert_number("400004"), "よんじゅうまんよん");
    assert_eq!(convert_number("400400"), "よんじゅうまんよんひゃく");
    assert_eq!(convert_number("404040"), "よんじゅうまんよんせんよんじゅう");
    assert_eq!(convert_number("444444"), "よんじゅうよんまんよんせんよんひゃくよんじゅうよん");
    assert_eq!(convert_number("500000"), "ごじゅうまん");
    assert_eq!(convert_number("500005"), "ごじゅうまんご");
    assert_eq!(convert_number("500500"), "ごじゅうまんごひゃく");
    assert_eq!(convert_number("505050"), "ごじゅうまんごせんごじゅう");
    assert_eq!(convert_number("555555"), "ごじゅうごまんごせんごひゃくごじゅうご");
    assert_eq!(convert_number("600000"), "ろくじゅうまん");
    assert_eq!(convert_number("600006"), "ろくじゅうまんろく");
    assert_eq!(convert_number("600600"), "ろくじゅうまんろっぴゃく");
    assert_eq!(convert_number("606060"), "ろくじゅうまんろくせんろくじゅう");
    assert_eq!(convert_number("666666"), "ろくじゅうろくまんろくせんろっぴゃくろくじゅうろく");
    assert_eq!(convert_number("700000"), "ななじゅうまん");
    assert_eq!(convert_number("700007"), "ななじゅうまんなな");
    assert_eq!(convert_number("700700"), "ななじゅうまんななひゃく");
    assert_eq!(convert_number("707070"), "ななじゅうまんななせんななじゅう");
    assert_eq!(convert_number("777777"), "ななじゅうななまんななせんななひゃくななじゅうなな");
    assert_eq!(convert_number("800000"), "はちじゅうまん");
    assert_eq!(convert_number("800008"), "はちじゅうまんはち");
    assert_eq!(convert_number("800800"), "はちじゅうまんはっぴゃく");
    assert_eq!(convert_number("808080"), "はちじゅうまんはっせんはちじゅう");
    assert_eq!(convert_number("888888"), "はちじゅうはちまんはっせんはっぴゃくはちじゅうはち");
    assert_eq!(convert_number("900000"), "きゅうじゅうまん");
    assert_eq!(convert_number("900009"), "きゅうじゅうまんきゅう");
    assert_eq!(convert_number("900900"), "きゅうじゅうまんきゅうひゃく");
    assert_eq!(convert_number("909090"), "きゅうじゅうまんきゅうせんきゅうじゅう");
    assert_eq!(convert_number("999999"), "きゅうじゅうきゅうまんきゅうせんきゅうひゃくきゅうじゅうきゅう");
    assert_eq!(convert_number("1000000"), "ひゃくまん");
    assert_eq!(convert_number("1111111"), "ひゃくじゅういちまんいっせんひゃくじゅういち");
    assert_eq!(convert_number("2222222"), "にひゃくにじゅうにまんにせんにひゃくにじゅうに");
    assert_eq!(convert_number("3333333"), "さんびゃくさんじゅうさんまんさんぜんさんびゃくさんじゅうさん");
    assert_eq!(convert_number("4444444"), "よんひゃくよんじゅうよんまんよんせんよんひゃくよんじゅうよん");
    assert_eq!(convert_number("5555555"), "ごひゃくごじゅうごまんごせんごひゃくごじゅうご");
    assert_eq!(convert_number("6666666"), "ろっぴゃくろくじゅうろくまんろくせんろっぴゃくろくじゅうろく");
    assert_eq!(convert_number("7777777"), "ななひゃくななじゅうななまんななせんななひゃくななじゅうなな");
    assert_eq!(convert_number("8888888"), "はっぴゃくはちじゅうはちまんはっせんはっぴゃくはちじゅうはち");
    assert_eq!(convert_number("9999999"), "きゅうひゃくきゅうじゅうきゅうまんきゅうせんきゅうひゃくきゅうじゅうきゅう");
    assert_eq!(convert_number("10000000"), "いっせんまん");
    assert_eq!(convert_number("10000001"), "いっせんまんいち");
    assert_eq!(convert_number("10001000"), "いっせんまんいっせん");
    assert_eq!(convert_number("10101010"), "いっせんじゅうまんいっせんじゅう");
    assert_eq!(convert_number("11111111"), "いっせんひゃくじゅういちまんいっせんひゃくじゅういち");
    assert_eq!(convert_number("20000002"), "にせんまんに");
    assert_eq!(convert_number("20002000"), "にせんまんにせん");
    assert_eq!(convert_number("20202020"), "にせんにじゅうまんにせんにじゅう");
    assert_eq!(convert_number("22222222"), "にせんにひゃくにじゅうにまんにせんにひゃくにじゅうに");
    assert_eq!(convert_number("30000003"), "さんぜんまんさん");
    assert_eq!(convert_number("30003000"), "さんぜんまんさんぜん");
    assert_eq!(convert_number("30303030"), "さんぜんさんじゅうまんさんぜんさんじゅう");
    assert_eq!(convert_number("33333333"), "さんぜんさんびゃくさんじゅうさんまんさんぜんさんびゃくさんじゅうさん");
    assert_eq!(convert_number("40000004"), "よんせんまんよん");
    assert_eq!(convert_number("40004000"), "よんせんまんよんせん");
    assert_eq!(convert_number("40404040"), "よんせんよんじゅうまんよんせんよんじゅう");
    assert_eq!(convert_number("44444444"), "よんせんよんひゃくよんじゅうよんまんよんせんよんひゃくよんじゅうよん");
    assert_eq!(convert_number("50000005"), "ごせんまんご");
    assert_eq!(convert_number("50005000"), "ごせんまんごせん");
    assert_eq!(convert_number("50505050"), "ごせんごじゅうまんごせんごじゅう");
    assert_eq!(convert_number("55555555"), "ごせんごひゃくごじゅうごまんごせんごひゃくごじゅうご");
    assert_eq!(convert_number("60000006"), "ろくせんまんろく");
    assert_eq!(convert_number("60006000"), "ろくせんまんろくせん");
    assert_eq!(convert_number("60606060"), "ろくせんろくじゅうまんろくせんろくじゅう");
    assert_eq!(convert_number("66666666"), "ろくせんろっぴゃくろくじゅうろくまんろくせんろっぴゃくろくじゅうろく");
    assert_eq!(convert_number("70000007"), "ななせんまんなな");
    assert_eq!(convert_number("70007000"), "ななせんまんななせん");
    assert_eq!(convert_number("70707070"), "ななせんななじゅうまんななせんななじゅう");
    assert_eq!(convert_number("77777777"), "ななせんななひゃくななじゅうななまんななせんななひゃくななじゅうなな");
    assert_eq!(convert_number("80000008"), "はっせんまんはち");
    assert_eq!(convert_number("80008000"), "はっせんまんはっせん");
    assert_eq!(convert_number("80808080"), "はっせんはちじゅうまんはっせんはちじゅう");
    assert_eq!(convert_number("88888888"), "はっせんはっぴゃくはちじゅうはちまんはっせんはっぴゃくはちじゅうはち");
    assert_eq!(convert_number("90000009"), "きゅうせんまんきゅう");
    assert_eq!(convert_number("90009000"), "きゅうせんまんきゅうせん");
    assert_eq!(convert_number("90909090"), "きゅうせんきゅうじゅうまんきゅうせんきゅうじゅう");
    assert_eq!(convert_number("99999999"), "きゅうせんきゅうひゃくきゅうじゅうきゅうまんきゅうせんきゅうひゃくきゅうじゅうきゅう");
    assert_eq!(convert_number("100000000"), "いちおく");
    assert_eq!(convert_number("100001001"), "いちおくいっせんいち");
    assert_eq!(convert_number("100100100"), "いちおくじゅうまんひゃく");
    assert_eq!(convert_number("111111111"), "いちおくいっせんひゃくじゅういちまんいっせんひゃくじゅういち");
    assert_eq!(convert_number("200002002"), "におくにせんに");
    assert_eq!(convert_number("200200200"), "におくにじゅうまんにひゃく");
    assert_eq!(convert_number("222222222"), "におくにせんにひゃくにじゅうにまんにせんにひゃくにじゅうに");
    assert_eq!(convert_number("300003003"), "さんおくさんぜんさん");
    assert_eq!(convert_number("300300300"), "さんおくさんじゅうまんさんびゃく");
    assert_eq!(convert_number("333333333"), "さんおくさんぜんさんびゃくさんじゅうさんまんさんぜんさんびゃくさんじゅうさん");
    assert_eq!(convert_number("400004004"), "よんおくよんせんよん");
    assert_eq!(convert_number("400400400"), "よんおくよんじゅうまんよんひゃく");
    assert_eq!(convert_number("444444444"), "よんおくよんせんよんひゃくよんじゅうよんまんよんせんよんひゃくよんじゅうよん");
    assert_eq!(convert_number("500005005"), "ごおくごせんご");
    assert_eq!(convert_number("500500500"), "ごおくごじゅうまんごひゃく");
    assert_eq!(convert_number("555555555"), "ごおくごせんごひゃくごじゅうごまんごせんごひゃくごじゅうご");
    assert_eq!(convert_number("600006006"), "ろくおくろくせんろく");
    assert_eq!(convert_number("600600600"), "ろくおくろくじゅうまんろっぴゃく");
    assert_eq!(convert_number("666666666"), "ろくおくろくせんろっぴゃくろくじゅうろくまんろくせんろっぴゃくろくじゅうろく");
    assert_eq!(convert_number("700007007"), "ななおくななせんなな");
    assert_eq!(convert_number("700700700"), "ななおくななじゅうまんななひゃく");
    assert_eq!(convert_number("777777777"), "ななおくななせんななひゃくななじゅうななまんななせんななひゃくななじゅうなな");
    assert_eq!(convert_number("800008008"), "はちおくはっせんはち");
    assert_eq!(convert_number("800800800"), "はちおくはちじゅうまんはっぴゃく");
    assert_eq!(convert_number("888888888"), "はちおくはっせんはっぴゃくはちじゅうはちまんはっせんはっぴゃくはちじゅうはち");
    assert_eq!(convert_number("900009009"), "きゅうおくきゅうせんきゅう");
    assert_eq!(convert_number("900900900"), "きゅうおくきゅうじゅうまんきゅうひゃく");
    assert_eq!(convert_number("999999999"), "きゅうおくきゅうせんきゅうひゃくきゅうじゅうきゅうまんきゅうせんきゅうひゃくきゅうじゅうきゅう");
    assert_eq!(convert_number("1000000000"), "じゅうおく");
    assert_eq!(convert_number("1000000001"), "じゅうおくいち");
    assert_eq!(convert_number("1000010000"), "じゅうおくいちまん");
    assert_eq!(convert_number("1010101010"), "じゅうおくいっせんじゅうまんいっせんじゅう");
    assert_eq!(convert_number("1111111111"), "じゅういちおくいっせんひゃくじゅういちまんいっせんひゃくじゅういち");
    assert_eq!(convert_number("2000000002"), "にじゅうおくに");
    assert_eq!(convert_number("2000020000"), "にじゅうおくにまん");
    assert_eq!(convert_number("2020202020"), "にじゅうおくにせんにじゅうまんにせんにじゅう");
    assert_eq!(convert_number("2222222222"), "にじゅうにおくにせんにひゃくにじゅうにまんにせんにひゃくにじゅうに");
    assert_eq!(convert_number("3000000003"), "さんじゅうおくさん");
    assert_eq!(convert_number("3000030000"), "さんじゅうおくさんまん");
    assert_eq!(convert_number("3030303030"), "さんじゅうおくさんぜんさんじゅうまんさんぜんさんじゅう");
    assert_eq!(convert_number("3333333333"), "さんじゅうさんおくさんぜんさんびゃくさんじゅうさんまんさんぜんさんびゃくさんじゅうさん");
    assert_eq!(convert_number("4000000004"), "よんじゅうおくよん");
    assert_eq!(convert_number("4000040000"), "よんじゅうおくよんまん");
    assert_eq!(convert_number("4040404040"), "よんじゅうおくよんせんよんじゅうまんよんせんよんじゅう");
    assert_eq!(convert_number("4444444444"), "よんじゅうよんおくよんせんよんひゃくよんじゅうよんまんよんせんよんひゃくよんじゅうよん");
    assert_eq!(convert_number("5000000005"), "ごじゅうおくご");
    assert_eq!(convert_number("5000050000"), "ごじゅうおくごまん");
    assert_eq!(convert_number("5050505050"), "ごじゅうおくごせんごじゅうまんごせんごじゅう");
    assert_eq!(convert_number("5555555555"), "ごじゅうごおくごせんごひゃくごじゅうごまんごせんごひゃくごじゅうご");
    assert_eq!(convert_number("6000000006"), "ろくじゅうおくろく");
    assert_eq!(convert_number("6000060000"), "ろくじゅうおくろくまん");
    assert_eq!(convert_number("6060606060"), "ろくじゅうおくろくせんろくじゅうまんろくせんろくじゅう");
    assert_eq!(convert_number("6666666666"), "ろくじゅうろくおくろくせんろっぴゃくろくじゅうろくまんろくせんろっぴゃくろくじゅうろく");
    assert_eq!(convert_number("7000000007"), "ななじゅうおくなな");
    assert_eq!(convert_number("7000070000"), "ななじゅうおくななまん");
    assert_eq!(convert_number("7070707070"), "ななじゅうおくななせんななじゅうまんななせんななじゅう");
    assert_eq!(convert_number("7777777777"), "ななじゅうななおくななせんななひゃくななじゅうななまんななせんななひゃくななじゅうなな");
    assert_eq!(convert_number("8000000008"), "はちじゅうおくはち");
    assert_eq!(convert_number("8000080000"), "はちじゅうおくはちまん");
    assert_eq!(convert_number("8080808080"), "はちじゅうおくはっせんはちじゅうまんはっせんはちじゅう");
    assert_eq!(convert_number("8888888888"), "はちじゅうはちおくはっせんはっぴゃくはちじゅうはちまんはっせんはっぴゃくはちじゅうはち");
    assert_eq!(convert_number("9000000009"), "きゅうじゅうおくきゅう");
    assert_eq!(convert_number("9000090000"), "きゅうじゅうおくきゅうまん");
    assert_eq!(convert_number("9090909090"), "きゅうじゅうおくきゅうせんきゅうじゅうまんきゅうせんきゅうじゅう");
    assert_eq!(convert_number("9999999999"), "きゅうじゅうきゅうおくきゅうせんきゅうひゃくきゅうじゅうきゅうまんきゅうせんきゅうひゃくきゅうじゅうきゅう");
    assert_eq!(convert_number("10000000000"), "ひゃくおく");
    assert_eq!(convert_number("11111111111"), "ひゃくじゅういちおくいっせんひゃくじゅういちまんいっせんひゃくじゅういち");
    assert_eq!(convert_number("22222222222"), "にひゃくにじゅうにおくにせんにひゃくにじゅうにまんにせんにひゃくにじゅうに");
    assert_eq!(convert_number("33333333333"), "さんびゃくさんじゅうさんおくさんぜんさんびゃくさんじゅうさんまんさんぜんさんびゃくさんじゅうさん");
    assert_eq!(convert_number("44444444444"), "よんひゃくよんじゅうよんおくよんせんよんひゃくよんじゅうよんまんよんせんよんひゃくよんじゅうよん");
    assert_eq!(convert_number("55555555555"), "ごひゃくごじゅうごおくごせんごひゃくごじゅうごまんごせんごひゃくごじゅうご");
    assert_eq!(convert_number("66666666666"), "ろっぴゃくろくじゅうろくおくろくせんろっぴゃくろくじゅうろくまんろくせんろっぴゃくろくじゅうろく");
    assert_eq!(convert_number("77777777777"), "ななひゃくななじゅうななおくななせんななひゃくななじゅうななまんななせんななひゃくななじゅうなな");
    assert_eq!(convert_number("88888888888"), "はっぴゃくはちじゅうはちおくはっせんはっぴゃくはちじゅうはちまんはっせんはっぴゃくはちじゅうはち");
    assert_eq!(convert_number("99999999999"), "きゅうひゃくきゅうじゅうきゅうおくきゅうせんきゅうひゃくきゅうじゅうきゅうまんきゅうせんきゅうひゃくきゅうじゅうきゅう");
    assert_eq!(convert_number("100000000000"), "いっせんおく");
    assert_eq!(convert_number("100000010001"), "いっせんおくいちまんいち");
    assert_eq!(convert_number("100000100000"), "いっせんおくじゅうまん");
    assert_eq!(convert_number("100001001001"), "いっせんおくひゃくまんいっせんいち");
    assert_eq!(convert_number("100010001000"), "いっせんおくいっせんまんいっせん");
    assert_eq!(convert_number("100100100100"), "いっせんいちおくじゅうまんひゃく");
    assert_eq!(convert_number("101010101010"), "いっせんじゅうおくいっせんじゅうまんいっせんじゅう");
    assert_eq!(convert_number("111111111111"), "いっせんひゃくじゅういちおくいっせんひゃくじゅういちまんいっせんひゃくじゅういち");
    assert_eq!(convert_number("200000020002"), "にせんおくにまんに");
    assert_eq!(convert_number("200000200000"), "にせんおくにじゅうまん");
    assert_eq!(convert_number("200002002002"), "にせんおくにひゃくまんにせんに");
    assert_eq!(convert_number("200020002000"), "にせんおくにせんまんにせん");
    assert_eq!(convert_number("200200200200"), "にせんにおくにじゅうまんにひゃく");
    assert_eq!(convert_number("202020202020"), "にせんにじゅうおくにせんにじゅうまんにせんにじゅう");
    assert_eq!(convert_number("222222222222"), "にせんにひゃくにじゅうにおくにせんにひゃくにじゅうにまんにせんにひゃくにじゅうに");
    assert_eq!(convert_number("300000030003"), "さんぜんおくさんまんさん");
    assert_eq!(convert_number("300000300000"), "さんぜんおくさんじゅうまん");
    assert_eq!(convert_number("300003003003"), "さんぜんおくさんびゃくまんさんぜんさん");
    assert_eq!(convert_number("300030003000"), "さんぜんおくさんぜんまんさんぜん");
    assert_eq!(convert_number("300300300300"), "さんぜんさんおくさんじゅうまんさんびゃく");
    assert_eq!(convert_number("303030303030"), "さんぜんさんじゅうおくさんぜんさんじゅうまんさんぜんさんじゅう");
    assert_eq!(convert_number("333333333333"), "さんぜんさんびゃくさんじゅうさんおくさんぜんさんびゃくさんじゅうさんまんさんぜんさんびゃくさんじゅうさん");
    assert_eq!(convert_number("400000040004"), "よんせんおくよんまんよん");
    assert_eq!(convert_number("400000400000"), "よんせんおくよんじゅうまん");
    assert_eq!(convert_number("400004004004"), "よんせんおくよんひゃくまんよんせんよん");
    assert_eq!(convert_number("400040004000"), "よんせんおくよんせんまんよんせん");
    assert_eq!(convert_number("400400400400"), "よんせんよんおくよんじゅうまんよんひゃく");
    assert_eq!(convert_number("404040404040"), "よんせんよんじゅうおくよんせんよんじゅうまんよんせんよんじゅう");
    assert_eq!(convert_number("444444444444"), "よんせんよんひゃくよんじゅうよんおくよんせんよんひゃくよんじゅうよんまんよんせんよんひゃくよんじゅうよん");
    assert_eq!(convert_number("500000050005"), "ごせんおくごまんご");
    assert_eq!(convert_number("500000500000"), "ごせんおくごじゅうまん");
    assert_eq!(convert_number("500005005005"), "ごせんおくごひゃくまんごせんご");
    assert_eq!(convert_number("500050005000"), "ごせんおくごせんまんごせん");
    assert_eq!(convert_number("500500500500"), "ごせんごおくごじゅうまんごひゃく");
    assert_eq!(convert_number("505050505050"), "ごせんごじゅうおくごせんごじゅうまんごせんごじゅう");
    assert_eq!(convert_number("555555555555"), "ごせんごひゃくごじゅうごおくごせんごひゃくごじゅうごまんごせんごひゃくごじゅうご");
    assert_eq!(convert_number("600000060006"), "ろくせんおくろくまんろく");
    assert_eq!(convert_number("600000600000"), "ろくせんおくろくじゅうまん");
    assert_eq!(convert_number("600006006006"), "ろくせんおくろっぴゃくまんろくせんろく");
    assert_eq!(convert_number("600060006000"), "ろくせんおくろくせんまんろくせん");
    assert_eq!(convert_number("600600600600"), "ろくせんろくおくろくじゅうまんろっぴゃく");
    assert_eq!(convert_number("606060606060"), "ろくせんろくじゅうおくろくせんろくじゅうまんろくせんろくじゅう");
    assert_eq!(convert_number("666666666666"), "ろくせんろっぴゃくろくじゅうろくおくろくせんろっぴゃくろくじゅうろくまんろくせんろっぴゃくろくじゅうろく");
    assert_eq!(convert_number("700000070007"), "ななせんおくななまんなな");
    assert_eq!(convert_number("700000700000"), "ななせんおくななじゅうまん");
    assert_eq!(convert_number("700007007007"), "ななせんおくななひゃくまんななせんなな");
    assert_eq!(convert_number("700070007000"), "ななせんおくななせんまんななせん");
    assert_eq!(convert_number("700700700700"), "ななせんななおくななじゅうまんななひゃく");
    assert_eq!(convert_number("707070707070"), "ななせんななじゅうおくななせんななじゅうまんななせんななじゅう");
    assert_eq!(convert_number("777777777777"), "ななせんななひゃくななじゅうななおくななせんななひゃくななじゅうななまんななせんななひゃくななじゅうなな");
    assert_eq!(convert_number("800000080008"), "はっせんおくはちまんはち");
    assert_eq!(convert_number("800000800000"), "はっせんおくはちじゅうまん");
    assert_eq!(convert_number("800008008008"), "はっせんおくはっぴゃくまんはっせんはち");
    assert_eq!(convert_number("800080008000"), "はっせんおくはっせんまんはっせん");
    assert_eq!(convert_number("800800800800"), "はっせんはちおくはちじゅうまんはっぴゃく");
    assert_eq!(convert_number("808080808080"), "はっせんはちじゅうおくはっせんはちじゅうまんはっせんはちじゅう");
    assert_eq!(convert_number("888888888888"), "はっせんはっぴゃくはちじゅうはちおくはっせんはっぴゃくはちじゅうはちまんはっせんはっぴゃくはちじゅうはち");
    assert_eq!(convert_number("900000090009"), "きゅうせんおくきゅうまんきゅう");
    assert_eq!(convert_number("900000900000"), "きゅうせんおくきゅうじゅうまん");
    assert_eq!(convert_number("900009009009"), "きゅうせんおくきゅうひゃくまんきゅうせんきゅう");
    assert_eq!(convert_number("900090009000"), "きゅうせんおくきゅうせんまんきゅうせん");
    assert_eq!(convert_number("900900900900"), "きゅうせんきゅうおくきゅうじゅうまんきゅうひゃく");
    assert_eq!(convert_number("909090909090"), "きゅうせんきゅうじゅうおくきゅうせんきゅうじゅうまんきゅうせんきゅうじゅう");
    assert_eq!(convert_number("999999999999"), "きゅうせんきゅうひゃくきゅうじゅうきゅうおくきゅうせんきゅうひゃくきゅうじゅうきゅうまんきゅうせんきゅうひゃくきゅうじゅうきゅう");
    assert_eq!(convert_number("1000000000000"), "いっちょう");
    assert_eq!(convert_number("1111111111111"), "いっちょういっせんひゃくじゅういちおくいっせんひゃくじゅういちまんいっせんひゃくじゅういち");
    assert_eq!(convert_number("2222222222222"), "にちょうにせんにひゃくにじゅうにおくにせんにひゃくにじゅうにまんにせんにひゃくにじゅうに");
    assert_eq!(convert_number("3333333333333"), "さんちょうさんぜんさんびゃくさんじゅうさんおくさんぜんさんびゃくさんじゅうさんまんさんぜんさんびゃくさんじゅうさん");
    assert_eq!(convert_number("4444444444444"), "よんちょうよんせんよんひゃくよんじゅうよんおくよんせんよんひゃくよんじゅうよんまんよんせんよんひゃくよんじゅうよん");
    assert_eq!(convert_number("5555555555555"), "ごちょうごせんごひゃくごじゅうごおくごせんごひゃくごじゅうごまんごせんごひゃくごじゅうご");
    assert_eq!(convert_number("6666666666666"), "ろくちょうろくせんろっぴゃくろくじゅうろくおくろくせんろっぴゃくろくじゅうろくまんろくせんろっぴゃくろくじゅうろく");
    assert_eq!(convert_number("7777777777777"), "ななちょうななせんななひゃくななじゅうななおくななせんななひゃくななじゅうななまんななせんななひゃくななじゅうなな");
    assert_eq!(convert_number("8888888888888"), "はちちょうはっせんはっぴゃくはちじゅうはちおくはっせんはっぴゃくはちじゅうはちまんはっせんはっぴゃくはちじゅうはち");
    assert_eq!(convert_number("9999999999999"), "きゅうちょうきゅうせんきゅうひゃくきゅうじゅうきゅうおくきゅうせんきゅうひゃくきゅうじゅうきゅうまんきゅうせんきゅうひゃくきゅうじゅうきゅう");
    assert_eq!(convert_number("10000000000000"), "じゅうちょう");
    assert_eq!(convert_number("10101010101010"), "じゅうちょういっせんじゅうおくいっせんじゅうまんいっせんじゅう");
    assert_eq!(convert_number("11111111111111"), "じゅういっちょういっせんひゃくじゅういちおくいっせんひゃくじゅういちまんいっせんひゃくじゅういち");
    assert_eq!(convert_number("20202020202020"), "にじゅうちょうにせんにじゅうおくにせんにじゅうまんにせんにじゅう");
    assert_eq!(convert_number("22222222222222"), "にじゅうにちょうにせんにひゃくにじゅうにおくにせんにひゃくにじゅうにまんにせんにひゃくにじゅうに");
    assert_eq!(convert_number("30303030303030"), "さんじゅうちょうさんぜんさんじゅうおくさんぜんさんじゅうまんさんぜんさんじゅう");
    assert_eq!(convert_number("33333333333333"), "さんじゅうさんちょうさんぜんさんびゃくさんじゅうさんおくさんぜんさんびゃくさんじゅうさんまんさんぜんさんびゃくさんじゅうさん");
    assert_eq!(convert_number("40404040404040"), "よんじゅうちょうよんせんよんじゅうおくよんせんよんじゅうまんよんせんよんじゅう");
    assert_eq!(convert_number("44444444444444"), "よんじゅうよんちょうよんせんよんひゃくよんじゅうよんおくよんせんよんひゃくよんじゅうよんまんよんせんよんひゃくよんじゅうよん");
    assert_eq!(convert_number("50505050505050"), "ごじゅうちょうごせんごじゅうおくごせんごじゅうまんごせんごじゅう");
    assert_eq!(convert_number("55555555555555"), "ごじゅうごちょうごせんごひゃくごじゅうごおくごせんごひゃくごじゅうごまんごせんごひゃくごじゅうご");
    assert_eq!(convert_number("60606060606060"), "ろくじゅうちょうろくせんろくじゅうおくろくせんろくじゅうまんろくせんろくじゅう");
    assert_eq!(convert_number("66666666666666"), "ろくじゅうろくちょうろくせんろっぴゃくろくじゅうろくおくろくせんろっぴゃくろくじゅうろくまんろくせんろっぴゃくろくじゅうろく");
    assert_eq!(convert_number("70707070707070"), "ななじゅうちょうななせんななじゅうおくななせんななじゅうまんななせんななじゅう");
    assert_eq!(convert_number("77777777777777"), "ななじゅうななちょうななせんななひゃくななじゅうななおくななせんななひゃくななじゅうななまんななせんななひゃくななじゅうなな");
    assert_eq!(convert_number("80808080808080"), "はちじゅうちょうはっせんはちじゅうおくはっせんはちじゅうまんはっせんはちじゅう");
    assert_eq!(convert_number("88888888888888"), "はちじゅうはちちょうはっせんはっぴゃくはちじゅうはちおくはっせんはっぴゃくはちじゅうはちまんはっせんはっぴゃくはちじゅうはち");
    assert_eq!(convert_number("90909090909090"), "きゅうじゅうちょうきゅうせんきゅうじゅうおくきゅうせんきゅうじゅうまんきゅうせんきゅうじゅう");
    assert_eq!(convert_number("99999999999999"), "きゅうじゅうきゅうちょうきゅうせんきゅうひゃくきゅうじゅうきゅうおくきゅうせんきゅうひゃくきゅうじゅうきゅうまんきゅうせんきゅうひゃくきゅうじゅうきゅう");
    assert_eq!(convert_number("100000000000000"), "ひゃくちょう");
    assert_eq!(convert_number("100000000100001"), "ひゃくちょうじゅうまんいち");
    assert_eq!(convert_number("100001000010000"), "ひゃくちょうじゅうおくいちまん");
    assert_eq!(convert_number("100001001001001"), "ひゃくちょうじゅうおくひゃくまんいっせんいち");
    assert_eq!(convert_number("100100100100100"), "ひゃくちょういっせんいちおくじゅうまんひゃく");
    assert_eq!(convert_number("111111111111111"), "ひゃくじゅういっちょういっせんひゃくじゅういちおくいっせんひゃくじゅういちまんいっせんひゃくじゅういち");
    assert_eq!(convert_number("200000000200002"), "にひゃくちょうにじゅうまんに");
    assert_eq!(convert_number("200002000020000"), "にひゃくちょうにじゅうおくにまん");
    assert_eq!(convert_number("200002002002002"), "にひゃくちょうにじゅうおくにひゃくまんにせんに");
    assert_eq!(convert_number("200200200200200"), "にひゃくちょうにせんにおくにじゅうまんにひゃく");
    assert_eq!(convert_number("222222222222222"), "にひゃくにじゅうにちょうにせんにひゃくにじゅうにおくにせんにひゃくにじゅうにまんにせんにひゃくにじゅうに");
    assert_eq!(convert_number("300000000300003"), "さんびゃくちょうさんじゅうまんさん");
    assert_eq!(convert_number("300003000030000"), "さんびゃくちょうさんじゅうおくさんまん");
    assert_eq!(convert_number("300003003003003"), "さんびゃくちょうさんじゅうおくさんびゃくまんさんぜんさん");
    assert_eq!(convert_number("300300300300300"), "さんびゃくちょうさんぜんさんおくさんじゅうまんさんびゃく");
    assert_eq!(convert_number("333333333333333"), "さんびゃくさんじゅうさんちょうさんぜんさんびゃくさんじゅうさんおくさんぜんさんびゃくさんじゅうさんまんさんぜんさんびゃくさんじゅうさん");
    assert_eq!(convert_number("400000000400004"), "よんひゃくちょうよんじゅうまんよん");
    assert_eq!(convert_number("400004000040000"), "よんひゃくちょうよんじゅうおくよんまん");
    assert_eq!(convert_number("400004004004004"), "よんひゃくちょうよんじゅうおくよんひゃくまんよんせんよん");
    assert_eq!(convert_number("400400400400400"), "よんひゃくちょうよんせんよんおくよんじゅうまんよんひゃく");
    assert_eq!(convert_number("444444444444444"), "よんひゃくよんじゅうよんちょうよんせんよんひゃくよんじゅうよんおくよんせんよんひゃくよんじゅうよんまんよんせんよんひゃくよんじゅうよん");
    assert_eq!(convert_number("500000000500005"), "ごひゃくちょうごじゅうまんご");
    assert_eq!(convert_number("500005000050000"), "ごひゃくちょうごじゅうおくごまん");
    assert_eq!(convert_number("500005005005005"), "ごひゃくちょうごじゅうおくごひゃくまんごせんご");
    assert_eq!(convert_number("500500500500500"), "ごひゃくちょうごせんごおくごじゅうまんごひゃく");
    assert_eq!(convert_number("555555555555555"), "ごひゃくごじゅうごちょうごせんごひゃくごじゅうごおくごせんごひゃくごじゅうごまんごせんごひゃくごじゅうご");
    assert_eq!(convert_number("600000000600006"), "ろっぴゃくちょうろくじゅうまんろく");
    assert_eq!(convert_number("600006000060000"), "ろっぴゃくちょうろくじゅうおくろくまん");
    assert_eq!(convert_number("600006006006006"), "ろっぴゃくちょうろくじゅうおくろっぴゃくまんろくせんろく");
    assert_eq!(convert_number("600600600600600"), "ろっぴゃくちょうろくせんろくおくろくじゅうまんろっぴゃく");
    assert_eq!(convert_number("666666666666666"), "ろっぴゃくろくじゅうろくちょうろくせんろっぴゃくろくじゅうろくおくろくせんろっぴゃくろくじゅうろくまんろくせんろっぴゃくろくじゅうろく");
    assert_eq!(convert_number("700000000700007"), "ななひゃくちょうななじゅうまんなな");
    assert_eq!(convert_number("700007000070000"), "ななひゃくちょうななじゅうおくななまん");
    assert_eq!(convert_number("700007007007007"), "ななひゃくちょうななじゅうおくななひゃくまんななせんなな");
    assert_eq!(convert_number("700700700700700"), "ななひゃくちょうななせんななおくななじゅうまんななひゃく");
    assert_eq!(convert_number("777777777777777"), "ななひゃくななじゅうななちょうななせんななひゃくななじゅうななおくななせんななひゃくななじゅうななまんななせんななひゃくななじゅうなな");
    assert_eq!(convert_number("800000000800008"), "はっぴゃくちょうはちじゅうまんはち");
    assert_eq!(convert_number("800008000080000"), "はっぴゃくちょうはちじゅうおくはちまん");
    assert_eq!(convert_number("800008008008008"), "はっぴゃくちょうはちじゅうおくはっぴゃくまんはっせんはち");
    assert_eq!(convert_number("800800800800800"), "はっぴゃくちょうはっせんはちおくはちじゅうまんはっぴゃく");
    assert_eq!(convert_number("888888888888888"), "はっぴゃくはちじゅうはちちょうはっせんはっぴゃくはちじゅうはちおくはっせんはっぴゃくはちじゅうはちまんはっせんはっぴゃくはちじゅうはち");
    assert_eq!(convert_number("900000000900009"), "きゅうひゃくちょうきゅうじゅうまんきゅう");
    assert_eq!(convert_number("900009000090000"), "きゅうひゃくちょうきゅうじゅうおくきゅうまん");
    assert_eq!(convert_number("900009009009009"), "きゅうひゃくちょうきゅうじゅうおくきゅうひゃくまんきゅうせんきゅう");
    assert_eq!(convert_number("900900900900900"), "きゅうひゃくちょうきゅうせんきゅうおくきゅうじゅうまんきゅうひゃく");
    assert_eq!(convert_number("999999999999999"), "きゅうひゃくきゅうじゅうきゅうちょうきゅうせんきゅうひゃくきゅうじゅうきゅうおくきゅうせんきゅうひゃくきゅうじゅうきゅうまんきゅうせんきゅうひゃくきゅうじゅうきゅう");
    assert_eq!(convert_number("1000000000000000"), "いっせんちょう");
    assert_eq!(convert_number("1000000100010001"), "いっせんちょういちおくいちまんいち");
    assert_eq!(convert_number("1000100010001000"), "いっせんちょういっせんおくいっせんまんいっせん");
    assert_eq!(convert_number("1010101010101010"), "いっせんじゅうちょういっせんじゅうおくいっせんじゅうまんいっせんじゅう");
    assert_eq!(convert_number("1111111111111111"), "いっせんひゃくじゅういっちょういっせんひゃくじゅういちおくいっせんひゃくじゅういちまんいっせんひゃくじゅういち");
    assert_eq!(convert_number("2000000200020002"), "にせんちょうにおくにまんに");
    assert_eq!(convert_number("2000200020002000"), "にせんちょうにせんおくにせんまんにせん");
    assert_eq!(convert_number("2020202020202020"), "にせんにじゅうちょうにせんにじゅうおくにせんにじゅうまんにせんにじゅう");
    assert_eq!(convert_number("2222222222222222"), "にせんにひゃくにじゅうにちょうにせんにひゃくにじゅうにおくにせんにひゃくにじゅうにまんにせんにひゃくにじゅうに");
    assert_eq!(convert_number("3000000300030003"), "さんぜんちょうさんおくさんまんさん");
    assert_eq!(convert_number("3000300030003000"), "さんぜんちょうさんぜんおくさんぜんまんさんぜん");
    assert_eq!(convert_number("3030303030303030"), "さんぜんさんじゅうちょうさんぜんさんじゅうおくさんぜんさんじゅうまんさんぜんさんじゅう");
    assert_eq!(convert_number("3333333333333333"), "さんぜんさんびゃくさんじゅうさんちょうさんぜんさんびゃくさんじゅうさんおくさんぜんさんびゃくさんじゅうさんまんさんぜんさんびゃくさんじゅうさん");
    assert_eq!(convert_number("4000000400040004"), "よんせんちょうよんおくよんまんよん");
    assert_eq!(convert_number("4000400040004000"), "よんせんちょうよんせんおくよんせんまんよんせん");
    assert_eq!(convert_number("4040404040404040"), "よんせんよんじゅうちょうよんせんよんじゅうおくよんせんよんじゅうまんよんせんよんじゅう");
    assert_eq!(convert_number("4444444444444444"), "よんせんよんひゃくよんじゅうよんちょうよんせんよんひゃくよんじゅうよんおくよんせんよんひゃくよんじゅうよんまんよんせんよんひゃくよんじゅうよん");
    assert_eq!(convert_number("5000000500050005"), "ごせんちょうごおくごまんご");
    assert_eq!(convert_number("5000500050005000"), "ごせんちょうごせんおくごせんまんごせん");
    assert_eq!(convert_number("5050505050505050"), "ごせんごじゅうちょうごせんごじゅうおくごせんごじゅうまんごせんごじゅう");
    assert_eq!(convert_number("5555555555555555"), "ごせんごひゃくごじゅうごちょうごせんごひゃくごじゅうごおくごせんごひゃくごじゅうごまんごせんごひゃくごじゅうご");
    assert_eq!(convert_number("6000000600060006"), "ろくせんちょうろくおくろくまんろく");
    assert_eq!(convert_number("6000600060006000"), "ろくせんちょうろくせんおくろくせんまんろくせん");
    assert_eq!(convert_number("6060606060606060"), "ろくせんろくじゅうちょうろくせんろくじゅうおくろくせんろくじゅうまんろくせんろくじゅう");
    assert_eq!(convert_number("6666666666666666"), "ろくせんろっぴゃくろくじゅうろくちょうろくせんろっぴゃくろくじゅうろくおくろくせんろっぴゃくろくじゅうろくまんろくせんろっぴゃくろくじゅうろく");
    assert_eq!(convert_number("7000000700070007"), "ななせんちょうななおくななまんなな");
    assert_eq!(convert_number("7000700070007000"), "ななせんちょうななせんおくななせんまんななせん");
    assert_eq!(convert_number("7070707070707070"), "ななせんななじゅうちょうななせんななじゅうおくななせんななじゅうまんななせんななじゅう");
    assert_eq!(convert_number("7777777777777777"), "ななせんななひゃくななじゅうななちょうななせんななひゃくななじゅうななおくななせんななひゃくななじゅうななまんななせんななひゃくななじゅうなな");
    assert_eq!(convert_number("8000000800080008"), "はっせんちょうはちおくはちまんはち");
    assert_eq!(convert_number("8000800080008000"), "はっせんちょうはっせんおくはっせんまんはっせん");
    assert_eq!(convert_number("8080808080808080"), "はっせんはちじゅうちょうはっせんはちじゅうおくはっせんはちじゅうまんはっせんはちじゅう");
    assert_eq!(convert_number("8888888888888888"), "はっせんはっぴゃくはちじゅうはちちょうはっせんはっぴゃくはちじゅうはちおくはっせんはっぴゃくはちじゅうはちまんはっせんはっぴゃくはちじゅうはち");
    assert_eq!(convert_number("9000000900090009"), "きゅうせんちょうきゅうおくきゅうまんきゅう");
    assert_eq!(convert_number("9000900090009000"), "きゅうせんちょうきゅうせんおくきゅうせんまんきゅうせん");
    assert_eq!(convert_number("9090909090909090"), "きゅうせんきゅうじゅうちょうきゅうせんきゅうじゅうおくきゅうせんきゅうじゅうまんきゅうせんきゅうじゅう");
    assert_eq!(convert_number("9999999999999999"), "きゅうせんきゅうひゃくきゅうじゅうきゅうちょうきゅうせんきゅうひゃくきゅうじゅうきゅうおくきゅうせんきゅうひゃくきゅうじゅうきゅうまんきゅうせんきゅうひゃくきゅうじゅうきゅう");
}