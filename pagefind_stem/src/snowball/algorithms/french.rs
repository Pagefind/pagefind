//! Generated from french.sbl by Snowball 3.0.0 - https://snowballstem.org/

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

static A_0: &'static [Among<Context>; 4] = &[
    Among("col", -1, -1, None),
    Among("ni", -1, 1, None),
    Among("par", -1, -1, None),
    Among("tap", -1, -1, None),
];

static A_1: &'static [Among<Context>; 7] = &[
    Among("", -1, 7, None),
    Among("H", 0, 6, None),
    Among("He", 1, 4, None),
    Among("Hi", 1, 5, None),
    Among("I", 0, 1, None),
    Among("U", 0, 2, None),
    Among("Y", 0, 3, None),
];

static A_2: &'static [Among<Context>; 6] = &[
    Among("iqU", -1, 3, None),
    Among("abl", -1, 3, None),
    Among("Ièr", -1, 4, None),
    Among("ièr", -1, 4, None),
    Among("eus", -1, 2, None),
    Among("iv", -1, 1, None),
];

static A_3: &'static [Among<Context>; 3] = &[
    Among("ic", -1, 2, None),
    Among("abil", -1, 1, None),
    Among("iv", -1, 3, None),
];

static A_4: &'static [Among<Context>; 44] = &[
    Among("iqUe", -1, 1, None),
    Among("atrice", -1, 2, None),
    Among("ance", -1, 1, None),
    Among("ence", -1, 5, None),
    Among("logie", -1, 3, None),
    Among("able", -1, 1, None),
    Among("isme", -1, 1, None),
    Among("euse", -1, 12, None),
    Among("iste", -1, 1, None),
    Among("ive", -1, 8, None),
    Among("if", -1, 8, None),
    Among("usion", -1, 4, None),
    Among("ation", -1, 2, None),
    Among("ution", -1, 4, None),
    Among("ateur", -1, 2, None),
    Among("iqUes", -1, 1, None),
    Among("atrices", -1, 2, None),
    Among("ances", -1, 1, None),
    Among("ences", -1, 5, None),
    Among("logies", -1, 3, None),
    Among("ables", -1, 1, None),
    Among("ismes", -1, 1, None),
    Among("euses", -1, 12, None),
    Among("istes", -1, 1, None),
    Among("ives", -1, 8, None),
    Among("ifs", -1, 8, None),
    Among("usions", -1, 4, None),
    Among("ations", -1, 2, None),
    Among("utions", -1, 4, None),
    Among("ateurs", -1, 2, None),
    Among("ments", -1, 16, None),
    Among("ements", 30, 6, None),
    Among("issements", 31, 13, None),
    Among("ités", -1, 7, None),
    Among("ment", -1, 16, None),
    Among("ement", 34, 6, None),
    Among("issement", 35, 13, None),
    Among("amment", 34, 14, None),
    Among("emment", 34, 15, None),
    Among("aux", -1, 10, None),
    Among("eaux", 39, 9, None),
    Among("eux", -1, 1, None),
    Among("oux", -1, 11, None),
    Among("ité", -1, 7, None),
];

static A_5: &'static [Among<Context>; 35] = &[
    Among("ira", -1, 1, None),
    Among("ie", -1, 1, None),
    Among("isse", -1, 1, None),
    Among("issante", -1, 1, None),
    Among("i", -1, 1, None),
    Among("irai", 4, 1, None),
    Among("ir", -1, 1, None),
    Among("iras", -1, 1, None),
    Among("ies", -1, 1, None),
    Among("îmes", -1, 1, None),
    Among("isses", -1, 1, None),
    Among("issantes", -1, 1, None),
    Among("îtes", -1, 1, None),
    Among("is", -1, 1, None),
    Among("irais", 13, 1, None),
    Among("issais", 13, 1, None),
    Among("irions", -1, 1, None),
    Among("issions", -1, 1, None),
    Among("irons", -1, 1, None),
    Among("issons", -1, 1, None),
    Among("issants", -1, 1, None),
    Among("it", -1, 1, None),
    Among("irait", 21, 1, None),
    Among("issait", 21, 1, None),
    Among("issant", -1, 1, None),
    Among("iraIent", -1, 1, None),
    Among("issaIent", -1, 1, None),
    Among("irent", -1, 1, None),
    Among("issent", -1, 1, None),
    Among("iront", -1, 1, None),
    Among("ît", -1, 1, None),
    Among("iriez", -1, 1, None),
    Among("issiez", -1, 1, None),
    Among("irez", -1, 1, None),
    Among("issez", -1, 1, None),
];

