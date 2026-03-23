//! Generated from dutch_porter.sbl by Snowball 3.0.0 - https://snowballstem.org/

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
    b_e_found: bool,
}

static A_0: &'static [Among<Context>; 11] = &[
    Among("", -1, 6, None),
    Among("á", 0, 1, None),
    Among("ä", 0, 1, None),
    Among("é", 0, 2, None),
    Among("ë", 0, 2, None),
    Among("í", 0, 3, None),
    Among("ï", 0, 3, None),
    Among("ó", 0, 4, None),
    Among("ö", 0, 4, None),
    Among("ú", 0, 5, None),
    Among("ü", 0, 5, None),
];

static A_1: &'static [Among<Context>; 3] = &[
    Among("", -1, 3, None),
    Among("I", 0, 2, None),
    Among("Y", 0, 1, None),
];

static A_2: &'static [Among<Context>; 3] = &[
    Among("dd", -1, -1, None),
    Among("kk", -1, -1, None),
    Among("tt", -1, -1, None),
];

static A_3: &'static [Among<Context>; 5] = &[
    Among("ene", -1, 2, None),
    Among("se", -1, 3, None),
    Among("en", -1, 2, None),
    Among("heden", 2, 1, None),
    Among("s", -1, 3, None),
];

static A_4: &'static [Among<Context>; 6] = &[
    Among("end", -1, 1, None),
    Among("ig", -1, 2, None),
    Among("ing", -1, 1, None),
    Among("lijk", -1, 3, None),
    Among("baar", -1, 4, None),
    Among("bar", -1, 5, None),
];

static A_5: &'static [Among<Context>; 4] = &[
    Among("aa", -1, -1, None),
    Among("ee", -1, -1, None),
    Among("oo", -1, -1, None),
    Among("uu", -1, -1, None),
];

static G_v: &'static [u8; 17] = &[17, 65, 16, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128];

static G_v_I: &'static [u8; 20] = &[1, 0, 0, 17, 65, 16, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128];

static G_v_j: &'static [u8; 17] = &[17, 67, 16, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128];

fn r_prelude(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    let v_1 = env.cursor;
    'replab0: loop{
        let v_2 = env.cursor;
        'lab1: for _ in 0..1 {
            env.bra = env.cursor;
            if (env.cursor + 1 >= env.limit || env.current.as_bytes()[(env.cursor + 1) as usize] as u8 >> 5 != 5 as u8 || ((340306450 as i32 >> (env.current.as_bytes()[(env.cursor + 1) as usize] as u8 & 0x1f)) & 1) == 0) {among_var = 6;}
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
                    if env.cursor >= env.limit {
                        break 'lab1;
                    }
                    env.next_char();
                }
                _ => ()
            }
            continue 'replab0;
        }
        env.cursor = v_2;
        break 'replab0;
    }
    env.cursor = v_1;
    let v_3 = env.cursor;
    'lab2: loop {
        env.bra = env.cursor;
        if !env.eq_s(&"y") {
            env.cursor = v_3;
            break 'lab2;
        }
        env.ket = env.cursor;
        env.slice_from("Y");
        break 'lab2;
    }
    'replab3: loop{
        let v_4 = env.cursor;
        'lab4: for _ in 0..1 {
            if !env.go_out_grouping(G_v, 97, 232) {
                break 'lab4;
            }
            env.next_char();
            let v_5 = env.cursor;
            'lab5: loop {
                env.bra = env.cursor;
                'lab6: loop {
                    let v_6 = env.cursor;
                    'lab7: loop {
                        if !env.eq_s(&"i") {
                            break 'lab7;
                        }
                        env.ket = env.cursor;
                        let v_7 = env.cursor;
                        'lab8: loop {
                            if !env.in_grouping(G_v, 97, 232) {
                                break 'lab8;
                            }
                            env.slice_from("I");
                            break 'lab8;
                        }
                        env.cursor = v_7;
                        break 'lab6;
                    }
                    env.cursor = v_6;
                    if !env.eq_s(&"y") {
                        env.cursor = v_5;
                        break 'lab5;
                    }
                    env.ket = env.cursor;
                    env.slice_from("Y");
                    break 'lab6;
                }
                break 'lab5;
            }
            continue 'replab3;
        }
        env.cursor = v_4;
        break 'replab3;
    }
    return true
}

