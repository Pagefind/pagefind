//! Generated from serbian.sbl by Snowball 3.0.0 - https://snowballstem.org/

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_mut)]
#![allow(unused_parens)]
#![allow(unused_variables)]
use snowball::SnowballEnv;
use snowball::Among;

#[derive(Clone)]
struct Context {
    i_p1: i32,
    b_no_diacritics: bool,
}

static A_0: &'static [Among<Context>; 30] = &[
    Among("а", -1, 1, None),
    Among("б", -1, 2, None),
    Among("в", -1, 3, None),
    Among("г", -1, 4, None),
    Among("д", -1, 5, None),
    Among("е", -1, 7, None),
    Among("ж", -1, 8, None),
    Among("з", -1, 9, None),
    Among("и", -1, 10, None),
    Among("к", -1, 12, None),
    Among("л", -1, 13, None),
    Among("м", -1, 15, None),
    Among("н", -1, 16, None),
    Among("о", -1, 18, None),
    Among("п", -1, 19, None),
    Among("р", -1, 20, None),
    Among("с", -1, 21, None),
    Among("т", -1, 22, None),
    Among("у", -1, 24, None),
    Among("ф", -1, 25, None),
    Among("х", -1, 26, None),
    Among("ц", -1, 27, None),
    Among("ч", -1, 28, None),
    Among("ш", -1, 30, None),
    Among("ђ", -1, 6, None),
    Among("ј", -1, 11, None),
    Among("љ", -1, 14, None),
    Among("њ", -1, 17, None),
    Among("ћ", -1, 23, None),
    Among("џ", -1, 29, None),
];

static A_1: &'static [Among<Context>; 130] = &[
    Among("daba", -1, 73, None),
    Among("ajaca", -1, 12, None),
    Among("ejaca", -1, 14, None),
    Among("ljaca", -1, 13, None),
    Among("njaca", -1, 85, None),
    Among("ojaca", -1, 15, None),
    Among("alaca", -1, 82, None),
    Among("elaca", -1, 83, None),
    Among("olaca", -1, 84, None),
    Among("maca", -1, 75, None),
    Among("naca", -1, 76, None),
    Among("raca", -1, 81, None),
    Among("saca", -1, 80, None),
    Among("vaca", -1, 79, None),
    Among("šaca", -1, 18, None),
    Among("aoca", -1, 82, None),
    Among("acaka", -1, 55, None),
    Among("ajaka", -1, 16, None),
    Among("ojaka", -1, 17, None),
    Among("anaka", -1, 78, None),
    Among("ataka", -1, 58, None),
    Among("etaka", -1, 59, None),
    Among("itaka", -1, 60, None),
    Among("otaka", -1, 61, None),
    Among("utaka", -1, 62, None),
    Among("ačaka", -1, 54, None),
    Among("esama", -1, 67, None),
    Among("izama", -1, 87, None),
    Among("jacima", -1, 5, None),
    Among("nicima", -1, 23, None),
    Among("ticima", -1, 24, None),
    Among("teticima", 30, 21, None),
    Among("zicima", -1, 25, None),
    Among("atcima", -1, 58, None),
    Among("utcima", -1, 62, None),
    Among("čcima", -1, 74, None),
    Among("pesima", -1, 2, None),
    Among("inzima", -1, 19, None),
    Among("lozima", -1, 1, None),
    Among("metara", -1, 68, None),
    Among("centara", -1, 69, None),
    Among("istara", -1, 70, None),
    Among("ekata", -1, 86, None),
    Among("anata", -1, 53, None),
    Among("nstava", -1, 22, None),
    Among("kustava", -1, 29, None),
    Among("ajac", -1, 12, None),
    Among("ejac", -1, 14, None),
    Among("ljac", -1, 13, None),
    Among("njac", -1, 85, None),
    Among("anjac", 49, 11, None),
    Among("ojac", -1, 15, None),
    Among("alac", -1, 82, None),
    Among("elac", -1, 83, None),
    Among("olac", -1, 84, None),
    Among("mac", -1, 75, None),
    Among("nac", -1, 76, None),
    Among("rac", -1, 81, None),
    Among("sac", -1, 80, None),
    Among("vac", -1, 79, None),
    Among("šac", -1, 18, None),
    Among("jebe", -1, 88, None),
    Among("olce", -1, 84, None),
    Among("kuse", -1, 27, None),
    Among("rave", -1, 42, None),
    Among("save", -1, 52, None),
    Among("šave", -1, 51, None),
    Among("baci", -1, 89, None),
    Among("jaci", -1, 5, None),
    Among("tvenici", -1, 20, None),
    Among("snici", -1, 26, None),
    Among("tetici", -1, 21, None),
    Among("bojci", -1, 4, None),
    Among("vojci", -1, 3, None),
    Among("ojsci", -1, 66, None),
    Among("atci", -1, 58, None),
    Among("itci", -1, 60, None),
    Among("utci", -1, 62, None),
    Among("čci", -1, 74, None),
    Among("pesi", -1, 2, None),
    Among("inzi", -1, 19, None),
    Among("lozi", -1, 1, None),
    Among("acak", -1, 55, None),
    Among("usak", -1, 57, None),
    Among("atak", -1, 58, None),
    Among("etak", -1, 59, None),
    Among("itak", -1, 60, None),
    Among("otak", -1, 61, None),
    Among("utak", -1, 62, None),
    Among("ačak", -1, 54, None),
    Among("ušak", -1, 56, None),
    Among("izam", -1, 87, None),
    Among("tican", -1, 65, None),
    Among("cajan", -1, 7, None),
    Among("čajan", -1, 6, None),
    Among("voljan", -1, 77, None),
    Among("eskan", -1, 63, None),
    Among("alan", -1, 40, None),
    Among("bilan", -1, 33, None),
    Among("gilan", -1, 37, None),
    Among("nilan", -1, 39, None),
    Among("rilan", -1, 38, None),
    Among("silan", -1, 36, None),
    Among("tilan", -1, 34, None),
    Among("avilan", -1, 35, None),
    Among("laran", -1, 9, None),
    Among("eran", -1, 8, None),
    Among("asan", -1, 91, None),
    Among("esan", -1, 10, None),
    Among("dusan", -1, 31, None),
    Among("kusan", -1, 28, None),
    Among("atan", -1, 47, None),
    Among("pletan", -1, 50, None),
    Among("tetan", -1, 49, None),
    Among("antan", -1, 32, None),
    Among("pravan", -1, 44, None),
    Among("stavan", -1, 43, None),
    Among("sivan", -1, 46, None),
    Among("tivan", -1, 45, None),
    Among("ozan", -1, 41, None),
    Among("tičan", -1, 64, None),
    Among("ašan", -1, 90, None),
    Among("dušan", -1, 30, None),
    Among("metar", -1, 68, None),
    Among("centar", -1, 69, None),
    Among("istar", -1, 70, None),
    Among("ekat", -1, 86, None),
    Among("enat", -1, 48, None),
    Among("oscu", -1, 72, None),
    Among("ošću", -1, 71, None),
];