static A_6: &'static [Among<Context>; 3] = &[
    Among("al", -1, 1, None),
    Among("épl", -1, -1, None),
    Among("auv", -1, -1, None),
];

static A_7: &'static [Among<Context>; 41] = &[
    Among("a", -1, 3, None),
    Among("era", 0, 2, None),
    Among("aise", -1, 4, None),
    Among("asse", -1, 3, None),
    Among("ante", -1, 3, None),
    Among("ée", -1, 2, None),
    Among("ai", -1, 3, None),
    Among("erai", 6, 2, None),
    Among("er", -1, 2, None),
    Among("as", -1, 3, None),
    Among("eras", 9, 2, None),
    Among("âmes", -1, 3, None),
    Among("aises", -1, 4, None),
    Among("asses", -1, 3, None),
    Among("antes", -1, 3, None),
    Among("âtes", -1, 3, None),
    Among("ées", -1, 2, None),
    Among("ais", -1, 4, None),
    Among("eais", 17, 2, None),
    Among("erais", 17, 2, None),
    Among("ions", -1, 1, None),
    Among("erions", 20, 2, None),
    Among("assions", 20, 3, None),
    Among("erons", -1, 2, None),
    Among("ants", -1, 3, None),
    Among("és", -1, 2, None),
    Among("ait", -1, 3, None),
    Among("erait", 26, 2, None),
    Among("ant", -1, 3, None),
    Among("aIent", -1, 3, None),
    Among("eraIent", 29, 2, None),
    Among("èrent", -1, 2, None),
    Among("assent", -1, 3, None),
    Among("eront", -1, 2, None),
    Among("ât", -1, 3, None),
    Among("ez", -1, 2, None),
    Among("iez", 35, 2, None),
    Among("eriez", 36, 2, None),
    Among("assiez", 36, 3, None),
    Among("erez", 35, 2, None),
    Among("é", -1, 2, None),
];

static A_8: &'static [Among<Context>; 6] = &[
    Among("e", -1, 3, None),
    Among("Ière", 0, 2, None),
    Among("ière", 0, 2, None),
    Among("ion", -1, 1, None),
    Among("Ier", -1, 2, None),
    Among("ier", -1, 2, None),
];

static A_9: &'static [Among<Context>; 5] = &[
    Among("ell", -1, -1, None),
    Among("eill", -1, -1, None),
    Among("enn", -1, -1, None),
    Among("onn", -1, -1, None),
    Among("ett", -1, -1, None),
];

static G_v: &'static [u8; 20] = &[17, 65, 16, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 130, 103, 8, 5];

static G_oux_ending: &'static [u8; 2] = &[65, 85];

static G_elision_char: &'static [u8; 3] = &[131, 14, 3];

static G_keep_with_s: &'static [u8; 17] = &[1, 65, 20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128];

fn r_elisions(env: &mut SnowballEnv, context: &mut Context) -> bool {
    env.bra = env.cursor;
    'lab0: loop {
        let v_1 = env.cursor;
        'lab1: loop {
            if !env.in_grouping(G_elision_char, 99, 116) {
                break 'lab1;
            }
            break 'lab0;
        }
        env.cursor = v_1;
        if !env.eq_s(&"qu") {
            return false;
        }
        break 'lab0;
    }
    if !env.eq_s(&"'") {
        return false;
    }
    env.ket = env.cursor;
    'lab2: loop {
        if env.cursor < env.limit {
            break 'lab2;
        }
        return false;
    }
    env.slice_del();
    return true
}

