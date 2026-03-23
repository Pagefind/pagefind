//! Generated from lithuanian.sbl by Snowball 3.0.0 - https://snowballstem.org/

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
}

static A_0: &'static [Among<Context>; 193] = &[
    Among("a", -1, -1, None),
    Among("ia", 0, -1, None),
    Among("osna", 0, -1, None),
    Among("iosna", 2, -1, None),
    Among("uosna", 2, -1, None),
    Among("iuosna", 4, -1, None),
    Among("ysna", 0, -1, None),
    Among("ėsna", 0, -1, None),
    Among("e", -1, -1, None),
    Among("ie", 8, -1, None),
    Among("enie", 9, -1, None),
    Among("oje", 8, -1, None),
    Among("ioje", 11, -1, None),
    Among("uje", 8, -1, None),
    Among("iuje", 13, -1, None),
    Among("yje", 8, -1, None),
    Among("enyje", 15, -1, None),
    Among("ėje", 8, -1, None),
    Among("ame", 8, -1, None),
    Among("iame", 18, -1, None),
    Among("sime", 8, -1, None),
    Among("ome", 8, -1, None),
    Among("ėme", 8, -1, None),
    Among("tumėme", 22, -1, None),
    Among("ose", 8, -1, None),
    Among("iose", 24, -1, None),
    Among("uose", 24, -1, None),
    Among("iuose", 26, -1, None),
    Among("yse", 8, -1, None),
    Among("enyse", 28, -1, None),
    Among("ėse", 8, -1, None),
    Among("ate", 8, -1, None),
    Among("iate", 31, -1, None),
    Among("ite", 8, -1, None),
    Among("kite", 33, -1, None),
    Among("site", 33, -1, None),
    Among("ote", 8, -1, None),
    Among("tute", 8, -1, None),
    Among("ėte", 8, -1, None),
    Among("tumėte", 38, -1, None),
    Among("i", -1, -1, None),
    Among("ai", 40, -1, None),
    Among("iai", 41, -1, None),
    Among("ei", 40, -1, None),
    Among("tumei", 43, -1, None),
    Among("ki", 40, -1, None),
    Among("imi", 40, -1, None),
    Among("umi", 40, -1, None),
    Among("iumi", 47, -1, None),
    Among("si", 40, -1, None),
    Among("asi", 49, -1, None),
    Among("iasi", 50, -1, None),
    Among("esi", 49, -1, None),
    Among("iesi", 52, -1, None),
    Among("siesi", 53, -1, None),
    Among("isi", 49, -1, None),
    Among("aisi", 55, -1, None),
    Among("eisi", 55, -1, None),
    Among("tumeisi", 57, -1, None),
    Among("uisi", 55, -1, None),
    Among("osi", 49, -1, None),
    Among("ėjosi", 60, -1, None),
    Among("uosi", 60, -1, None),
    Among("iuosi", 62, -1, None),
    Among("siuosi", 63, -1, None),
    Among("usi", 49, -1, None),
    Among("ausi", 65, -1, None),
    Among("čiausi", 66, -1, None),
    Among("ąsi", 49, -1, None),
    Among("ėsi", 49, -1, None),
    Among("ųsi", 49, -1, None),
    Among("tųsi", 70, -1, None),
    Among("ti", 40, -1, None),
    Among("enti", 72, -1, None),
    Among("inti", 72, -1, None),
    Among("oti", 72, -1, None),
    Among("ioti", 75, -1, None),
    Among("uoti", 75, -1, None),
    Among("iuoti", 77, -1, None),
    Among("auti", 72, -1, None),
    Among("iauti", 79, -1, None),
    Among("yti", 72, -1, None),
    Among("ėti", 72, -1, None),
    Among("telėti", 82, -1, None),
    Among("inėti", 82, -1, None),
    Among("terėti", 82, -1, None),
    Among("ui", 40, -1, None),
    Among("iui", 86, -1, None),
    Among("eniui", 87, -1, None),
    Among("oj", -1, -1, None),
    Among("ėj", -1, -1, None),
    Among("k", -1, -1, None),
    Among("am", -1, -1, None),
    Among("iam", 92, -1, None),
    Among("iem", -1, -1, None),
    Among("im", -1, -1, None),
    Among("sim", 95, -1, None),
    Among("om", -1, -1, None),
    Among("tum", -1, -1, None),
    Among("ėm", -1, -1, None),
    Among("tumėm", 99, -1, None),
    Among("an", -1, -1, None),
    Among("on", -1, -1, None),
    Among("ion", 102, -1, None),
    Among("un", -1, -1, None),
    Among("iun", 104, -1, None),
    Among("ėn", -1, -1, None),
    Among("o", -1, -1, None),
    Among("io", 107, -1, None),
    Among("enio", 108, -1, None),
    Among("ėjo", 107, -1, None),
    Among("uo", 107, -1, None),
    Among("s", -1, -1, None),
    Among("as", 112, -1, None),
    Among("ias", 113, -1, None),
    Among("es", 112, -1, None),
    Among("ies", 115, -1, None),
    Among("is", 112, -1, None),
    Among("ais", 117, -1, None),
    Among("iais", 118, -1, None),
    Among("tumeis", 117, -1, None),
    Among("imis", 117, -1, None),
    Among("enimis", 121, -1, None),
    Among("omis", 117, -1, None),
    Among("iomis", 123, -1, None),
    Among("umis", 117, -1, None),
    Among("ėmis", 117, -1, None),
    Among("enis", 117, -1, None),
    Among("asis", 117, -1, None),
    Among("ysis", 117, -1, None),
    Among("ams", 112, -1, None),
    Among("iams", 130, -1, None),
    Among("iems", 112, -1, None),
    Among("ims", 112, -1, None),
    Among("enims", 133, -1, None),
    Among("oms", 112, -1, None),
    Among("ioms", 135, -1, None),
    Among("ums", 112, -1, None),
    Among("ėms", 112, -1, None),
    Among("ens", 112, -1, None),
    Among("os", 112, -1, None),
    Among("ios", 140, -1, None),
    Among("uos", 140, -1, None),
    Among("iuos", 142, -1, None),
    Among("us", 112, -1, None),
    Among("aus", 144, -1, None),
    Among("iaus", 145, -1, None),
    Among("ius", 144, -1, None),
    Among("ys", 112, -1, None),
    Among("enys", 148, -1, None),
    Among("ąs", 112, -1, None),
    Among("iąs", 150, -1, None),
    Among("ės", 112, -1, None),
    Among("amės", 152, -1, None),
    Among("iamės", 153, -1, None),
    Among("imės", 152, -1, None),
    Among("kimės", 155, -1, None),
    Among("simės", 155, -1, None),
    Among("omės", 152, -1, None),
    Among("ėmės", 152, -1, None),
    Among("tumėmės", 159, -1, None),
    Among("atės", 152, -1, None),
    Among("iatės", 161, -1, None),
    Among("sitės", 152, -1, None),
    Among("otės", 152, -1, None),
    Among("ėtės", 152, -1, None),
    Among("tumėtės", 165, -1, None),
    Among("ūs", 112, -1, None),
    Among("įs", 112, -1, None),
    Among("tųs", 112, -1, None),
    Among("at", -1, -1, None),
    Among("iat", 170, -1, None),
    Among("it", -1, -1, None),
    Among("sit", 172, -1, None),
    Among("ot", -1, -1, None),
    Among("ėt", -1, -1, None),
    Among("tumėt", 175, -1, None),
    Among("u", -1, -1, None),
    Among("au", 177, -1, None),
    Among("iau", 178, -1, None),
    Among("čiau", 179, -1, None),
    Among("iu", 177, -1, None),
    Among("eniu", 181, -1, None),
    Among("siu", 181, -1, None),
    Among("y", -1, -1, None),
    Among("ą", -1, -1, None),
    Among("ią", 185, -1, None),
    Among("ė", -1, -1, None),
    Among("ę", -1, -1, None),
    Among("į", -1, -1, None),
    Among("enį", 189, -1, None),
    Among("ų", -1, -1, None),
    Among("ių", 191, -1, None),
];