static A_2: &'static [Among<Context>; 2035] = &[
    Among("aca", -1, 124, None),
    Among("eca", -1, 125, None),
    Among("uca", -1, 126, None),
    Among("ga", -1, 20, None),
    Among("acega", 3, 124, None),
    Among("ecega", 3, 125, None),
    Among("ucega", 3, 126, None),
    Among("anjijega", 3, 84, None),
    Among("enjijega", 3, 85, None),
    Among("snjijega", 3, 122, None),
    Among("šnjijega", 3, 86, None),
    Among("kijega", 3, 95, None),
    Among("skijega", 11, 1, None),
    Among("škijega", 11, 2, None),
    Among("elijega", 3, 83, None),
    Among("nijega", 3, 13, None),
    Among("osijega", 3, 123, None),
    Among("atijega", 3, 120, None),
    Among("evitijega", 3, 92, None),
    Among("ovitijega", 3, 93, None),
    Among("astijega", 3, 94, None),
    Among("avijega", 3, 77, None),
    Among("evijega", 3, 78, None),
    Among("ivijega", 3, 79, None),
    Among("ovijega", 3, 80, None),
    Among("ošijega", 3, 91, None),
    Among("anjega", 3, 84, None),
    Among("enjega", 3, 85, None),
    Among("snjega", 3, 122, None),
    Among("šnjega", 3, 86, None),
    Among("kega", 3, 95, None),
    Among("skega", 30, 1, None),
    Among("škega", 30, 2, None),
    Among("elega", 3, 83, None),
    Among("nega", 3, 13, None),
    Among("anega", 34, 10, None),
    Among("enega", 34, 87, None),
    Among("snega", 34, 159, None),
    Among("šnega", 34, 88, None),
    Among("osega", 3, 123, None),
    Among("atega", 3, 120, None),
    Among("evitega", 3, 92, None),
    Among("ovitega", 3, 93, None),
    Among("astega", 3, 94, None),
    Among("avega", 3, 77, None),
    Among("evega", 3, 78, None),
    Among("ivega", 3, 79, None),
    Among("ovega", 3, 80, None),
    Among("aćega", 3, 14, None),
    Among("ećega", 3, 15, None),
    Among("ućega", 3, 16, None),
    Among("ošega", 3, 91, None),
    Among("acoga", 3, 124, None),
    Among("ecoga", 3, 125, None),
    Among("ucoga", 3, 126, None),
    Among("anjoga", 3, 84, None),
    Among("enjoga", 3, 85, None),
    Among("snjoga", 3, 122, None),
    Among("šnjoga", 3, 86, None),
    Among("koga", 3, 95, None),
    Among("skoga", 59, 1, None),
    Among("škoga", 59, 2, None),
    Among("loga", 3, 19, None),
    Among("eloga", 62, 83, None),
    Among("noga", 3, 13, None),
    Among("cinoga", 64, 137, None),
    Among("činoga", 64, 89, None),
    Among("osoga", 3, 123, None),
    Among("atoga", 3, 120, None),
    Among("evitoga", 3, 92, None),
    Among("ovitoga", 3, 93, None),
    Among("astoga", 3, 94, None),
    Among("avoga", 3, 77, None),
    Among("evoga", 3, 78, None),
    Among("ivoga", 3, 79, None),
    Among("ovoga", 3, 80, None),
    Among("aćoga", 3, 14, None),
    Among("ećoga", 3, 15, None),
    Among("ućoga", 3, 16, None),
    Among("ošoga", 3, 91, None),
    Among("uga", 3, 18, None),
    Among("aja", -1, 109, None),
    Among("caja", 81, 26, None),
    Among("laja", 81, 30, None),
    Among("raja", 81, 31, None),
    Among("ćaja", 81, 28, None),
    Among("čaja", 81, 27, None),
    Among("đaja", 81, 29, None),
    Among("bija", -1, 32, None),
    Among("cija", -1, 33, None),
    Among("dija", -1, 34, None),
    Among("fija", -1, 40, None),
    Among("gija", -1, 39, None),
    Among("anjija", -1, 84, None),
    Among("enjija", -1, 85, None),
    Among("snjija", -1, 122, None),
    Among("šnjija", -1, 86, None),
    Among("kija", -1, 95, None),
    Among("skija", 97, 1, None),
    Among("škija", 97, 2, None),
    Among("lija", -1, 24, None),
    Among("elija", 100, 83, None),
    Among("mija", -1, 37, None),
    Among("nija", -1, 13, None),
    Among("ganija", 103, 9, None),
    Among("manija", 103, 6, None),
    Among("panija", 103, 7, None),
    Among("ranija", 103, 8, None),
    Among("tanija", 103, 5, None),
    Among("pija", -1, 41, None),
    Among("rija", -1, 42, None),
    Among("rarija", 110, 21, None),
    Among("sija", -1, 23, None),
    Among("osija", 112, 123, None),
    Among("tija", -1, 44, None),
    Among("atija", 114, 120, None),
    Among("evitija", 114, 92, None),
    Among("ovitija", 114, 93, None),
    Among("otija", 114, 22, None),
    Among("astija", 114, 94, None),
    Among("avija", -1, 77, None),
    Among("evija", -1, 78, None),
    Among("ivija", -1, 79, None),
    Among("ovija", -1, 80, None),
    Among("zija", -1, 45, None),
    Among("ošija", -1, 91, None),
    Among("žija", -1, 38, None),
    Among("anja", -1, 84, None),
    Among("enja", -1, 85, None),
    Among("snja", -1, 122, None),
    Among("šnja", -1, 86, None),
    Among("ka", -1, 95, None),
    Among("ska", 131, 1, None),
    Among("ška", 131, 2, None),
    Among("ala", -1, 104, None),
    Among("acala", 134, 128, None),
    Among("astajala", 134, 106, None),
    Among("istajala", 134, 107, None),
    Among("ostajala", 134, 108, None),
    Among("ijala", 134, 47, None),
    Among("injala", 134, 114, None),
    Among("nala", 134, 46, None),
    Among("irala", 134, 100, None),
    Among("urala", 134, 105, None),
    Among("tala", 134, 113, None),
    Among("astala", 144, 110, None),
    Among("istala", 144, 111, None),
    Among("ostala", 144, 112, None),
    Among("avala", 134, 97, None),
    Among("evala", 134, 96, None),
    Among("ivala", 134, 98, None),
    Among("ovala", 134, 76, None),
    Among("uvala", 134, 99, None),
    Among("ačala", 134, 102, None),
    Among("ela", -1, 83, None),
    Among("ila", -1, 116, None),
    Among("acila", 155, 124, None),
    Among("lucila", 155, 121, None),
    Among("nila", 155, 103, None),
    Among("astanila", 158, 110, None),
    Among("istanila", 158, 111, None),
    Among("ostanila", 158, 112, None),
    Among("rosila", 155, 127, None),
    Among("jetila", 155, 118, None),
    Among("ozila", 155, 48, None),
    Among("ačila", 155, 101, None),
    Among("lučila", 155, 117, None),
    Among("rošila", 155, 90, None),
    Among("ola", -1, 50, None),
    Among("asla", -1, 115, None),
    Among("nula", -1, 13, None),
    Among("gama", -1, 20, None),
    Among("logama", 171, 19, None),
    Among("ugama", 171, 18, None),
    Among("ajama", -1, 109, None),
    Among("cajama", 174, 26, None),
    Among("lajama", 174, 30, None),
    Among("rajama", 174, 31, None),
    Among("ćajama", 174, 28, None),
    Among("čajama", 174, 27, None),
    Among("đajama", 174, 29, None),
    Among("bijama", -1, 32, None),
    Among("cijama", -1, 33, None),
    Among("dijama", -1, 34, None),
    Among("fijama", -1, 40, None),
    Among("gijama", -1, 39, None),
    Among("lijama", -1, 35, None),
    Among("mijama", -1, 37, None),
    Among("nijama", -1, 36, None),
    Among("ganijama", 188, 9, None),
    Among("manijama", 188, 6, None),
    Among("panijama", 188, 7, None),
    Among("ranijama", 188, 8, None),
    Among("tanijama", 188, 5, None),
    Among("pijama", -1, 41, None),
    Among("rijama", -1, 42, None),
    Among("sijama", -1, 43, None),
    Among("tijama", -1, 44, None),
    Among("zijama", -1, 45, None),
    Among("žijama", -1, 38, None),
    Among("alama", -1, 104, None),
    Among("ijalama", 200, 47, None),
    Among("nalama", 200, 46, None),
    Among("elama", -1, 119, None),
    Among("ilama", -1, 116, None),
    Among("ramama", -1, 52, None),
    Among("lemama", -1, 51, None),
    Among("inama", -1, 11, None),
    Among("cinama", 207, 137, None),
    Among("činama", 207, 89, None),
    Among("rama", -1, 52, None),
    Among("arama", 210, 53, None),
    Among("drama", 210, 54, None),
    Among("erama", 210, 55, None),
    Among("orama", 210, 56, None),
    Among("basama", -1, 135, None),
    Among("gasama", -1, 131, None),
    Among("jasama", -1, 129, None),
    Among("kasama", -1, 133, None),
    Among("nasama", -1, 132, None),
    Among("tasama", -1, 130, None),
    Among("vasama", -1, 134, None),
    Among("esama", -1, 152, None),
    Among("isama", -1, 154, None),
    Among("etama", -1, 70, None),
    Among("estama", -1, 71, None),
    Among("istama", -1, 72, None),
    Among("kstama", -1, 73, None),
    Among("ostama", -1, 74, None),
    Among("avama", -1, 77, None),
    Among("evama", -1, 78, None),
    Among("ivama", -1, 79, None),
    Among("bašama", -1, 63, None),
    Among("gašama", -1, 64, None),
    Among("jašama", -1, 61, None),
    Among("kašama", -1, 62, None),
    Among("našama", -1, 60, None),
    Among("tašama", -1, 59, None),
    Among("vašama", -1, 65, None),
    Among("ešama", -1, 66, None),
    Among("išama", -1, 67, None),
    Among("lema", -1, 51, None),
    Among("acima", -1, 124, None),
    Among("ecima", -1, 125, None),
    Among("ucima", -1, 126, None),
    Among("ajima", -1, 109, None),
    Among("cajima", 245, 26, None),
    Among("lajima", 245, 30, None),
    Among("rajima", 245, 31, None),
    Among("ćajima", 245, 28, None),
    Among("čajima", 245, 27, None),
    Among("đajima", 245, 29, None),
    Among("bijima", -1, 32, None),
    Among("cijima", -1, 33, None),
    Among("dijima", -1, 34, None),
    Among("fijima", -1, 40, None),
    Among("gijima", -1, 39, None),
    Among("anjijima", -1, 84, None),
    Among("enjijima", -1, 85, None),
    Among("snjijima", -1, 122, None),
    Among("šnjijima", -1, 86, None),
    Among("kijima", -1, 95, None),
    Among("skijima", 261, 1, None),
    Among("škijima", 261, 2, None),
    Among("lijima", -1, 35, None),
    Among("elijima", 264, 83, None),
    Among("mijima", -1, 37, None),
    Among("nijima", -1, 13, None),
    Among("ganijima", 267, 9, None),
    Among("manijima", 267, 6, None),
    Among("panijima", 267, 7, None),
    Among("ranijima", 267, 8, None),
    Among("tanijima", 267, 5, None),
    Among("pijima", -1, 41, None),
    Among("rijima", -1, 42, None),
    Among("sijima", -1, 43, None),
    Among("osijima", 275, 123, None),
    Among("tijima", -1, 44, None),
    Among("atijima", 277, 120, None),
    Among("evitijima", 277, 92, None),
    Among("ovitijima", 277, 93, None),
    Among("astijima", 277, 94, None),
    Among("avijima", -1, 77, None),
    Among("evijima", -1, 78, None),
    Among("ivijima", -1, 79, None),
    Among("ovijima", -1, 80, None),
    Among("zijima", -1, 45, None),
    Among("ošijima", -1, 91, None),
    Among("žijima", -1, 38, None),
    Among("anjima", -1, 84, None),
    Among("enjima", -1, 85, None),
    Among("snjima", -1, 122, None),
    Among("šnjima", -1, 86, None),
    Among("kima", -1, 95, None),
    Among("skima", 293, 1, None),
    Among("škima", 293, 2, None),
    Among("alima", -1, 104, None),
    Among("ijalima", 296, 47, None),
    Among("nalima", 296, 46, None),
    Among("elima", -1, 83, None),
    Among("ilima", -1, 116, None),
    Among("ozilima", 300, 48, None),
    Among("olima", -1, 50, None),
    Among("lemima", -1, 51, None),
    Among("nima", -1, 13, None),
    Among("anima", 304, 10, None),
    Among("inima", 304, 11, None),
    Among("cinima", 306, 137, None),
    Among("činima", 306, 89, None),
    Among("onima", 304, 12, None),
    Among("arima", -1, 53, None),
    Among("drima", -1, 54, None),
    Among("erima", -1, 55, None),
    Among("orima", -1, 56, None),
    Among("basima", -1, 135, None),
    Among("gasima", -1, 131, None),
    Among("jasima", -1, 129, None),
    Among("kasima", -1, 133, None),
    Among("nasima", -1, 132, None),
    Among("tasima", -1, 130, None),
    Among("vasima", -1, 134, None),
    Among("esima", -1, 57, None),
    Among("isima", -1, 58, None),
    Among("osima", -1, 123, None),
    Among("atima", -1, 120, None),
    Among("ikatima", 324, 68, None),
    Among("latima", 324, 69, None),
    Among("etima", -1, 70, None),
    Among("evitima", -1, 92, None),
    Among("ovitima", -1, 93, None),
    Among("astima", -1, 94, None),
    Among("estima", -1, 71, None),
    Among("istima", -1, 72, None),
    Among("kstima", -1, 73, None),
    Among("ostima", -1, 74, None),
    Among("ištima", -1, 75, None),
    Among("avima", -1, 77, None),
    Among("evima", -1, 78, None),
    Among("ajevima", 337, 109, None),
    Among("cajevima", 338, 26, None),
    Among("lajevima", 338, 30, None),
    Among("rajevima", 338, 31, None),
    Among("ćajevima", 338, 28, None),
    Among("čajevima", 338, 27, None),
    Among("đajevima", 338, 29, None),
    Among("ivima", -1, 79, None),
    Among("ovima", -1, 80, None),
    Among("govima", 346, 20, None),
    Among("ugovima", 347, 17, None),
    Among("lovima", 346, 82, None),
    Among("olovima", 349, 49, None),
    Among("movima", 346, 81, None),
    Among("onovima", 346, 12, None),
    Among("stvima", -1, 3, None),
    Among("štvima", -1, 4, None),
    Among("aćima", -1, 14, None),
    Among("ećima", -1, 15, None),
    Among("ućima", -1, 16, None),
    Among("bašima", -1, 63, None),
    Among("gašima", -1, 64, None),
    Among("jašima", -1, 61, None),
    Among("kašima", -1, 62, None),
    Among("našima", -1, 60, None),
    Among("tašima", -1, 59, None),
    Among("vašima", -1, 65, None),
    Among("ešima", -1, 66, None),
    Among("išima", -1, 67, None),
    Among("ošima", -1, 91, None),
    Among("na", -1, 13, None),
    Among("ana", 368, 10, None),
    Among("acana", 369, 128, None),
    Among("urana", 369, 105, None),
    Among("tana", 369, 113, None),
    Among("avana", 369, 97, None),
    Among("evana", 369, 96, None),
    Among("ivana", 369, 98, None),
    Among("uvana", 369, 99, None),
    Among("ačana", 369, 102, None),
    Among("acena", 368, 124, None),
    Among("lucena", 368, 121, None),
    Among("ačena", 368, 101, None),
    Among("lučena", 368, 117, None),
    Among("ina", 368, 11, None),
    Among("cina", 382, 137, None),
    Among("anina", 382, 10, None),
    Among("čina", 382, 89, None),
    Among("ona", 368, 12, None),
    Among("ara", -1, 53, None),
    Among("dra", -1, 54, None),
    Among("era", -1, 55, None),
    Among("ora", -1, 56, None),
    Among("basa", -1, 135, None),
    Among("gasa", -1, 131, None),
    Among("jasa", -1, 129, None),
    Among("kasa", -1, 133, None),
    Among("nasa", -1, 132, None),
    Among("tasa", -1, 130, None),
    Among("vasa", -1, 134, None),
    Among("esa", -1, 57, None),
    Among("isa", -1, 58, None),
    Among("osa", -1, 123, None),
    Among("ata", -1, 120, None),
    Among("ikata", 401, 68, None),
    Among("lata", 401, 69, None),
    Among("eta", -1, 70, None),
    Among("evita", -1, 92, None),
    Among("ovita", -1, 93, None),
    Among("asta", -1, 94, None),
    Among("esta", -1, 71, None),
    Among("ista", -1, 72, None),
    Among("ksta", -1, 73, None),
    Among("osta", -1, 74, None),
    Among("nuta", -1, 13, None),
    Among("išta", -1, 75, None),
    Among("ava", -1, 77, None),
    Among("eva", -1, 78, None),
    Among("ajeva", 415, 109, None),
    Among("cajeva", 416, 26, None),
    Among("lajeva", 416, 30, None),
    Among("rajeva", 416, 31, None),
    Among("ćajeva", 416, 28, None),
    Among("čajeva", 416, 27, None),
    Among("đajeva", 416, 29, None),
    Among("iva", -1, 79, None),
    Among("ova", -1, 80, None),
    Among("gova", 424, 20, None),
    Among("ugova", 425, 17, None),
    Among("lova", 424, 82, None),
    Among("olova", 427, 49, None),
    Among("mova", 424, 81, None),
    Among("onova", 424, 12, None),
    Among("stva", -1, 3, None),
    Among("štva", -1, 4, None),
    Among("aća", -1, 14, None),
    Among("eća", -1, 15, None),
    Among("uća", -1, 16, None),
    Among("baša", -1, 63, None),
    Among("gaša", -1, 64, None),
    Among("jaša", -1, 61, None),
    Among("kaša", -1, 62, None),
    Among("naša", -1, 60, None),
    Among("taša", -1, 59, None),
    Among("vaša", -1, 65, None),
    Among("eša", -1, 66, None),
    Among("iša", -1, 67, None),
    Among("oša", -1, 91, None),
    Among("ace", -1, 124, None),
    Among("ece", -1, 125, None),
    Among("uce", -1, 126, None),
    Among("luce", 448, 121, None),
    Among("astade", -1, 110, None),
    Among("istade", -1, 111, None),
    Among("ostade", -1, 112, None),
    Among("ge", -1, 20, None),
    Among("loge", 453, 19, None),
    Among("uge", 453, 18, None),
    Among("aje", -1, 104, None),
    Among("caje", 456, 26, None),
    Among("laje", 456, 30, None),
    Among("raje", 456, 31, None),
    Among("astaje", 456, 106, None),
    Among("istaje", 456, 107, None),
    Among("ostaje", 456, 108, None),
    Among("ćaje", 456, 28, None),
    Among("čaje", 456, 27, None),
    Among("đaje", 456, 29, None),
    Among("ije", -1, 116, None),
    Among("bije", 466, 32, None),
    Among("cije", 466, 33, None),
    Among("dije", 466, 34, None),
    Among("fije", 466, 40, None),
    Among("gije", 466, 39, None),
    Among("anjije", 466, 84, None),
    Among("enjije", 466, 85, None),
    Among("snjije", 466, 122, None),
    Among("šnjije", 466, 86, None),
    Among("kije", 466, 95, None),
    Among("skije", 476, 1, None),
    Among("škije", 476, 2, None),
    Among("lije", 466, 35, None),
    Among("elije", 479, 83, None),
    Among("mije", 466, 37, None),
    Among("nije", 466, 13, None),
    Among("ganije", 482, 9, None),
    Among("manije", 482, 6, None),
    Among("panije", 482, 7, None),
    Among("ranije", 482, 8, None),
    Among("tanije", 482, 5, None),
    Among("pije", 466, 41, None),
    Among("rije", 466, 42, None),
    Among("sije", 466, 43, None),
    Among("osije", 490, 123, None),
    Among("tije", 466, 44, None),
    Among("atije", 492, 120, None),
    Among("evitije", 492, 92, None),
    Among("ovitije", 492, 93, None),
    Among("astije", 492, 94, None),
    Among("avije", 466, 77, None),
    Among("evije", 466, 78, None),
    Among("ivije", 466, 79, None),
    Among("ovije", 466, 80, None),
    Among("zije", 466, 45, None),
    Among("ošije", 466, 91, None),
    Among("žije", 466, 38, None),
    Among("anje", -1, 84, None),
    Among("enje", -1, 85, None),
    Among("snje", -1, 122, None),
    Among("šnje", -1, 86, None),
    Among("uje", -1, 25, None),
    Among("lucuje", 508, 121, None),
    Among("iruje", 508, 100, None),
    Among("lučuje", 508, 117, None),
    Among("ke", -1, 95, None),
    Among("ske", 512, 1, None),
    Among("ške", 512, 2, None),
    Among("ale", -1, 104, None),
    Among("acale", 515, 128, None),
    Among("astajale", 515, 106, None),
    Among("istajale", 515, 107, None),
    Among("ostajale", 515, 108, None),
    Among("ijale", 515, 47, None),
    Among("injale", 515, 114, None),
    Among("nale", 515, 46, None),
    Among("irale", 515, 100, None),
    Among("urale", 515, 105, None),
    Among("tale", 515, 113, None),
    Among("astale", 525, 110, None),
    Among("istale", 525, 111, None),
    Among("ostale", 525, 112, None),
    Among("avale", 515, 97, None),
    Among("evale", 515, 96, None),
    Among("ivale", 515, 98, None),
    Among("ovale", 515, 76, None),
    Among("uvale", 515, 99, None),
    Among("ačale", 515, 102, None),
    Among("ele", -1, 83, None),
    Among("ile", -1, 116, None),
    Among("acile", 536, 124, None),
    Among("lucile", 536, 121, None),
    Among("nile", 536, 103, None),
    Among("rosile", 536, 127, None),
    Among("jetile", 536, 118, None),
    Among("ozile", 536, 48, None),
    Among("ačile", 536, 101, None),
    Among("lučile", 536, 117, None),
    Among("rošile", 536, 90, None),
    Among("ole", -1, 50, None),
    Among("asle", -1, 115, None),
    Among("nule", -1, 13, None),
    Among("rame", -1, 52, None),
    Among("leme", -1, 51, None),
    Among("acome", -1, 124, None),
    Among("ecome", -1, 125, None),
    Among("ucome", -1, 126, None),
    Among("anjome", -1, 84, None),
    Among("enjome", -1, 85, None),
    Among("snjome", -1, 122, None),
    Among("šnjome", -1, 86, None),
    Among("kome", -1, 95, None),
    Among("skome", 558, 1, None),
    Among("škome", 558, 2, None),
    Among("elome", -1, 83, None),
    Among("nome", -1, 13, None),
    Among("cinome", 562, 137, None),
    Among("činome", 562, 89, None),
    Among("osome", -1, 123, None),
    Among("atome", -1, 120, None),
    Among("evitome", -1, 92, None),
    Among("ovitome", -1, 93, None),
    Among("astome", -1, 94, None),
    Among("avome", -1, 77, None),
    Among("evome", -1, 78, None),
    Among("ivome", -1, 79, None),
    Among("ovome", -1, 80, None),
    Among("aćome", -1, 14, None),
    Among("ećome", -1, 15, None),
    Among("ućome", -1, 16, None),
    Among("ošome", -1, 91, None),
    Among("ne", -1, 13, None),
    Among("ane", 578, 10, None),
    Among("acane", 579, 128, None),
    Among("urane", 579, 105, None),
    Among("tane", 579, 113, None),
    Among("astane", 582, 110, None),
    Among("istane", 582, 111, None),
    Among("ostane", 582, 112, None),
    Among("avane", 579, 97, None),
    Among("evane", 579, 96, None),
    Among("ivane", 579, 98, None),
    Among("uvane", 579, 99, None),
    Among("ačane", 579, 102, None),
    Among("acene", 578, 124, None),
    Among("lucene", 578, 121, None),
    Among("ačene", 578, 101, None),
    Among("lučene", 578, 117, None),
    Among("ine", 578, 11, None),
    Among("cine", 595, 137, None),
    Among("anine", 595, 10, None),
    Among("čine", 595, 89, None),
    Among("one", 578, 12, None),
    Among("are", -1, 53, None),
    Among("dre", -1, 54, None),
    Among("ere", -1, 55, None),
    Among("ore", -1, 56, None),
    Among("ase", -1, 161, None),
    Among("base", 604, 135, None),
    Among("acase", 604, 128, None),
    Among("gase", 604, 131, None),
    Among("jase", 604, 129, None),
    Among("astajase", 608, 138, None),
    Among("istajase", 608, 139, None),
    Among("ostajase", 608, 140, None),
    Among("injase", 608, 150, None),
    Among("kase", 604, 133, None),
    Among("nase", 604, 132, None),
    Among("irase", 604, 155, None),
    Among("urase", 604, 156, None),
    Among("tase", 604, 130, None),
    Among("vase", 604, 134, None),
    Among("avase", 618, 144, None),
    Among("evase", 618, 145, None),
    Among("ivase", 618, 146, None),
    Among("ovase", 618, 148, None),
    Among("uvase", 618, 147, None),
    Among("ese", -1, 57, None),
    Among("ise", -1, 58, None),
    Among("acise", 625, 124, None),
    Among("lucise", 625, 121, None),
    Among("rosise", 625, 127, None),
    Among("jetise", 625, 149, None),
    Among("ose", -1, 123, None),
    Among("astadose", 630, 141, None),
    Among("istadose", 630, 142, None),
    Among("ostadose", 630, 143, None),
    Among("ate", -1, 104, None),
    Among("acate", 634, 128, None),
    Among("ikate", 634, 68, None),
    Among("late", 634, 69, None),
    Among("irate", 634, 100, None),
    Among("urate", 634, 105, None),
    Among("tate", 634, 113, None),
    Among("avate", 634, 97, None),
    Among("evate", 634, 96, None),
    Among("ivate", 634, 98, None),
    Among("uvate", 634, 99, None),
    Among("ačate", 634, 102, None),
    Among("ete", -1, 70, None),
    Among("astadete", 646, 110, None),
    Among("istadete", 646, 111, None),
    Among("ostadete", 646, 112, None),
    Among("astajete", 646, 106, None),
    Among("istajete", 646, 107, None),
    Among("ostajete", 646, 108, None),
    Among("ijete", 646, 116, None),
    Among("injete", 646, 114, None),
    Among("ujete", 646, 25, None),
    Among("lucujete", 655, 121, None),
    Among("irujete", 655, 100, None),
    Among("lučujete", 655, 117, None),
    Among("nete", 646, 13, None),
    Among("astanete", 659, 110, None),
    Among("istanete", 659, 111, None),
    Among("ostanete", 659, 112, None),
    Among("astete", 646, 115, None),
    Among("ite", -1, 116, None),
    Among("acite", 664, 124, None),
    Among("lucite", 664, 121, None),
    Among("nite", 664, 13, None),
    Among("astanite", 667, 110, None),
    Among("istanite", 667, 111, None),
    Among("ostanite", 667, 112, None),
    Among("rosite", 664, 127, None),
    Among("jetite", 664, 118, None),
    Among("astite", 664, 115, None),
    Among("evite", 664, 92, None),
    Among("ovite", 664, 93, None),
    Among("ačite", 664, 101, None),
    Among("lučite", 664, 117, None),
    Among("rošite", 664, 90, None),
    Among("ajte", -1, 104, None),
    Among("urajte", 679, 105, None),
    Among("tajte", 679, 113, None),
    Among("astajte", 681, 106, None),
    Among("istajte", 681, 107, None),
    Among("ostajte", 681, 108, None),
    Among("avajte", 679, 97, None),
    Among("evajte", 679, 96, None),
    Among("ivajte", 679, 98, None),
    Among("uvajte", 679, 99, None),
    Among("ijte", -1, 116, None),
    Among("lucujte", -1, 121, None),
    Among("irujte", -1, 100, None),
    Among("lučujte", -1, 117, None),
    Among("aste", -1, 94, None),
    Among("acaste", 693, 128, None),
    Among("astajaste", 693, 106, None),
    Among("istajaste", 693, 107, None),
    Among("ostajaste", 693, 108, None),
    Among("injaste", 693, 114, None),
    Among("iraste", 693, 100, None),
    Among("uraste", 693, 105, None),
    Among("taste", 693, 113, None),
    Among("avaste", 693, 97, None),
    Among("evaste", 693, 96, None),
    Among("ivaste", 693, 98, None),
    Among("ovaste", 693, 76, None),
    Among("uvaste", 693, 99, None),
    Among("ačaste", 693, 102, None),
    Among("este", -1, 71, None),
    Among("iste", -1, 72, None),
    Among("aciste", 709, 124, None),
    Among("luciste", 709, 121, None),
    Among("niste", 709, 103, None),
    Among("rosiste", 709, 127, None),
    Among("jetiste", 709, 118, None),
    Among("ačiste", 709, 101, None),
    Among("lučiste", 709, 117, None),
    Among("rošiste", 709, 90, None),
    Among("kste", -1, 73, None),
    Among("oste", -1, 74, None),
    Among("astadoste", 719, 110, None),
    Among("istadoste", 719, 111, None),
    Among("ostadoste", 719, 112, None),
    Among("nuste", -1, 13, None),
    Among("ište", -1, 75, None),
    Among("ave", -1, 77, None),
    Among("eve", -1, 78, None),
    Among("ajeve", 726, 109, None),
    Among("cajeve", 727, 26, None),
    Among("lajeve", 727, 30, None),
    Among("rajeve", 727, 31, None),
    Among("ćajeve", 727, 28, None),
    Among("čajeve", 727, 27, None),
    Among("đajeve", 727, 29, None),
    Among("ive", -1, 79, None),
    Among("ove", -1, 80, None),
    Among("gove", 735, 20, None),
    Among("ugove", 736, 17, None),
    Among("love", 735, 82, None),
    Among("olove", 738, 49, None),
    Among("move", 735, 81, None),
    Among("onove", 735, 12, None),
    Among("aće", -1, 14, None),
    Among("eće", -1, 15, None),
    Among("uće", -1, 16, None),
    Among("ače", -1, 101, None),
    Among("luče", -1, 117, None),
    Among("aše", -1, 104, None),
    Among("baše", 747, 63, None),
    Among("gaše", 747, 64, None),
    Among("jaše", 747, 61, None),
    Among("astajaše", 750, 106, None),
    Among("istajaše", 750, 107, None),
    Among("ostajaše", 750, 108, None),
    Among("injaše", 750, 114, None),
    Among("kaše", 747, 62, None),
    Among("naše", 747, 60, None),
    Among("iraše", 747, 100, None),
    Among("uraše", 747, 105, None),
    Among("taše", 747, 59, None),
    Among("vaše", 747, 65, None),
    Among("avaše", 760, 97, None),
    Among("evaše", 760, 96, None),
    Among("ivaše", 760, 98, None),
    Among("ovaše", 760, 76, None),
    Among("uvaše", 760, 99, None),
    Among("ačaše", 747, 102, None),
    Among("eše", -1, 66, None),
    Among("iše", -1, 67, None),
    Among("jetiše", 768, 118, None),
    Among("ačiše", 768, 101, None),
    Among("lučiše", 768, 117, None),
    Among("rošiše", 768, 90, None),
    Among("oše", -1, 91, None),
    Among("astadoše", 773, 110, None),
    Among("istadoše", 773, 111, None),
    Among("ostadoše", 773, 112, None),
    Among("aceg", -1, 124, None),
    Among("eceg", -1, 125, None),
    Among("uceg", -1, 126, None),
    Among("anjijeg", -1, 84, None),
    Among("enjijeg", -1, 85, None),
    Among("snjijeg", -1, 122, None),
    Among("šnjijeg", -1, 86, None),
    Among("kijeg", -1, 95, None),
    Among("skijeg", 784, 1, None),
    Among("škijeg", 784, 2, None),
    Among("elijeg", -1, 83, None),
    Among("nijeg", -1, 13, None),
    Among("osijeg", -1, 123, None),
    Among("atijeg", -1, 120, None),
    Among("evitijeg", -1, 92, None),
    Among("ovitijeg", -1, 93, None),
    Among("astijeg", -1, 94, None),
    Among("avijeg", -1, 77, None),
    Among("evijeg", -1, 78, None),
    Among("ivijeg", -1, 79, None),
    Among("ovijeg", -1, 80, None),
    Among("ošijeg", -1, 91, None),
    Among("anjeg", -1, 84, None),
    Among("enjeg", -1, 85, None),
    Among("snjeg", -1, 122, None),
    Among("šnjeg", -1, 86, None),
    Among("keg", -1, 95, None),
    Among("eleg", -1, 83, None),
    Among("neg", -1, 13, None),
    Among("aneg", 805, 10, None),
    Among("eneg", 805, 87, None),
    Among("sneg", 805, 159, None),
    Among("šneg", 805, 88, None),
    Among("oseg", -1, 123, None),
    Among("ateg", -1, 120, None),
    Among("aveg", -1, 77, None),
    Among("eveg", -1, 78, None),
    Among("iveg", -1, 79, None),
    Among("oveg", -1, 80, None),
    Among("aćeg", -1, 14, None),
    Among("ećeg", -1, 15, None),
    Among("ućeg", -1, 16, None),
    Among("ošeg", -1, 91, None),
    Among("acog", -1, 124, None),
    Among("ecog", -1, 125, None),
    Among("ucog", -1, 126, None),
    Among("anjog", -1, 84, None),
    Among("enjog", -1, 85, None),
    Among("snjog", -1, 122, None),
    Among("šnjog", -1, 86, None),
    Among("kog", -1, 95, None),
    Among("skog", 827, 1, None),
    Among("škog", 827, 2, None),
    Among("elog", -1, 83, None),
    Among("nog", -1, 13, None),
    Among("cinog", 831, 137, None),
    Among("činog", 831, 89, None),
    Among("osog", -1, 123, None),
    Among("atog", -1, 120, None),
    Among("evitog", -1, 92, None),
    Among("ovitog", -1, 93, None),
    Among("astog", -1, 94, None),
    Among("avog", -1, 77, None),
    Among("evog", -1, 78, None),
    Among("ivog", -1, 79, None),
    Among("ovog", -1, 80, None),
    Among("aćog", -1, 14, None),
    Among("ećog", -1, 15, None),
    Among("ućog", -1, 16, None),
    Among("ošog", -1, 91, None),
    Among("ah", -1, 104, None),
    Among("acah", 847, 128, None),
    Among("astajah", 847, 106, None),
    Among("istajah", 847, 107, None),
    Among("ostajah", 847, 108, None),
    Among("injah", 847, 114, None),
    Among("irah", 847, 100, None),
    Among("urah", 847, 105, None),
    Among("tah", 847, 113, None),
    Among("avah", 847, 97, None),
    Among("evah", 847, 96, None),
    Among("ivah", 847, 98, None),
    Among("ovah", 847, 76, None),
    Among("uvah", 847, 99, None),
    Among("ačah", 847, 102, None),
    Among("ih", -1, 116, None),
    Among("acih", 862, 124, None),
    Among("ecih", 862, 125, None),
    Among("ucih", 862, 126, None),
    Among("lucih", 865, 121, None),
    Among("anjijih", 862, 84, None),
    Among("enjijih", 862, 85, None),
    Among("snjijih", 862, 122, None),
    Among("šnjijih", 862, 86, None),
    Among("kijih", 862, 95, None),
    Among("skijih", 871, 1, None),
    Among("škijih", 871, 2, None),
    Among("elijih", 862, 83, None),
    Among("nijih", 862, 13, None),
    Among("osijih", 862, 123, None),
    Among("atijih", 862, 120, None),
    Among("evitijih", 862, 92, None),
    Among("ovitijih", 862, 93, None),
    Among("astijih", 862, 94, None),
    Among("avijih", 862, 77, None),
    Among("evijih", 862, 78, None),
    Among("ivijih", 862, 79, None),
    Among("ovijih", 862, 80, None),
    Among("ošijih", 862, 91, None),
    Among("anjih", 862, 84, None),
    Among("enjih", 862, 85, None),
    Among("snjih", 862, 122, None),
    Among("šnjih", 862, 86, None),
    Among("kih", 862, 95, None),
    Among("skih", 890, 1, None),
    Among("ških", 890, 2, None),
    Among("elih", 862, 83, None),
    Among("nih", 862, 13, None),
    Among("cinih", 894, 137, None),
    Among("činih", 894, 89, None),
    Among("osih", 862, 123, None),
    Among("rosih", 897, 127, None),
    Among("atih", 862, 120, None),
    Among("jetih", 862, 118, None),
    Among("evitih", 862, 92, None),
    Among("ovitih", 862, 93, None),
    Among("astih", 862, 94, None),
    Among("avih", 862, 77, None),
    Among("evih", 862, 78, None),
    Among("ivih", 862, 79, None),
    Among("ovih", 862, 80, None),
    Among("aćih", 862, 14, None),
    Among("ećih", 862, 15, None),
    Among("ućih", 862, 16, None),
    Among("ačih", 862, 101, None),
    Among("lučih", 862, 117, None),
    Among("oših", 862, 91, None),
    Among("roših", 913, 90, None),
    Among("astadoh", -1, 110, None),
    Among("istadoh", -1, 111, None),
    Among("ostadoh", -1, 112, None),
    Among("acuh", -1, 124, None),
    Among("ecuh", -1, 125, None),
    Among("ucuh", -1, 126, None),
    Among("aćuh", -1, 14, None),
    Among("ećuh", -1, 15, None),
    Among("ućuh", -1, 16, None),
    Among("aci", -1, 124, None),
    Among("aceci", -1, 124, None),
    Among("ieci", -1, 162, None),
    Among("ajuci", -1, 161, None),
    Among("irajuci", 927, 155, None),
    Among("urajuci", 927, 156, None),
    Among("astajuci", 927, 138, None),
    Among("istajuci", 927, 139, None),
    Among("ostajuci", 927, 140, None),
    Among("avajuci", 927, 144, None),
    Among("evajuci", 927, 145, None),
    Among("ivajuci", 927, 146, None),
    Among("uvajuci", 927, 147, None),
    Among("ujuci", -1, 157, None),
    Among("lucujuci", 937, 121, None),
    Among("irujuci", 937, 155, None),
    Among("luci", -1, 121, None),
    Among("nuci", -1, 164, None),
    Among("etuci", -1, 153, None),
    Among("astuci", -1, 136, None),
    Among("gi", -1, 20, None),
    Among("ugi", 944, 18, None),
    Among("aji", -1, 109, None),
    Among("caji", 946, 26, None),
    Among("laji", 946, 30, None),
    Among("raji", 946, 31, None),
    Among("ćaji", 946, 28, None),
    Among("čaji", 946, 27, None),
    Among("đaji", 946, 29, None),
    Among("biji", -1, 32, None),
    Among("ciji", -1, 33, None),
    Among("diji", -1, 34, None),
    Among("fiji", -1, 40, None),
    Among("giji", -1, 39, None),
    Among("anjiji", -1, 84, None),
    Among("enjiji", -1, 85, None),
    Among("snjiji", -1, 122, None),
    Among("šnjiji", -1, 86, None),
    Among("kiji", -1, 95, None),
    Among("skiji", 962, 1, None),
    Among("škiji", 962, 2, None),
    Among("liji", -1, 35, None),
    Among("eliji", 965, 83, None),
    Among("miji", -1, 37, None),
    Among("niji", -1, 13, None),
    Among("ganiji", 968, 9, None),
    Among("maniji", 968, 6, None),
    Among("paniji", 968, 7, None),
    Among("raniji", 968, 8, None),
    Among("taniji", 968, 5, None),
    Among("piji", -1, 41, None),
    Among("riji", -1, 42, None),
    Among("siji", -1, 43, None),
    Among("osiji", 976, 123, None),
    Among("tiji", -1, 44, None),
    Among("atiji", 978, 120, None),
    Among("evitiji", 978, 92, None),
    Among("ovitiji", 978, 93, None),
    Among("astiji", 978, 94, None),
    Among("aviji", -1, 77, None),
    Among("eviji", -1, 78, None),
    Among("iviji", -1, 79, None),
    Among("oviji", -1, 80, None),
    Among("ziji", -1, 45, None),
    Among("ošiji", -1, 91, None),
    Among("žiji", -1, 38, None),
    Among("anji", -1, 84, None),
    Among("enji", -1, 85, None),
    Among("snji", -1, 122, None),
    Among("šnji", -1, 86, None),
    Among("ki", -1, 95, None),
    Among("ski", 994, 1, None),
    Among("ški", 994, 2, None),
    Among("ali", -1, 104, None),
    Among("acali", 997, 128, None),
    Among("astajali", 997, 106, None),
    Among("istajali", 997, 107, None),
    Among("ostajali", 997, 108, None),
    Among("ijali", 997, 47, None),
    Among("injali", 997, 114, None),
    Among("nali", 997, 46, None),
    Among("irali", 997, 100, None),
    Among("urali", 997, 105, None),
    Among("tali", 997, 113, None),
    Among("astali", 1007, 110, None),
    Among("istali", 1007, 111, None),
    Among("ostali", 1007, 112, None),
    Among("avali", 997, 97, None),
    Among("evali", 997, 96, None),
    Among("ivali", 997, 98, None),
    Among("ovali", 997, 76, None),
    Among("uvali", 997, 99, None),
    Among("ačali", 997, 102, None),
    Among("eli", -1, 83, None),
    Among("ili", -1, 116, None),
    Among("acili", 1018, 124, None),
    Among("lucili", 1018, 121, None),
    Among("nili", 1018, 103, None),
    Among("rosili", 1018, 127, None),
    Among("jetili", 1018, 118, None),
    Among("ozili", 1018, 48, None),
    Among("ačili", 1018, 101, None),
    Among("lučili", 1018, 117, None),
    Among("rošili", 1018, 90, None),
    Among("oli", -1, 50, None),
    Among("asli", -1, 115, None),
    Among("nuli", -1, 13, None),
    Among("rami", -1, 52, None),
    Among("lemi", -1, 51, None),
    Among("ni", -1, 13, None),
    Among("ani", 1033, 10, None),
    Among("acani", 1034, 128, None),
    Among("urani", 1034, 105, None),
    Among("tani", 1034, 113, None),
    Among("avani", 1034, 97, None),
    Among("evani", 1034, 96, None),
    Among("ivani", 1034, 98, None),
    Among("uvani", 1034, 99, None),
    Among("ačani", 1034, 102, None),
    Among("aceni", 1033, 124, None),
    Among("luceni", 1033, 121, None),
    Among("ačeni", 1033, 101, None),
    Among("lučeni", 1033, 117, None),
    Among("ini", 1033, 11, None),
    Among("cini", 1047, 137, None),
    Among("čini", 1047, 89, None),
    Among("oni", 1033, 12, None),
    Among("ari", -1, 53, None),
    Among("dri", -1, 54, None),
    Among("eri", -1, 55, None),
    Among("ori", -1, 56, None),
    Among("basi", -1, 135, None),
    Among("gasi", -1, 131, None),
    Among("jasi", -1, 129, None),
    Among("kasi", -1, 133, None),
    Among("nasi", -1, 132, None),
    Among("tasi", -1, 130, None),
    Among("vasi", -1, 134, None),
    Among("esi", -1, 152, None),
    Among("isi", -1, 154, None),
    Among("osi", -1, 123, None),
    Among("avsi", -1, 161, None),
    Among("acavsi", 1065, 128, None),
    Among("iravsi", 1065, 155, None),
    Among("tavsi", 1065, 160, None),
    Among("etavsi", 1068, 153, None),
    Among("astavsi", 1068, 141, None),
    Among("istavsi", 1068, 142, None),
    Among("ostavsi", 1068, 143, None),
    Among("ivsi", -1, 162, None),
    Among("nivsi", 1073, 158, None),
    Among("rosivsi", 1073, 127, None),
    Among("nuvsi", -1, 164, None),
    Among("ati", -1, 104, None),
    Among("acati", 1077, 128, None),
    Among("astajati", 1077, 106, None),
    Among("istajati", 1077, 107, None),
    Among("ostajati", 1077, 108, None),
    Among("injati", 1077, 114, None),
    Among("ikati", 1077, 68, None),
    Among("lati", 1077, 69, None),
    Among("irati", 1077, 100, None),
    Among("urati", 1077, 105, None),
    Among("tati", 1077, 113, None),
    Among("astati", 1087, 110, None),
    Among("istati", 1087, 111, None),
    Among("ostati", 1087, 112, None),
    Among("avati", 1077, 97, None),
    Among("evati", 1077, 96, None),
    Among("ivati", 1077, 98, None),
    Among("ovati", 1077, 76, None),
    Among("uvati", 1077, 99, None),
    Among("ačati", 1077, 102, None),
    Among("eti", -1, 70, None),
    Among("iti", -1, 116, None),
    Among("aciti", 1098, 124, None),
    Among("luciti", 1098, 121, None),
    Among("niti", 1098, 103, None),
    Among("rositi", 1098, 127, None),
    Among("jetiti", 1098, 118, None),
    Among("eviti", 1098, 92, None),
    Among("oviti", 1098, 93, None),
    Among("ačiti", 1098, 101, None),
    Among("lučiti", 1098, 117, None),
    Among("rošiti", 1098, 90, None),
    Among("asti", -1, 94, None),
    Among("esti", -1, 71, None),
    Among("isti", -1, 72, None),
    Among("ksti", -1, 73, None),
    Among("osti", -1, 74, None),
    Among("nuti", -1, 13, None),
    Among("avi", -1, 77, None),
    Among("evi", -1, 78, None),
    Among("ajevi", 1116, 109, None),
    Among("cajevi", 1117, 26, None),
    Among("lajevi", 1117, 30, None),
    Among("rajevi", 1117, 31, None),
    Among("ćajevi", 1117, 28, None),
    Among("čajevi", 1117, 27, None),
    Among("đajevi", 1117, 29, None),
    Among("ivi", -1, 79, None),
    Among("ovi", -1, 80, None),
    Among("govi", 1125, 20, None),
    Among("ugovi", 1126, 17, None),
    Among("lovi", 1125, 82, None),
    Among("olovi", 1128, 49, None),
    Among("movi", 1125, 81, None),
    Among("onovi", 1125, 12, None),
    Among("ieći", -1, 116, None),
    Among("ačeći", -1, 101, None),
    Among("ajući", -1, 104, None),
    Among("irajući", 1134, 100, None),
    Among("urajući", 1134, 105, None),
    Among("astajući", 1134, 106, None),
    Among("istajući", 1134, 107, None),
    Among("ostajući", 1134, 108, None),
    Among("avajući", 1134, 97, None),
    Among("evajući", 1134, 96, None),
    Among("ivajući", 1134, 98, None),
    Among("uvajući", 1134, 99, None),
    Among("ujući", -1, 25, None),
    Among("irujući", 1144, 100, None),
    Among("lučujući", 1144, 117, None),
    Among("nući", -1, 13, None),
    Among("etući", -1, 70, None),
    Among("astući", -1, 115, None),
    Among("ači", -1, 101, None),
    Among("luči", -1, 117, None),
    Among("baši", -1, 63, None),
    Among("gaši", -1, 64, None),
    Among("jaši", -1, 61, None),
    Among("kaši", -1, 62, None),
    Among("naši", -1, 60, None),
    Among("taši", -1, 59, None),
    Among("vaši", -1, 65, None),
    Among("eši", -1, 66, None),
    Among("iši", -1, 67, None),
    Among("oši", -1, 91, None),
    Among("avši", -1, 104, None),
    Among("iravši", 1162, 100, None),
    Among("tavši", 1162, 113, None),
    Among("etavši", 1164, 70, None),
    Among("astavši", 1164, 110, None),
    Among("istavši", 1164, 111, None),
    Among("ostavši", 1164, 112, None),
    Among("ačavši", 1162, 102, None),
    Among("ivši", -1, 116, None),
    Among("nivši", 1170, 103, None),
    Among("rošivši", 1170, 90, None),
    Among("nuvši", -1, 13, None),
    Among("aj", -1, 104, None),
    Among("uraj", 1174, 105, None),
    Among("taj", 1174, 113, None),
    Among("avaj", 1174, 97, None),
    Among("evaj", 1174, 96, None),
    Among("ivaj", 1174, 98, None),
    Among("uvaj", 1174, 99, None),
    Among("ij", -1, 116, None),
    Among("acoj", -1, 124, None),
    Among("ecoj", -1, 125, None),
    Among("ucoj", -1, 126, None),
    Among("anjijoj", -1, 84, None),
    Among("enjijoj", -1, 85, None),
    Among("snjijoj", -1, 122, None),
    Among("šnjijoj", -1, 86, None),
    Among("kijoj", -1, 95, None),
    Among("skijoj", 1189, 1, None),
    Among("škijoj", 1189, 2, None),
    Among("elijoj", -1, 83, None),
    Among("nijoj", -1, 13, None),
    Among("osijoj", -1, 123, None),
    Among("evitijoj", -1, 92, None),
    Among("ovitijoj", -1, 93, None),
    Among("astijoj", -1, 94, None),
    Among("avijoj", -1, 77, None),
    Among("evijoj", -1, 78, None),
    Among("ivijoj", -1, 79, None),
    Among("ovijoj", -1, 80, None),
    Among("ošijoj", -1, 91, None),
    Among("anjoj", -1, 84, None),
    Among("enjoj", -1, 85, None),
    Among("snjoj", -1, 122, None),
    Among("šnjoj", -1, 86, None),
    Among("koj", -1, 95, None),
    Among("skoj", 1207, 1, None),
    Among("škoj", 1207, 2, None),
    Among("aloj", -1, 104, None),
    Among("eloj", -1, 83, None),
    Among("noj", -1, 13, None),
    Among("cinoj", 1212, 137, None),
    Among("činoj", 1212, 89, None),
    Among("osoj", -1, 123, None),
    Among("atoj", -1, 120, None),
    Among("evitoj", -1, 92, None),
    Among("ovitoj", -1, 93, None),
    Among("astoj", -1, 94, None),
    Among("avoj", -1, 77, None),
    Among("evoj", -1, 78, None),
    Among("ivoj", -1, 79, None),
    Among("ovoj", -1, 80, None),
    Among("aćoj", -1, 14, None),
    Among("ećoj", -1, 15, None),
    Among("ućoj", -1, 16, None),
    Among("ošoj", -1, 91, None),
    Among("lucuj", -1, 121, None),
    Among("iruj", -1, 100, None),
    Among("lučuj", -1, 117, None),
    Among("al", -1, 104, None),
    Among("iral", 1231, 100, None),
    Among("ural", 1231, 105, None),
    Among("el", -1, 119, None),
    Among("il", -1, 116, None),
    Among("am", -1, 104, None),
    Among("acam", 1236, 128, None),
    Among("iram", 1236, 100, None),
    Among("uram", 1236, 105, None),
    Among("tam", 1236, 113, None),
    Among("avam", 1236, 97, None),
    Among("evam", 1236, 96, None),
    Among("ivam", 1236, 98, None),
    Among("uvam", 1236, 99, None),
    Among("ačam", 1236, 102, None),
    Among("em", -1, 119, None),
    Among("acem", 1246, 124, None),
    Among("ecem", 1246, 125, None),
    Among("ucem", 1246, 126, None),
    Among("astadem", 1246, 110, None),
    Among("istadem", 1246, 111, None),
    Among("ostadem", 1246, 112, None),
    Among("ajem", 1246, 104, None),
    Among("cajem", 1253, 26, None),
    Among("lajem", 1253, 30, None),
    Among("rajem", 1253, 31, None),
    Among("astajem", 1253, 106, None),
    Among("istajem", 1253, 107, None),
    Among("ostajem", 1253, 108, None),
    Among("ćajem", 1253, 28, None),
    Among("čajem", 1253, 27, None),
    Among("đajem", 1253, 29, None),
    Among("ijem", 1246, 116, None),
    Among("anjijem", 1263, 84, None),
    Among("enjijem", 1263, 85, None),
    Among("snjijem", 1263, 123, None),
    Among("šnjijem", 1263, 86, None),
    Among("kijem", 1263, 95, None),
    Among("skijem", 1268, 1, None),
    Among("škijem", 1268, 2, None),
    Among("lijem", 1263, 24, None),
    Among("elijem", 1271, 83, None),
    Among("nijem", 1263, 13, None),
    Among("rarijem", 1263, 21, None),
    Among("sijem", 1263, 23, None),
    Among("osijem", 1275, 123, None),
    Among("atijem", 1263, 120, None),
    Among("evitijem", 1263, 92, None),
    Among("ovitijem", 1263, 93, None),
    Among("otijem", 1263, 22, None),
    Among("astijem", 1263, 94, None),
    Among("avijem", 1263, 77, None),
    Among("evijem", 1263, 78, None),
    Among("ivijem", 1263, 79, None),
    Among("ovijem", 1263, 80, None),
    Among("ošijem", 1263, 91, None),
    Among("anjem", 1246, 84, None),
    Among("enjem", 1246, 85, None),
    Among("injem", 1246, 114, None),
    Among("snjem", 1246, 122, None),
    Among("šnjem", 1246, 86, None),
    Among("ujem", 1246, 25, None),
    Among("lucujem", 1292, 121, None),
    Among("irujem", 1292, 100, None),
    Among("lučujem", 1292, 117, None),
    Among("kem", 1246, 95, None),
    Among("skem", 1296, 1, None),
    Among("škem", 1296, 2, None),
    Among("elem", 1246, 83, None),
    Among("nem", 1246, 13, None),
    Among("anem", 1300, 10, None),
    Among("astanem", 1301, 110, None),
    Among("istanem", 1301, 111, None),
    Among("ostanem", 1301, 112, None),
    Among("enem", 1300, 87, None),
    Among("snem", 1300, 159, None),
    Among("šnem", 1300, 88, None),
    Among("basem", 1246, 135, None),
    Among("gasem", 1246, 131, None),
    Among("jasem", 1246, 129, None),
    Among("kasem", 1246, 133, None),
    Among("nasem", 1246, 132, None),
    Among("tasem", 1246, 130, None),
    Among("vasem", 1246, 134, None),
    Among("esem", 1246, 152, None),
    Among("isem", 1246, 154, None),
    Among("osem", 1246, 123, None),
    Among("atem", 1246, 120, None),
    Among("etem", 1246, 70, None),
    Among("evitem", 1246, 92, None),
    Among("ovitem", 1246, 93, None),
    Among("astem", 1246, 94, None),
    Among("istem", 1246, 151, None),
    Among("ištem", 1246, 75, None),
    Among("avem", 1246, 77, None),
    Among("evem", 1246, 78, None),
    Among("ivem", 1246, 79, None),
    Among("aćem", 1246, 14, None),
    Among("ećem", 1246, 15, None),
    Among("ućem", 1246, 16, None),
    Among("bašem", 1246, 63, None),
    Among("gašem", 1246, 64, None),
    Among("jašem", 1246, 61, None),
    Among("kašem", 1246, 62, None),
    Among("našem", 1246, 60, None),
    Among("tašem", 1246, 59, None),
    Among("vašem", 1246, 65, None),
    Among("ešem", 1246, 66, None),
    Among("išem", 1246, 67, None),
    Among("ošem", 1246, 91, None),
    Among("im", -1, 116, None),
    Among("acim", 1341, 124, None),
    Among("ecim", 1341, 125, None),
    Among("ucim", 1341, 126, None),
    Among("lucim", 1344, 121, None),
    Among("anjijim", 1341, 84, None),
    Among("enjijim", 1341, 85, None),
    Among("snjijim", 1341, 122, None),
    Among("šnjijim", 1341, 86, None),
    Among("kijim", 1341, 95, None),
    Among("skijim", 1350, 1, None),
    Among("škijim", 1350, 2, None),
    Among("elijim", 1341, 83, None),
    Among("nijim", 1341, 13, None),
    Among("osijim", 1341, 123, None),
    Among("atijim", 1341, 120, None),
    Among("evitijim", 1341, 92, None),
    Among("ovitijim", 1341, 93, None),
    Among("astijim", 1341, 94, None),
    Among("avijim", 1341, 77, None),
    Among("evijim", 1341, 78, None),
    Among("ivijim", 1341, 79, None),
    Among("ovijim", 1341, 80, None),
    Among("ošijim", 1341, 91, None),
    Among("anjim", 1341, 84, None),
    Among("enjim", 1341, 85, None),
    Among("snjim", 1341, 122, None),
    Among("šnjim", 1341, 86, None),
    Among("kim", 1341, 95, None),
    Among("skim", 1369, 1, None),
    Among("škim", 1369, 2, None),
    Among("elim", 1341, 83, None),
    Among("nim", 1341, 13, None),
    Among("cinim", 1373, 137, None),
    Among("činim", 1373, 89, None),
    Among("osim", 1341, 123, None),
    Among("rosim", 1376, 127, None),
    Among("atim", 1341, 120, None),
    Among("jetim", 1341, 118, None),
    Among("evitim", 1341, 92, None),
    Among("ovitim", 1341, 93, None),
    Among("astim", 1341, 94, None),
    Among("avim", 1341, 77, None),
    Among("evim", 1341, 78, None),
    Among("ivim", 1341, 79, None),
    Among("ovim", 1341, 80, None),
    Among("aćim", 1341, 14, None),
    Among("ećim", 1341, 15, None),
    Among("ućim", 1341, 16, None),
    Among("ačim", 1341, 101, None),
    Among("lučim", 1341, 117, None),
    Among("ošim", 1341, 91, None),
    Among("rošim", 1392, 90, None),
    Among("acom", -1, 124, None),
    Among("ecom", -1, 125, None),
    Among("ucom", -1, 126, None),
    Among("gom", -1, 20, None),
    Among("logom", 1397, 19, None),
    Among("ugom", 1397, 18, None),
    Among("bijom", -1, 32, None),
    Among("cijom", -1, 33, None),
    Among("dijom", -1, 34, None),
    Among("fijom", -1, 40, None),
    Among("gijom", -1, 39, None),
    Among("lijom", -1, 35, None),
    Among("mijom", -1, 37, None),
    Among("nijom", -1, 36, None),
    Among("ganijom", 1407, 9, None),
    Among("manijom", 1407, 6, None),
    Among("panijom", 1407, 7, None),
    Among("ranijom", 1407, 8, None),
    Among("tanijom", 1407, 5, None),
    Among("pijom", -1, 41, None),
    Among("rijom", -1, 42, None),
    Among("sijom", -1, 43, None),
    Among("tijom", -1, 44, None),
    Among("zijom", -1, 45, None),
    Among("žijom", -1, 38, None),
    Among("anjom", -1, 84, None),
    Among("enjom", -1, 85, None),
    Among("snjom", -1, 122, None),
    Among("šnjom", -1, 86, None),
    Among("kom", -1, 95, None),
    Among("skom", 1423, 1, None),
    Among("škom", 1423, 2, None),
    Among("alom", -1, 104, None),
    Among("ijalom", 1426, 47, None),
    Among("nalom", 1426, 46, None),
    Among("elom", -1, 83, None),
    Among("ilom", -1, 116, None),
    Among("ozilom", 1430, 48, None),
    Among("olom", -1, 50, None),
    Among("ramom", -1, 52, None),
    Among("lemom", -1, 51, None),
    Among("nom", -1, 13, None),
    Among("anom", 1435, 10, None),
    Among("inom", 1435, 11, None),
    Among("cinom", 1437, 137, None),
    Among("aninom", 1437, 10, None),
    Among("činom", 1437, 89, None),
    Among("onom", 1435, 12, None),
    Among("arom", -1, 53, None),
    Among("drom", -1, 54, None),
    Among("erom", -1, 55, None),
    Among("orom", -1, 56, None),
    Among("basom", -1, 135, None),
    Among("gasom", -1, 131, None),
    Among("jasom", -1, 129, None),
    Among("kasom", -1, 133, None),
    Among("nasom", -1, 132, None),
    Among("tasom", -1, 130, None),
    Among("vasom", -1, 134, None),
    Among("esom", -1, 57, None),
    Among("isom", -1, 58, None),
    Among("osom", -1, 123, None),
    Among("atom", -1, 120, None),
    Among("ikatom", 1456, 68, None),
    Among("latom", 1456, 69, None),
    Among("etom", -1, 70, None),
    Among("evitom", -1, 92, None),
    Among("ovitom", -1, 93, None),
    Among("astom", -1, 94, None),
    Among("estom", -1, 71, None),
    Among("istom", -1, 72, None),
    Among("kstom", -1, 73, None),
    Among("ostom", -1, 74, None),
    Among("avom", -1, 77, None),
    Among("evom", -1, 78, None),
    Among("ivom", -1, 79, None),
    Among("ovom", -1, 80, None),
    Among("lovom", 1470, 82, None),
    Among("movom", 1470, 81, None),
    Among("stvom", -1, 3, None),
    Among("štvom", -1, 4, None),
    Among("aćom", -1, 14, None),
    Among("ećom", -1, 15, None),
    Among("ućom", -1, 16, None),
    Among("bašom", -1, 63, None),
    Among("gašom", -1, 64, None),
    Among("jašom", -1, 61, None),
    Among("kašom", -1, 62, None),
    Among("našom", -1, 60, None),
    Among("tašom", -1, 59, None),
    Among("vašom", -1, 65, None),
    Among("ešom", -1, 66, None),
    Among("išom", -1, 67, None),
    Among("ošom", -1, 91, None),
    Among("an", -1, 104, None),
    Among("acan", 1488, 128, None),
    Among("iran", 1488, 100, None),
    Among("uran", 1488, 105, None),
    Among("tan", 1488, 113, None),
    Among("avan", 1488, 97, None),
    Among("evan", 1488, 96, None),
    Among("ivan", 1488, 98, None),
    Among("uvan", 1488, 99, None),
    Among("ačan", 1488, 102, None),
    Among("acen", -1, 124, None),
    Among("lucen", -1, 121, None),
    Among("ačen", -1, 101, None),
    Among("lučen", -1, 117, None),
    Among("anin", -1, 10, None),
    Among("ao", -1, 104, None),
    Among("acao", 1503, 128, None),
    Among("astajao", 1503, 106, None),
    Among("istajao", 1503, 107, None),
    Among("ostajao", 1503, 108, None),
    Among("injao", 1503, 114, None),
    Among("irao", 1503, 100, None),
    Among("urao", 1503, 105, None),
    Among("tao", 1503, 113, None),
    Among("astao", 1511, 110, None),
    Among("istao", 1511, 111, None),
    Among("ostao", 1511, 112, None),
    Among("avao", 1503, 97, None),
    Among("evao", 1503, 96, None),
    Among("ivao", 1503, 98, None),
    Among("ovao", 1503, 76, None),
    Among("uvao", 1503, 99, None),
    Among("ačao", 1503, 102, None),
    Among("go", -1, 20, None),
    Among("ugo", 1521, 18, None),
    Among("io", -1, 116, None),
    Among("acio", 1523, 124, None),
    Among("lucio", 1523, 121, None),
    Among("lio", 1523, 24, None),
    Among("nio", 1523, 103, None),
    Among("rario", 1523, 21, None),
    Among("sio", 1523, 23, None),
    Among("rosio", 1529, 127, None),
    Among("jetio", 1523, 118, None),
    Among("otio", 1523, 22, None),
    Among("ačio", 1523, 101, None),
    Among("lučio", 1523, 117, None),
    Among("rošio", 1523, 90, None),
    Among("bijo", -1, 32, None),
    Among("cijo", -1, 33, None),
    Among("dijo", -1, 34, None),
    Among("fijo", -1, 40, None),
    Among("gijo", -1, 39, None),
    Among("lijo", -1, 35, None),
    Among("mijo", -1, 37, None),
    Among("nijo", -1, 36, None),
    Among("pijo", -1, 41, None),
    Among("rijo", -1, 42, None),
    Among("sijo", -1, 43, None),
    Among("tijo", -1, 44, None),
    Among("zijo", -1, 45, None),
    Among("žijo", -1, 38, None),
    Among("anjo", -1, 84, None),
    Among("enjo", -1, 85, None),
    Among("snjo", -1, 122, None),
    Among("šnjo", -1, 86, None),
    Among("ko", -1, 95, None),
    Among("sko", 1554, 1, None),
    Among("ško", 1554, 2, None),
    Among("alo", -1, 104, None),
    Among("acalo", 1557, 128, None),
    Among("astajalo", 1557, 106, None),
    Among("istajalo", 1557, 107, None),
    Among("ostajalo", 1557, 108, None),
    Among("ijalo", 1557, 47, None),
    Among("injalo", 1557, 114, None),
    Among("nalo", 1557, 46, None),
    Among("iralo", 1557, 100, None),
    Among("uralo", 1557, 105, None),
    Among("talo", 1557, 113, None),
    Among("astalo", 1567, 110, None),
    Among("istalo", 1567, 111, None),
    Among("ostalo", 1567, 112, None),
    Among("avalo", 1557, 97, None),
    Among("evalo", 1557, 96, None),
    Among("ivalo", 1557, 98, None),
    Among("ovalo", 1557, 76, None),
    Among("uvalo", 1557, 99, None),
    Among("ačalo", 1557, 102, None),
    Among("elo", -1, 83, None),
    Among("ilo", -1, 116, None),
    Among("acilo", 1578, 124, None),
    Among("lucilo", 1578, 121, None),
    Among("nilo", 1578, 103, None),
    Among("rosilo", 1578, 127, None),
    Among("jetilo", 1578, 118, None),
    Among("ačilo", 1578, 101, None),
    Among("lučilo", 1578, 117, None),
    Among("rošilo", 1578, 90, None),
    Among("aslo", -1, 115, None),
    Among("nulo", -1, 13, None),
    Among("amo", -1, 104, None),
    Among("acamo", 1589, 128, None),
    Among("ramo", 1589, 52, None),
    Among("iramo", 1591, 100, None),
    Among("uramo", 1591, 105, None),
    Among("tamo", 1589, 113, None),
    Among("avamo", 1589, 97, None),
    Among("evamo", 1589, 96, None),
    Among("ivamo", 1589, 98, None),
    Among("uvamo", 1589, 99, None),
    Among("ačamo", 1589, 102, None),
    Among("emo", -1, 119, None),
    Among("astademo", 1600, 110, None),
    Among("istademo", 1600, 111, None),
    Among("ostademo", 1600, 112, None),
    Among("astajemo", 1600, 106, None),
    Among("istajemo", 1600, 107, None),
    Among("ostajemo", 1600, 108, None),
    Among("ijemo", 1600, 116, None),
    Among("injemo", 1600, 114, None),
    Among("ujemo", 1600, 25, None),
    Among("lucujemo", 1609, 121, None),
    Among("irujemo", 1609, 100, None),
    Among("lučujemo", 1609, 117, None),
    Among("lemo", 1600, 51, None),
    Among("nemo", 1600, 13, None),
    Among("astanemo", 1614, 110, None),
    Among("istanemo", 1614, 111, None),
    Among("ostanemo", 1614, 112, None),
    Among("etemo", 1600, 70, None),
    Among("astemo", 1600, 115, None),
    Among("imo", -1, 116, None),
    Among("acimo", 1620, 124, None),
    Among("lucimo", 1620, 121, None),
    Among("nimo", 1620, 13, None),
    Among("astanimo", 1623, 110, None),
    Among("istanimo", 1623, 111, None),
    Among("ostanimo", 1623, 112, None),
    Among("rosimo", 1620, 127, None),
    Among("etimo", 1620, 70, None),
    Among("jetimo", 1628, 118, None),
    Among("astimo", 1620, 115, None),
    Among("ačimo", 1620, 101, None),
    Among("lučimo", 1620, 117, None),
    Among("rošimo", 1620, 90, None),
    Among("ajmo", -1, 104, None),
    Among("urajmo", 1634, 105, None),
    Among("tajmo", 1634, 113, None),
    Among("astajmo", 1636, 106, None),
    Among("istajmo", 1636, 107, None),
    Among("ostajmo", 1636, 108, None),
    Among("avajmo", 1634, 97, None),
    Among("evajmo", 1634, 96, None),
    Among("ivajmo", 1634, 98, None),
    Among("uvajmo", 1634, 99, None),
    Among("ijmo", -1, 116, None),
    Among("ujmo", -1, 25, None),
    Among("lucujmo", 1645, 121, None),
    Among("irujmo", 1645, 100, None),
    Among("lučujmo", 1645, 117, None),
    Among("asmo", -1, 104, None),
    Among("acasmo", 1649, 128, None),
    Among("astajasmo", 1649, 106, None),
    Among("istajasmo", 1649, 107, None),
    Among("ostajasmo", 1649, 108, None),
    Among("injasmo", 1649, 114, None),
    Among("irasmo", 1649, 100, None),
    Among("urasmo", 1649, 105, None),
    Among("tasmo", 1649, 113, None),
    Among("avasmo", 1649, 97, None),
    Among("evasmo", 1649, 96, None),
    Among("ivasmo", 1649, 98, None),
    Among("ovasmo", 1649, 76, None),
    Among("uvasmo", 1649, 99, None),
    Among("ačasmo", 1649, 102, None),
    Among("ismo", -1, 116, None),
    Among("acismo", 1664, 124, None),
    Among("lucismo", 1664, 121, None),
    Among("nismo", 1664, 103, None),
    Among("rosismo", 1664, 127, None),
    Among("jetismo", 1664, 118, None),
    Among("ačismo", 1664, 101, None),
    Among("lučismo", 1664, 117, None),
    Among("rošismo", 1664, 90, None),
    Among("astadosmo", -1, 110, None),
    Among("istadosmo", -1, 111, None),
    Among("ostadosmo", -1, 112, None),
    Among("nusmo", -1, 13, None),
    Among("no", -1, 13, None),
    Among("ano", 1677, 104, None),
    Among("acano", 1678, 128, None),
    Among("urano", 1678, 105, None),
    Among("tano", 1678, 113, None),
    Among("avano", 1678, 97, None),
    Among("evano", 1678, 96, None),
    Among("ivano", 1678, 98, None),
    Among("uvano", 1678, 99, None),
    Among("ačano", 1678, 102, None),
    Among("aceno", 1677, 124, None),
    Among("luceno", 1677, 121, None),
    Among("ačeno", 1677, 101, None),
    Among("lučeno", 1677, 117, None),
    Among("ino", 1677, 11, None),
    Among("cino", 1691, 137, None),
    Among("čino", 1691, 89, None),
    Among("ato", -1, 120, None),
    Among("ikato", 1694, 68, None),
    Among("lato", 1694, 69, None),
    Among("eto", -1, 70, None),
    Among("evito", -1, 92, None),
    Among("ovito", -1, 93, None),
    Among("asto", -1, 94, None),
    Among("esto", -1, 71, None),
    Among("isto", -1, 72, None),
    Among("ksto", -1, 73, None),
    Among("osto", -1, 74, None),
    Among("nuto", -1, 13, None),
    Among("nuo", -1, 13, None),
    Among("avo", -1, 77, None),
    Among("evo", -1, 78, None),
    Among("ivo", -1, 79, None),
    Among("ovo", -1, 80, None),
    Among("stvo", -1, 3, None),
    Among("štvo", -1, 4, None),
    Among("as", -1, 161, None),
    Among("acas", 1713, 128, None),
    Among("iras", 1713, 155, None),
    Among("uras", 1713, 156, None),
    Among("tas", 1713, 160, None),
    Among("avas", 1713, 144, None),
    Among("evas", 1713, 145, None),
    Among("ivas", 1713, 146, None),
    Among("uvas", 1713, 147, None),
    Among("es", -1, 163, None),
    Among("astades", 1722, 141, None),
    Among("istades", 1722, 142, None),
    Among("ostades", 1722, 143, None),
    Among("astajes", 1722, 138, None),
    Among("istajes", 1722, 139, None),
    Among("ostajes", 1722, 140, None),
    Among("ijes", 1722, 162, None),
    Among("injes", 1722, 150, None),
    Among("ujes", 1722, 157, None),
    Among("lucujes", 1731, 121, None),
    Among("irujes", 1731, 155, None),
    Among("nes", 1722, 164, None),
    Among("astanes", 1734, 141, None),
    Among("istanes", 1734, 142, None),
    Among("ostanes", 1734, 143, None),
    Among("etes", 1722, 153, None),
    Among("astes", 1722, 136, None),
    Among("is", -1, 162, None),
    Among("acis", 1740, 124, None),
    Among("lucis", 1740, 121, None),
    Among("nis", 1740, 158, None),
    Among("rosis", 1740, 127, None),
    Among("jetis", 1740, 149, None),
    Among("at", -1, 104, None),
    Among("acat", 1746, 128, None),
    Among("astajat", 1746, 106, None),
    Among("istajat", 1746, 107, None),
    Among("ostajat", 1746, 108, None),
    Among("injat", 1746, 114, None),
    Among("irat", 1746, 100, None),
    Among("urat", 1746, 105, None),
    Among("tat", 1746, 113, None),
    Among("astat", 1754, 110, None),
    Among("istat", 1754, 111, None),
    Among("ostat", 1754, 112, None),
    Among("avat", 1746, 97, None),
    Among("evat", 1746, 96, None),
    Among("ivat", 1746, 98, None),
    Among("irivat", 1760, 100, None),
    Among("ovat", 1746, 76, None),
    Among("uvat", 1746, 99, None),
    Among("ačat", 1746, 102, None),
    Among("it", -1, 116, None),
    Among("acit", 1765, 124, None),
    Among("lucit", 1765, 121, None),
    Among("rosit", 1765, 127, None),
    Among("jetit", 1765, 118, None),
    Among("ačit", 1765, 101, None),
    Among("lučit", 1765, 117, None),
    Among("rošit", 1765, 90, None),
    Among("nut", -1, 13, None),
    Among("astadu", -1, 110, None),
    Among("istadu", -1, 111, None),
    Among("ostadu", -1, 112, None),
    Among("gu", -1, 20, None),
    Among("logu", 1777, 19, None),
    Among("ugu", 1777, 18, None),
    Among("ahu", -1, 104, None),
    Among("acahu", 1780, 128, None),
    Among("astajahu", 1780, 106, None),
    Among("istajahu", 1780, 107, None),
    Among("ostajahu", 1780, 108, None),
    Among("injahu", 1780, 114, None),
    Among("irahu", 1780, 100, None),
    Among("urahu", 1780, 105, None),
    Among("avahu", 1780, 97, None),
    Among("evahu", 1780, 96, None),
    Among("ivahu", 1780, 98, None),
    Among("ovahu", 1780, 76, None),
    Among("uvahu", 1780, 99, None),
    Among("ačahu", 1780, 102, None),
    Among("aju", -1, 104, None),
    Among("caju", 1794, 26, None),
    Among("acaju", 1795, 128, None),
    Among("laju", 1794, 30, None),
    Among("raju", 1794, 31, None),
    Among("iraju", 1798, 100, None),
    Among("uraju", 1798, 105, None),
    Among("taju", 1794, 113, None),
    Among("astaju", 1801, 106, None),
    Among("istaju", 1801, 107, None),
    Among("ostaju", 1801, 108, None),
    Among("avaju", 1794, 97, None),
    Among("evaju", 1794, 96, None),
    Among("ivaju", 1794, 98, None),
    Among("uvaju", 1794, 99, None),
    Among("ćaju", 1794, 28, None),
    Among("čaju", 1794, 27, None),
    Among("ačaju", 1810, 102, None),
    Among("đaju", 1794, 29, None),
    Among("iju", -1, 116, None),
    Among("biju", 1813, 32, None),
    Among("ciju", 1813, 33, None),
    Among("diju", 1813, 34, None),
    Among("fiju", 1813, 40, None),
    Among("giju", 1813, 39, None),
    Among("anjiju", 1813, 84, None),
    Among("enjiju", 1813, 85, None),
    Among("snjiju", 1813, 122, None),
    Among("šnjiju", 1813, 86, None),
    Among("kiju", 1813, 95, None),
    Among("liju", 1813, 24, None),
    Among("eliju", 1824, 83, None),
    Among("miju", 1813, 37, None),
    Among("niju", 1813, 13, None),
    Among("ganiju", 1827, 9, None),
    Among("maniju", 1827, 6, None),
    Among("paniju", 1827, 7, None),
    Among("raniju", 1827, 8, None),
    Among("taniju", 1827, 5, None),
    Among("piju", 1813, 41, None),
    Among("riju", 1813, 42, None),
    Among("rariju", 1834, 21, None),
    Among("siju", 1813, 23, None),
    Among("osiju", 1836, 123, None),
    Among("tiju", 1813, 44, None),
    Among("atiju", 1838, 120, None),
    Among("otiju", 1838, 22, None),
    Among("aviju", 1813, 77, None),
    Among("eviju", 1813, 78, None),
    Among("iviju", 1813, 79, None),
    Among("oviju", 1813, 80, None),
    Among("ziju", 1813, 45, None),
    Among("ošiju", 1813, 91, None),
    Among("žiju", 1813, 38, None),
    Among("anju", -1, 84, None),
    Among("enju", -1, 85, None),
    Among("snju", -1, 122, None),
    Among("šnju", -1, 86, None),
    Among("uju", -1, 25, None),
    Among("lucuju", 1852, 121, None),
    Among("iruju", 1852, 100, None),
    Among("lučuju", 1852, 117, None),
    Among("ku", -1, 95, None),
    Among("sku", 1856, 1, None),
    Among("šku", 1856, 2, None),
    Among("alu", -1, 104, None),
    Among("ijalu", 1859, 47, None),
    Among("nalu", 1859, 46, None),
    Among("elu", -1, 83, None),
    Among("ilu", -1, 116, None),
    Among("ozilu", 1863, 48, None),
    Among("olu", -1, 50, None),
    Among("ramu", -1, 52, None),
    Among("acemu", -1, 124, None),
    Among("ecemu", -1, 125, None),
    Among("ucemu", -1, 126, None),
    Among("anjijemu", -1, 84, None),
    Among("enjijemu", -1, 85, None),
    Among("snjijemu", -1, 122, None),
    Among("šnjijemu", -1, 86, None),
    Among("kijemu", -1, 95, None),
    Among("skijemu", 1874, 1, None),
    Among("škijemu", 1874, 2, None),
    Among("elijemu", -1, 83, None),
    Among("nijemu", -1, 13, None),
    Among("osijemu", -1, 123, None),
    Among("atijemu", -1, 120, None),
    Among("evitijemu", -1, 92, None),
    Among("ovitijemu", -1, 93, None),
    Among("astijemu", -1, 94, None),
    Among("avijemu", -1, 77, None),
    Among("evijemu", -1, 78, None),
    Among("ivijemu", -1, 79, None),
    Among("ovijemu", -1, 80, None),
    Among("ošijemu", -1, 91, None),
    Among("anjemu", -1, 84, None),
    Among("enjemu", -1, 85, None),
    Among("snjemu", -1, 122, None),
    Among("šnjemu", -1, 86, None),
    Among("kemu", -1, 95, None),
    Among("skemu", 1893, 1, None),
    Among("škemu", 1893, 2, None),
    Among("lemu", -1, 51, None),
    Among("elemu", 1896, 83, None),
    Among("nemu", -1, 13, None),
    Among("anemu", 1898, 10, None),
    Among("enemu", 1898, 87, None),
    Among("snemu", 1898, 159, None),
    Among("šnemu", 1898, 88, None),
    Among("osemu", -1, 123, None),
    Among("atemu", -1, 120, None),
    Among("evitemu", -1, 92, None),
    Among("ovitemu", -1, 93, None),
    Among("astemu", -1, 94, None),
    Among("avemu", -1, 77, None),
    Among("evemu", -1, 78, None),
    Among("ivemu", -1, 79, None),
    Among("ovemu", -1, 80, None),
    Among("aćemu", -1, 14, None),
    Among("ećemu", -1, 15, None),
    Among("ućemu", -1, 16, None),
    Among("ošemu", -1, 91, None),
    Among("acomu", -1, 124, None),
    Among("ecomu", -1, 125, None),
    Among("ucomu", -1, 126, None),
    Among("anjomu", -1, 84, None),
    Among("enjomu", -1, 85, None),
    Among("snjomu", -1, 122, None),
    Among("šnjomu", -1, 86, None),
    Among("komu", -1, 95, None),
    Among("skomu", 1923, 1, None),
    Among("škomu", 1923, 2, None),
    Among("elomu", -1, 83, None),
    Among("nomu", -1, 13, None),
    Among("cinomu", 1927, 137, None),
    Among("činomu", 1927, 89, None),
    Among("osomu", -1, 123, None),
    Among("atomu", -1, 120, None),
    Among("evitomu", -1, 92, None),
    Among("ovitomu", -1, 93, None),
    Among("astomu", -1, 94, None),
    Among("avomu", -1, 77, None),
    Among("evomu", -1, 78, None),
    Among("ivomu", -1, 79, None),
    Among("ovomu", -1, 80, None),
    Among("aćomu", -1, 14, None),
    Among("ećomu", -1, 15, None),
    Among("ućomu", -1, 16, None),
    Among("ošomu", -1, 91, None),
    Among("nu", -1, 13, None),
    Among("anu", 1943, 10, None),
    Among("astanu", 1944, 110, None),
    Among("istanu", 1944, 111, None),
    Among("ostanu", 1944, 112, None),
    Among("inu", 1943, 11, None),
    Among("cinu", 1948, 137, None),
    Among("aninu", 1948, 10, None),
    Among("činu", 1948, 89, None),
    Among("onu", 1943, 12, None),
    Among("aru", -1, 53, None),
    Among("dru", -1, 54, None),
    Among("eru", -1, 55, None),
    Among("oru", -1, 56, None),
    Among("basu", -1, 135, None),
    Among("gasu", -1, 131, None),
    Among("jasu", -1, 129, None),
    Among("kasu", -1, 133, None),
    Among("nasu", -1, 132, None),
    Among("tasu", -1, 130, None),
    Among("vasu", -1, 134, None),
    Among("esu", -1, 57, None),
    Among("isu", -1, 58, None),
    Among("osu", -1, 123, None),
    Among("atu", -1, 120, None),
    Among("ikatu", 1967, 68, None),
    Among("latu", 1967, 69, None),
    Among("etu", -1, 70, None),
    Among("evitu", -1, 92, None),
    Among("ovitu", -1, 93, None),
    Among("astu", -1, 94, None),
    Among("estu", -1, 71, None),
    Among("istu", -1, 72, None),
    Among("kstu", -1, 73, None),
    Among("ostu", -1, 74, None),
    Among("ištu", -1, 75, None),
    Among("avu", -1, 77, None),
    Among("evu", -1, 78, None),
    Among("ivu", -1, 79, None),
    Among("ovu", -1, 80, None),
    Among("lovu", 1982, 82, None),
    Among("movu", 1982, 81, None),
    Among("stvu", -1, 3, None),
    Among("štvu", -1, 4, None),
    Among("bašu", -1, 63, None),
    Among("gašu", -1, 64, None),
    Among("jašu", -1, 61, None),
    Among("kašu", -1, 62, None),
    Among("našu", -1, 60, None),
    Among("tašu", -1, 59, None),
    Among("vašu", -1, 65, None),
    Among("ešu", -1, 66, None),
    Among("išu", -1, 67, None),
    Among("ošu", -1, 91, None),
    Among("avav", -1, 97, None),
    Among("evav", -1, 96, None),
    Among("ivav", -1, 98, None),
    Among("uvav", -1, 99, None),
    Among("kov", -1, 95, None),
    Among("aš", -1, 104, None),
    Among("iraš", 2002, 100, None),
    Among("uraš", 2002, 105, None),
    Among("taš", 2002, 113, None),
    Among("avaš", 2002, 97, None),
    Among("evaš", 2002, 96, None),
    Among("ivaš", 2002, 98, None),
    Among("uvaš", 2002, 99, None),
    Among("ačaš", 2002, 102, None),
    Among("eš", -1, 119, None),
    Among("astadeš", 2011, 110, None),
    Among("istadeš", 2011, 111, None),
    Among("ostadeš", 2011, 112, None),
    Among("astaješ", 2011, 106, None),
    Among("istaješ", 2011, 107, None),
    Among("ostaješ", 2011, 108, None),
    Among("iješ", 2011, 116, None),
    Among("inješ", 2011, 114, None),
    Among("uješ", 2011, 25, None),
    Among("iruješ", 2020, 100, None),
    Among("lučuješ", 2020, 117, None),
    Among("neš", 2011, 13, None),
    Among("astaneš", 2023, 110, None),
    Among("istaneš", 2023, 111, None),
    Among("ostaneš", 2023, 112, None),
    Among("eteš", 2011, 70, None),
    Among("asteš", 2011, 115, None),
    Among("iš", -1, 116, None),
    Among("niš", 2029, 103, None),
    Among("jetiš", 2029, 118, None),
    Among("ačiš", 2029, 101, None),
    Among("lučiš", 2029, 117, None),
    Among("rošiš", 2029, 90, None),
];