fn r_prelude(env: &mut SnowballEnv, context: &mut Context) -> bool {
    'replab0: loop{
        let v_1 = env.cursor;
        'lab1: for _ in 0..1 {
            'golab2: loop {
                let v_2 = env.cursor;
                'lab3: loop {
                    'lab4: loop {
                        let v_3 = env.cursor;
                        'lab5: loop {
                            if !env.in_grouping(G_v, 97, 251) {
                                break 'lab5;
                            }
                            env.bra = env.cursor;
                            'lab6: loop {
                                let v_4 = env.cursor;
                                'lab7: loop {
                                    if !env.eq_s(&"u") {
                                        break 'lab7;
                                    }
                                    env.ket = env.cursor;
                                    if !env.in_grouping(G_v, 97, 251) {
                                        break 'lab7;
                                    }
                                    env.slice_from("U");
                                    break 'lab6;
                                }
                                env.cursor = v_4;
                                'lab8: loop {
                                    if !env.eq_s(&"i") {
                                        break 'lab8;
                                    }
                                    env.ket = env.cursor;
                                    if !env.in_grouping(G_v, 97, 251) {
                                        break 'lab8;
                                    }
                                    env.slice_from("I");
                                    break 'lab6;
                                }
                                env.cursor = v_4;
                                if !env.eq_s(&"y") {
                                    break 'lab5;
                                }
                                env.ket = env.cursor;
                                env.slice_from("Y");
                                break 'lab6;
                            }
                            break 'lab4;
                        }
                        env.cursor = v_3;
                        'lab9: loop {
                            env.bra = env.cursor;
                            if !env.eq_s(&"ë") {
                                break 'lab9;
                            }
                            env.ket = env.cursor;
                            env.slice_from("He");
                            break 'lab4;
                        }
                        env.cursor = v_3;
                        'lab10: loop {
                            env.bra = env.cursor;
                            if !env.eq_s(&"ï") {
                                break 'lab10;
                            }
                            env.ket = env.cursor;
                            env.slice_from("Hi");
                            break 'lab4;
                        }
                        env.cursor = v_3;
                        'lab11: loop {
                            env.bra = env.cursor;
                            if !env.eq_s(&"y") {
                                break 'lab11;
                            }
                            env.ket = env.cursor;
                            if !env.in_grouping(G_v, 97, 251) {
                                break 'lab11;
                            }
                            env.slice_from("Y");
                            break 'lab4;
                        }
                        env.cursor = v_3;
                        if !env.eq_s(&"q") {
                            break 'lab3;
                        }
                        env.bra = env.cursor;
                        if !env.eq_s(&"u") {
                            break 'lab3;
                        }
                        env.ket = env.cursor;
                        env.slice_from("U");
                        break 'lab4;
                    }
                    env.cursor = v_2;
                    break 'golab2;
                }
                env.cursor = v_2;
                if env.cursor >= env.limit {
                    break 'lab1;
                }
                env.next_char();
            }
            continue 'replab0;
        }
        env.cursor = v_1;
        break 'replab0;
    }
    return true
}