static A_1: &'static [Among<Context>; 62] = &[
    Among("ing", -1, -1, None),
    Among("aj", -1, -1, None),
    Among("iaj", 1, -1, None),
    Among("iej", -1, -1, None),
    Among("oj", -1, -1, None),
    Among("ioj", 4, -1, None),
    Among("uoj", 4, -1, None),
    Among("iuoj", 6, -1, None),
    Among("auj", -1, -1, None),
    Among("ąj", -1, -1, None),
    Among("iąj", 9, -1, None),
    Among("ėj", -1, -1, None),
    Among("ųj", -1, -1, None),
    Among("iųj", 12, -1, None),
    Among("ok", -1, -1, None),
    Among("iok", 14, -1, None),
    Among("iuk", -1, -1, None),
    Among("uliuk", 16, -1, None),
    Among("učiuk", 16, -1, None),
    Among("išk", -1, -1, None),
    Among("iul", -1, -1, None),
    Among("yl", -1, -1, None),
    Among("ėl", -1, -1, None),
    Among("am", -1, -1, None),
    Among("dam", 23, -1, None),
    Among("jam", 23, -1, None),
    Among("zgan", -1, -1, None),
    Among("ain", -1, -1, None),
    Among("esn", -1, -1, None),
    Among("op", -1, -1, None),
    Among("iop", 29, -1, None),
    Among("ias", -1, -1, None),
    Among("ies", -1, -1, None),
    Among("ais", -1, -1, None),
    Among("iais", 33, -1, None),
    Among("os", -1, -1, None),
    Among("ios", 35, -1, None),
    Among("uos", 35, -1, None),
    Among("iuos", 37, -1, None),
    Among("aus", -1, -1, None),
    Among("iaus", 39, -1, None),
    Among("ąs", -1, -1, None),
    Among("iąs", 41, -1, None),
    Among("ęs", -1, -1, None),
    Among("utėait", -1, -1, None),
    Among("ant", -1, -1, None),
    Among("iant", 45, -1, None),
    Among("siant", 46, -1, None),
    Among("int", -1, -1, None),
    Among("ot", -1, -1, None),
    Among("uot", 49, -1, None),
    Among("iuot", 50, -1, None),
    Among("yt", -1, -1, None),
    Among("ėt", -1, -1, None),
    Among("ykšt", -1, -1, None),
    Among("iau", -1, -1, None),
    Among("dav", -1, -1, None),
    Among("sv", -1, -1, None),
    Among("šv", -1, -1, None),
    Among("ykšč", -1, -1, None),
    Among("ę", -1, -1, None),
    Among("ėję", 60, -1, None),
];