static A_3: &'static [Among<Context>; 26] = &[
    Among("a", -1, 1, None),
    Among("oga", 0, 1, None),
    Among("ama", 0, 1, None),
    Among("ima", 0, 1, None),
    Among("ena", 0, 1, None),
    Among("e", -1, 1, None),
    Among("og", -1, 1, None),
    Among("anog", 6, 1, None),
    Among("enog", 6, 1, None),
    Among("anih", -1, 1, None),
    Among("enih", -1, 1, None),
    Among("i", -1, 1, None),
    Among("ani", 11, 1, None),
    Among("eni", 11, 1, None),
    Among("anoj", -1, 1, None),
    Among("enoj", -1, 1, None),
    Among("anim", -1, 1, None),
    Among("enim", -1, 1, None),
    Among("om", -1, 1, None),
    Among("enom", 18, 1, None),
    Among("o", -1, 1, None),
    Among("ano", 20, 1, None),
    Among("eno", 20, 1, None),
    Among("ost", -1, 1, None),
    Among("u", -1, 1, None),
    Among("enu", 24, 1, None),
];

static G_v: &'static [u8; 3] = &[17, 65, 16];

static G_sa: &'static [u8; 15] = &[65, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 128];

static G_ca: &'static [u8; 36] = &[119, 95, 23, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 136, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 0, 0, 0, 16];

