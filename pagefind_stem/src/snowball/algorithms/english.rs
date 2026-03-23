//! Generated from english.sbl by Snowball 3.0.0 - https://snowballstem.org/

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_mut)]
#![allow(unused_parens)]
#![allow(unused_variables)]
use snowball::SnowballEnv;
use snowball::Among;

#[derive(Clone)]
struct Context {
    b_Y_found: bool,
    i_p2: i32,
    i_p1: i32,
}

static A_0: &'static [Among<Context>; 9] = &[
    Among("arsen", -1, -1, None),
    Among("commun", -1, -1, None),
    Among("emerg", -1, -1, None),
    Among("gener", -1, -1, None),
    Among("inter", -1, -1, None),
    Among("later", -1, -1, None),
    Among("organ", -1, -1, None),
    Among("past", -1, -1, None),
    Among("univers", -1, -1, None),
];

static A_1: &'static [Among<Context>; 3] = &[
    Among("'", -1, 1, None),
    Among("'s'", 0, 1, None),
    Among("'s", -1, 1, None),
];

static A_2: &'static [Among<Context>; 6] = &[
    Among("ied", -1, 2, None),
    Among("s", -1, 3, None),
    Among("ies", 1, 2, None),
    Among("sses", 1, 1, None),
    Among("ss", 1, -1, None),
    Among("us", 1, -1, None),
];

static A_3: &'static [Among<Context>; 3] = &[
    Among("succ", -1, 1, None),
    Among("proc", -1, 1, None),
    Among("exc", -1, 1, None),
];

static A_4: &'static [Among<Context>; 7] = &[
    Among("even", -1, 2, None),
    Among("cann", -1, 2, None),
    Among("inn", -1, 2, None),
    Among("earr", -1, 2, None),
    Among("herr", -1, 2, None),
    Among("out", -1, 2, None),
    Among("y", -1, 1, None),
];

static A_5: &'static [Among<Context>; 7] = &[
    Among("", -1, -1, None),
    Among("ed", 0, 2, None),
    Among("eed", 1, 1, None),
    Among("ing", 0, 3, None),
    Among("edly", 0, 2, None),
    Among("eedly", 4, 1, None),
    Among("ingly", 0, 2, None),
];

static A_6: &'static [Among<Context>; 13] = &[
    Among("", -1, 3, None),
    Among("bb", 0, 2, None),
    Among("dd", 0, 2, None),
    Among("ff", 0, 2, None),
    Among("gg", 0, 2, None),
    Among("bl", 0, 1, None),
    Among("mm", 0, 2, None),
    Among("nn", 0, 2, None),
    Among("pp", 0, 2, None),
    Among("rr", 0, 2, None),
    Among("at", 0, 1, None),
    Among("tt", 0, 2, None),
    Among("iz", 0, 1, None),
];

static A_7: &'static [Among<Context>; 25] = &[
    Among("anci", -1, 3, None),
    Among("enci", -1, 2, None),
    Among("ogi", -1, 14, None),
    Among("li", -1, 16, None),
    Among("bli", 3, 12, None),
    Among("abli", 4, 4, None),
    Among("alli", 3, 8, None),
    Among("fulli", 3, 9, None),
    Among("lessli", 3, 15, None),
    Among("ousli", 3, 10, None),
    Among("entli", 3, 5, None),
    Among("aliti", -1, 8, None),
    Among("biliti", -1, 12, None),
    Among("iviti", -1, 11, None),
    Among("tional", -1, 1, None),
    Among("ational", 14, 7, None),
    Among("alism", -1, 8, None),
    Among("ation", -1, 7, None),
    Among("ization", 17, 6, None),
    Among("izer", -1, 6, None),
    Among("ator", -1, 7, None),
    Among("iveness", -1, 11, None),
    Among("fulness", -1, 9, None),
    Among("ousness", -1, 10, None),
    Among("ogist", -1, 13, None),
];

static A_8: &'static [Among<Context>; 9] = &[
    Among("icate", -1, 4, None),
    Among("ative", -1, 6, None),
    Among("alize", -1, 3, None),
    Among("iciti", -1, 4, None),
    Among("ical", -1, 4, None),
    Among("tional", -1, 1, None),
    Among("ational", 5, 2, None),
    Among("ful", -1, 5, None),
    Among("ness", -1, 5, None),
];

