//! Generated from catalan.sbl by Snowball 3.0.0 - https://snowballstem.org/

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_mut)]
#![allow(unused_parens)]
#![allow(unused_variables)]
use snowball::SnowballEnv;
use snowball::Among;

#[derive(Clone)]
struct Context {
    i_p2: i32,
    i_p1: i32,
}

static A_0: &'static [Among<Context>; 13] = &[
    Among("", -1, 7, None),
    Among("·", 0, 6, None),
    Among("à", 0, 1, None),
    Among("á", 0, 1, None),
    Among("è", 0, 2, None),
    Among("é", 0, 2, None),
    Among("ì", 0, 3, None),
    Among("í", 0, 3, None),
    Among("ï", 0, 3, None),
    Among("ò", 0, 4, None),
    Among("ó", 0, 4, None),
    Among("ú", 0, 5, None),
    Among("ü", 0, 5, None),
];

static A_1: &'static [Among<Context>; 39] = &[
    Among("la", -1, 1, None),
    Among("-la", 0, 1, None),
    Among("sela", 0, 1, None),
    Among("le", -1, 1, None),
    Among("me", -1, 1, None),
    Among("-me", 4, 1, None),
    Among("se", -1, 1, None),
    Among("-te", -1, 1, None),
    Among("hi", -1, 1, None),
    Among("'hi", 8, 1, None),
    Among("li", -1, 1, None),
    Among("-li", 10, 1, None),
    Among("'l", -1, 1, None),
    Among("'m", -1, 1, None),
    Among("-m", -1, 1, None),
    Among("'n", -1, 1, None),
    Among("-n", -1, 1, None),
    Among("ho", -1, 1, None),
    Among("'ho", 17, 1, None),
    Among("lo", -1, 1, None),
    Among("selo", 19, 1, None),
    Among("'s", -1, 1, None),
    Among("las", -1, 1, None),
    Among("selas", 22, 1, None),
    Among("les", -1, 1, None),
    Among("-les", 24, 1, None),
    Among("'ls", -1, 1, None),
    Among("-ls", -1, 1, None),
    Among("'ns", -1, 1, None),
    Among("-ns", -1, 1, None),
    Among("ens", -1, 1, None),
    Among("los", -1, 1, None),
    Among("selos", 31, 1, None),
    Among("nos", -1, 1, None),
    Among("-nos", 33, 1, None),
    Among("vos", -1, 1, None),
    Among("us", -1, 1, None),
    Among("-us", 36, 1, None),
    Among("'t", -1, 1, None),
];

