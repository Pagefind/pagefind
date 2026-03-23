//! Generated from german.sbl by Snowball 3.0.0 - https://snowballstem.org/

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

static A_0: &'static [Among<Context>; 6] = &[
    Among("", -1, 5, None),
    Among("ae", 0, 2, None),
    Among("oe", 0, 3, None),
    Among("qu", 0, -1, None),
    Among("ue", 0, 4, None),
    Among("ß", 0, 1, None),
];

static A_1: &'static [Among<Context>; 6] = &[
    Among("", -1, 5, None),
    Among("U", 0, 2, None),
    Among("Y", 0, 1, None),
    Among("ä", 0, 3, None),
    Among("ö", 0, 4, None),
    Among("ü", 0, 2, None),
];

static A_2: &'static [Among<Context>; 11] = &[
    Among("e", -1, 3, None),
    Among("em", -1, 1, None),
    Among("en", -1, 3, None),
    Among("erinnen", 2, 2, None),
    Among("erin", -1, 2, None),
    Among("ln", -1, 5, None),
    Among("ern", -1, 2, None),
    Among("er", -1, 2, None),
    Among("s", -1, 4, None),
    Among("es", 8, 3, None),
    Among("lns", 8, 5, None),
];

static A_3: &'static [Among<Context>; 5] = &[
    Among("tick", -1, -1, None),
    Among("plan", -1, -1, None),
    Among("geordn", -1, -1, None),
    Among("intern", -1, -1, None),
    Among("tr", -1, -1, None),
];

static A_4: &'static [Among<Context>; 5] = &[
    Among("en", -1, 1, None),
    Among("er", -1, 1, None),
    Among("et", -1, 3, None),
    Among("st", -1, 2, None),
    Among("est", 3, 1, None),
];

static A_5: &'static [Among<Context>; 2] = &[
    Among("ig", -1, 1, None),
    Among("lich", -1, 1, None),
];

static A_6: &'static [Among<Context>; 8] = &[
    Among("end", -1, 1, None),
    Among("ig", -1, 2, None),
    Among("ung", -1, 1, None),
    Among("lich", -1, 3, None),
    Among("isch", -1, 2, None),
    Among("ik", -1, 2, None),
    Among("heit", -1, 3, None),
    Among("keit", -1, 4, None),
];

static G_v: &'static [u8; 20] = &[17, 65, 16, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 32, 8];

static G_et_ending: &'static [u8; 18] = &[1, 128, 198, 227, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128];

static G_s_ending: &'static [u8; 3] = &[117, 30, 5];

static G_st_ending: &'static [u8; 3] = &[117, 30, 4];

