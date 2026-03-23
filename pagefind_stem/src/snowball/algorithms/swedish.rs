//! Generated from swedish.sbl by Snowball 3.0.0 - https://snowballstem.org/

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

static A_0: &'static [Among<Context>; 21] = &[
    Among("fab", -1, -1, None),
    Among("h", -1, -1, None),
    Among("pak", -1, -1, None),
    Among("rak", -1, -1, None),
    Among("stak", -1, -1, None),
    Among("kom", -1, -1, None),
    Among("iet", -1, -1, None),
    Among("cit", -1, -1, None),
    Among("dit", -1, -1, None),
    Among("alit", -1, -1, None),
    Among("ilit", -1, -1, None),
    Among("mit", -1, -1, None),
    Among("nit", -1, -1, None),
    Among("pit", -1, -1, None),
    Among("rit", -1, -1, None),
    Among("sit", -1, -1, None),
    Among("tit", -1, -1, None),
    Among("uit", -1, -1, None),
    Among("ivit", -1, -1, None),
    Among("kvit", -1, -1, None),
    Among("xit", -1, -1, None),
];

static A_1: &'static [Among<Context>; 38] = &[
    Among("a", -1, 1, None),
    Among("arna", 0, 1, None),
    Among("erna", 0, 1, None),
    Among("heterna", 2, 1, None),
    Among("orna", 0, 1, None),
    Among("ad", -1, 1, None),
    Among("e", -1, 1, None),
    Among("ade", 6, 1, None),
    Among("ande", 6, 1, None),
    Among("arne", 6, 1, None),
    Among("are", 6, 1, None),
    Among("aste", 6, 1, None),
    Among("en", -1, 1, None),
    Among("anden", 12, 1, None),
    Among("aren", 12, 1, None),
    Among("heten", 12, 1, None),
    Among("ern", -1, 1, None),
    Among("ar", -1, 1, None),
    Among("er", -1, 1, None),
    Among("heter", 18, 1, None),
    Among("or", -1, 1, None),
    Among("s", -1, 2, None),
    Among("as", 21, 1, None),
    Among("arnas", 22, 1, None),
    Among("ernas", 22, 1, None),
    Among("ornas", 22, 1, None),
    Among("es", 21, 1, None),
    Among("ades", 26, 1, None),
    Among("andes", 26, 1, None),
    Among("ens", 21, 1, None),
    Among("arens", 29, 1, None),
    Among("hetens", 29, 1, None),
    Among("erns", 21, 1, None),
    Among("at", -1, 1, None),
    Among("et", -1, 3, None),
    Among("andet", 34, 1, None),
    Among("het", 34, 1, None),
    Among("ast", -1, 1, None),
];

static A_2: &'static [Among<Context>; 7] = &[
    Among("dd", -1, -1, None),
    Among("gd", -1, -1, None),
    Among("nn", -1, -1, None),
    Among("dt", -1, -1, None),
    Among("gt", -1, -1, None),
    Among("kt", -1, -1, None),
    Among("tt", -1, -1, None),
];

static A_3: &'static [Among<Context>; 5] = &[
    Among("ig", -1, 1, None),
    Among("lig", 0, 1, None),
    Among("els", -1, 1, None),
    Among("fullt", -1, 3, None),
    Among("öst", -1, 2, None),
];

static G_v: &'static [u8; 19] = &[17, 65, 16, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24, 0, 32];

static G_s_ending: &'static [u8; 3] = &[119, 127, 149];

static G_ost_ending: &'static [u8; 2] = &[173, 58];

fn r_mark_regions(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut i_x : i32;
    context.i_p1 = env.limit;
    let v_1 = env.cursor;
    if !env.hop(3) {
        return false;
    }
    i_x = env.cursor;
    env.cursor = v_1;
    if !env.go_out_grouping(G_v, 97, 246) {
        return false;
    }
    env.next_char();
    if !env.go_in_grouping(G_v, 97, 246) {
        return false;
    }
    env.next_char();
    context.i_p1 = env.cursor;
    'lab0: loop {
        if context.i_p1 >= i_x{
            break 'lab0;
        }
        context.i_p1 = i_x;
        break 'lab0;
    }
    return true
}