static A_2: &'static [Among<Context>; 200] = &[
    Among("ica", -1, 4, None),
    Among("lógica", 0, 3, None),
    Among("enca", -1, 1, None),
    Among("ada", -1, 2, None),
    Among("ancia", -1, 1, None),
    Among("encia", -1, 1, None),
    Among("ència", -1, 1, None),
    Among("ícia", -1, 1, None),
    Among("logia", -1, 3, None),
    Among("inia", -1, 1, None),
    Among("íinia", 9, 1, None),
    Among("eria", -1, 1, None),
    Among("ària", -1, 1, None),
    Among("atòria", -1, 1, None),
    Among("alla", -1, 1, None),
    Among("ella", -1, 1, None),
    Among("ívola", -1, 1, None),
    Among("ima", -1, 1, None),
    Among("íssima", 17, 1, None),
    Among("quíssima", 18, 5, None),
    Among("ana", -1, 1, None),
    Among("ina", -1, 1, None),
    Among("era", -1, 1, None),
    Among("sfera", 22, 1, None),
    Among("ora", -1, 1, None),
    Among("dora", 24, 1, None),
    Among("adora", 25, 1, None),
    Among("adura", -1, 1, None),
    Among("esa", -1, 1, None),
    Among("osa", -1, 1, None),
    Among("assa", -1, 1, None),
    Among("essa", -1, 1, None),
    Among("issa", -1, 1, None),
    Among("eta", -1, 1, None),
    Among("ita", -1, 1, None),
    Among("ota", -1, 1, None),
    Among("ista", -1, 1, None),
    Among("ialista", 36, 1, None),
    Among("ionista", 36, 1, None),
    Among("iva", -1, 1, None),
    Among("ativa", 39, 1, None),
    Among("nça", -1, 1, None),
    Among("logía", -1, 3, None),
    Among("ic", -1, 4, None),
    Among("ístic", 43, 1, None),
    Among("enc", -1, 1, None),
    Among("esc", -1, 1, None),
    Among("ud", -1, 1, None),
    Among("atge", -1, 1, None),
    Among("ble", -1, 1, None),
    Among("able", 49, 1, None),
    Among("ible", 49, 1, None),
    Among("isme", -1, 1, None),
    Among("ialisme", 52, 1, None),
    Among("ionisme", 52, 1, None),
    Among("ivisme", 52, 1, None),
    Among("aire", -1, 1, None),
    Among("icte", -1, 1, None),
    Among("iste", -1, 1, None),
    Among("ici", -1, 1, None),
    Among("íci", -1, 1, None),
    Among("logi", -1, 3, None),
    Among("ari", -1, 1, None),
    Among("tori", -1, 1, None),
    Among("al", -1, 1, None),
    Among("il", -1, 1, None),
    Among("all", -1, 1, None),
    Among("ell", -1, 1, None),
    Among("ívol", -1, 1, None),
    Among("isam", -1, 1, None),
    Among("issem", -1, 1, None),
    Among("ìssem", -1, 1, None),
    Among("íssem", -1, 1, None),
    Among("íssim", -1, 1, None),
    Among("quíssim", 73, 5, None),
    Among("amen", -1, 1, None),
    Among("ìssin", -1, 1, None),
    Among("ar", -1, 1, None),
    Among("ificar", 77, 1, None),
    Among("egar", 77, 1, None),
    Among("ejar", 77, 1, None),
    Among("itar", 77, 1, None),
    Among("itzar", 77, 1, None),
    Among("fer", -1, 1, None),
    Among("or", -1, 1, None),
    Among("dor", 84, 1, None),
    Among("dur", -1, 1, None),
    Among("doras", -1, 1, None),
    Among("ics", -1, 4, None),
    Among("lógics", 88, 3, None),
    Among("uds", -1, 1, None),
    Among("nces", -1, 1, None),
    Among("ades", -1, 2, None),
    Among("ancies", -1, 1, None),
    Among("encies", -1, 1, None),
    Among("ències", -1, 1, None),
    Among("ícies", -1, 1, None),
    Among("logies", -1, 3, None),
    Among("inies", -1, 1, None),
    Among("ínies", -1, 1, None),
    Among("eries", -1, 1, None),
    Among("àries", -1, 1, None),
    Among("atòries", -1, 1, None),
    Among("bles", -1, 1, None),
    Among("ables", 103, 1, None),
    Among("ibles", 103, 1, None),
    Among("imes", -1, 1, None),
    Among("íssimes", 106, 1, None),
    Among("quíssimes", 107, 5, None),
    Among("formes", -1, 1, None),
    Among("ismes", -1, 1, None),
    Among("ialismes", 110, 1, None),
    Among("ines", -1, 1, None),
    Among("eres", -1, 1, None),
    Among("ores", -1, 1, None),
    Among("dores", 114, 1, None),
    Among("idores", 115, 1, None),
    Among("dures", -1, 1, None),
    Among("eses", -1, 1, None),
    Among("oses", -1, 1, None),
    Among("asses", -1, 1, None),
    Among("ictes", -1, 1, None),
    Among("ites", -1, 1, None),
    Among("otes", -1, 1, None),
    Among("istes", -1, 1, None),
    Among("ialistes", 124, 1, None),
    Among("ionistes", 124, 1, None),
    Among("iques", -1, 4, None),
    Among("lógiques", 127, 3, None),
    Among("ives", -1, 1, None),
    Among("atives", 129, 1, None),
    Among("logíes", -1, 3, None),
    Among("allengües", -1, 1, None),
    Among("icis", -1, 1, None),
    Among("ícis", -1, 1, None),
    Among("logis", -1, 3, None),
    Among("aris", -1, 1, None),
    Among("toris", -1, 1, None),
    Among("ls", -1, 1, None),
    Among("als", 138, 1, None),
    Among("ells", 138, 1, None),
    Among("ims", -1, 1, None),
    Among("íssims", 141, 1, None),
    Among("quíssims", 142, 5, None),
    Among("ions", -1, 1, None),
    Among("cions", 144, 1, None),
    Among("acions", 145, 2, None),
    Among("esos", -1, 1, None),
    Among("osos", -1, 1, None),
    Among("assos", -1, 1, None),
    Among("issos", -1, 1, None),
    Among("ers", -1, 1, None),
    Among("ors", -1, 1, None),
    Among("dors", 152, 1, None),
    Among("adors", 153, 1, None),
    Among("idors", 153, 1, None),
    Among("ats", -1, 1, None),
    Among("itats", 156, 1, None),
    Among("bilitats", 157, 1, None),
    Among("ivitats", 157, 1, None),
    Among("ativitats", 159, 1, None),
    Among("ïtats", 156, 1, None),
    Among("ets", -1, 1, None),
    Among("ants", -1, 1, None),
    Among("ents", -1, 1, None),
    Among("ments", 164, 1, None),
    Among("aments", 165, 1, None),
    Among("ots", -1, 1, None),
    Among("uts", -1, 1, None),
    Among("ius", -1, 1, None),
    Among("trius", 169, 1, None),
    Among("atius", 169, 1, None),
    Among("ès", -1, 1, None),
    Among("és", -1, 1, None),
    Among("ís", -1, 1, None),
    Among("dís", 174, 1, None),
    Among("ós", -1, 1, None),
    Among("itat", -1, 1, None),
    Among("bilitat", 177, 1, None),
    Among("ivitat", 177, 1, None),
    Among("ativitat", 179, 1, None),
    Among("ïtat", -1, 1, None),
    Among("et", -1, 1, None),
    Among("ant", -1, 1, None),
    Among("ent", -1, 1, None),
    Among("ient", 184, 1, None),
    Among("ment", 184, 1, None),
    Among("ament", 186, 1, None),
    Among("isament", 187, 1, None),
    Among("ot", -1, 1, None),
    Among("isseu", -1, 1, None),
    Among("ìsseu", -1, 1, None),
    Among("ísseu", -1, 1, None),
    Among("triu", -1, 1, None),
    Among("íssiu", -1, 1, None),
    Among("atiu", -1, 1, None),
    Among("ó", -1, 1, None),
    Among("ió", 196, 1, None),
    Among("ció", 197, 1, None),
    Among("ació", 198, 1, None),
];

