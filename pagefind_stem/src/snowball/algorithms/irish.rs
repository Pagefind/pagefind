//! Generated from irish.sbl by Snowball 3.0.0 - https://snowballstem.org/

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
    i_pV: i32,
}

static A_0: &'static [Among<Context>; 24] = &[
    Among("b'", -1, 1, None),
    Among("bh", -1, 4, None),
    Among("bhf", 1, 2, None),
    Among("bp", -1, 8, None),
    Among("ch", -1, 5, None),
    Among("d'", -1, 1, None),
    Among("d'fh", 5, 2, None),
    Among("dh", -1, 6, None),
    Among("dt", -1, 9, None),
    Among("fh", -1, 2, None),
    Among("gc", -1, 5, None),
    Among("gh", -1, 7, None),
    Among("h-", -1, 1, None),
    Among("m'", -1, 1, None),
    Among("mb", -1, 4, None),
    Among("mh", -1, 10, None),
    Among("n-", -1, 1, None),
    Among("nd", -1, 6, None),
    Among("ng", -1, 7, None),
    Among("ph", -1, 8, None),
    Among("sh", -1, 3, None),
    Among("t-", -1, 1, None),
    Among("th", -1, 9, None),
    Among("ts", -1, 3, None),
];

static A_1: &'static [Among<Context>; 16] = &[
    Among("íochta", -1, 1, None),
    Among("aíochta", 0, 1, None),
    Among("ire", -1, 2, None),
    Among("aire", 2, 2, None),
    Among("abh", -1, 1, None),
    Among("eabh", 4, 1, None),
    Among("ibh", -1, 1, None),
    Among("aibh", 6, 1, None),
    Among("amh", -1, 1, None),
    Among("eamh", 8, 1, None),
    Among("imh", -1, 1, None),
    Among("aimh", 10, 1, None),
    Among("íocht", -1, 1, None),
    Among("aíocht", 12, 1, None),
    Among("irí", -1, 2, None),
    Among("airí", 14, 2, None),
];

static A_2: &'static [Among<Context>; 25] = &[
    Among("óideacha", -1, 6, None),
    Among("patacha", -1, 5, None),
    Among("achta", -1, 1, None),
    Among("arcachta", 2, 2, None),
    Among("eachta", 2, 1, None),
    Among("grafaíochta", -1, 4, None),
    Among("paite", -1, 5, None),
    Among("ach", -1, 1, None),
    Among("each", 7, 1, None),
    Among("óideach", 8, 6, None),
    Among("gineach", 8, 3, None),
    Among("patach", 7, 5, None),
    Among("grafaíoch", -1, 4, None),
    Among("pataigh", -1, 5, None),
    Among("óidigh", -1, 6, None),
    Among("achtúil", -1, 1, None),
    Among("eachtúil", 15, 1, None),
    Among("gineas", -1, 3, None),
    Among("ginis", -1, 3, None),
    Among("acht", -1, 1, None),
    Among("arcacht", 19, 2, None),
    Among("eacht", 19, 1, None),
    Among("grafaíocht", -1, 4, None),
    Among("arcachtaí", -1, 2, None),
    Among("grafaíochtaí", -1, 4, None),
];

static A_3: &'static [Among<Context>; 12] = &[
    Among("imid", -1, 1, None),
    Among("aimid", 0, 1, None),
    Among("ímid", -1, 1, None),
    Among("aímid", 2, 1, None),
    Among("adh", -1, 2, None),
    Among("eadh", 4, 2, None),
    Among("faidh", -1, 1, None),
    Among("fidh", -1, 1, None),
    Among("áil", -1, 2, None),
    Among("ain", -1, 2, None),
    Among("tear", -1, 2, None),
    Among("tar", -1, 2, None),
];

static G_v: &'static [u8; 20] = &[17, 65, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 17, 4, 2];