static A_9: &'static [Among<Context>; 18] = &[
    Among("ic", -1, 1, None),
    Among("ance", -1, 1, None),
    Among("ence", -1, 1, None),
    Among("able", -1, 1, None),
    Among("ible", -1, 1, None),
    Among("ate", -1, 1, None),
    Among("ive", -1, 1, None),
    Among("ize", -1, 1, None),
    Among("iti", -1, 1, None),
    Among("al", -1, 1, None),
    Among("ism", -1, 1, None),
    Among("ion", -1, 2, None),
    Among("er", -1, 1, None),
    Among("ous", -1, 1, None),
    Among("ant", -1, 1, None),
    Among("ent", -1, 1, None),
    Among("ment", 15, 1, None),
    Among("ement", 16, 1, None),
];

static A_10: &'static [Among<Context>; 2] = &[
    Among("e", -1, 1, None),
    Among("l", -1, 2, None),
];

static A_11: &'static [Among<Context>; 15] = &[
    Among("andes", -1, -1, None),
    Among("atlas", -1, -1, None),
    Among("bias", -1, -1, None),
    Among("cosmos", -1, -1, None),
    Among("early", -1, 6, None),
    Among("gently", -1, 4, None),
    Among("howe", -1, -1, None),
    Among("idly", -1, 3, None),
    Among("news", -1, -1, None),
    Among("only", -1, 7, None),
    Among("singly", -1, 8, None),
    Among("skies", -1, 2, None),
    Among("skis", -1, 1, None),
    Among("sky", -1, -1, None),
    Among("ugly", -1, 5, None),
];

static G_aeo: &'static [u8; 2] = &[17, 64];

static G_v: &'static [u8; 4] = &[17, 65, 16, 1];

static G_v_WXY: &'static [u8; 5] = &[1, 17, 65, 208, 1];

static G_valid_LI: &'static [u8; 3] = &[55, 141, 2];

fn r_prelude(env: &mut SnowballEnv, context: &mut Context) -> bool {
    context.b_Y_found = false;
    let v_1 = env.cursor;
    'lab0: loop {
        env.bra = env.cursor;
        if !env.eq_s(&"'") {
            break 'lab0;
        }
        env.ket = env.cursor;
        env.slice_del();
        break 'lab0;
    }
    env.cursor = v_1;
    let v_2 = env.cursor;
    'lab1: loop {
        env.bra = env.cursor;
        if !env.eq_s(&"y") {
            break 'lab1;
        }
        env.ket = env.cursor;
        env.slice_from("Y");
        context.b_Y_found = true;
        break 'lab1;
    }
    env.cursor = v_2;
    let v_3 = env.cursor;
    'lab2: loop {
        'replab3: loop{
            let v_4 = env.cursor;
            'lab4: for _ in 0..1 {
                'golab5: loop {
                    let v_5 = env.cursor;
                    'lab6: loop {
                        if !env.in_grouping(G_v, 97, 121) {
                            break 'lab6;
                        }
                        env.bra = env.cursor;
                        if !env.eq_s(&"y") {
                            break 'lab6;
                        }
                        env.ket = env.cursor;
                        env.cursor = v_5;
                        break 'golab5;
                    }
                    env.cursor = v_5;
                    if env.cursor >= env.limit {
                        break 'lab4;
                    }
                    env.next_char();
                }
                env.slice_from("Y");
                context.b_Y_found = true;
                continue 'replab3;
            }
            env.cursor = v_4;
            break 'replab3;
        }
        break 'lab2;
    }
    env.cursor = v_3;
    return true
}