static A_2: &'static [Among<Context>; 11] = &[
    Among("ojime", -1, 7, None),
    Among("ėjime", -1, 3, None),
    Among("avime", -1, 6, None),
    Among("okate", -1, 8, None),
    Among("aite", -1, 1, None),
    Among("uote", -1, 2, None),
    Among("asius", -1, 5, None),
    Among("okatės", -1, 8, None),
    Among("aitės", -1, 1, None),
    Among("uotės", -1, 2, None),
    Among("esiu", -1, 4, None),
];

static A_3: &'static [Among<Context>; 2] = &[
    Among("č", -1, 1, None),
    Among("dž", -1, 2, None),
];

static G_v: &'static [u8; 35] = &[17, 65, 16, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 16, 0, 64, 1, 0, 64, 0, 0, 0, 0, 0, 0, 0, 4, 4];

fn r_step1(env: &mut SnowballEnv, context: &mut Context) -> bool {
    if env.cursor < context.i_p1 {
        return false;
    }
    let v_1 = env.limit_backward;
    env.limit_backward = context.i_p1;
    env.ket = env.cursor;
    if env.find_among_b(A_0, context) == 0 {
        env.limit_backward = v_1;
        return false;
    }
    env.bra = env.cursor;
    env.limit_backward = v_1;
    env.slice_del();
    return true
}