static G_rg: &'static [u8; 1] = &[1];

fn r_cyr_to_lat(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    let v_1 = env.cursor;
    'lab0: loop {
        'replab1: loop{
            let v_2 = env.cursor;
            'lab2: for _ in 0..1 {
                'golab3: loop {
                    let v_3 = env.cursor;
                    'lab4: loop {
                        env.bra = env.cursor;
                        among_var = env.find_among(A_0, context);
                        if among_var == 0 {
                            break 'lab4;
                        }
                        env.ket = env.cursor;
                        match among_var {
                            1 => {
                                env.slice_from("a");
                            }
                            2 => {
                                env.slice_from("b");
                            }
                            3 => {
                                env.slice_from("v");
                            }
                            4 => {
                                env.slice_from("g");
                            }
                            5 => {
                                env.slice_from("d");
                            }
                            6 => {
                                env.slice_from("đ");
                            }
                            7 => {
                                env.slice_from("e");
                            }
                            8 => {
                                env.slice_from("ž");
                            }
                            9 => {
                                env.slice_from("z");
                            }
                            10 => {
                                env.slice_from("i");
                            }
                            11 => {
                                env.slice_from("j");
                            }
                            12 => {
                                env.slice_from("k");
                            }
                            13 => {
                                env.slice_from("l");
                            }
                            14 => {
                                env.slice_from("lj");
                            }
                            15 => {
                                env.slice_from("m");
                            }
                            16 => {
                                env.slice_from("n");
                            }
                            17 => {
                                env.slice_from("nj");
                            }
                            18 => {
                                env.slice_from("o");
                            }
                            19 => {
                                env.slice_from("p");
                            }
                            20 => {
                                env.slice_from("r");
                            }
                            21 => {
                                env.slice_from("s");
                            }
                            22 => {
                                env.slice_from("t");
                            }
                            23 => {
                                env.slice_from("ć");
                            }
                            24 => {
                                env.slice_from("u");
                            }
                            25 => {
                                env.slice_from("f");
                            }
                            26 => {
                                env.slice_from("h");
                            }
                            27 => {
                                env.slice_from("c");
                            }
                            28 => {
                                env.slice_from("č");
                            }
                            29 => {
                                env.slice_from("dž");
                            }
                            30 => {
                                env.slice_from("š");
                            }
                            _ => ()
                        }
                        env.cursor = v_3;
                        break 'golab3;
                    }
                    env.cursor = v_3;
                    if env.cursor >= env.limit {
                        break 'lab2;
                    }
                    env.next_char();
                }
                continue 'replab1;
            }
            env.cursor = v_2;
            break 'replab1;
        }
        break 'lab0;
    }
    env.cursor = v_1;
    return true
}