fn r_prelude(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    let v_1 = env.cursor;
    'replab0: loop{
        let v_2 = env.cursor;
        'lab1: for _ in 0..1 {
            'golab2: loop {
                let v_3 = env.cursor;
                'lab3: loop {
                    if !env.in_grouping(G_v, 97, 252) {
                        break 'lab3;
                    }
                    env.bra = env.cursor;
                    'lab4: loop {
                        let v_4 = env.cursor;
                        'lab5: loop {
                            if !env.eq_s(&"u") {
                                break 'lab5;
                            }
                            env.ket = env.cursor;
                            if !env.in_grouping(G_v, 97, 252) {
                                break 'lab5;
                            }
                            env.slice_from("U");
                            break 'lab4;
                        }
                        env.cursor = v_4;
                        if !env.eq_s(&"y") {
                            break 'lab3;
                        }
                        env.ket = env.cursor;
                        if !env.in_grouping(G_v, 97, 252) {
                            break 'lab3;
                        }
                        env.slice_from("Y");
                        break 'lab4;
                    }
                    env.cursor = v_3;
                    break 'golab2;
                }
                env.cursor = v_3;
                if env.cursor >= env.limit {
                    break 'lab1;
                }
                env.next_char();
            }
            continue 'replab0;
        }
        env.cursor = v_2;
        break 'replab0;
    }
    env.cursor = v_1;
    'replab6: loop{
        let v_5 = env.cursor;
        'lab7: for _ in 0..1 {
            env.bra = env.cursor;
            among_var = env.find_among(A_0, context);
            env.ket = env.cursor;
            match among_var {
                1 => {
                    env.slice_from("ss");
                }
                2 => {
                    env.slice_from("ä");
                }
                3 => {
                    env.slice_from("ö");
                }
                4 => {
                    env.slice_from("ü");
                }
                5 => {
                    if env.cursor >= env.limit {
                        break 'lab7;
                    }
                    env.next_char();
                }
                _ => ()
            }
            continue 'replab6;
        }
        env.cursor = v_5;
        break 'replab6;
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
    if !env.go_out_grouping(G_v, 97, 252) {
        return false;
    }
    env.next_char();
    if !env.go_in_grouping(G_v, 97, 252) {
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
    if !env.go_out_grouping(G_v, 97, 252) {
        return false;
    }
    env.next_char();
    if !env.go_in_grouping(G_v, 97, 252) {
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
            among_var = env.find_among(A_1, context);
            env.ket = env.cursor;
            match among_var {
                1 => {
                    env.slice_from("y");
                }
                2 => {
                    env.slice_from("u");
                }
                3 => {
                    env.slice_from("a");
                }
                4 => {
                    env.slice_from("o");
                }
                5 => {
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

fn r_standard_suffix(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    let v_1 = env.limit - env.cursor;
    'lab0: loop {
        env.ket = env.cursor;
        if (env.cursor <= env.limit_backward || env.current.as_bytes()[(env.cursor - 1) as usize] as u8 >> 5 != 3 as u8 || ((811040 as i32 >> (env.current.as_bytes()[(env.cursor - 1) as usize] as u8 & 0x1f)) & 1) == 0) {
            break 'lab0;
        }

        among_var = env.find_among_b(A_2, context);
        if among_var == 0 {
            break 'lab0;
        }
        env.bra = env.cursor;
        if !r_R1(env, context) {
            break 'lab0;
        }
        match among_var {
            1 => {
                let v_2 = env.limit - env.cursor;
                'lab1: loop {
                    if !env.eq_s_b(&"syst") {
                        break 'lab1;
                    }
                    break 'lab0;
                }
                env.cursor = env.limit - v_2;
                env.slice_del();
            }
            2 => {
                env.slice_del();
            }
            3 => {
                env.slice_del();
                let v_3 = env.limit - env.cursor;
                'lab2: loop {
                    env.ket = env.cursor;
                    if !env.eq_s_b(&"s") {
                        env.cursor = env.limit - v_3;
                        break 'lab2;
                    }
                    env.bra = env.cursor;
                    if !env.eq_s_b(&"nis") {
                        env.cursor = env.limit - v_3;
                        break 'lab2;
                    }
                    env.slice_del();
                    break 'lab2;
                }
            }
            4 => {
                if !env.in_grouping_b(G_s_ending, 98, 116) {
                    break 'lab0;
                }
                env.slice_del();
            }
            5 => {
                env.slice_from("l");
            }
            _ => ()
        }
        break 'lab0;
    }
    env.cursor = env.limit - v_1;
    let v_4 = env.limit - env.cursor;
    'lab3: loop {
        env.ket = env.cursor;
        if (env.cursor - 1 <= env.limit_backward || env.current.as_bytes()[(env.cursor - 1) as usize] as u8 >> 5 != 3 as u8 || ((1327104 as i32 >> (env.current.as_bytes()[(env.cursor - 1) as usize] as u8 & 0x1f)) & 1) == 0) {
            break 'lab3;
        }

        among_var = env.find_among_b(A_4, context);
        if among_var == 0 {
            break 'lab3;
        }
        env.bra = env.cursor;
        if !r_R1(env, context) {
            break 'lab3;
        }
        match among_var {
            1 => {
                env.slice_del();
            }
            2 => {
                if !env.in_grouping_b(G_st_ending, 98, 116) {
                    break 'lab3;
                }
                if !env.hop_back(3) {
                    break 'lab3;
                }
                env.slice_del();
            }
            3 => {
                let v_5 = env.limit - env.cursor;
                if !env.in_grouping_b(G_et_ending, 85, 228) {
                    break 'lab3;
                }
                env.cursor = env.limit - v_5;
                let v_6 = env.limit - env.cursor;
                'lab4: loop {
                    if (env.cursor - 1 <= env.limit_backward || env.current.as_bytes()[(env.cursor - 1) as usize] as u8 >> 5 != 3 as u8 || ((280576 as i32 >> (env.current.as_bytes()[(env.cursor - 1) as usize] as u8 & 0x1f)) & 1) == 0) {
                        break 'lab4;
                    }

                    if env.find_among_b(A_3, context) == 0 {
                        break 'lab4;
                    }
                    break 'lab3;
                }
                env.cursor = env.limit - v_6;
                env.slice_del();
            }
            _ => ()
        }
        break 'lab3;
    }
    env.cursor = env.limit - v_4;
    let v_7 = env.limit - env.cursor;
    'lab5: loop {
        env.ket = env.cursor;
        if (env.cursor - 1 <= env.limit_backward || env.current.as_bytes()[(env.cursor - 1) as usize] as u8 >> 5 != 3 as u8 || ((1051024 as i32 >> (env.current.as_bytes()[(env.cursor - 1) as usize] as u8 & 0x1f)) & 1) == 0) {
            break 'lab5;
        }

        among_var = env.find_among_b(A_6, context);
        if among_var == 0 {
            break 'lab5;
        }
        env.bra = env.cursor;
        if !r_R2(env, context) {
            break 'lab5;
        }
        match among_var {
            1 => {
                env.slice_del();
                let v_8 = env.limit - env.cursor;
                'lab6: loop {
                    env.ket = env.cursor;
                    if !env.eq_s_b(&"ig") {
                        env.cursor = env.limit - v_8;
                        break 'lab6;
                    }
                    env.bra = env.cursor;
                    let v_9 = env.limit - env.cursor;
                    'lab7: loop {
                        if !env.eq_s_b(&"e") {
                            break 'lab7;
                        }
                        env.cursor = env.limit - v_8;
                        break 'lab6;
                    }
                    env.cursor = env.limit - v_9;
                    if !r_R2(env, context) {
                        env.cursor = env.limit - v_8;
                        break 'lab6;
                    }
                    env.slice_del();
                    break 'lab6;
                }
            }
            2 => {
                let v_10 = env.limit - env.cursor;
                'lab8: loop {
                    if !env.eq_s_b(&"e") {
                        break 'lab8;
                    }
                    break 'lab5;
                }
                env.cursor = env.limit - v_10;
                env.slice_del();
            }
            3 => {
                env.slice_del();
                let v_11 = env.limit - env.cursor;
                'lab9: loop {
                    env.ket = env.cursor;
                    'lab10: loop {
                        let v_12 = env.limit - env.cursor;
                        'lab11: loop {
                            if !env.eq_s_b(&"er") {
                                break 'lab11;
                            }
                            break 'lab10;
                        }
                        env.cursor = env.limit - v_12;
                        if !env.eq_s_b(&"en") {
                            env.cursor = env.limit - v_11;
                            break 'lab9;
                        }
                        break 'lab10;
                    }
                    env.bra = env.cursor;
                    if !r_R1(env, context) {
                        env.cursor = env.limit - v_11;
                        break 'lab9;
                    }
                    env.slice_del();
                    break 'lab9;
                }
            }
            4 => {
                env.slice_del();
                let v_13 = env.limit - env.cursor;
                'lab12: loop {
                    env.ket = env.cursor;
                    if (env.cursor - 1 <= env.limit_backward || (env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 103 as u8 && env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 104 as u8)) {
                        env.cursor = env.limit - v_13;
                        break 'lab12;
                    }

                    if env.find_among_b(A_5, context) == 0 {
                        env.cursor = env.limit - v_13;
                        break 'lab12;
                    }
                    env.bra = env.cursor;
                    if !r_R2(env, context) {
                        env.cursor = env.limit - v_13;
                        break 'lab12;
                    }
                    env.slice_del();
                    break 'lab12;
                }
            }
            _ => ()
        }
        break 'lab5;
    }
    env.cursor = env.limit - v_7;
    return true
}

pub fn stem(env: &mut SnowballEnv) -> bool {
    let mut context = &mut Context {
        i_p2: 0,
        i_p1: 0,
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