fn r_mark_regions(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    context.i_pV = env.limit;
    context.i_p1 = env.limit;
    context.i_p2 = env.limit;
    let v_1 = env.cursor;
    'lab0: loop {
        'lab1: loop {
            let v_2 = env.cursor;
            'lab2: loop {
                if !env.in_grouping(G_v, 97, 251) {
                    break 'lab2;
                }
                if !env.in_grouping(G_v, 97, 251) {
                    break 'lab2;
                }
                if env.cursor >= env.limit {
                    break 'lab2;
                }
                env.next_char();
                break 'lab1;
            }
            env.cursor = v_2;
            'lab3: loop {
                if (env.cursor + 1 >= env.limit || env.current.as_bytes()[(env.cursor + 1) as usize] as u8 >> 5 != 3 as u8 || ((33282 as i32 >> (env.current.as_bytes()[(env.cursor + 1) as usize] as u8 & 0x1f)) & 1) == 0) {
                    break 'lab3;
                }

                among_var = env.find_among(A_0, context);
                if among_var == 0 {
                    break 'lab3;
                }
                match among_var {
                    1 => {
                        if !env.in_grouping(G_v, 97, 251) {
                            break 'lab3;
                        }
                    }
                    _ => ()
                }
                break 'lab1;
            }
            env.cursor = v_2;
            if env.cursor >= env.limit {
                break 'lab0;
            }
            env.next_char();
            if !env.go_out_grouping(G_v, 97, 251) {
                break 'lab0;
            }
            env.next_char();
            break 'lab1;
        }
        context.i_pV = env.cursor;
        break 'lab0;
    }
    env.cursor = v_1;
    let v_3 = env.cursor;
    'lab4: loop {
        if !env.go_out_grouping(G_v, 97, 251) {
            break 'lab4;
        }
        env.next_char();
        if !env.go_in_grouping(G_v, 97, 251) {
            break 'lab4;
        }
        env.next_char();
        context.i_p1 = env.cursor;
        if !env.go_out_grouping(G_v, 97, 251) {
            break 'lab4;
        }
        env.next_char();
        if !env.go_in_grouping(G_v, 97, 251) {
            break 'lab4;
        }
        env.next_char();
        context.i_p2 = env.cursor;
        break 'lab4;
    }
    env.cursor = v_3;
    return true
}