fn r_prelude(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let v_1 = env.cursor;
    'lab0: loop {
        'replab1: loop{
            let v_2 = env.cursor;
            'lab2: for _ in 0..1 {
                'golab3: loop {
                    let v_3 = env.cursor;
                    'lab4: loop {
                        if !env.in_grouping(G_ca, 98, 382) {
                            break 'lab4;
                        }
                        env.bra = env.cursor;
                        if !env.eq_s(&"ije") {
                            break 'lab4;
                        }
                        env.ket = env.cursor;
                        if !env.in_grouping(G_ca, 98, 382) {
                            break 'lab4;
                        }
                        env.slice_from("e");
                        env.cursor = v_3;
                        break 'golab3;
                    }
                    env.cursor = v_3;
                    if env.cursor >= env.limit {
                        break 'lab2;
                    }
                    env.next_char();
                }
                continue 'replab1;
            }
            env.cursor = v_2;
            break 'replab1;
        }
        break 'lab0;
    }
    env.cursor = v_1;
    let v_4 = env.cursor;
    'lab5: loop {
        'replab6: loop{
            let v_5 = env.cursor;
            'lab7: for _ in 0..1 {
                'golab8: loop {
                    let v_6 = env.cursor;
                    'lab9: loop {
                        if !env.in_grouping(G_ca, 98, 382) {
                            break 'lab9;
                        }
                        env.bra = env.cursor;
                        if !env.eq_s(&"je") {
                            break 'lab9;
                        }
                        env.ket = env.cursor;
                        if !env.in_grouping(G_ca, 98, 382) {
                            break 'lab9;
                        }
                        env.slice_from("e");
                        env.cursor = v_6;
                        break 'golab8;
                    }
                    env.cursor = v_6;
                    if env.cursor >= env.limit {
                        break 'lab7;
                    }
                    env.next_char();
                }
                continue 'replab6;
            }
            env.cursor = v_5;
            break 'replab6;
        }
        break 'lab5;
    }
    env.cursor = v_4;
    let v_7 = env.cursor;
    'lab10: loop {
        'replab11: loop{
            let v_8 = env.cursor;
            'lab12: for _ in 0..1 {
                'golab13: loop {
                    let v_9 = env.cursor;
                    'lab14: loop {
                        env.bra = env.cursor;
                        if !env.eq_s(&"dj") {
                            break 'lab14;
                        }
                        env.ket = env.cursor;
                        env.slice_from("đ");
                        env.cursor = v_9;
                        break 'golab13;
                    }
                    env.cursor = v_9;
                    if env.cursor >= env.limit {
                        break 'lab12;
                    }
                    env.next_char();
                }
                continue 'replab11;
            }
            env.cursor = v_8;
            break 'replab11;
        }
        break 'lab10;
    }
    env.cursor = v_7;
    return true
}

