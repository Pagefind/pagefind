//! Generated from hungarian.sbl by Snowball 3.0.0 - https://snowballstem.org/

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

static A_0: &'static [Among<Context>; 2] = &[
    Among("á", -1, 1, None),
    Among("é", -1, 2, None),
];

static A_1: &'static [Among<Context>; 23] = &[
    Among("bb", -1, -1, None),
    Among("cc", -1, -1, None),
    Among("dd", -1, -1, None),
    Among("ff", -1, -1, None),
    Among("gg", -1, -1, None),
    Among("jj", -1, -1, None),
    Among("kk", -1, -1, None),
    Among("ll", -1, -1, None),
    Among("mm", -1, -1, None),
    Among("nn", -1, -1, None),
    Among("pp", -1, -1, None),
    Among("rr", -1, -1, None),
    Among("ccs", -1, -1, None),
    Among("ss", -1, -1, None),
    Among("zzs", -1, -1, None),
    Among("tt", -1, -1, None),
    Among("vv", -1, -1, None),
    Among("ggy", -1, -1, None),
    Among("lly", -1, -1, None),
    Among("nny", -1, -1, None),
    Among("tty", -1, -1, None),
    Among("ssz", -1, -1, None),
    Among("zz", -1, -1, None),
];

static A_2: &'static [Among<Context>; 2] = &[
    Among("al", -1, 1, None),
    Among("el", -1, 1, None),
];

static A_3: &'static [Among<Context>; 44] = &[
    Among("ba", -1, -1, None),
    Among("ra", -1, -1, None),
    Among("be", -1, -1, None),
    Among("re", -1, -1, None),
    Among("ig", -1, -1, None),
    Among("nak", -1, -1, None),
    Among("nek", -1, -1, None),
    Among("val", -1, -1, None),
    Among("vel", -1, -1, None),
    Among("ul", -1, -1, None),
    Among("ből", -1, -1, None),
    Among("ről", -1, -1, None),
    Among("től", -1, -1, None),
    Among("nál", -1, -1, None),
    Among("nél", -1, -1, None),
    Among("ból", -1, -1, None),
    Among("ról", -1, -1, None),
    Among("tól", -1, -1, None),
    Among("ül", -1, -1, None),
    Among("n", -1, -1, None),
    Among("an", 19, -1, None),
    Among("ban", 20, -1, None),
    Among("en", 19, -1, None),
    Among("ben", 22, -1, None),
    Among("képpen", 22, -1, None),
    Among("on", 19, -1, None),
    Among("ön", 19, -1, None),
    Among("képp", -1, -1, None),
    Among("kor", -1, -1, None),
    Among("t", -1, -1, None),
    Among("at", 29, -1, None),
    Among("et", 29, -1, None),
    Among("ként", 29, -1, None),
    Among("anként", 32, -1, None),
    Among("enként", 32, -1, None),
    Among("onként", 32, -1, None),
    Among("ot", 29, -1, None),
    Among("ért", 29, -1, None),
    Among("öt", 29, -1, None),
    Among("hez", -1, -1, None),
    Among("hoz", -1, -1, None),
    Among("höz", -1, -1, None),
    Among("vá", -1, -1, None),
    Among("vé", -1, -1, None),
];

static A_4: &'static [Among<Context>; 3] = &[
    Among("án", -1, 2, None),
    Among("én", -1, 1, None),
    Among("ánként", -1, 2, None),
];

static A_5: &'static [Among<Context>; 6] = &[
    Among("stul", -1, 1, None),
    Among("astul", 0, 1, None),
    Among("ástul", 0, 2, None),
    Among("stül", -1, 1, None),
    Among("estül", 3, 1, None),
    Among("éstül", 3, 3, None),
];

static A_6: &'static [Among<Context>; 2] = &[
    Among("á", -1, 1, None),
    Among("é", -1, 1, None),
];

static A_7: &'static [Among<Context>; 7] = &[
    Among("k", -1, 3, None),
    Among("ak", 0, 3, None),
    Among("ek", 0, 3, None),
    Among("ok", 0, 3, None),
    Among("ák", 0, 1, None),
    Among("ék", 0, 2, None),
    Among("ök", 0, 3, None),
];