static A_3: &'static [Among<Context>; 283] = &[
    Among("aba", -1, 1, None),
    Among("esca", -1, 1, None),
    Among("isca", -1, 1, None),
    Among("ïsca", -1, 1, None),
    Among("ada", -1, 1, None),
    Among("ida", -1, 1, None),
    Among("uda", -1, 1, None),
    Among("ïda", -1, 1, None),
    Among("ia", -1, 1, None),
    Among("aria", 8, 1, None),
    Among("iria", 8, 1, None),
    Among("ara", -1, 1, None),
    Among("iera", -1, 1, None),
    Among("ira", -1, 1, None),
    Among("adora", -1, 1, None),
    Among("ïra", -1, 1, None),
    Among("ava", -1, 1, None),
    Among("ixa", -1, 1, None),
    Among("itza", -1, 1, None),
    Among("ía", -1, 1, None),
    Among("aría", 19, 1, None),
    Among("ería", 19, 1, None),
    Among("iría", 19, 1, None),
    Among("ïa", -1, 1, None),
    Among("isc", -1, 1, None),
    Among("ïsc", -1, 1, None),
    Among("ad", -1, 1, None),
    Among("ed", -1, 1, None),
    Among("id", -1, 1, None),
    Among("ie", -1, 1, None),
    Among("re", -1, 1, None),
    Among("dre", 30, 1, None),
    Among("ase", -1, 1, None),
    Among("iese", -1, 1, None),
    Among("aste", -1, 1, None),
    Among("iste", -1, 1, None),
    Among("ii", -1, 1, None),
    Among("ini", -1, 1, None),
    Among("esqui", -1, 1, None),
    Among("eixi", -1, 1, None),
    Among("itzi", -1, 1, None),
    Among("am", -1, 1, None),
    Among("em", -1, 1, None),
    Among("arem", 42, 1, None),
    Among("irem", 42, 1, None),
    Among("àrem", 42, 1, None),
    Among("írem", 42, 1, None),
    Among("àssem", 42, 1, None),
    Among("éssem", 42, 1, None),
    Among("iguem", 42, 1, None),
    Among("ïguem", 42, 1, None),
    Among("avem", 42, 1, None),
    Among("àvem", 42, 1, None),
    Among("ávem", 42, 1, None),
    Among("irìem", 42, 1, None),
    Among("íem", 42, 1, None),
    Among("aríem", 55, 1, None),
    Among("iríem", 55, 1, None),
    Among("assim", -1, 1, None),
    Among("essim", -1, 1, None),
    Among("issim", -1, 1, None),
    Among("àssim", -1, 1, None),
    Among("èssim", -1, 1, None),
    Among("éssim", -1, 1, None),
    Among("íssim", -1, 1, None),
    Among("ïm", -1, 1, None),
    Among("an", -1, 1, None),
    Among("aban", 66, 1, None),
    Among("arian", 66, 1, None),
    Among("aran", 66, 1, None),
    Among("ieran", 66, 1, None),
    Among("iran", 66, 1, None),
    Among("ían", 66, 1, None),
    Among("arían", 72, 1, None),
    Among("erían", 72, 1, None),
    Among("irían", 72, 1, None),
    Among("en", -1, 1, None),
    Among("ien", 76, 1, None),
    Among("arien", 77, 1, None),
    Among("irien", 77, 1, None),
    Among("aren", 76, 1, None),
    Among("eren", 76, 1, None),
    Among("iren", 76, 1, None),
    Among("àren", 76, 1, None),
    Among("ïren", 76, 1, None),
    Among("asen", 76, 1, None),
    Among("iesen", 76, 1, None),
    Among("assen", 76, 1, None),
    Among("essen", 76, 1, None),
    Among("issen", 76, 1, None),
    Among("éssen", 76, 1, None),
    Among("ïssen", 76, 1, None),
    Among("esquen", 76, 1, None),
    Among("isquen", 76, 1, None),
    Among("ïsquen", 76, 1, None),
    Among("aven", 76, 1, None),
    Among("ixen", 76, 1, None),
    Among("eixen", 96, 1, None),
    Among("ïxen", 76, 1, None),
    Among("ïen", 76, 1, None),
    Among("in", -1, 1, None),
    Among("inin", 100, 1, None),
    Among("sin", 100, 1, None),
    Among("isin", 102, 1, None),
    Among("assin", 102, 1, None),
    Among("essin", 102, 1, None),
    Among("issin", 102, 1, None),
    Among("ïssin", 102, 1, None),
    Among("esquin", 100, 1, None),
    Among("eixin", 100, 1, None),
    Among("aron", -1, 1, None),
    Among("ieron", -1, 1, None),
    Among("arán", -1, 1, None),
    Among("erán", -1, 1, None),
    Among("irán", -1, 1, None),
    Among("iïn", -1, 1, None),
    Among("ado", -1, 1, None),
    Among("ido", -1, 1, None),
    Among("ando", -1, 2, None),
    Among("iendo", -1, 1, None),
    Among("io", -1, 1, None),
    Among("ixo", -1, 1, None),
    Among("eixo", 121, 1, None),
    Among("ïxo", -1, 1, None),
    Among("itzo", -1, 1, None),
    Among("ar", -1, 1, None),
    Among("tzar", 125, 1, None),
    Among("er", -1, 1, None),
    Among("eixer", 127, 1, None),
    Among("ir", -1, 1, None),
    Among("ador", -1, 1, None),
    Among("as", -1, 1, None),
    Among("abas", 131, 1, None),
    Among("adas", 131, 1, None),
    Among("idas", 131, 1, None),
    Among("aras", 131, 1, None),
    Among("ieras", 131, 1, None),
    Among("ías", 131, 1, None),
    Among("arías", 137, 1, None),
    Among("erías", 137, 1, None),
    Among("irías", 137, 1, None),
    Among("ids", -1, 1, None),
    Among("es", -1, 1, None),
    Among("ades", 142, 1, None),
    Among("ides", 142, 1, None),
    Among("udes", 142, 1, None),
    Among("ïdes", 142, 1, None),
    Among("atges", 142, 1, None),
    Among("ies", 142, 1, None),
    Among("aries", 148, 1, None),
    Among("iries", 148, 1, None),
    Among("ares", 142, 1, None),
    Among("ires", 142, 1, None),
    Among("adores", 142, 1, None),
    Among("ïres", 142, 1, None),
    Among("ases", 142, 1, None),
    Among("ieses", 142, 1, None),
    Among("asses", 142, 1, None),
    Among("esses", 142, 1, None),
    Among("isses", 142, 1, None),
    Among("ïsses", 142, 1, None),
    Among("ques", 142, 1, None),
    Among("esques", 161, 1, None),
    Among("ïsques", 161, 1, None),
    Among("aves", 142, 1, None),
    Among("ixes", 142, 1, None),
    Among("eixes", 165, 1, None),
    Among("ïxes", 142, 1, None),
    Among("ïes", 142, 1, None),
    Among("abais", -1, 1, None),
    Among("arais", -1, 1, None),
    Among("ierais", -1, 1, None),
    Among("íais", -1, 1, None),
    Among("aríais", 172, 1, None),
    Among("eríais", 172, 1, None),
    Among("iríais", 172, 1, None),
    Among("aseis", -1, 1, None),
    Among("ieseis", -1, 1, None),
    Among("asteis", -1, 1, None),
    Among("isteis", -1, 1, None),
    Among("inis", -1, 1, None),
    Among("sis", -1, 1, None),
    Among("isis", 181, 1, None),
    Among("assis", 181, 1, None),
    Among("essis", 181, 1, None),
    Among("issis", 181, 1, None),
    Among("ïssis", 181, 1, None),
    Among("esquis", -1, 1, None),
    Among("eixis", -1, 1, None),
    Among("itzis", -1, 1, None),
    Among("áis", -1, 1, None),
    Among("aréis", -1, 1, None),
    Among("eréis", -1, 1, None),
    Among("iréis", -1, 1, None),
    Among("ams", -1, 1, None),
    Among("ados", -1, 1, None),
    Among("idos", -1, 1, None),
    Among("amos", -1, 1, None),
    Among("ábamos", 197, 1, None),
    Among("áramos", 197, 1, None),
    Among("iéramos", 197, 1, None),
    Among("íamos", 197, 1, None),
    Among("aríamos", 201, 1, None),
    Among("eríamos", 201, 1, None),
    Among("iríamos", 201, 1, None),
    Among("aremos", -1, 1, None),
    Among("eremos", -1, 1, None),
    Among("iremos", -1, 1, None),
    Among("ásemos", -1, 1, None),
    Among("iésemos", -1, 1, None),
    Among("imos", -1, 1, None),
    Among("adors", -1, 1, None),
    Among("ass", -1, 1, None),
    Among("erass", 212, 1, None),
    Among("ess", -1, 1, None),
    Among("ats", -1, 1, None),
    Among("its", -1, 1, None),
    Among("ents", -1, 1, None),
    Among("às", -1, 1, None),
    Among("aràs", 218, 1, None),
    Among("iràs", 218, 1, None),
    Among("arás", -1, 1, None),
    Among("erás", -1, 1, None),
    Among("irás", -1, 1, None),
    Among("és", -1, 1, None),
    Among("arés", 224, 1, None),
    Among("ís", -1, 1, None),
    Among("iïs", -1, 1, None),
    Among("at", -1, 1, None),
    Among("it", -1, 1, None),
    Among("ant", -1, 1, None),
    Among("ent", -1, 1, None),
    Among("int", -1, 1, None),
    Among("ut", -1, 1, None),
    Among("ït", -1, 1, None),
    Among("au", -1, 1, None),
    Among("erau", 235, 1, None),
    Among("ieu", -1, 1, None),
    Among("ineu", -1, 1, None),
    Among("areu", -1, 1, None),
    Among("ireu", -1, 1, None),
    Among("àreu", -1, 1, None),
    Among("íreu", -1, 1, None),
    Among("asseu", -1, 1, None),
    Among("esseu", -1, 1, None),
    Among("eresseu", 244, 1, None),
    Among("àsseu", -1, 1, None),
    Among("ésseu", -1, 1, None),
    Among("igueu", -1, 1, None),
    Among("ïgueu", -1, 1, None),
    Among("àveu", -1, 1, None),
    Among("áveu", -1, 1, None),
    Among("itzeu", -1, 1, None),
    Among("ìeu", -1, 1, None),
    Among("irìeu", 253, 1, None),
    Among("íeu", -1, 1, None),
    Among("aríeu", 255, 1, None),
    Among("iríeu", 255, 1, None),
    Among("assiu", -1, 1, None),
    Among("issiu", -1, 1, None),
    Among("àssiu", -1, 1, None),
    Among("èssiu", -1, 1, None),
    Among("éssiu", -1, 1, None),
    Among("íssiu", -1, 1, None),
    Among("ïu", -1, 1, None),
    Among("ix", -1, 1, None),
    Among("eix", 265, 1, None),
    Among("ïx", -1, 1, None),
    Among("itz", -1, 1, None),
    Among("ià", -1, 1, None),
    Among("arà", -1, 1, None),
    Among("irà", -1, 1, None),
    Among("itzà", -1, 1, None),
    Among("ará", -1, 1, None),
    Among("erá", -1, 1, None),
    Among("irá", -1, 1, None),
    Among("irè", -1, 1, None),
    Among("aré", -1, 1, None),
    Among("eré", -1, 1, None),
    Among("iré", -1, 1, None),
    Among("í", -1, 1, None),
    Among("iï", -1, 1, None),
    Among("ió", -1, 1, None),
];