fn r_postlude(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    'replab0: loop{
        let v_1 = env.cursor;
        'lab1: for _ in 0..1 {
            env.bra = env.cursor;
            if (env.cursor >= env.limit || env.current.as_bytes()[(env.cursor + 0) as usize] as u8 >> 5 != 2 as u8 || ((35652352 as i32 >> (env.current.as_bytes()[(env.cursor + 0) as usize] as u8 & 0x1f)) & 1) == 0) {among_var = 7;}
            else {
                among_var = env.find_among(A_1, context);
            }
            env.ket = env.cursor;
            match among_var {
                1 => {
                    env.slice_from("i");
                }
                2 => {
                    env.slice_from("u");
                }
                3 => {
                    env.slice_from("y");
                }
                4 => {
                    env.slice_from("ë");
                }
                5 => {
                    env.slice_from("ï");
                }
                6 => {
                    env.slice_del();
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

fn r_RV(env: &mut SnowballEnv, context: &mut Context) -> bool {
    return context.i_pV <= env.cursor
}

fn r_R1(env: &mut SnowballEnv, context: &mut Context) -> bool {
    return context.i_p1 <= env.cursor
}

fn r_R2(env: &mut SnowballEnv, context: &mut Context) -> bool {
    return context.i_p2 <= env.cursor
}

fn r_standard_suffix(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    env.ket = env.cursor;
    among_var = env.find_among_b(A_4, context);
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
            if !r_R2(env, context) {
                return false;
            }
            env.slice_del();
            let v_1 = env.limit - env.cursor;
            'lab0: loop {
                env.ket = env.cursor;
                if !env.eq_s_b(&"ic") {
                    env.cursor = env.limit - v_1;
                    break 'lab0;
                }
                env.bra = env.cursor;
                'lab1: loop {
                    let v_2 = env.limit - env.cursor;
                    'lab2: loop {
                        if !r_R2(env, context) {
                            break 'lab2;
                        }
                        env.slice_del();
                        break 'lab1;
                    }
                    env.cursor = env.limit - v_2;
                    env.slice_from("iqU");
                    break 'lab1;
                }
                break 'lab0;
            }
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
            env.slice_from("u");
        }
        5 => {
            if !r_R2(env, context) {
                return false;
            }
            env.slice_from("ent");
        }
        6 => {
            if !r_RV(env, context) {
                return false;
            }
            env.slice_del();
            let v_3 = env.limit - env.cursor;
            'lab3: loop {
                env.ket = env.cursor;
                among_var = env.find_among_b(A_2, context);
                if among_var == 0 {
                    env.cursor = env.limit - v_3;
                    break 'lab3;
                }
                env.bra = env.cursor;
                match among_var {
                    1 => {
                        if !r_R2(env, context) {
                            env.cursor = env.limit - v_3;
                            break 'lab3;
                        }
                        env.slice_del();
                        env.ket = env.cursor;
                        if !env.eq_s_b(&"at") {
                            env.cursor = env.limit - v_3;
                            break 'lab3;
                        }
                        env.bra = env.cursor;
                        if !r_R2(env, context) {
                            env.cursor = env.limit - v_3;
                            break 'lab3;
                        }
                        env.slice_del();
                    }
                    2 => {
                        'lab4: loop {
                            let v_4 = env.limit - env.cursor;
                            'lab5: loop {
                                if !r_R2(env, context) {
                                    break 'lab5;
                                }
                                env.slice_del();
                                break 'lab4;
                            }
                            env.cursor = env.limit - v_4;
                            if !r_R1(env, context) {
                                env.cursor = env.limit - v_3;
                                break 'lab3;
                            }
                            env.slice_from("eux");
                            break 'lab4;
                        }
                    }
                    3 => {
                        if !r_R2(env, context) {
                            env.cursor = env.limit - v_3;
                            break 'lab3;
                        }
                        env.slice_del();
                    }
                    4 => {
                        if !r_RV(env, context) {
                            env.cursor = env.limit - v_3;
                            break 'lab3;
                        }
                        env.slice_from("i");
                    }
                    _ => ()
                }
                break 'lab3;
            }
        }
        7 => {
            if !r_R2(env, context) {
                return false;
            }
            env.slice_del();
            let v_5 = env.limit - env.cursor;
            'lab6: loop {
                env.ket = env.cursor;
                if (env.cursor - 1 <= env.limit_backward || env.current.as_bytes()[(env.cursor - 1) as usize] as u8 >> 5 != 3 as u8 || ((4198408 as i32 >> (env.current.as_bytes()[(env.cursor - 1) as usize] as u8 & 0x1f)) & 1) == 0) {
                    env.cursor = env.limit - v_5;
                    break 'lab6;
                }

                among_var = env.find_among_b(A_3, context);
                if among_var == 0 {
                    env.cursor = env.limit - v_5;
                    break 'lab6;
                }
                env.bra = env.cursor;
                match among_var {
                    1 => {
                        'lab7: loop {
                            let v_6 = env.limit - env.cursor;
                            'lab8: loop {
                                if !r_R2(env, context) {
                                    break 'lab8;
                                }
                                env.slice_del();
                                break 'lab7;
                            }
                            env.cursor = env.limit - v_6;
                            env.slice_from("abl");
                            break 'lab7;
                        }
                    }
                    2 => {
                        'lab9: loop {
                            let v_7 = env.limit - env.cursor;
                            'lab10: loop {
                                if !r_R2(env, context) {
                                    break 'lab10;
                                }
                                env.slice_del();
                                break 'lab9;
                            }
                            env.cursor = env.limit - v_7;
                            env.slice_from("iqU");
                            break 'lab9;
                        }
                    }
                    3 => {
                        if !r_R2(env, context) {
                            env.cursor = env.limit - v_5;
                            break 'lab6;
                        }
                        env.slice_del();
                    }
                    _ => ()
                }
                break 'lab6;
            }
        }
        8 => {
            if !r_R2(env, context) {
                return false;
            }
            env.slice_del();
            let v_8 = env.limit - env.cursor;
            'lab11: loop {
                env.ket = env.cursor;
                if !env.eq_s_b(&"at") {
                    env.cursor = env.limit - v_8;
                    break 'lab11;
                }
                env.bra = env.cursor;
                if !r_R2(env, context) {
                    env.cursor = env.limit - v_8;
                    break 'lab11;
                }
                env.slice_del();
                env.ket = env.cursor;
                if !env.eq_s_b(&"ic") {
                    env.cursor = env.limit - v_8;
                    break 'lab11;
                }
                env.bra = env.cursor;
                'lab12: loop {
                    let v_9 = env.limit - env.cursor;
                    'lab13: loop {
                        if !r_R2(env, context) {
                            break 'lab13;
                        }
                        env.slice_del();
                        break 'lab12;
                    }
                    env.cursor = env.limit - v_9;
                    env.slice_from("iqU");
                    break 'lab12;
                }
                break 'lab11;
            }
        }
        9 => {
            env.slice_from("eau");
        }
        10 => {
            if !r_R1(env, context) {
                return false;
            }
            env.slice_from("al");
        }
        11 => {
            if !env.in_grouping_b(G_oux_ending, 98, 112) {
                return false;
            }
            env.slice_from("ou");
        }
        12 => {
            'lab14: loop {
                let v_10 = env.limit - env.cursor;
                'lab15: loop {
                    if !r_R2(env, context) {
                        break 'lab15;
                    }
                    env.slice_del();
                    break 'lab14;
                }
                env.cursor = env.limit - v_10;
                if !r_R1(env, context) {
                    return false;
                }
                env.slice_from("eux");
                break 'lab14;
            }
        }
        13 => {
            if !r_R1(env, context) {
                return false;
            }
            if !env.out_grouping_b(G_v, 97, 251) {
                return false;
            }
            env.slice_del();
        }
        14 => {
            if !r_RV(env, context) {
                return false;
            }
            env.slice_from("ant");
            return false;
        }
        15 => {
            if !r_RV(env, context) {
                return false;
            }
            env.slice_from("ent");
            return false;
        }
        16 => {
            let v_11 = env.limit - env.cursor;
            if !env.in_grouping_b(G_v, 97, 251) {
                return false;
            }
            if !r_RV(env, context) {
                return false;
            }
            env.cursor = env.limit - v_11;
            env.slice_del();
            return false;
        }
        _ => ()
    }
    return true
}