static A_8: &'static [Among<Context>; 12] = &[
    Among("éi", -1, 1, None),
    Among("áéi", 0, 3, None),
    Among("ééi", 0, 2, None),
    Among("é", -1, 1, None),
    Among("ké", 3, 1, None),
    Among("aké", 4, 1, None),
    Among("eké", 4, 1, None),
    Among("oké", 4, 1, None),
    Among("áké", 4, 3, None),
    Among("éké", 4, 2, None),
    Among("öké", 4, 1, None),
    Among("éé", 3, 2, None),
];

static A_9: &'static [Among<Context>; 31] = &[
    Among("a", -1, 1, None),
    Among("ja", 0, 1, None),
    Among("d", -1, 1, None),
    Among("ad", 2, 1, None),
    Among("ed", 2, 1, None),
    Among("od", 2, 1, None),
    Among("ád", 2, 2, None),
    Among("éd", 2, 3, None),
    Among("öd", 2, 1, None),
    Among("e", -1, 1, None),
    Among("je", 9, 1, None),
    Among("nk", -1, 1, None),
    Among("unk", 11, 1, None),
    Among("ánk", 11, 2, None),
    Among("énk", 11, 3, None),
    Among("ünk", 11, 1, None),
    Among("uk", -1, 1, None),
    Among("juk", 16, 1, None),
    Among("ájuk", 17, 2, None),
    Among("ük", -1, 1, None),
    Among("jük", 19, 1, None),
    Among("éjük", 20, 3, None),
    Among("m", -1, 1, None),
    Among("am", 22, 1, None),
    Among("em", 22, 1, None),
    Among("om", 22, 1, None),
    Among("ám", 22, 2, None),
    Among("ém", 22, 3, None),
    Among("o", -1, 1, None),
    Among("á", -1, 2, None),
    Among("é", -1, 3, None),
];

static A_10: &'static [Among<Context>; 42] = &[
    Among("id", -1, 1, None),
    Among("aid", 0, 1, None),
    Among("jaid", 1, 1, None),
    Among("eid", 0, 1, None),
    Among("jeid", 3, 1, None),
    Among("áid", 0, 2, None),
    Among("éid", 0, 3, None),
    Among("i", -1, 1, None),
    Among("ai", 7, 1, None),
    Among("jai", 8, 1, None),
    Among("ei", 7, 1, None),
    Among("jei", 10, 1, None),
    Among("ái", 7, 2, None),
    Among("éi", 7, 3, None),
    Among("itek", -1, 1, None),
    Among("eitek", 14, 1, None),
    Among("jeitek", 15, 1, None),
    Among("éitek", 14, 3, None),
    Among("ik", -1, 1, None),
    Among("aik", 18, 1, None),
    Among("jaik", 19, 1, None),
    Among("eik", 18, 1, None),
    Among("jeik", 21, 1, None),
    Among("áik", 18, 2, None),
    Among("éik", 18, 3, None),
    Among("ink", -1, 1, None),
    Among("aink", 25, 1, None),
    Among("jaink", 26, 1, None),
    Among("eink", 25, 1, None),
    Among("jeink", 28, 1, None),
    Among("áink", 25, 2, None),
    Among("éink", 25, 3, None),
    Among("aitok", -1, 1, None),
    Among("jaitok", 32, 1, None),
    Among("áitok", -1, 2, None),
    Among("im", -1, 1, None),
    Among("aim", 35, 1, None),
    Among("jaim", 36, 1, None),
    Among("eim", 35, 1, None),
    Among("jeim", 38, 1, None),
    Among("áim", 35, 2, None),
    Among("éim", 35, 3, None),
];

static G_v: &'static [u8; 35] = &[17, 65, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 17, 36, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1];