fn r_mark_regions(env: &mut SnowballEnv, context: &mut Context) -> bool {
    context.i_p1 = env.limit;
    context.i_p2 = env.limit;
    let v_1 = env.cursor;
    'lab0: loop {
        'lab1: loop {
            let v_2 = env.cursor;
            'lab2: loop {
                if (env.cursor + 3 >= env.limit || env.current.as_bytes()[(env.cursor + 3) as usize] as u8 >> 5 != 3 as u8 || ((5513250 as i32 >> (env.current.as_bytes()[(env.cursor + 3) as usize] as u8 & 0x1f)) & 1) == 0) {
                    break 'lab2;
                }

                if env.find_among(A_0, context) == 0 {
                    break 'lab2;
                }
                break 'lab1;
            }
            env.cursor = v_2;
            if !env.go_out_grouping(G_v, 97, 121) {
                break 'lab0;
            }
            env.next_char();
            if !env.go_in_grouping(G_v, 97, 121) {
                break 'lab0;
            }
            env.next_char();
            break 'lab1;
        }
        context.i_p1 = env.cursor;
        if !env.go_out_grouping(G_v, 97, 121) {
            break 'lab0;
        }
        env.next_char();
        if !env.go_in_grouping(G_v, 97, 121) {
            break 'lab0;
        }
        env.next_char();
        context.i_p2 = env.cursor;
        break 'lab0;
    }
    env.cursor = v_1;
    return true
}

fn r_shortv(env: &mut SnowballEnv, context: &mut Context) -> bool {
    'lab0: loop {
        let v_1 = env.limit - env.cursor;
        'lab1: loop {
            if !env.out_grouping_b(G_v_WXY, 89, 121) {
                break 'lab1;
            }
            if !env.in_grouping_b(G_v, 97, 121) {
                break 'lab1;
            }
            if !env.out_grouping_b(G_v, 97, 121) {
                break 'lab1;
            }
            break 'lab0;
        }
        env.cursor = env.limit - v_1;
        'lab2: loop {
            if !env.out_grouping_b(G_v, 97, 121) {
                break 'lab2;
            }
            if !env.in_grouping_b(G_v, 97, 121) {
                break 'lab2;
            }
            if env.cursor > env.limit_backward {
                break 'lab2;
            }
            break 'lab0;
        }
        env.cursor = env.limit - v_1;
        if !env.eq_s_b(&"past") {
            return false;
        }
        break 'lab0;
    }
    return true
}

fn r_R1(env: &mut SnowballEnv, context: &mut Context) -> bool {
    return context.i_p1 <= env.cursor
}

fn r_R2(env: &mut SnowballEnv, context: &mut Context) -> bool {
    return context.i_p2 <= env.cursor
}

fn r_Step_1a(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    let v_1 = env.limit - env.cursor;
    'lab0: loop {
        env.ket = env.cursor;
        if (env.cursor <= env.limit_backward || (env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 39 as u8 && env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 115 as u8)) {
            env.cursor = env.limit - v_1;
            break 'lab0;
        }

        if env.find_among_b(A_1, context) == 0 {
            env.cursor = env.limit - v_1;
            break 'lab0;
        }
        env.bra = env.cursor;
        env.slice_del();
        break 'lab0;
    }
    env.ket = env.cursor;
    if (env.cursor <= env.limit_backward || (env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 100 as u8 && env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 115 as u8)) {
        return false;
    }

    among_var = env.find_among_b(A_2, context);
    if among_var == 0 {
        return false;
    }
    env.bra = env.cursor;
    match among_var {
        1 => {
            env.slice_from("ss");
        }
        2 => {
            'lab1: loop {
                let v_2 = env.limit - env.cursor;
                'lab2: loop {
                    if !env.hop_back(2) {
                        break 'lab2;
                    }
                    env.slice_from("i");
                    break 'lab1;
                }
                env.cursor = env.limit - v_2;
                env.slice_from("ie");
                break 'lab1;
            }
        }
        3 => {
            if env.cursor <= env.limit_backward {
                return false;
            }
            env.previous_char();
            if !env.go_out_grouping_b(G_v, 97, 121) {
                return false;
            }
            env.previous_char();
            env.slice_del();
        }
        _ => ()
    }
    return true
}