fn r_i_verb_suffix(env: &mut SnowballEnv, context: &mut Context) -> bool {
    if env.cursor < context.i_pV {
        return false;
    }
    let v_1 = env.limit_backward;
    env.limit_backward = context.i_pV;
    env.ket = env.cursor;
    if (env.cursor <= env.limit_backward || env.current.as_bytes()[(env.cursor - 1) as usize] as u8 >> 5 != 3 as u8 || ((68944418 as i32 >> (env.current.as_bytes()[(env.cursor - 1) as usize] as u8 & 0x1f)) & 1) == 0) {
        env.limit_backward = v_1;
        return false;
    }

    if env.find_among_b(A_5, context) == 0 {
        env.limit_backward = v_1;
        return false;
    }
    env.bra = env.cursor;
    let v_2 = env.limit - env.cursor;
    'lab0: loop {
        if !env.eq_s_b(&"H") {
            break 'lab0;
        }
        env.limit_backward = v_1;
        return false;
    }
    env.cursor = env.limit - v_2;
    if !env.out_grouping_b(G_v, 97, 251) {
        env.limit_backward = v_1;
        return false;
    }
    env.slice_del();
    env.limit_backward = v_1;
    return true
}

fn r_verb_suffix(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    if env.cursor < context.i_pV {
        return false;
    }
    let v_1 = env.limit_backward;
    env.limit_backward = context.i_pV;
    env.ket = env.cursor;
    among_var = env.find_among_b(A_7, context);
    if among_var == 0 {
        env.limit_backward = v_1;
        return false;
    }
    env.bra = env.cursor;
    env.limit_backward = v_1;
    match among_var {
        1 => {
            if !r_R2(env, context) {
                return false;
            }
            env.slice_del();
        }
        2 => {
            env.slice_del();
        }
        3 => {
            let v_2 = env.limit - env.cursor;
            'lab0: loop {
                if !env.eq_s_b(&"e") {
                    env.cursor = env.limit - v_2;
                    break 'lab0;
                }
                if !r_RV(env, context) {
                    env.cursor = env.limit - v_2;
                    break 'lab0;
                }
                env.bra = env.cursor;
                break 'lab0;
            }
            env.slice_del();
        }
        4 => {
            let v_3 = env.limit - env.cursor;
            'lab1: loop {
                if (env.cursor - 1 <= env.limit_backward || (env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 108 as u8 && env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 118 as u8)) {
                    break 'lab1;
                }

                among_var = env.find_among_b(A_6, context);
                if among_var == 0 {
                    break 'lab1;
                }
                match among_var {
                    1 => {
                        if env.cursor <= env.limit_backward {
                            break 'lab1;
                        }
                        env.previous_char();
                        if env.cursor > env.limit_backward {
                            break 'lab1;
                        }
                    }
                    _ => ()
                }
                return false;
            }
            env.cursor = env.limit - v_3;
            env.slice_del();
        }
        _ => ()
    }
    return true
}