static A_4: &'static [Among<Context>; 22] = &[
    Among("a", -1, 1, None),
    Among("e", -1, 1, None),
    Among("i", -1, 1, None),
    Among("ïn", -1, 1, None),
    Among("o", -1, 1, None),
    Among("ir", -1, 1, None),
    Among("s", -1, 1, None),
    Among("is", 6, 1, None),
    Among("os", 6, 1, None),
    Among("ïs", 6, 1, None),
    Among("it", -1, 1, None),
    Among("eu", -1, 1, None),
    Among("iu", -1, 1, None),
    Among("iqu", -1, 2, None),
    Among("itz", -1, 1, None),
    Among("à", -1, 1, None),
    Among("á", -1, 1, None),
    Among("é", -1, 1, None),
    Among("ì", -1, 1, None),
    Among("í", -1, 1, None),
    Among("ï", -1, 1, None),
    Among("ó", -1, 1, None),
];

static G_v: &'static [u8; 20] = &[17, 65, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 129, 81, 6, 10];

fn r_mark_regions(env: &mut SnowballEnv, context: &mut Context) -> bool {
    context.i_p1 = env.limit;
    context.i_p2 = env.limit;
    let v_1 = env.cursor;
    'lab0: loop {
        if !env.go_out_grouping(G_v, 97, 252) {
            break 'lab0;
        }
        env.next_char();
        if !env.go_in_grouping(G_v, 97, 252) {
            break 'lab0;
        }
        env.next_char();
        context.i_p1 = env.cursor;
        if !env.go_out_grouping(G_v, 97, 252) {
            break 'lab0;
        }
        env.next_char();
        if !env.go_in_grouping(G_v, 97, 252) {
            break 'lab0;
        }
        env.next_char();
        context.i_p2 = env.cursor;
        break 'lab0;
    }
    env.cursor = v_1;
    return true
}