fn r_mark_regions(env: &mut SnowballEnv, context: &mut Context) -> bool {
    context.b_no_diacritics = true;
    let v_1 = env.cursor;
    'lab0: loop {
        if !env.go_out_grouping(G_sa, 263, 382) {
            break 'lab0;
        }
        env.next_char();
        context.b_no_diacritics = false;
        break 'lab0;
    }
    env.cursor = v_1;
    context.i_p1 = env.limit;
    let v_2 = env.cursor;
    'lab1: loop {
        if !env.go_out_grouping(G_v, 97, 117) {
            break 'lab1;
        }
        env.next_char();
        context.i_p1 = env.cursor;
        if context.i_p1 >= 2{
            break 'lab1;
        }
        if !env.go_in_grouping(G_v, 97, 117) {
            break 'lab1;
        }
        env.next_char();
        context.i_p1 = env.cursor;
        break 'lab1;
    }
    env.cursor = v_2;
    let v_3 = env.cursor;
    'lab2: loop {
        'golab3: loop {
            'lab4: loop {
                if !env.eq_s(&"r") {
                    break 'lab4;
                }
                break 'golab3;
            }
            if env.cursor >= env.limit {
                break 'lab2;
            }
            env.next_char();
        }
        'lab5: loop {
            let v_4 = env.cursor;
            'lab6: loop {
                if env.cursor < 2{
                    break 'lab6;
                }
                break 'lab5;
            }
            env.cursor = v_4;
            if !env.go_in_grouping(G_rg, 114, 114) {
                break 'lab2;
            }
            env.next_char();
            break 'lab5;
        }
        if (context.i_p1 - env.cursor) <= 1{
            break 'lab2;
        }
        context.i_p1 = env.cursor;
        break 'lab2;
    }
    env.cursor = v_3;
    return true
}