fn r_mark_regions(env: &mut SnowballEnv, context: &mut Context) -> bool {
    context.i_p1 = env.limit;
    'lab0: loop {
        let v_1 = env.cursor;
        'lab1: loop {
            if !env.in_grouping(G_v, 97, 369) {
                break 'lab1;
            }
            let v_2 = env.cursor;
            'lab2: loop {
                if !env.go_in_grouping(G_v, 97, 369) {
                    break 'lab2;
                }
                env.next_char();
                context.i_p1 = env.cursor;
                break 'lab2;
            }
            env.cursor = v_2;
            break 'lab0;
        }
        env.cursor = v_1;
        if !env.go_out_grouping(G_v, 97, 369) {
            return false;
        }
        env.next_char();
        context.i_p1 = env.cursor;
        break 'lab0;
    }
    return true
}

fn r_R1(env: &mut SnowballEnv, context: &mut Context) -> bool {
    return context.i_p1 <= env.cursor
}

fn r_v_ending(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    env.ket = env.cursor;
    if (env.cursor - 1 <= env.limit_backward || (env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 161 as u8 && env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 169 as u8)) {
        return false;
    }

    among_var = env.find_among_b(A_0, context);
    if among_var == 0 {
        return false;
    }
    env.bra = env.cursor;
    if !r_R1(env, context) {
        return false;
    }
    match among_var {
        1 => {
            env.slice_from("a");
        }
        2 => {
            env.slice_from("e");
        }
        _ => ()
    }
    return true
}

fn r_double(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let v_1 = env.limit - env.cursor;
    if (env.cursor - 1 <= env.limit_backward || env.current.as_bytes()[(env.cursor - 1) as usize] as u8 >> 5 != 3 as u8 || ((106790108 as i32 >> (env.current.as_bytes()[(env.cursor - 1) as usize] as u8 & 0x1f)) & 1) == 0) {
        return false;
    }

    if env.find_among_b(A_1, context) == 0 {
        return false;
    }
    env.cursor = env.limit - v_1;
    return true
}

fn r_undouble(env: &mut SnowballEnv, context: &mut Context) -> bool {
    if env.cursor <= env.limit_backward {
        return false;
    }
    env.previous_char();
    env.ket = env.cursor;
    if env.cursor <= env.limit_backward {
        return false;
    }
    env.previous_char();
    env.bra = env.cursor;
    env.slice_del();
    return true
}

fn r_instrum(env: &mut SnowballEnv, context: &mut Context) -> bool {
    env.ket = env.cursor;
    if (env.cursor - 1 <= env.limit_backward || env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 108 as u8) {
        return false;
    }

    if env.find_among_b(A_2, context) == 0 {
        return false;
    }
    env.bra = env.cursor;
    if !r_R1(env, context) {
        return false;
    }
    if !r_double(env, context) {
        return false;
    }
    env.slice_del();
    return r_undouble(env, context);
}

fn r_case(env: &mut SnowballEnv, context: &mut Context) -> bool {
    env.ket = env.cursor;
    if env.find_among_b(A_3, context) == 0 {
        return false;
    }
    env.bra = env.cursor;
    if !r_R1(env, context) {
        return false;
    }
    env.slice_del();
    return r_v_ending(env, context);
}

fn r_case_special(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    env.ket = env.cursor;
    if (env.cursor - 2 <= env.limit_backward || (env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 110 as u8 && env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 116 as u8)) {
        return false;
    }

    among_var = env.find_among_b(A_4, context);
    if among_var == 0 {
        return false;
    }
    env.bra = env.cursor;
    if !r_R1(env, context) {
        return false;
    }
    match among_var {
        1 => {
            env.slice_from("e");
        }
        2 => {
            env.slice_from("a");
        }
        _ => ()
    }
    return true
}

fn r_case_other(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    env.ket = env.cursor;
    if (env.cursor - 3 <= env.limit_backward || env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 108 as u8) {
        return false;
    }

    among_var = env.find_among_b(A_5, context);
    if among_var == 0 {
        return false;
    }
    env.bra = env.cursor;
    if !r_R1(env, context) {
        return false;
    }
    match among_var {
        1 => {
            env.slice_del();
        }
        2 => {
            env.slice_from("a");
        }
        3 => {
            env.slice_from("e");
        }
        _ => ()
    }
    return true
}