fn r_mark_regions(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut i_x : i32;
    context.i_p1 = env.limit;
    context.i_p2 = env.limit;
    let v_1 = env.cursor;
    if !env.hop(3) {
        return false;
    }
    i_x = env.cursor;
    env.cursor = v_1;
    if !env.go_out_grouping(G_v, 97, 232) {
        return false;
    }
    env.next_char();
    if !env.go_in_grouping(G_v, 97, 232) {
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
    if !env.go_out_grouping(G_v, 97, 232) {
        return false;
    }
    env.next_char();
    if !env.go_in_grouping(G_v, 97, 232) {
        return false;
    }
    env.next_char();
    context.i_p2 = env.cursor;
    return true
}

fn r_postlude(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    'replab0: loop{
        let v_1 = env.cursor;
        'lab1: for _ in 0..1 {
            env.bra = env.cursor;
            if (env.cursor >= env.limit || (env.current.as_bytes()[(env.cursor + 0) as usize] as u8 != 73 as u8 && env.current.as_bytes()[(env.cursor + 0) as usize] as u8 != 89 as u8)) {among_var = 3;}
            else {
                among_var = env.find_among(A_1, context);
            }
            env.ket = env.cursor;
            match among_var {
                1 => {
                    env.slice_from("y");
                }
                2 => {
                    env.slice_from("i");
                }
                3 => {
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

fn r_undouble(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let v_1 = env.limit - env.cursor;
    if (env.cursor - 1 <= env.limit_backward || env.current.as_bytes()[(env.cursor - 1) as usize] as u8 >> 5 != 3 as u8 || ((1050640 as i32 >> (env.current.as_bytes()[(env.cursor - 1) as usize] as u8 & 0x1f)) & 1) == 0) {
        return false;
    }

    if env.find_among_b(A_2, context) == 0 {
        return false;
    }
    env.cursor = env.limit - v_1;
    env.ket = env.cursor;
    if env.cursor <= env.limit_backward {
        return false;
    }
    env.previous_char();
    env.bra = env.cursor;
    env.slice_del();
    return true
}

fn r_e_ending(env: &mut SnowballEnv, context: &mut Context) -> bool {
    context.b_e_found = false;
    env.ket = env.cursor;
    if !env.eq_s_b(&"e") {
        return false;
    }
    env.bra = env.cursor;
    if !r_R1(env, context) {
        return false;
    }
    let v_1 = env.limit - env.cursor;
    if !env.out_grouping_b(G_v, 97, 232) {
        return false;
    }
    env.cursor = env.limit - v_1;
    env.slice_del();
    context.b_e_found = true;
    return r_undouble(env, context);
}

fn r_en_ending(env: &mut SnowballEnv, context: &mut Context) -> bool {
    if !r_R1(env, context) {
        return false;
    }
    let v_1 = env.limit - env.cursor;
    if !env.out_grouping_b(G_v, 97, 232) {
        return false;
    }
    env.cursor = env.limit - v_1;
    let v_2 = env.limit - env.cursor;
    'lab0: loop {
        if !env.eq_s_b(&"gem") {
            break 'lab0;
        }
        return false;
    }
    env.cursor = env.limit - v_2;
    env.slice_del();
    return r_undouble(env, context);
}

fn r_standard_suffix(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    let v_1 = env.limit - env.cursor;
    'lab0: loop {
        env.ket = env.cursor;
        if (env.cursor <= env.limit_backward || env.current.as_bytes()[(env.cursor - 1) as usize] as u8 >> 5 != 3 as u8 || ((540704 as i32 >> (env.current.as_bytes()[(env.cursor - 1) as usize] as u8 & 0x1f)) & 1) == 0) {
            break 'lab0;
        }

        among_var = env.find_among_b(A_3, context);
        if among_var == 0 {
            break 'lab0;
        }
        env.bra = env.cursor;
        match among_var {
            1 => {
                if !r_R1(env, context) {
                    break 'lab0;
                }
                env.slice_from("heid");
            }
            2 => {
                if !r_en_ending(env, context) {
                    break 'lab0;
                }
            }
            3 => {
                if !r_R1(env, context) {
                    break 'lab0;
                }
                if !env.out_grouping_b(G_v_j, 97, 232) {
                    break 'lab0;
                }
                env.slice_del();
            }
            _ => ()
        }
        break 'lab0;
    }
    env.cursor = env.limit - v_1;
    let v_2 = env.limit - env.cursor;
    r_e_ending(env, context);
    env.cursor = env.limit - v_2;
    let v_3 = env.limit - env.cursor;
    'lab1: loop {
        env.ket = env.cursor;
        if !env.eq_s_b(&"heid") {
            break 'lab1;
        }
        env.bra = env.cursor;
        if !r_R2(env, context) {
            break 'lab1;
        }
        let v_4 = env.limit - env.cursor;
        'lab2: loop {
            if !env.eq_s_b(&"c") {
                break 'lab2;
            }
            break 'lab1;
        }
        env.cursor = env.limit - v_4;
        env.slice_del();
        env.ket = env.cursor;
        if !env.eq_s_b(&"en") {
            break 'lab1;
        }
        env.bra = env.cursor;
        if !r_en_ending(env, context) {
            break 'lab1;
        }
        break 'lab1;
    }
    env.cursor = env.limit - v_3;
    let v_5 = env.limit - env.cursor;
    'lab3: loop {
        env.ket = env.cursor;
        if (env.cursor - 1 <= env.limit_backward || env.current.as_bytes()[(env.cursor - 1) as usize] as u8 >> 5 != 3 as u8 || ((264336 as i32 >> (env.current.as_bytes()[(env.cursor - 1) as usize] as u8 & 0x1f)) & 1) == 0) {
            break 'lab3;
        }

        among_var = env.find_among_b(A_4, context);
        if among_var == 0 {
            break 'lab3;
        }
        env.bra = env.cursor;
        match among_var {
            1 => {
                if !r_R2(env, context) {
                    break 'lab3;
                }
                env.slice_del();
                'lab4: loop {
                    let v_6 = env.limit - env.cursor;
                    'lab5: loop {
                        env.ket = env.cursor;
                        if !env.eq_s_b(&"ig") {
                            break 'lab5;
                        }
                        env.bra = env.cursor;
                        if !r_R2(env, context) {
                            break 'lab5;
                        }
                        let v_7 = env.limit - env.cursor;
                        'lab6: loop {
                            if !env.eq_s_b(&"e") {
                                break 'lab6;
                            }
                            break 'lab5;
                        }
                        env.cursor = env.limit - v_7;
                        env.slice_del();
                        break 'lab4;
                    }
                    env.cursor = env.limit - v_6;
                    if !r_undouble(env, context) {
                        break 'lab3;
                    }
                    break 'lab4;
                }
            }
            2 => {
                if !r_R2(env, context) {
                    break 'lab3;
                }
                let v_8 = env.limit - env.cursor;
                'lab7: loop {
                    if !env.eq_s_b(&"e") {
                        break 'lab7;
                    }
                    break 'lab3;
                }
                env.cursor = env.limit - v_8;
                env.slice_del();
            }
            3 => {
                if !r_R2(env, context) {
                    break 'lab3;
                }
                env.slice_del();
                if !r_e_ending(env, context) {
                    break 'lab3;
                }
            }
            4 => {
                if !r_R2(env, context) {
                    break 'lab3;
                }
                env.slice_del();
            }
            5 => {
                if !r_R2(env, context) {
                    break 'lab3;
                }
                if !context.b_e_found {
                    break 'lab3;
                }
                env.slice_del();
            }
            _ => ()
        }
        break 'lab3;
    }
    env.cursor = env.limit - v_5;
    let v_9 = env.limit - env.cursor;
    'lab8: loop {
        if !env.out_grouping_b(G_v_I, 73, 232) {
            break 'lab8;
        }
        let v_10 = env.limit - env.cursor;
        if (env.cursor - 1 <= env.limit_backward || env.current.as_bytes()[(env.cursor - 1) as usize] as u8 >> 5 != 3 as u8 || ((2129954 as i32 >> (env.current.as_bytes()[(env.cursor - 1) as usize] as u8 & 0x1f)) & 1) == 0) {
            break 'lab8;
        }

        if env.find_among_b(A_5, context) == 0 {
            break 'lab8;
        }
        if !env.out_grouping_b(G_v, 97, 232) {
            break 'lab8;
        }
        env.cursor = env.limit - v_10;
        env.ket = env.cursor;
        if env.cursor <= env.limit_backward {
            break 'lab8;
        }
        env.previous_char();
        env.bra = env.cursor;
        env.slice_del();
        break 'lab8;
    }
    env.cursor = env.limit - v_9;
    return true
}

pub fn stem(env: &mut SnowballEnv) -> bool {
    let mut context = &mut Context {
        i_p2: 0,
        i_p1: 0,
        b_e_found: false,
    };
    let v_1 = env.cursor;
    r_prelude(env, context);
    env.cursor = v_1;
    let v_2 = env.cursor;
    r_mark_regions(env, context);
    env.cursor = v_2;
    env.limit_backward = env.cursor;
    env.cursor = env.limit;
    r_standard_suffix(env, context);
    env.cursor = env.limit_backward;
    let v_3 = env.cursor;
    r_postlude(env, context);
    env.cursor = v_3;
    return true
}