fn r_R1(env: &mut SnowballEnv, context: &mut Context) -> bool {
    return context.i_p1 <= env.cursor
}

fn r_Step_1(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    env.ket = env.cursor;
    if (env.cursor - 2 <= env.limit_backward || env.current.as_bytes()[(env.cursor - 1) as usize] as u8 >> 5 != 3 as u8 || ((3435050 as i32 >> (env.current.as_bytes()[(env.cursor - 1) as usize] as u8 & 0x1f)) & 1) == 0) {
        return false;
    }

    among_var = env.find_among_b(A_1, context);
    if among_var == 0 {
        return false;
    }
    env.bra = env.cursor;
    match among_var {
        1 => {
            env.slice_from("loga");
        }
        2 => {
            env.slice_from("peh");
        }
        3 => {
            env.slice_from("vojka");
        }
        4 => {
            env.slice_from("bojka");
        }
        5 => {
            env.slice_from("jak");
        }
        6 => {
            env.slice_from("čajni");
        }
        7 => {
            if !context.b_no_diacritics {
                return false;
            }
            env.slice_from("cajni");
        }
        8 => {
            env.slice_from("erni");
        }
        9 => {
            env.slice_from("larni");
        }
        10 => {
            env.slice_from("esni");
        }
        11 => {
            env.slice_from("anjca");
        }
        12 => {
            env.slice_from("ajca");
        }
        13 => {
            env.slice_from("ljca");
        }
        14 => {
            env.slice_from("ejca");
        }
        15 => {
            env.slice_from("ojca");
        }
        16 => {
            env.slice_from("ajka");
        }
        17 => {
            env.slice_from("ojka");
        }
        18 => {
            env.slice_from("šca");
        }
        19 => {
            env.slice_from("ing");
        }
        20 => {
            env.slice_from("tvenik");
        }
        21 => {
            env.slice_from("tetika");
        }
        22 => {
            env.slice_from("nstva");
        }
        23 => {
            env.slice_from("nik");
        }
        24 => {
            env.slice_from("tik");
        }
        25 => {
            env.slice_from("zik");
        }
        26 => {
            env.slice_from("snik");
        }
        27 => {
            env.slice_from("kusi");
        }
        28 => {
            env.slice_from("kusni");
        }
        29 => {
            env.slice_from("kustva");
        }
        30 => {
            env.slice_from("dušni");
        }
        31 => {
            if !context.b_no_diacritics {
                return false;
            }
            env.slice_from("dusni");
        }
        32 => {
            env.slice_from("antni");
        }
        33 => {
            env.slice_from("bilni");
        }
        34 => {
            env.slice_from("tilni");
        }
        35 => {
            env.slice_from("avilni");
        }
        36 => {
            env.slice_from("silni");
        }
        37 => {
            env.slice_from("gilni");
        }
        38 => {
            env.slice_from("rilni");
        }
        39 => {
            env.slice_from("nilni");
        }
        40 => {
            env.slice_from("alni");
        }
        41 => {
            env.slice_from("ozni");
        }
        42 => {
            env.slice_from("ravi");
        }
        43 => {
            env.slice_from("stavni");
        }
        44 => {
            env.slice_from("pravni");
        }
        45 => {
            env.slice_from("tivni");
        }
        46 => {
            env.slice_from("sivni");
        }
        47 => {
            env.slice_from("atni");
        }
        48 => {
            env.slice_from("enta");
        }
        49 => {
            env.slice_from("tetni");
        }
        50 => {
            env.slice_from("pletni");
        }
        51 => {
            env.slice_from("šavi");
        }
        52 => {
            if !context.b_no_diacritics {
                return false;
            }
            env.slice_from("savi");
        }
        53 => {
            env.slice_from("anta");
        }
        54 => {
            env.slice_from("ačka");
        }
        55 => {
            if !context.b_no_diacritics {
                return false;
            }
            env.slice_from("acka");
        }
        56 => {
            env.slice_from("uška");
        }
        57 => {
            if !context.b_no_diacritics {
                return false;
            }
            env.slice_from("uska");
        }
        58 => {
            env.slice_from("atka");
        }
        59 => {
            env.slice_from("etka");
        }
        60 => {
            env.slice_from("itka");
        }
        61 => {
            env.slice_from("otka");
        }
        62 => {
            env.slice_from("utka");
        }
        63 => {
            env.slice_from("eskna");
        }
        64 => {
            env.slice_from("tični");
        }
        65 => {
            if !context.b_no_diacritics {
                return false;
            }
            env.slice_from("ticni");
        }
        66 => {
            env.slice_from("ojska");
        }
        67 => {
            env.slice_from("esma");
        }
        68 => {
            env.slice_from("metra");
        }
        69 => {
            env.slice_from("centra");
        }
        70 => {
            env.slice_from("istra");
        }
        71 => {
            env.slice_from("osti");
        }
        72 => {
            if !context.b_no_diacritics {
                return false;
            }
            env.slice_from("osti");
        }
        73 => {
            env.slice_from("dba");
        }
        74 => {
            env.slice_from("čka");
        }
        75 => {
            env.slice_from("mca");
        }
        76 => {
            env.slice_from("nca");
        }
        77 => {
            env.slice_from("voljni");
        }
        78 => {
            env.slice_from("anki");
        }
        79 => {
            env.slice_from("vca");
        }
        80 => {
            env.slice_from("sca");
        }
        81 => {
            env.slice_from("rca");
        }
        82 => {
            env.slice_from("alca");
        }
        83 => {
            env.slice_from("elca");
        }
        84 => {
            env.slice_from("olca");
        }
        85 => {
            env.slice_from("njca");
        }
        86 => {
            env.slice_from("ekta");
        }
        87 => {
            env.slice_from("izma");
        }
        88 => {
            env.slice_from("jebi");
        }
        89 => {
            env.slice_from("baci");
        }
        90 => {
            env.slice_from("ašni");
        }
        91 => {
            if !context.b_no_diacritics {
                return false;
            }
            env.slice_from("asni");
        }
        _ => ()
    }
    return true
}