fn r_cleaning(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    'replab0: loop{
        let v_1 = env.cursor;
        'lab1: for _ in 0..1 {
            env.bra = env.cursor;
            if (env.cursor + 1 >= env.limit || env.current.as_bytes()[(env.cursor + 1) as usize] as u8 >> 5 != 5 as u8 || ((344765187 as i32 >> (env.current.as_bytes()[(env.cursor + 1) as usize] as u8 & 0x1f)) & 1) == 0) {among_var = 7;}
            else {
                among_var = env.find_among(A_0, context);
            }
            env.ket = env.cursor;
            match among_var {
                1 => {
                    env.slice_from("a");
                }
                2 => {
                    env.slice_from("e");
                }
                3 => {
                    env.slice_from("i");
                }
                4 => {
                    env.slice_from("o");
                }
                5 => {
                    env.slice_from("u");
                }
                6 => {
                    env.slice_from(".");
                }
                7 => {
                    if env.cursor >= env.limit {
                        break 'lab1;
                    }
                    env.next_char();
                }
                _ => ()
            }
            continue 'replab0;
        }
        env.cursor = v_1;
        break 'replab0;
    }
    return true
}

fn r_R1(env: &mut SnowballEnv, context: &mut Context) -> bool {
    return context.i_p1 <= env.cursor
}