fn r_et_condition(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let v_1 = env.limit - env.cursor;
    if !env.out_grouping_b(G_v, 97, 246) {
        return false;
    }
    if !env.in_grouping_b(G_v, 97, 246) {
        return false;
    }
    'lab0: loop {
        if env.cursor > env.limit_backward {
            break 'lab0;
        }
        return false;
    }
    env.cursor = env.limit - v_1;
    let v_2 = env.limit - env.cursor;
    'lab1: loop {
        if (env.cursor <= env.limit_backward || env.current.as_bytes()[(env.cursor - 1) as usize] as u8 >> 5 != 3 as u8 || ((1059076 as i32 >> (env.current.as_bytes()[(env.cursor - 1) as usize] as u8 & 0x1f)) & 1) == 0) {
            break 'lab1;
        }

        if env.find_among_b(A_0, context) == 0 {
            break 'lab1;
        }
        return false;
    }
    env.cursor = env.limit - v_2;
    return true
}

fn r_main_suffix(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    if env.cursor < context.i_p1 {
        return false;
    }
    let v_1 = env.limit_backward;
    env.limit_backward = context.i_p1;
    env.ket = env.cursor;
    if (env.cursor <= env.limit_backward || env.current.as_bytes()[(env.cursor - 1) as usize] as u8 >> 5 != 3 as u8 || ((1851442 as i32 >> (env.current.as_bytes()[(env.cursor - 1) as usize] as u8 & 0x1f)) & 1) == 0) {
        env.limit_backward = v_1;
        return false;
    }

    among_var = env.find_among_b(A_1, context);
    if among_var == 0 {
        env.limit_backward = v_1;
        return false;
    }
    env.bra = env.cursor;
    env.limit_backward = v_1;
    match among_var {
        1 => {
            env.slice_del();
        }
        2 => {
            'lab0: loop {
                let v_2 = env.limit - env.cursor;
                'lab1: loop {
                    if !env.eq_s_b(&"et") {
                        break 'lab1;
                    }
                    if !r_et_condition(env, context) {
                        break 'lab1;
                    }
                    env.bra = env.cursor;
                    break 'lab0;
                }
                env.cursor = env.limit - v_2;
                if !env.in_grouping_b(G_s_ending, 98, 121) {
                    return false;
                }
                break 'lab0;
            }
            env.slice_del();
        }
        3 => {
            if !r_et_condition(env, context) {
                return false;
            }
            env.slice_del();
        }
        _ => ()
    }
    return true
}

fn r_consonant_pair(env: &mut SnowballEnv, context: &mut Context) -> bool {
    if env.cursor < context.i_p1 {
        return false;
    }
    let v_1 = env.limit_backward;
    env.limit_backward = context.i_p1;
    let v_2 = env.limit - env.cursor;
    if (env.cursor - 1 <= env.limit_backward || env.current.as_bytes()[(env.cursor - 1) as usize] as u8 >> 5 != 3 as u8 || ((1064976 as i32 >> (env.current.as_bytes()[(env.cursor - 1) as usize] as u8 & 0x1f)) & 1) == 0) {
        env.limit_backward = v_1;
        return false;
    }

    if env.find_among_b(A_2, context) == 0 {
        env.limit_backward = v_1;
        return false;
    }
    env.cursor = env.limit - v_2;
    env.ket = env.cursor;
    if env.cursor <= env.limit_backward {
        env.limit_backward = v_1;
        return false;
    }
    env.previous_char();
    env.bra = env.cursor;
    env.slice_del();
    env.limit_backward = v_1;
    return true
}

fn r_other_suffix(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    if env.cursor < context.i_p1 {
        return false;
    }
    let v_1 = env.limit_backward;
    env.limit_backward = context.i_p1;
    env.ket = env.cursor;
    if (env.cursor - 1 <= env.limit_backward || env.current.as_bytes()[(env.cursor - 1) as usize] as u8 >> 5 != 3 as u8 || ((1572992 as i32 >> (env.current.as_bytes()[(env.cursor - 1) as usize] as u8 & 0x1f)) & 1) == 0) {
        env.limit_backward = v_1;
        return false;
    }

    among_var = env.find_among_b(A_3, context);
    if among_var == 0 {
        env.limit_backward = v_1;
        return false;
    }
    env.bra = env.cursor;
    env.limit_backward = v_1;
    match among_var {
        1 => {
            env.slice_del();
        }
        2 => {
            if !env.in_grouping_b(G_ost_ending, 105, 118) {
                return false;
            }
            env.slice_from("ös");
        }
        3 => {
            env.slice_from("full");
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
    r_main_suffix(env, context);
    env.cursor = env.limit - v_2;
    let v_3 = env.limit - env.cursor;
    r_consonant_pair(env, context);
    env.cursor = env.limit - v_3;
    let v_4 = env.limit - env.cursor;
    r_other_suffix(env, context);
    env.cursor = env.limit - v_4;
    env.cursor = env.limit_backward;
    return true
}