fn r_step2(env: &mut SnowballEnv, context: &mut Context) -> bool {
    'replab0: loop{
        let v_1 = env.limit - env.cursor;
        'lab1: for _ in 0..1 {
            if env.cursor < context.i_p1 {
                break 'lab1;
            }
            let v_2 = env.limit_backward;
            env.limit_backward = context.i_p1;
            env.ket = env.cursor;
            if env.find_among_b(A_1, context) == 0 {
                env.limit_backward = v_2;
                break 'lab1;
            }
            env.bra = env.cursor;
            env.limit_backward = v_2;
            env.slice_del();
            continue 'replab0;
        }
        env.cursor = env.limit - v_1;
        break 'replab0;
    }
    return true
}

fn r_fix_conflicts(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    env.ket = env.cursor;
    if (env.cursor - 3 <= env.limit_backward || env.current.as_bytes()[(env.cursor - 1) as usize] as u8 >> 5 != 3 as u8 || ((2621472 as i32 >> (env.current.as_bytes()[(env.cursor - 1) as usize] as u8 & 0x1f)) & 1) == 0) {
        return false;
    }

    among_var = env.find_among_b(A_2, context);
    if among_var == 0 {
        return false;
    }
    env.bra = env.cursor;
    match among_var {
        1 => {
            env.slice_from("aitė");
        }
        2 => {
            env.slice_from("uotė");
        }
        3 => {
            env.slice_from("ėjimas");
        }
        4 => {
            env.slice_from("esys");
        }
        5 => {
            env.slice_from("asys");
        }
        6 => {
            env.slice_from("avimas");
        }
        7 => {
            env.slice_from("ojimas");
        }
        8 => {
            env.slice_from("okatė");
        }
        _ => ()
    }
    return true
}

fn r_fix_chdz(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    env.ket = env.cursor;
    if (env.cursor - 1 <= env.limit_backward || (env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 141 as u8 && env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 190 as u8)) {
        return false;
    }

    among_var = env.find_among_b(A_3, context);
    if among_var == 0 {
        return false;
    }
    env.bra = env.cursor;
    match among_var {
        1 => {
            env.slice_from("t");
        }
        2 => {
            env.slice_from("d");
        }
        _ => ()
    }
    return true
}

fn r_fix_gd(env: &mut SnowballEnv, context: &mut Context) -> bool {
    env.ket = env.cursor;
    if !env.eq_s_b(&"gd") {
        return false;
    }
    env.bra = env.cursor;
    env.slice_from("g");
    return true
}

pub fn stem(env: &mut SnowballEnv) -> bool {
    let mut context = &mut Context {
        i_p1: 0,
    };
    context.i_p1 = env.limit;
    let v_1 = env.cursor;
    'lab0: loop {
        let v_2 = env.cursor;
        'lab1: loop {
            if !env.eq_s(&"a") {
                env.cursor = v_2;
                break 'lab1;
            }
            if (env.current.chars().count() as i32) <= 6{
                env.cursor = v_2;
                break 'lab1;
            }
            break 'lab1;
        }
        if !env.go_out_grouping(G_v, 97, 371) {
            break 'lab0;
        }
        env.next_char();
        if !env.go_in_grouping(G_v, 97, 371) {
            break 'lab0;
        }
        env.next_char();
        context.i_p1 = env.cursor;
        break 'lab0;
    }
    env.cursor = v_1;
    env.limit_backward = env.cursor;
    env.cursor = env.limit;
    let v_3 = env.limit - env.cursor;
    r_fix_conflicts(env, context);
    env.cursor = env.limit - v_3;
    let v_4 = env.limit - env.cursor;
    r_step1(env, context);
    env.cursor = env.limit - v_4;
    let v_5 = env.limit - env.cursor;
    r_fix_chdz(env, context);
    env.cursor = env.limit - v_5;
    let v_6 = env.limit - env.cursor;
    r_step2(env, context);
    env.cursor = env.limit - v_6;
    let v_7 = env.limit - env.cursor;
    r_fix_chdz(env, context);
    env.cursor = env.limit - v_7;
    let v_8 = env.limit - env.cursor;
    r_fix_gd(env, context);
    env.cursor = env.limit - v_8;
    env.cursor = env.limit_backward;
    return true
}