fn r_Step_1b(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    env.ket = env.cursor;
    if (env.cursor - 1 <= env.limit_backward || env.current.as_bytes()[(env.cursor - 1) as usize] as u8 >> 5 != 3 as u8 || ((33554576 as i32 >> (env.current.as_bytes()[(env.cursor - 1) as usize] as u8 & 0x1f)) & 1) == 0) {among_var = -1;}
    else {
        among_var = env.find_among_b(A_5, context);
    }
    env.bra = env.cursor;
    'lab0: loop {
        let v_1 = env.limit - env.cursor;
        'lab1: loop {
            match among_var {
                1 => {
                    let v_2 = env.limit - env.cursor;
                    'lab2: loop {
                        if !r_R1(env, context) {
                            break 'lab2;
                        }
                        'lab3: loop {
                            let v_3 = env.limit - env.cursor;
                            'lab4: loop {
                                if (env.cursor - 2 <= env.limit_backward || env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 99 as u8) {
                                    break 'lab4;
                                }

                                if env.find_among_b(A_3, context) == 0 {
                                    break 'lab4;
                                }
                                if env.cursor > env.limit_backward {
                                    break 'lab4;
                                }
                                break 'lab3;
                            }
                            env.cursor = env.limit - v_3;
                            env.slice_from("ee");
                            break 'lab3;
                        }
                        break 'lab2;
                    }
                    env.cursor = env.limit - v_2;
                }
                2 => {
                    break 'lab1;
                }
                3 => {
                    if (env.cursor <= env.limit_backward || env.current.as_bytes()[(env.cursor - 1) as usize] as u8 >> 5 != 3 as u8 || ((34881536 as i32 >> (env.current.as_bytes()[(env.cursor - 1) as usize] as u8 & 0x1f)) & 1) == 0) {
                        break 'lab1;
                    }

                    among_var = env.find_among_b(A_4, context);
                    if among_var == 0 {
                        break 'lab1;
                    }
                    match among_var {
                        1 => {
                            let v_4 = env.limit - env.cursor;
                            if !env.out_grouping_b(G_v, 97, 121) {
                                break 'lab1;
                            }
                            if env.cursor > env.limit_backward {
                                break 'lab1;
                            }
                            env.cursor = env.limit - v_4;
                            env.bra = env.cursor;
                            env.slice_from("ie");
                        }
                        2 => {
                            if env.cursor > env.limit_backward {
                                break 'lab1;
                            }
                        }
                        _ => ()
                    }
                }
                _ => ()
            }
            break 'lab0;
        }
        env.cursor = env.limit - v_1;
        let v_5 = env.limit - env.cursor;
        if !env.go_out_grouping_b(G_v, 97, 121) {
            return false;
        }
        env.previous_char();
        env.cursor = env.limit - v_5;
        env.slice_del();
        env.ket = env.cursor;
        env.bra = env.cursor;
        let v_6 = env.limit - env.cursor;
        if (env.cursor - 1 <= env.limit_backward || env.current.as_bytes()[(env.cursor - 1) as usize] as u8 >> 5 != 3 as u8 || ((68514004 as i32 >> (env.current.as_bytes()[(env.cursor - 1) as usize] as u8 & 0x1f)) & 1) == 0) {among_var = 3;}
        else {
            among_var = env.find_among_b(A_6, context);
        }
        match among_var {
            1 => {
                env.slice_from("e");
                return false;
            }
            2 => {
                let v_7 = env.limit - env.cursor;
                'lab5: loop {
                    if !env.in_grouping_b(G_aeo, 97, 111) {
                        break 'lab5;
                    }
                    if env.cursor > env.limit_backward {
                        break 'lab5;
                    }
                    return false;
                }
                env.cursor = env.limit - v_7;
            }
            3 => {
                if env.cursor != context.i_p1 {
                    return false;
                }
                let v_8 = env.limit - env.cursor;
                if !r_shortv(env, context) {
                    return false;
                }
                env.cursor = env.limit - v_8;
                env.slice_from("e");
                return false;
            }
            _ => ()
        }
        env.cursor = env.limit - v_6;
        env.ket = env.cursor;
        if env.cursor <= env.limit_backward {
            return false;
        }
        env.previous_char();
        env.bra = env.cursor;
        env.slice_del();
        break 'lab0;
    }
    return true
}