fn r_factive(env: &mut SnowballEnv, context: &mut Context) -> bool {
    env.ket = env.cursor;
    if (env.cursor - 1 <= env.limit_backward || (env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 161 as u8 && env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 169 as u8)) {
        return false;
    }

    if env.find_among_b(A_6, context) == 0 {
        return false;
    }
    env.bra = env.cursor;
    if !r_R1(env, context) {
        return false;
    }
    if !r_double(env, context) {
        return false;
    }
    env.slice_del();
    return r_undouble(env, context);
}

fn r_plural(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    env.ket = env.cursor;
    if (env.cursor <= env.limit_backward || env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 107 as u8) {
        return false;
    }

    among_var = env.find_among_b(A_7, context);
    if among_var == 0 {
        return false;
    }
    env.bra = env.cursor;
    if !r_R1(env, context) {
        return false;
    }
    match among_var {
        1 => {
            env.slice_from("a");
        }
        2 => {
            env.slice_from("e");
        }
        3 => {
            env.slice_del();
        }
        _ => ()
    }
    return true
}

fn r_owned(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    env.ket = env.cursor;
    if (env.cursor - 1 <= env.limit_backward || (env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 105 as u8 && env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 169 as u8)) {
        return false;
    }

    among_var = env.find_among_b(A_8, context);
    if among_var == 0 {
        return false;
    }
    env.bra = env.cursor;
    if !r_R1(env, context) {
        return false;
    }
    match among_var {
        1 => {
            env.slice_del();
        }
        2 => {
            env.slice_from("e");
        }
        3 => {
            env.slice_from("a");
        }
        _ => ()
    }
    return true
}

fn r_sing_owner(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    env.ket = env.cursor;
    among_var = env.find_among_b(A_9, context);
    if among_var == 0 {
        return false;
    }
    env.bra = env.cursor;
    if !r_R1(env, context) {
        return false;
    }
    match among_var {
        1 => {
            env.slice_del();
        }
        2 => {
            env.slice_from("a");
        }
        3 => {
            env.slice_from("e");
        }
        _ => ()
    }
    return true
}

fn r_plur_owner(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    env.ket = env.cursor;
    if (env.cursor <= env.limit_backward || env.current.as_bytes()[(env.cursor - 1) as usize] as u8 >> 5 != 3 as u8 || ((10768 as i32 >> (env.current.as_bytes()[(env.cursor - 1) as usize] as u8 & 0x1f)) & 1) == 0) {
        return false;
    }

    among_var = env.find_among_b(A_10, context);
    if among_var == 0 {
        return false;
    }
    env.bra = env.cursor;
    if !r_R1(env, context) {
        return false;
    }
    match among_var {
        1 => {
            env.slice_del();
        }
        2 => {
            env.slice_from("a");
        }
        3 => {
            env.slice_from("e");
        }
        _ => ()
    }
    return true
}

pub fn stem(env: &mut SnowballEnv) -> bool {
    let mut context = &mut Context {
        i_p1: 0,
    };
    let v_1 = env.cursor;
    r_mark_regions(env, context);
    env.cursor = v_1;
    env.limit_backward = env.cursor;
    env.cursor = env.limit;
    let v_2 = env.limit - env.cursor;
    r_instrum(env, context);
    env.cursor = env.limit - v_2;
    let v_3 = env.limit - env.cursor;
    r_case(env, context);
    env.cursor = env.limit - v_3;
    let v_4 = env.limit - env.cursor;
    r_case_special(env, context);
    env.cursor = env.limit - v_4;
    let v_5 = env.limit - env.cursor;
    r_case_other(env, context);
    env.cursor = env.limit - v_5;
    let v_6 = env.limit - env.cursor;
    r_factive(env, context);
    env.cursor = env.limit - v_6;
    let v_7 = env.limit - env.cursor;
    r_owned(env, context);
    env.cursor = env.limit - v_7;
    let v_8 = env.limit - env.cursor;
    r_sing_owner(env, context);
    env.cursor = env.limit - v_8;
    let v_9 = env.limit - env.cursor;
    r_plur_owner(env, context);
    env.cursor = env.limit - v_9;
    let v_10 = env.limit - env.cursor;
    r_plural(env, context);
    env.cursor = env.limit - v_10;
    env.cursor = env.limit_backward;
    return true
}