fn r_Step_2(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    env.ket = env.cursor;
    among_var = env.find_among_b(A_2, context);
    if among_var == 0 {
        return false;
    }
    env.bra = env.cursor;
    if !r_R1(env, context) {
        return false;
    }
    match among_var {
        1 => {
            env.slice_from("sk");
        }
        2 => {
            env.slice_from("šk");
        }
        3 => {
            env.slice_from("stv");
        }
        4 => {
            env.slice_from("štv");
        }
        5 => {
            env.slice_from("tanij");
        }
        6 => {
            env.slice_from("manij");
        }
        7 => {
            env.slice_from("panij");
        }
        8 => {
            env.slice_from("ranij");
        }
        9 => {
            env.slice_from("ganij");
        }
        10 => {
            env.slice_from("an");
        }
        11 => {
            env.slice_from("in");
        }
        12 => {
            env.slice_from("on");
        }
        13 => {
            env.slice_from("n");
        }
        14 => {
            env.slice_from("ać");
        }
        15 => {
            env.slice_from("eć");
        }
        16 => {
            env.slice_from("uć");
        }
        17 => {
            env.slice_from("ugov");
        }
        18 => {
            env.slice_from("ug");
        }
        19 => {
            env.slice_from("log");
        }
        20 => {
            env.slice_from("g");
        }
        21 => {
            env.slice_from("rari");
        }
        22 => {
            env.slice_from("oti");
        }
        23 => {
            env.slice_from("si");
        }
        24 => {
            env.slice_from("li");
        }
        25 => {
            env.slice_from("uj");
        }
        26 => {
            env.slice_from("caj");
        }
        27 => {
            env.slice_from("čaj");
        }
        28 => {
            env.slice_from("ćaj");
        }
        29 => {
            env.slice_from("đaj");
        }
        30 => {
            env.slice_from("laj");
        }
        31 => {
            env.slice_from("raj");
        }
        32 => {
            env.slice_from("bij");
        }
        33 => {
            env.slice_from("cij");
        }
        34 => {
            env.slice_from("dij");
        }
        35 => {
            env.slice_from("lij");
        }
        36 => {
            env.slice_from("nij");
        }
        37 => {
            env.slice_from("mij");
        }
        38 => {
            env.slice_from("žij");
        }
        39 => {
            env.slice_from("gij");
        }
        40 => {
            env.slice_from("fij");
        }
        41 => {
            env.slice_from("pij");
        }
        42 => {
            env.slice_from("rij");
        }
        43 => {
            env.slice_from("sij");
        }
        44 => {
            env.slice_from("tij");
        }
        45 => {
            env.slice_from("zij");
        }
        46 => {
            env.slice_from("nal");
        }
        47 => {
            env.slice_from("ijal");
        }
        48 => {
            env.slice_from("ozil");
        }
        49 => {
            env.slice_from("olov");
        }
        50 => {
            env.slice_from("ol");
        }
        51 => {
            env.slice_from("lem");
        }
        52 => {
            env.slice_from("ram");
        }
        53 => {
            env.slice_from("ar");
        }
        54 => {
            env.slice_from("dr");
        }
        55 => {
            env.slice_from("er");
        }
        56 => {
            env.slice_from("or");
        }
        57 => {
            env.slice_from("es");
        }
        58 => {
            env.slice_from("is");
        }
        59 => {
            env.slice_from("taš");
        }
        60 => {
            env.slice_from("naš");
        }
        61 => {
            env.slice_from("jaš");
        }
        62 => {
            env.slice_from("kaš");
        }
        63 => {
            env.slice_from("baš");
        }
        64 => {
            env.slice_from("gaš");
        }
        65 => {
            env.slice_from("vaš");
        }
        66 => {
            env.slice_from("eš");
        }
        67 => {
            env.slice_from("iš");
        }
        68 => {
            env.slice_from("ikat");
        }
        69 => {
            env.slice_from("lat");
        }
        70 => {
            env.slice_from("et");
        }
        71 => {
            env.slice_from("est");
        }
        72 => {
            env.slice_from("ist");
        }
        73 => {
            env.slice_from("kst");
        }
        74 => {
            env.slice_from("ost");
        }
        75 => {
            env.slice_from("išt");
        }
        76 => {
            env.slice_from("ova");
        }
        77 => {
            env.slice_from("av");
        }
        78 => {
            env.slice_from("ev");
        }
        79 => {
            env.slice_from("iv");
        }
        80 => {
            env.slice_from("ov");
        }
        81 => {
            env.slice_from("mov");
        }
        82 => {
            env.slice_from("lov");
        }
        83 => {
            env.slice_from("el");
        }
        84 => {
            env.slice_from("anj");
        }
        85 => {
            env.slice_from("enj");
        }
        86 => {
            env.slice_from("šnj");
        }
        87 => {
            env.slice_from("en");
        }
        88 => {
            env.slice_from("šn");
        }
        89 => {
            env.slice_from("čin");
        }
        90 => {
            env.slice_from("roši");
        }
        91 => {
            env.slice_from("oš");
        }
        92 => {
            env.slice_from("evit");
        }
        93 => {
            env.slice_from("ovit");
        }
        94 => {
            env.slice_from("ast");
        }
        95 => {
            env.slice_from("k");
        }
        96 => {
            env.slice_from("eva");
        }
        97 => {
            env.slice_from("ava");
        }
        98 => {
            env.slice_from("iva");
        }
        99 => {
            env.slice_from("uva");
        }
        100 => {
            env.slice_from("ir");
        }
        101 => {
            env.slice_from("ač");
        }
        102 => {
            env.slice_from("ača");
        }
        103 => {
            env.slice_from("ni");
        }
        104 => {
            env.slice_from("a");
        }
        105 => {
            env.slice_from("ur");
        }
        106 => {
            env.slice_from("astaj");
        }
        107 => {
            env.slice_from("istaj");
        }
        108 => {
            env.slice_from("ostaj");
        }
        109 => {
            env.slice_from("aj");
        }
        110 => {
            env.slice_from("asta");
        }
        111 => {
            env.slice_from("ista");
        }
        112 => {
            env.slice_from("osta");
        }
        113 => {
            env.slice_from("ta");
        }
        114 => {
            env.slice_from("inj");
        }
        115 => {
            env.slice_from("as");
        }
        116 => {
            env.slice_from("i");
        }
        117 => {
            env.slice_from("luč");
        }
        118 => {
            env.slice_from("jeti");
        }
        119 => {
            env.slice_from("e");
        }
        120 => {
            env.slice_from("at");
        }
        121 => {
            if !context.b_no_diacritics {
                return false;
            }
            env.slice_from("luc");
        }
        122 => {
            if !context.b_no_diacritics {
                return false;
            }
            env.slice_from("snj");
        }
        123 => {
            if !context.b_no_diacritics {
                return false;
            }
            env.slice_from("os");
        }
        124 => {
            if !context.b_no_diacritics {
                return false;
            }
            env.slice_from("ac");
        }
        125 => {
            if !context.b_no_diacritics {
                return false;
            }
            env.slice_from("ec");
        }
        126 => {
            if !context.b_no_diacritics {
                return false;
            }
            env.slice_from("uc");
        }
        127 => {
            if !context.b_no_diacritics {
                return false;
            }
            env.slice_from("rosi");
        }
        128 => {
            if !context.b_no_diacritics {
                return false;
            }
            env.slice_from("aca");
        }
        129 => {
            if !context.b_no_diacritics {
                return false;
            }
            env.slice_from("jas");
        }
        130 => {
            if !context.b_no_diacritics {
                return false;
            }
            env.slice_from("tas");
        }
        131 => {
            if !context.b_no_diacritics {
                return false;
            }
            env.slice_from("gas");
        }
        132 => {
            if !context.b_no_diacritics {
                return false;
            }
            env.slice_from("nas");
        }
        133 => {
            if !context.b_no_diacritics {
                return false;
            }
            env.slice_from("kas");
        }
        134 => {
            if !context.b_no_diacritics {
                return false;
            }
            env.slice_from("vas");
        }
        135 => {
            if !context.b_no_diacritics {
                return false;
            }
            env.slice_from("bas");
        }
        136 => {
            if !context.b_no_diacritics {
                return false;
            }
            env.slice_from("as");
        }
        137 => {
            if !context.b_no_diacritics {
                return false;
            }
            env.slice_from("cin");
        }
        138 => {
            if !context.b_no_diacritics {
                return false;
            }
            env.slice_from("astaj");
        }
        139 => {
            if !context.b_no_diacritics {
                return false;
            }
            env.slice_from("istaj");
        }
        140 => {
            if !context.b_no_diacritics {
                return false;
            }
            env.slice_from("ostaj");
        }
        141 => {
            if !context.b_no_diacritics {
                return false;
            }
            env.slice_from("asta");
        }
        142 => {
            if !context.b_no_diacritics {
                return false;
            }
            env.slice_from("ista");
        }
        143 => {
            if !context.b_no_diacritics {
                return false;
            }
            env.slice_from("osta");
        }
        144 => {
            if !context.b_no_diacritics {
                return false;
            }
            env.slice_from("ava");
        }
        145 => {
            if !context.b_no_diacritics {
                return false;
            }
            env.slice_from("eva");
        }
        146 => {
            if !context.b_no_diacritics {
                return false;
            }
            env.slice_from("iva");
        }
        147 => {
            if !context.b_no_diacritics {
                return false;
            }
            env.slice_from("uva");
        }
        148 => {
            if !context.b_no_diacritics {
                return false;
            }
            env.slice_from("ova");
        }
        149 => {
            if !context.b_no_diacritics {
                return false;
            }
            env.slice_from("jeti");
        }
        150 => {
            if !context.b_no_diacritics {
                return false;
            }
            env.slice_from("inj");
        }
        151 => {
            if !context.b_no_diacritics {
                return false;
            }
            env.slice_from("ist");
        }
        152 => {
            if !context.b_no_diacritics {
                return false;
            }
            env.slice_from("es");
        }
        153 => {
            if !context.b_no_diacritics {
                return false;
            }
            env.slice_from("et");
        }
        154 => {
            if !context.b_no_diacritics {
                return false;
            }
            env.slice_from("is");
        }
        155 => {
            if !context.b_no_diacritics {
                return false;
            }
            env.slice_from("ir");
        }
        156 => {
            if !context.b_no_diacritics {
                return false;
            }
            env.slice_from("ur");
        }
        157 => {
            if !context.b_no_diacritics {
                return false;
            }
            env.slice_from("uj");
        }
        158 => {
            if !context.b_no_diacritics {
                return false;
            }
            env.slice_from("ni");
        }
        159 => {
            if !context.b_no_diacritics {
                return false;
            }
            env.slice_from("sn");
        }
        160 => {
            if !context.b_no_diacritics {
                return false;
            }
            env.slice_from("ta");
        }
        161 => {
            if !context.b_no_diacritics {
                return false;
            }
            env.slice_from("a");
        }
        162 => {
            if !context.b_no_diacritics {
                return false;
            }
            env.slice_from("i");
        }
        163 => {
            if !context.b_no_diacritics {
                return false;
            }
            env.slice_from("e");
        }
        164 => {
            if !context.b_no_diacritics {
                return false;
            }
            env.slice_from("n");
        }
        _ => ()
    }
    return true
}

fn r_Step_3(env: &mut SnowballEnv, context: &mut Context) -> bool {
    env.ket = env.cursor;
    if (env.cursor <= env.limit_backward || env.current.as_bytes()[(env.cursor - 1) as usize] as u8 >> 5 != 3 as u8 || ((3188642 as i32 >> (env.current.as_bytes()[(env.cursor - 1) as usize] as u8 & 0x1f)) & 1) == 0) {
        return false;
    }

    if env.find_among_b(A_3, context) == 0 {
        return false;
    }
    env.bra = env.cursor;
    if !r_R1(env, context) {
        return false;
    }
    env.slice_from("");
    return true
}

pub fn stem(env: &mut SnowballEnv) -> bool {
    let mut context = &mut Context {
        i_p1: 0,
        b_no_diacritics: false,
    };
    r_cyr_to_lat(env, context);
    r_prelude(env, context);
    r_mark_regions(env, context);
    env.limit_backward = env.cursor;
    env.cursor = env.limit;
    let v_1 = env.limit - env.cursor;
    r_Step_1(env, context);
    env.cursor = env.limit - v_1;
    let v_2 = env.limit - env.cursor;
    'lab0: loop {
        'lab1: loop {
            let v_3 = env.limit - env.cursor;
            'lab2: loop {
                if !r_Step_2(env, context) {
                    break 'lab2;
                }
                break 'lab1;
            }
            env.cursor = env.limit - v_3;
            if !r_Step_3(env, context) {
                break 'lab0;
            }
            break 'lab1;
        }
        break 'lab0;
    }
    env.cursor = env.limit - v_2;
    env.cursor = env.limit_backward;
    return true
}