fn r_Step_1c(env: &mut SnowballEnv, context: &mut Context) -> bool {
    env.ket = env.cursor;
    'lab0: loop {
        let v_1 = env.limit - env.cursor;
        'lab1: loop {
            if !env.eq_s_b(&"y") {
                break 'lab1;
            }
            break 'lab0;
        }
        env.cursor = env.limit - v_1;
        if !env.eq_s_b(&"Y") {
            return false;
        }
        break 'lab0;
    }
    env.bra = env.cursor;
    if !env.out_grouping_b(G_v, 97, 121) {
        return false;
    }
    'lab2: loop {
        if env.cursor > env.limit_backward {
            break 'lab2;
        }
        return false;
    }
    env.slice_from("i");
    return true
}

fn r_Step_2(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    env.ket = env.cursor;
    if (env.cursor - 1 <= env.limit_backward || env.current.as_bytes()[(env.cursor - 1) as usize] as u8 >> 5 != 3 as u8 || ((1864192 as i32 >> (env.current.as_bytes()[(env.cursor - 1) as usize] as u8 & 0x1f)) & 1) == 0) {
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
            env.slice_from("tion");
        }
        2 => {
            env.slice_from("ence");
        }
        3 => {
            env.slice_from("ance");
        }
        4 => {
            env.slice_from("able");
        }
        5 => {
            env.slice_from("ent");
        }
        6 => {
            env.slice_from("ize");
        }
        7 => {
            env.slice_from("ate");
        }
        8 => {
            env.slice_from("al");
        }
        9 => {
            env.slice_from("ful");
        }
        10 => {
            env.slice_from("ous");
        }
        11 => {
            env.slice_from("ive");
        }
        12 => {
            env.slice_from("ble");
        }
        13 => {
            env.slice_from("og");
        }
        14 => {
            if !env.eq_s_b(&"l") {
                return false;
            }
            env.slice_from("og");
        }
        15 => {
            env.slice_from("less");
        }
        16 => {
            if !env.in_grouping_b(G_valid_LI, 99, 116) {
                return false;
            }
            env.slice_del();
        }
        _ => ()
    }
    return true
}

fn r_Step_3(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    env.ket = env.cursor;
    if (env.cursor - 2 <= env.limit_backward || env.current.as_bytes()[(env.cursor - 1) as usize] as u8 >> 5 != 3 as u8 || ((528928 as i32 >> (env.current.as_bytes()[(env.cursor - 1) as usize] as u8 & 0x1f)) & 1) == 0) {
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
            env.slice_from("tion");
        }
        2 => {
            env.slice_from("ate");
        }
        3 => {
            env.slice_from("al");
        }
        4 => {
            env.slice_from("ic");
        }
        5 => {
            env.slice_del();
        }
        6 => {
            if !r_R2(env, context) {
                return false;
            }
            env.slice_del();
        }
        _ => ()
    }
    return true
}

fn r_Step_4(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    env.ket = env.cursor;
    if (env.cursor - 1 <= env.limit_backward || env.current.as_bytes()[(env.cursor - 1) as usize] as u8 >> 5 != 3 as u8 || ((1864232 as i32 >> (env.current.as_bytes()[(env.cursor - 1) as usize] as u8 & 0x1f)) & 1) == 0) {
        return false;
    }

    among_var = env.find_among_b(A_9, context);
    if among_var == 0 {
        return false;
    }
    env.bra = env.cursor;
    if !r_R2(env, context) {
        return false;
    }
    match among_var {
        1 => {
            env.slice_del();
        }
        2 => {
            'lab0: loop {
                let v_1 = env.limit - env.cursor;
                'lab1: loop {
                    if !env.eq_s_b(&"s") {
                        break 'lab1;
                    }
                    break 'lab0;
                }
                env.cursor = env.limit - v_1;
                if !env.eq_s_b(&"t") {
                    return false;
                }
                break 'lab0;
            }
            env.slice_del();
        }
        _ => ()
    }
    return true
}

fn r_Step_5(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    env.ket = env.cursor;
    if (env.cursor <= env.limit_backward || (env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 101 as u8 && env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 108 as u8)) {
        return false;
    }

    among_var = env.find_among_b(A_10, context);
    if among_var == 0 {
        return false;
    }
    env.bra = env.cursor;
    match among_var {
        1 => {
            'lab0: loop {
                'lab1: loop {
                    if !r_R2(env, context) {
                        break 'lab1;
                    }
                    break 'lab0;
                }
                if !r_R1(env, context) {
                    return false;
                }
                let v_1 = env.limit - env.cursor;
                'lab2: loop {
                    if !r_shortv(env, context) {
                        break 'lab2;
                    }
                    return false;
                }
                env.cursor = env.limit - v_1;
                break 'lab0;
            }
            env.slice_del();
        }
        2 => {
            if !r_R2(env, context) {
                return false;
            }
            if !env.eq_s_b(&"l") {
                return false;
            }
            env.slice_del();
        }
        _ => ()
    }
    return true
}