fn r_R2(env: &mut SnowballEnv, context: &mut Context) -> bool {
    return context.i_p2 <= env.cursor
}

fn r_attached_pronoun(env: &mut SnowballEnv, context: &mut Context) -> bool {
    env.ket = env.cursor;
    if (env.cursor - 1 <= env.limit_backward || env.current.as_bytes()[(env.cursor - 1) as usize] as u8 >> 5 != 3 as u8 || ((1634850 as i32 >> (env.current.as_bytes()[(env.cursor - 1) as usize] as u8 & 0x1f)) & 1) == 0) {
        return false;
    }

    if env.find_among_b(A_1, context) == 0 {
        return false;
    }
    env.bra = env.cursor;
    if !r_R1(env, context) {
        return false;
    }
    env.slice_del();
    return true
}

fn r_standard_suffix(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    env.ket = env.cursor;
    among_var = env.find_among_b(A_2, context);
    if among_var == 0 {
        return false;
    }
    env.bra = env.cursor;
    match among_var {
        1 => {
            if !r_R1(env, context) {
                return false;
            }
            env.slice_del();
        }
        2 => {
            if !r_R2(env, context) {
                return false;
            }
            env.slice_del();
        }
        3 => {
            if !r_R2(env, context) {
                return false;
            }
            env.slice_from("log");
        }
        4 => {
            if !r_R2(env, context) {
                return false;
            }
            env.slice_from("ic");
        }
        5 => {
            if !r_R1(env, context) {
                return false;
            }
            env.slice_from("c");
        }
        _ => ()
    }
    return true
}