fn r_residual_suffix(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    let v_1 = env.limit - env.cursor;
    'lab0: loop {
        env.ket = env.cursor;
        if !env.eq_s_b(&"s") {
            env.cursor = env.limit - v_1;
            break 'lab0;
        }
        env.bra = env.cursor;
        let v_2 = env.limit - env.cursor;
        'lab1: loop {
            let v_3 = env.limit - env.cursor;
            'lab2: loop {
                if !env.eq_s_b(&"Hi") {
                    break 'lab2;
                }
                break 'lab1;
            }
            env.cursor = env.limit - v_3;
            if !env.out_grouping_b(G_keep_with_s, 97, 232) {
                env.cursor = env.limit - v_1;
                break 'lab0;
            }
            break 'lab1;
        }
        env.cursor = env.limit - v_2;
        env.slice_del();
        break 'lab0;
    }
    if env.cursor < context.i_pV {
        return false;
    }
    let v_4 = env.limit_backward;
    env.limit_backward = context.i_pV;
    env.ket = env.cursor;
    if (env.cursor <= env.limit_backward || env.current.as_bytes()[(env.cursor - 1) as usize] as u8 >> 5 != 3 as u8 || ((278560 as i32 >> (env.current.as_bytes()[(env.cursor - 1) as usize] as u8 & 0x1f)) & 1) == 0) {
        env.limit_backward = v_4;
        return false;
    }

    among_var = env.find_among_b(A_8, context);
    if among_var == 0 {
        env.limit_backward = v_4;
        return false;
    }
    env.bra = env.cursor;
    match among_var {
        1 => {
            if !r_R2(env, context) {
                env.limit_backward = v_4;
                return false;
            }
            'lab3: loop {
                let v_5 = env.limit - env.cursor;
                'lab4: loop {
                    if !env.eq_s_b(&"s") {
                        break 'lab4;
                    }
                    break 'lab3;
                }
                env.cursor = env.limit - v_5;
                if !env.eq_s_b(&"t") {
                    env.limit_backward = v_4;
                    return false;
                }
                break 'lab3;
            }
            env.slice_del();
        }
        2 => {
            env.slice_from("i");
        }
        3 => {
            env.slice_del();
        }
        _ => ()
    }
    env.limit_backward = v_4;
    return true
}