fn r_exception1(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    env.bra = env.cursor;
    if (env.cursor + 2 >= env.limit || env.current.as_bytes()[(env.cursor + 2) as usize] as u8 >> 5 != 3 as u8 || ((42750482 as i32 >> (env.current.as_bytes()[(env.cursor + 2) as usize] as u8 & 0x1f)) & 1) == 0) {
        return false;
    }

    among_var = env.find_among(A_11, context);
    if among_var == 0 {
        return false;
    }
    env.ket = env.cursor;
    if env.cursor < env.limit {
        return false;
    }
    match among_var {
        1 => {
            env.slice_from("ski");
        }
        2 => {
            env.slice_from("sky");
        }
        3 => {
            env.slice_from("idl");
        }
        4 => {
            env.slice_from("gentl");
        }
        5 => {
            env.slice_from("ugli");
        }
        6 => {
            env.slice_from("earli");
        }
        7 => {
            env.slice_from("onli");
        }
        8 => {
            env.slice_from("singl");
        }
        _ => ()
    }
    return true
}

fn r_postlude(env: &mut SnowballEnv, context: &mut Context) -> bool {
    if !context.b_Y_found {
        return false;
    }
    'replab0: loop{
        let v_1 = env.cursor;
        'lab1: for _ in 0..1 {
            'golab2: loop {
                let v_2 = env.cursor;
                'lab3: loop {
                    env.bra = env.cursor;
                    if !env.eq_s(&"Y") {
                        break 'lab3;
                    }
                    env.ket = env.cursor;
                    env.cursor = v_2;
                    break 'golab2;
                }
                env.cursor = v_2;
                if env.cursor >= env.limit {
                    break 'lab1;
                }
                env.next_char();
            }
            env.slice_from("y");
            continue 'replab0;
        }
        env.cursor = v_1;
        break 'replab0;
    }
    return true
}

pub fn stem(env: &mut SnowballEnv) -> bool {
    let mut context = &mut Context {
        b_Y_found: false,
        i_p2: 0,
        i_p1: 0,
    };
    'lab0: loop {
        let v_1 = env.cursor;
        'lab1: loop {
            if !r_exception1(env, context) {
                break 'lab1;
            }
            break 'lab0;
        }
        env.cursor = v_1;
        'lab2: loop {
            let v_2 = env.cursor;
            'lab3: loop {
                if !env.hop(3) {
                    break 'lab3;
                }
                break 'lab2;
            }
            env.cursor = v_2;
            break 'lab0;
        }
        env.cursor = v_1;
        r_prelude(env, context);
        r_mark_regions(env, context);
        env.limit_backward = env.cursor;
        env.cursor = env.limit;
        let v_3 = env.limit - env.cursor;
        r_Step_1a(env, context);
        env.cursor = env.limit - v_3;
        let v_4 = env.limit - env.cursor;
        r_Step_1b(env, context);
        env.cursor = env.limit - v_4;
        let v_5 = env.limit - env.cursor;
        r_Step_1c(env, context);
        env.cursor = env.limit - v_5;
        let v_6 = env.limit - env.cursor;
        r_Step_2(env, context);
        env.cursor = env.limit - v_6;
        let v_7 = env.limit - env.cursor;
        r_Step_3(env, context);
        env.cursor = env.limit - v_7;
        let v_8 = env.limit - env.cursor;
        r_Step_4(env, context);
        env.cursor = env.limit - v_8;
        let v_9 = env.limit - env.cursor;
        r_Step_5(env, context);
        env.cursor = env.limit - v_9;
        env.cursor = env.limit_backward;
        let v_10 = env.cursor;
        r_postlude(env, context);
        env.cursor = v_10;
        break 'lab0;
    }
    return true
}