fn r_verb_suffix(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    env.ket = env.cursor;
    among_var = env.find_among_b(A_3, context);
    if among_var == 0 {
        return false;
    }
    env.bra = env.cursor;
    match among_var {
        1 => {
            if !r_R1(env, context) {
                return false;
            }
            env.slice_del();
        }
        2 => {
            if !r_R2(env, context) {
                return false;
            }
            env.slice_del();
        }
        _ => ()
    }
    return true
}

fn r_residual_suffix(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    env.ket = env.cursor;
    among_var = env.find_among_b(A_4, context);
    if among_var == 0 {
        return false;
    }
    env.bra = env.cursor;
    match among_var {
        1 => {
            if !r_R1(env, context) {
                return false;
            }
            env.slice_del();
        }
        2 => {
            if !r_R1(env, context) {
                return false;
            }
            env.slice_from("ic");
        }
        _ => ()
    }
    return true
}

pub fn stem(env: &mut SnowballEnv) -> bool {
    let mut context = &mut Context {
        i_p2: 0,
        i_p1: 0,
    };
    r_mark_regions(env, context);
    env.limit_backward = env.cursor;
    env.cursor = env.limit;
    let v_1 = env.limit - env.cursor;
    r_attached_pronoun(env, context);
    env.cursor = env.limit - v_1;
    let v_2 = env.limit - env.cursor;
    'lab0: loop {
        'lab1: loop {
            let v_3 = env.limit - env.cursor;
            'lab2: loop {
                if !r_standard_suffix(env, context) {
                    break 'lab2;
                }
                break 'lab1;
            }
            env.cursor = env.limit - v_3;
            if !r_verb_suffix(env, context) {
                break 'lab0;
            }
            break 'lab1;
        }
        break 'lab0;
    }
    env.cursor = env.limit - v_2;
    let v_4 = env.limit - env.cursor;
    r_residual_suffix(env, context);
    env.cursor = env.limit - v_4;
    env.cursor = env.limit_backward;
    let v_5 = env.cursor;
    r_cleaning(env, context);
    env.cursor = v_5;
    return true
}