fn r_mark_regions(env: &mut SnowballEnv, context: &mut Context) -> bool {
    context.i_pV = env.limit;
    context.i_p1 = env.limit;
    context.i_p2 = env.limit;
    let v_1 = env.cursor;
    'lab0: loop {
        if !env.go_out_grouping(G_v, 97, 250) {
            break 'lab0;
        }
        env.next_char();
        context.i_pV = env.cursor;
        if !env.go_in_grouping(G_v, 97, 250) {
            break 'lab0;
        }
        env.next_char();
        context.i_p1 = env.cursor;
        if !env.go_out_grouping(G_v, 97, 250) {
            break 'lab0;
        }
        env.next_char();
        if !env.go_in_grouping(G_v, 97, 250) {
            break 'lab0;
        }
        env.next_char();
        context.i_p2 = env.cursor;
        break 'lab0;
    }
    env.cursor = v_1;
    return true
}

fn r_initial_morph(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    env.bra = env.cursor;
    among_var = env.find_among(A_0, context);
    if among_var == 0 {
        return false;
    }
    env.ket = env.cursor;
    match among_var {
        1 => {
            env.slice_del();
        }
        2 => {
            env.slice_from("f");
        }
        3 => {
            env.slice_from("s");
        }
        4 => {
            env.slice_from("b");
        }
        5 => {
            env.slice_from("c");
        }
        6 => {
            env.slice_from("d");
        }
        7 => {
            env.slice_from("g");
        }
        8 => {
            env.slice_from("p");
        }
        9 => {
            env.slice_from("t");
        }
        10 => {
            env.slice_from("m");
        }
        _ => ()
    }
    return true
}

fn r_RV(env: &mut SnowballEnv, context: &mut Context) -> bool {
    return context.i_pV <= env.cursor
}

fn r_R1(env: &mut SnowballEnv, context: &mut Context) -> bool {
    return context.i_p1 <= env.cursor
}

fn r_R2(env: &mut SnowballEnv, context: &mut Context) -> bool {
    return context.i_p2 <= env.cursor
}

fn r_noun_sfx(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    env.ket = env.cursor;
    among_var = env.find_among_b(A_1, context);
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

fn r_deriv(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    env.ket = env.cursor;
    among_var = env.find_among_b(A_2, context);
    if among_var == 0 {
        return false;
    }
    env.bra = env.cursor;
    match among_var {
        1 => {
            if !r_R2(env, context) {
                return false;
            }
            env.slice_del();
        }
        2 => {
            env.slice_from("arc");
        }
        3 => {
            env.slice_from("gin");
        }
        4 => {
            env.slice_from("graf");
        }
        5 => {
            env.slice_from("paite");
        }
        6 => {
            env.slice_from("óid");
        }
        _ => ()
    }
    return true
}

fn r_verb_sfx(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    env.ket = env.cursor;
    if (env.cursor - 2 <= env.limit_backward || env.current.as_bytes()[(env.cursor - 1) as usize] as u8 >> 5 != 3 as u8 || ((282896 as i32 >> (env.current.as_bytes()[(env.cursor - 1) as usize] as u8 & 0x1f)) & 1) == 0) {
        return false;
    }

    among_var = env.find_among_b(A_3, context);
    if among_var == 0 {
        return false;
    }
    env.bra = env.cursor;
    match among_var {
        1 => {
            if !r_RV(env, context) {
                return false;
            }
            env.slice_del();
        }
        2 => {
            if !r_R1(env, context) {
                return false;
            }
            env.slice_del();
        }
        _ => ()
    }
    return true
}

pub fn stem(env: &mut SnowballEnv) -> bool {
    let mut context = &mut Context {
        i_p2: 0,
        i_p1: 0,
        i_pV: 0,
    };
    let v_1 = env.cursor;
    r_initial_morph(env, context);
    env.cursor = v_1;
    r_mark_regions(env, context);
    env.limit_backward = env.cursor;
    env.cursor = env.limit;
    let v_2 = env.limit - env.cursor;
    r_noun_sfx(env, context);
    env.cursor = env.limit - v_2;
    let v_3 = env.limit - env.cursor;
    r_deriv(env, context);
    env.cursor = env.limit - v_3;
    let v_4 = env.limit - env.cursor;
    r_verb_sfx(env, context);
    env.cursor = env.limit - v_4;
    env.cursor = env.limit_backward;
    return true
}