fn r_un_double(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let v_1 = env.limit - env.cursor;
    if (env.cursor - 2 <= env.limit_backward || env.current.as_bytes()[(env.cursor - 1) as usize] as u8 >> 5 != 3 as u8 || ((1069056 as i32 >> (env.current.as_bytes()[(env.cursor - 1) as usize] as u8 & 0x1f)) & 1) == 0) {
        return false;
    }

    if env.find_among_b(A_9, context) == 0 {
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

fn r_un_accent(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut v_1 = 1;
    'replab0: loop{
        'lab1: for _ in 0..1 {
            if !env.out_grouping_b(G_v, 97, 251) {
                break 'lab1;
            }
            v_1 -= 1;
            continue 'replab0;
        }
        break 'replab0;
    }
    if v_1 > 0 {
        return false;
    }
    env.ket = env.cursor;
    'lab2: loop {
        let v_2 = env.limit - env.cursor;
        'lab3: loop {
            if !env.eq_s_b(&"é") {
                break 'lab3;
            }
            break 'lab2;
        }
        env.cursor = env.limit - v_2;
        if !env.eq_s_b(&"è") {
            return false;
        }
        break 'lab2;
    }
    env.bra = env.cursor;
    env.slice_from("e");
    return true
}

pub fn stem(env: &mut SnowballEnv) -> bool {
    let mut context = &mut Context {
        i_p2: 0,
        i_p1: 0,
        i_pV: 0,
    };
    let v_1 = env.cursor;
    r_elisions(env, context);
    env.cursor = v_1;
    let v_2 = env.cursor;
    r_prelude(env, context);
    env.cursor = v_2;
    r_mark_regions(env, context);
    env.limit_backward = env.cursor;
    env.cursor = env.limit;
    let v_3 = env.limit - env.cursor;
    'lab0: loop {
        'lab1: loop {
            let v_4 = env.limit - env.cursor;
            'lab2: loop {
                let v_5 = env.limit - env.cursor;
                'lab3: loop {
                    let v_6 = env.limit - env.cursor;
                    'lab4: loop {
                        if !r_standard_suffix(env, context) {
                            break 'lab4;
                        }
                        break 'lab3;
                    }
                    env.cursor = env.limit - v_6;
                    'lab5: loop {
                        if !r_i_verb_suffix(env, context) {
                            break 'lab5;
                        }
                        break 'lab3;
                    }
                    env.cursor = env.limit - v_6;
                    if !r_verb_suffix(env, context) {
                        break 'lab2;
                    }
                    break 'lab3;
                }
                env.cursor = env.limit - v_5;
                let v_7 = env.limit - env.cursor;
                'lab6: loop {
                    env.ket = env.cursor;
                    'lab7: loop {
                        let v_8 = env.limit - env.cursor;
                        'lab8: loop {
                            if !env.eq_s_b(&"Y") {
                                break 'lab8;
                            }
                            env.bra = env.cursor;
                            env.slice_from("i");
                            break 'lab7;
                        }
                        env.cursor = env.limit - v_8;
                        if !env.eq_s_b(&"ç") {
                            env.cursor = env.limit - v_7;
                            break 'lab6;
                        }
                        env.bra = env.cursor;
                        env.slice_from("c");
                        break 'lab7;
                    }
                    break 'lab6;
                }
                break 'lab1;
            }
            env.cursor = env.limit - v_4;
            if !r_residual_suffix(env, context) {
                break 'lab0;
            }
            break 'lab1;
        }
        break 'lab0;
    }
    env.cursor = env.limit - v_3;
    let v_9 = env.limit - env.cursor;
    r_un_double(env, context);
    env.cursor = env.limit - v_9;
    let v_10 = env.limit - env.cursor;
    r_un_accent(env, context);
    env.cursor = env.limit - v_10;
    env.cursor = env.limit_backward;
    let v_11 = env.cursor;
    r_postlude(env, context);
    env.cursor = v_11;
    return true
}
