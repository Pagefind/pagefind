//! Generated from arabic.sbl by Snowball 3.0.0 - https://snowballstem.org/

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_mut)]
#![allow(unused_parens)]
#![allow(unused_variables)]
use snowball::SnowballEnv;
use snowball::Among;

#[derive(Clone)]
struct Context {
    b_is_defined: bool,
    b_is_verb: bool,
    b_is_noun: bool,
}

static A_0: &'static [Among<Context>; 144] = &[
    Among("\u{0640}", -1, 1, None),
    Among("\u{064B}", -1, 1, None),
    Among("\u{064C}", -1, 1, None),
    Among("\u{064D}", -1, 1, None),
    Among("\u{064E}", -1, 1, None),
    Among("\u{064F}", -1, 1, None),
    Among("\u{0650}", -1, 1, None),
    Among("\u{0651}", -1, 1, None),
    Among("\u{0652}", -1, 1, None),
    Among("\u{0660}", -1, 2, None),
    Among("\u{0661}", -1, 3, None),
    Among("\u{0662}", -1, 4, None),
    Among("\u{0663}", -1, 5, None),
    Among("\u{0664}", -1, 6, None),
    Among("\u{0665}", -1, 7, None),
    Among("\u{0666}", -1, 8, None),
    Among("\u{0667}", -1, 9, None),
    Among("\u{0668}", -1, 10, None),
    Among("\u{0669}", -1, 11, None),
    Among("\u{FE80}", -1, 12, None),
    Among("\u{FE81}", -1, 16, None),
    Among("\u{FE82}", -1, 16, None),
    Among("\u{FE83}", -1, 13, None),
    Among("\u{FE84}", -1, 13, None),
    Among("\u{FE85}", -1, 17, None),
    Among("\u{FE86}", -1, 17, None),
    Among("\u{FE87}", -1, 14, None),
    Among("\u{FE88}", -1, 14, None),
    Among("\u{FE89}", -1, 15, None),
    Among("\u{FE8A}", -1, 15, None),
    Among("\u{FE8B}", -1, 15, None),
    Among("\u{FE8C}", -1, 15, None),
    Among("\u{FE8D}", -1, 18, None),
    Among("\u{FE8E}", -1, 18, None),
    Among("\u{FE8F}", -1, 19, None),
    Among("\u{FE90}", -1, 19, None),
    Among("\u{FE91}", -1, 19, None),
    Among("\u{FE92}", -1, 19, None),
    Among("\u{FE93}", -1, 20, None),
    Among("\u{FE94}", -1, 20, None),
    Among("\u{FE95}", -1, 21, None),
    Among("\u{FE96}", -1, 21, None),
    Among("\u{FE97}", -1, 21, None),
    Among("\u{FE98}", -1, 21, None),
    Among("\u{FE99}", -1, 22, None),
    Among("\u{FE9A}", -1, 22, None),
    Among("\u{FE9B}", -1, 22, None),
    Among("\u{FE9C}", -1, 22, None),
    Among("\u{FE9D}", -1, 23, None),
    Among("\u{FE9E}", -1, 23, None),
    Among("\u{FE9F}", -1, 23, None),
    Among("\u{FEA0}", -1, 23, None),
    Among("\u{FEA1}", -1, 24, None),
    Among("\u{FEA2}", -1, 24, None),
    Among("\u{FEA3}", -1, 24, None),
    Among("\u{FEA4}", -1, 24, None),
    Among("\u{FEA5}", -1, 25, None),
    Among("\u{FEA6}", -1, 25, None),
    Among("\u{FEA7}", -1, 25, None),
    Among("\u{FEA8}", -1, 25, None),
    Among("\u{FEA9}", -1, 26, None),
    Among("\u{FEAA}", -1, 26, None),
    Among("\u{FEAB}", -1, 27, None),
    Among("\u{FEAC}", -1, 27, None),
    Among("\u{FEAD}", -1, 28, None),
    Among("\u{FEAE}", -1, 28, None),
    Among("\u{FEAF}", -1, 29, None),
    Among("\u{FEB0}", -1, 29, None),
    Among("\u{FEB1}", -1, 30, None),
    Among("\u{FEB2}", -1, 30, None),
    Among("\u{FEB3}", -1, 30, None),
    Among("\u{FEB4}", -1, 30, None),
    Among("\u{FEB5}", -1, 31, None),
    Among("\u{FEB6}", -1, 31, None),
    Among("\u{FEB7}", -1, 31, None),
    Among("\u{FEB8}", -1, 31, None),
    Among("\u{FEB9}", -1, 32, None),
    Among("\u{FEBA}", -1, 32, None),
    Among("\u{FEBB}", -1, 32, None),
    Among("\u{FEBC}", -1, 32, None),
    Among("\u{FEBD}", -1, 33, None),
    Among("\u{FEBE}", -1, 33, None),
    Among("\u{FEBF}", -1, 33, None),
    Among("\u{FEC0}", -1, 33, None),
    Among("\u{FEC1}", -1, 34, None),
    Among("\u{FEC2}", -1, 34, None),
    Among("\u{FEC3}", -1, 34, None),
    Among("\u{FEC4}", -1, 34, None),
    Among("\u{FEC5}", -1, 35, None),
    Among("\u{FEC6}", -1, 35, None),
    Among("\u{FEC7}", -1, 35, None),
    Among("\u{FEC8}", -1, 35, None),
    Among("\u{FEC9}", -1, 36, None),
    Among("\u{FECA}", -1, 36, None),
    Among("\u{FECB}", -1, 36, None),
    Among("\u{FECC}", -1, 36, None),
    Among("\u{FECD}", -1, 37, None),
    Among("\u{FECE}", -1, 37, None),
    Among("\u{FECF}", -1, 37, None),
    Among("\u{FED0}", -1, 37, None),
    Among("\u{FED1}", -1, 38, None),
    Among("\u{FED2}", -1, 38, None),
    Among("\u{FED3}", -1, 38, None),
    Among("\u{FED4}", -1, 38, None),
    Among("\u{FED5}", -1, 39, None),
    Among("\u{FED6}", -1, 39, None),
    Among("\u{FED7}", -1, 39, None),
    Among("\u{FED8}", -1, 39, None),
    Among("\u{FED9}", -1, 40, None),
    Among("\u{FEDA}", -1, 40, None),
    Among("\u{FEDB}", -1, 40, None),
    Among("\u{FEDC}", -1, 40, None),
    Among("\u{FEDD}", -1, 41, None),
    Among("\u{FEDE}", -1, 41, None),
    Among("\u{FEDF}", -1, 41, None),
    Among("\u{FEE0}", -1, 41, None),
    Among("\u{FEE1}", -1, 42, None),
    Among("\u{FEE2}", -1, 42, None),
    Among("\u{FEE3}", -1, 42, None),
    Among("\u{FEE4}", -1, 42, None),
    Among("\u{FEE5}", -1, 43, None),
    Among("\u{FEE6}", -1, 43, None),
    Among("\u{FEE7}", -1, 43, None),
    Among("\u{FEE8}", -1, 43, None),
    Among("\u{FEE9}", -1, 44, None),
    Among("\u{FEEA}", -1, 44, None),
    Among("\u{FEEB}", -1, 44, None),
    Among("\u{FEEC}", -1, 44, None),
    Among("\u{FEED}", -1, 45, None),
    Among("\u{FEEE}", -1, 45, None),
    Among("\u{FEEF}", -1, 46, None),
    Among("\u{FEF0}", -1, 46, None),
    Among("\u{FEF1}", -1, 47, None),
    Among("\u{FEF2}", -1, 47, None),
    Among("\u{FEF3}", -1, 47, None),
    Among("\u{FEF4}", -1, 47, None),
    Among("\u{FEF5}", -1, 51, None),
    Among("\u{FEF6}", -1, 51, None),
    Among("\u{FEF7}", -1, 49, None),
    Among("\u{FEF8}", -1, 49, None),
    Among("\u{FEF9}", -1, 50, None),
    Among("\u{FEFA}", -1, 50, None),
    Among("\u{FEFB}", -1, 48, None),
    Among("\u{FEFC}", -1, 48, None),
];

static A_1: &'static [Among<Context>; 5] = &[
    Among("\u{0622}", -1, 1, None),
    Among("\u{0623}", -1, 1, None),
    Among("\u{0624}", -1, 1, None),
    Among("\u{0625}", -1, 1, None),
    Among("\u{0626}", -1, 1, None),
];

static A_2: &'static [Among<Context>; 5] = &[
    Among("\u{0622}", -1, 1, None),
    Among("\u{0623}", -1, 1, None),
    Among("\u{0624}", -1, 2, None),
    Among("\u{0625}", -1, 1, None),
    Among("\u{0626}", -1, 3, None),
];

static A_3: &'static [Among<Context>; 4] = &[
    Among("\u{0627}\u{0644}", -1, 2, None),
    Among("\u{0628}\u{0627}\u{0644}", -1, 1, None),
    Among("\u{0643}\u{0627}\u{0644}", -1, 1, None),
    Among("\u{0644}\u{0644}", -1, 2, None),
];

static A_4: &'static [Among<Context>; 5] = &[
    Among("\u{0623}\u{0622}", -1, 2, None),
    Among("\u{0623}\u{0623}", -1, 1, None),
    Among("\u{0623}\u{0624}", -1, 1, None),
    Among("\u{0623}\u{0625}", -1, 4, None),
    Among("\u{0623}\u{0627}", -1, 3, None),
];

static A_5: &'static [Among<Context>; 2] = &[
    Among("\u{0641}", -1, 1, None),
    Among("\u{0648}", -1, 1, None),
];

static A_6: &'static [Among<Context>; 4] = &[
    Among("\u{0627}\u{0644}", -1, 2, None),
    Among("\u{0628}\u{0627}\u{0644}", -1, 1, None),
    Among("\u{0643}\u{0627}\u{0644}", -1, 1, None),
    Among("\u{0644}\u{0644}", -1, 2, None),
];

static A_7: &'static [Among<Context>; 4] = &[
    Among("\u{0628}", -1, 1, None),
    Among("\u{0628}\u{0627}", 0, -1, None),
    Among("\u{0628}\u{0628}", 0, 2, None),
    Among("\u{0643}\u{0643}", -1, 3, None),
];

static A_8: &'static [Among<Context>; 4] = &[
    Among("\u{0633}\u{0623}", -1, 4, None),
    Among("\u{0633}\u{062A}", -1, 2, None),
    Among("\u{0633}\u{0646}", -1, 3, None),
    Among("\u{0633}\u{064A}", -1, 1, None),
];

static A_9: &'static [Among<Context>; 3] = &[
    Among("\u{062A}\u{0633}\u{062A}", -1, 1, None),
    Among("\u{0646}\u{0633}\u{062A}", -1, 1, None),
    Among("\u{064A}\u{0633}\u{062A}", -1, 1, None),
];

static A_10: &'static [Among<Context>; 10] = &[
    Among("\u{0643}", -1, 1, None),
    Among("\u{0643}\u{0645}", -1, 2, None),
    Among("\u{0647}\u{0645}", -1, 2, None),
    Among("\u{0647}\u{0646}", -1, 2, None),
    Among("\u{0647}", -1, 1, None),
    Among("\u{064A}", -1, 1, None),
    Among("\u{0643}\u{0645}\u{0627}", -1, 3, None),
    Among("\u{0647}\u{0645}\u{0627}", -1, 3, None),
    Among("\u{0646}\u{0627}", -1, 2, None),
    Among("\u{0647}\u{0627}", -1, 2, None),
];

static A_11: &'static [Among<Context>; 3] = &[
    Among("\u{0648}", -1, 1, None),
    Among("\u{064A}", -1, 1, None),
    Among("\u{0627}", -1, 1, None),
];

static A_12: &'static [Among<Context>; 12] = &[
    Among("\u{0643}", -1, 1, None),
    Among("\u{0643}\u{0645}", -1, 2, None),
    Among("\u{0647}\u{0645}", -1, 2, None),
    Among("\u{0643}\u{0646}", -1, 2, None),
    Among("\u{0647}\u{0646}", -1, 2, None),
    Among("\u{0647}", -1, 1, None),
    Among("\u{0643}\u{0645}\u{0648}", -1, 3, None),
    Among("\u{0646}\u{064A}", -1, 2, None),
    Among("\u{0643}\u{0645}\u{0627}", -1, 3, None),
    Among("\u{0647}\u{0645}\u{0627}", -1, 3, None),
    Among("\u{0646}\u{0627}", -1, 2, None),
    Among("\u{0647}\u{0627}", -1, 2, None),
];

static A_13: &'static [Among<Context>; 11] = &[
    Among("\u{0646}", -1, 1, None),
    Among("\u{0648}\u{0646}", 0, 3, None),
    Among("\u{064A}\u{0646}", 0, 3, None),
    Among("\u{0627}\u{0646}", 0, 3, None),
    Among("\u{062A}\u{0646}", 0, 2, None),
    Among("\u{064A}", -1, 1, None),
    Among("\u{0627}", -1, 1, None),
    Among("\u{062A}\u{0645}\u{0627}", 6, 4, None),
    Among("\u{0646}\u{0627}", 6, 2, None),
    Among("\u{062A}\u{0627}", 6, 2, None),
    Among("\u{062A}", -1, 1, None),
];

static A_14: &'static [Among<Context>; 2] = &[
    Among("\u{062A}\u{0645}", -1, 1, None),
    Among("\u{0648}\u{0627}", -1, 1, None),
];

static A_15: &'static [Among<Context>; 2] = &[
    Among("\u{0648}", -1, 1, None),
    Among("\u{062A}\u{0645}\u{0648}", 0, 2, None),
];

fn r_Normalize_pre(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    let v_1 = env.cursor;
    'lab0: loop {
        'replab1: loop{
            let v_2 = env.cursor;
            'lab2: for _ in 0..1 {
                'lab3: loop {
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
                                env.slice_del();
                            }
                            2 => {
                                env.slice_from("0");
                            }
                            3 => {
                                env.slice_from("1");
                            }
                            4 => {
                                env.slice_from("2");
                            }
                            5 => {
                                env.slice_from("3");
                            }
                            6 => {
                                env.slice_from("4");
                            }
                            7 => {
                                env.slice_from("5");
                            }
                            8 => {
                                env.slice_from("6");
                            }
                            9 => {
                                env.slice_from("7");
                            }
                            10 => {
                                env.slice_from("8");
                            }
                            11 => {
                                env.slice_from("9");
                            }
                            12 => {
                                env.slice_from("\u{0621}");
                            }
                            13 => {
                                env.slice_from("\u{0623}");
                            }
                            14 => {
                                env.slice_from("\u{0625}");
                            }
                            15 => {
                                env.slice_from("\u{0626}");
                            }
                            16 => {
                                env.slice_from("\u{0622}");
                            }
                            17 => {
                                env.slice_from("\u{0624}");
                            }
                            18 => {
                                env.slice_from("\u{0627}");
                            }
                            19 => {
                                env.slice_from("\u{0628}");
                            }
                            20 => {
                                env.slice_from("\u{0629}");
                            }
                            21 => {
                                env.slice_from("\u{062A}");
                            }
                            22 => {
                                env.slice_from("\u{062B}");
                            }
                            23 => {
                                env.slice_from("\u{062C}");
                            }
                            24 => {
                                env.slice_from("\u{062D}");
                            }
                            25 => {
                                env.slice_from("\u{062E}");
                            }
                            26 => {
                                env.slice_from("\u{062F}");
                            }
                            27 => {
                                env.slice_from("\u{0630}");
                            }
                            28 => {
                                env.slice_from("\u{0631}");
                            }
                            29 => {
                                env.slice_from("\u{0632}");
                            }
                            30 => {
                                env.slice_from("\u{0633}");
                            }
                            31 => {
                                env.slice_from("\u{0634}");
                            }
                            32 => {
                                env.slice_from("\u{0635}");
                            }
                            33 => {
                                env.slice_from("\u{0636}");
                            }
                            34 => {
                                env.slice_from("\u{0637}");
                            }
                            35 => {
                                env.slice_from("\u{0638}");
                            }
                            36 => {
                                env.slice_from("\u{0639}");
                            }
                            37 => {
                                env.slice_from("\u{063A}");
                            }
                            38 => {
                                env.slice_from("\u{0641}");
                            }
                            39 => {
                                env.slice_from("\u{0642}");
                            }
                            40 => {
                                env.slice_from("\u{0643}");
                            }
                            41 => {
                                env.slice_from("\u{0644}");
                            }
                            42 => {
                                env.slice_from("\u{0645}");
                            }
                            43 => {
                                env.slice_from("\u{0646}");
                            }
                            44 => {
                                env.slice_from("\u{0647}");
                            }
                            45 => {
                                env.slice_from("\u{0648}");
                            }
                            46 => {
                                env.slice_from("\u{0649}");
                            }
                            47 => {
                                env.slice_from("\u{064A}");
                            }
                            48 => {
                                env.slice_from("\u{0644}\u{0627}");
                            }
                            49 => {
                                env.slice_from("\u{0644}\u{0623}");
                            }
                            50 => {
                                env.slice_from("\u{0644}\u{0625}");
                            }
                            51 => {
                                env.slice_from("\u{0644}\u{0622}");
                            }
                            _ => ()
                        }
                        break 'lab3;
                    }
                    env.cursor = v_3;
                    if env.cursor >= env.limit {
                        break 'lab2;
                    }
                    env.next_char();
                    break 'lab3;
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

fn r_Normalize_post(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    let v_1 = env.cursor;
    'lab0: loop {
        env.limit_backward = env.cursor;
        env.cursor = env.limit;
        env.ket = env.cursor;
        if (env.cursor - 1 <= env.limit_backward || env.current.as_bytes()[(env.cursor - 1) as usize] as u8 >> 5 != 5 as u8 || ((124 as i32 >> (env.current.as_bytes()[(env.cursor - 1) as usize] as u8 & 0x1f)) & 1) == 0) {
            break 'lab0;
        }

        if env.find_among_b(A_1, context) == 0 {
            break 'lab0;
        }
        env.bra = env.cursor;
        env.slice_from("\u{0621}");
        env.cursor = env.limit_backward;
        break 'lab0;
    }
    env.cursor = v_1;
    let v_2 = env.cursor;
    'lab1: loop {
        'replab2: loop{
            let v_3 = env.cursor;
            'lab3: for _ in 0..1 {
                'lab4: loop {
                    let v_4 = env.cursor;
                    'lab5: loop {
                        env.bra = env.cursor;
                        if (env.cursor + 1 >= env.limit || env.current.as_bytes()[(env.cursor + 1) as usize] as u8 >> 5 != 5 as u8 || ((124 as i32 >> (env.current.as_bytes()[(env.cursor + 1) as usize] as u8 & 0x1f)) & 1) == 0) {
                            break 'lab5;
                        }

                        among_var = env.find_among(A_2, context);
                        if among_var == 0 {
                            break 'lab5;
                        }
                        env.ket = env.cursor;
                        match among_var {
                            1 => {
                                env.slice_from("\u{0627}");
                            }
                            2 => {
                                env.slice_from("\u{0648}");
                            }
                            3 => {
                                env.slice_from("\u{064A}");
                            }
                            _ => ()
                        }
                        break 'lab4;
                    }
                    env.cursor = v_4;
                    if env.cursor >= env.limit {
                        break 'lab3;
                    }
                    env.next_char();
                    break 'lab4;
                }
                continue 'replab2;
            }
            env.cursor = v_3;
            break 'replab2;
        }
        break 'lab1;
    }
    env.cursor = v_2;
    return true
}

fn r_Checks1(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    env.bra = env.cursor;
    if (env.cursor + 3 >= env.limit || (env.current.as_bytes()[(env.cursor + 3) as usize] as u8 != 132 as u8 && env.current.as_bytes()[(env.cursor + 3) as usize] as u8 != 167 as u8)) {
        return false;
    }

    among_var = env.find_among(A_3, context);
    if among_var == 0 {
        return false;
    }
    env.ket = env.cursor;
    match among_var {
        1 => {
            if (env.current.chars().count() as i32) <= 4{
                return false;
            }
            context.b_is_noun = true;
            context.b_is_verb = false;
            context.b_is_defined = true;
        }
        2 => {
            if (env.current.chars().count() as i32) <= 3{
                return false;
            }
            context.b_is_noun = true;
            context.b_is_verb = false;
            context.b_is_defined = true;
        }
        _ => ()
    }
    return true
}

fn r_Prefix_Step1(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    env.bra = env.cursor;
    if (env.cursor + 3 >= env.limit || env.current.as_bytes()[(env.cursor + 3) as usize] as u8 >> 5 != 5 as u8 || ((188 as i32 >> (env.current.as_bytes()[(env.cursor + 3) as usize] as u8 & 0x1f)) & 1) == 0) {
        return false;
    }

    among_var = env.find_among(A_4, context);
    if among_var == 0 {
        return false;
    }
    env.ket = env.cursor;
    match among_var {
        1 => {
            if (env.current.chars().count() as i32) <= 3{
                return false;
            }
            env.slice_from("\u{0623}");
        }
        2 => {
            if (env.current.chars().count() as i32) <= 3{
                return false;
            }
            env.slice_from("\u{0622}");
        }
        3 => {
            if (env.current.chars().count() as i32) <= 3{
                return false;
            }
            env.slice_from("\u{0627}");
        }
        4 => {
            if (env.current.chars().count() as i32) <= 3{
                return false;
            }
            env.slice_from("\u{0625}");
        }
        _ => ()
    }
    return true
}

fn r_Prefix_Step2(env: &mut SnowballEnv, context: &mut Context) -> bool {
    env.bra = env.cursor;
    if (env.cursor + 1 >= env.limit || (env.current.as_bytes()[(env.cursor + 1) as usize] as u8 != 129 as u8 && env.current.as_bytes()[(env.cursor + 1) as usize] as u8 != 136 as u8)) {
        return false;
    }

    if env.find_among(A_5, context) == 0 {
        return false;
    }
    env.ket = env.cursor;
    if (env.current.chars().count() as i32) <= 3{
        return false;
    }
    let v_1 = env.cursor;
    'lab0: loop {
        if !env.eq_s(&"\u{0627}") {
            break 'lab0;
        }
        return false;
    }
    env.cursor = v_1;
    env.slice_del();
    return true
}

fn r_Prefix_Step3a_Noun(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    env.bra = env.cursor;
    if (env.cursor + 3 >= env.limit || (env.current.as_bytes()[(env.cursor + 3) as usize] as u8 != 132 as u8 && env.current.as_bytes()[(env.cursor + 3) as usize] as u8 != 167 as u8)) {
        return false;
    }

    among_var = env.find_among(A_6, context);
    if among_var == 0 {
        return false;
    }
    env.ket = env.cursor;
    match among_var {
        1 => {
            if (env.current.chars().count() as i32) <= 5{
                return false;
            }
            env.slice_del();
        }
        2 => {
            if (env.current.chars().count() as i32) <= 4{
                return false;
            }
            env.slice_del();
        }
        _ => ()
    }
    return true
}

fn r_Prefix_Step3b_Noun(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    env.bra = env.cursor;
    if (env.cursor + 1 >= env.limit || (env.current.as_bytes()[(env.cursor + 1) as usize] as u8 != 168 as u8 && env.current.as_bytes()[(env.cursor + 1) as usize] as u8 != 131 as u8)) {
        return false;
    }

    among_var = env.find_among(A_7, context);
    if among_var == 0 {
        return false;
    }
    env.ket = env.cursor;
    match among_var {
        1 => {
            if (env.current.chars().count() as i32) <= 3{
                return false;
            }
            env.slice_del();
        }
        2 => {
            if (env.current.chars().count() as i32) <= 3{
                return false;
            }
            env.slice_from("\u{0628}");
        }
        3 => {
            if (env.current.chars().count() as i32) <= 3{
                return false;
            }
            env.slice_from("\u{0643}");
        }
        _ => ()
    }
    return true
}

fn r_Prefix_Step3_Verb(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    env.bra = env.cursor;
    among_var = env.find_among(A_8, context);
    if among_var == 0 {
        return false;
    }
    env.ket = env.cursor;
    match among_var {
        1 => {
            if (env.current.chars().count() as i32) <= 4{
                return false;
            }
            env.slice_from("\u{064A}");
        }
        2 => {
            if (env.current.chars().count() as i32) <= 4{
                return false;
            }
            env.slice_from("\u{062A}");
        }
        3 => {
            if (env.current.chars().count() as i32) <= 4{
                return false;
            }
            env.slice_from("\u{0646}");
        }
        4 => {
            if (env.current.chars().count() as i32) <= 4{
                return false;
            }
            env.slice_from("\u{0623}");
        }
        _ => ()
    }
    return true
}

fn r_Prefix_Step4_Verb(env: &mut SnowballEnv, context: &mut Context) -> bool {
    env.bra = env.cursor;
    if (env.cursor + 5 >= env.limit || env.current.as_bytes()[(env.cursor + 5) as usize] as u8 != 170 as u8) {
        return false;
    }

    if env.find_among(A_9, context) == 0 {
        return false;
    }
    env.ket = env.cursor;
    if (env.current.chars().count() as i32) <= 4{
        return false;
    }
    context.b_is_verb = true;
    context.b_is_noun = false;
    env.slice_from("\u{0627}\u{0633}\u{062A}");
    return true
}

fn r_Suffix_Noun_Step1a(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    env.ket = env.cursor;
    among_var = env.find_among_b(A_10, context);
    if among_var == 0 {
        return false;
    }
    env.bra = env.cursor;
    match among_var {
        1 => {
            if (env.current.chars().count() as i32) < 4{
                return false;
            }
            env.slice_del();
        }
        2 => {
            if (env.current.chars().count() as i32) < 5{
                return false;
            }
            env.slice_del();
        }
        3 => {
            if (env.current.chars().count() as i32) < 6{
                return false;
            }
            env.slice_del();
        }
        _ => ()
    }
    return true
}

fn r_Suffix_Noun_Step1b(env: &mut SnowballEnv, context: &mut Context) -> bool {
    env.ket = env.cursor;
    if !env.eq_s_b(&"\u{0646}") {
        return false;
    }
    env.bra = env.cursor;
    if (env.current.chars().count() as i32) <= 5{
        return false;
    }
    env.slice_del();
    return true
}

fn r_Suffix_Noun_Step2a(env: &mut SnowballEnv, context: &mut Context) -> bool {
    env.ket = env.cursor;
    if env.find_among_b(A_11, context) == 0 {
        return false;
    }
    env.bra = env.cursor;
    if (env.current.chars().count() as i32) <= 4{
        return false;
    }
    env.slice_del();
    return true
}

fn r_Suffix_Noun_Step2b(env: &mut SnowballEnv, context: &mut Context) -> bool {
    env.ket = env.cursor;
    if !env.eq_s_b(&"\u{0627}\u{062A}") {
        return false;
    }
    env.bra = env.cursor;
    if (env.current.chars().count() as i32) < 5{
        return false;
    }
    env.slice_del();
    return true
}

fn r_Suffix_Noun_Step2c1(env: &mut SnowballEnv, context: &mut Context) -> bool {
    env.ket = env.cursor;
    if !env.eq_s_b(&"\u{062A}") {
        return false;
    }
    env.bra = env.cursor;
    if (env.current.chars().count() as i32) < 4{
        return false;
    }
    env.slice_del();
    return true
}

fn r_Suffix_Noun_Step2c2(env: &mut SnowballEnv, context: &mut Context) -> bool {
    env.ket = env.cursor;
    if !env.eq_s_b(&"\u{0629}") {
        return false;
    }
    env.bra = env.cursor;
    if (env.current.chars().count() as i32) < 4{
        return false;
    }
    env.slice_del();
    return true
}

fn r_Suffix_Noun_Step3(env: &mut SnowballEnv, context: &mut Context) -> bool {
    env.ket = env.cursor;
    if !env.eq_s_b(&"\u{064A}") {
        return false;
    }
    env.bra = env.cursor;
    if (env.current.chars().count() as i32) < 3{
        return false;
    }
    env.slice_del();
    return true
}

fn r_Suffix_Verb_Step1(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    env.ket = env.cursor;
    among_var = env.find_among_b(A_12, context);
    if among_var == 0 {
        return false;
    }
    env.bra = env.cursor;
    match among_var {
        1 => {
            if (env.current.chars().count() as i32) < 4{
                return false;
            }
            env.slice_del();
        }
        2 => {
            if (env.current.chars().count() as i32) < 5{
                return false;
            }
            env.slice_del();
        }
        3 => {
            if (env.current.chars().count() as i32) < 6{
                return false;
            }
            env.slice_del();
        }
        _ => ()
    }
    return true
}

fn r_Suffix_Verb_Step2a(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    env.ket = env.cursor;
    among_var = env.find_among_b(A_13, context);
    if among_var == 0 {
        return false;
    }
    env.bra = env.cursor;
    match among_var {
        1 => {
            if (env.current.chars().count() as i32) < 4{
                return false;
            }
            env.slice_del();
        }
        2 => {
            if (env.current.chars().count() as i32) < 5{
                return false;
            }
            env.slice_del();
        }
        3 => {
            if (env.current.chars().count() as i32) <= 5{
                return false;
            }
            env.slice_del();
        }
        4 => {
            if (env.current.chars().count() as i32) < 6{
                return false;
            }
            env.slice_del();
        }
        _ => ()
    }
    return true
}

fn r_Suffix_Verb_Step2b(env: &mut SnowballEnv, context: &mut Context) -> bool {
    env.ket = env.cursor;
    if (env.cursor - 3 <= env.limit_backward || (env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 133 as u8 && env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 167 as u8)) {
        return false;
    }

    if env.find_among_b(A_14, context) == 0 {
        return false;
    }
    env.bra = env.cursor;
    if (env.current.chars().count() as i32) < 5{
        return false;
    }
    env.slice_del();
    return true
}

fn r_Suffix_Verb_Step2c(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    env.ket = env.cursor;
    if (env.cursor - 1 <= env.limit_backward || env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 136 as u8) {
        return false;
    }

    among_var = env.find_among_b(A_15, context);
    if among_var == 0 {
        return false;
    }
    env.bra = env.cursor;
    match among_var {
        1 => {
            if (env.current.chars().count() as i32) < 4{
                return false;
            }
            env.slice_del();
        }
        2 => {
            if (env.current.chars().count() as i32) < 6{
                return false;
            }
            env.slice_del();
        }
        _ => ()
    }
    return true
}

fn r_Suffix_All_alef_maqsura(env: &mut SnowballEnv, context: &mut Context) -> bool {
    env.ket = env.cursor;
    if !env.eq_s_b(&"\u{0649}") {
        return false;
    }
    env.bra = env.cursor;
    env.slice_from("\u{064A}");
    return true
}

pub fn stem(env: &mut SnowballEnv) -> bool {
    let mut context = &mut Context {
        b_is_defined: false,
        b_is_verb: false,
        b_is_noun: false,
    };
    context.b_is_noun = true;
    context.b_is_verb = true;
    context.b_is_defined = false;
    let v_1 = env.cursor;
    r_Checks1(env, context);
    env.cursor = v_1;
    r_Normalize_pre(env, context);
    env.limit_backward = env.cursor;
    env.cursor = env.limit;
    let v_2 = env.limit - env.cursor;
    'lab0: loop {
        'lab1: loop {
            let v_3 = env.limit - env.cursor;
            'lab2: loop {
                if !context.b_is_verb {
                    break 'lab2;
                }
                'lab3: loop {
                    let v_4 = env.limit - env.cursor;
                    'lab4: loop {
                        let mut v_5 = 1;
                        'replab5: loop{
                            let v_6 = env.limit - env.cursor;
                            'lab6: for _ in 0..1 {
                                if !r_Suffix_Verb_Step1(env, context) {
                                    break 'lab6;
                                }
                                v_5 -= 1;
                                continue 'replab5;
                            }
                            env.cursor = env.limit - v_6;
                            break 'replab5;
                        }
                        if v_5 > 0 {
                            break 'lab4;
                        }
                        'lab7: loop {
                            let v_7 = env.limit - env.cursor;
                            'lab8: loop {
                                if !r_Suffix_Verb_Step2a(env, context) {
                                    break 'lab8;
                                }
                                break 'lab7;
                            }
                            env.cursor = env.limit - v_7;
                            'lab9: loop {
                                if !r_Suffix_Verb_Step2c(env, context) {
                                    break 'lab9;
                                }
                                break 'lab7;
                            }
                            env.cursor = env.limit - v_7;
                            if env.cursor <= env.limit_backward {
                                break 'lab4;
                            }
                            env.previous_char();
                            break 'lab7;
                        }
                        break 'lab3;
                    }
                    env.cursor = env.limit - v_4;
                    'lab10: loop {
                        if !r_Suffix_Verb_Step2b(env, context) {
                            break 'lab10;
                        }
                        break 'lab3;
                    }
                    env.cursor = env.limit - v_4;
                    if !r_Suffix_Verb_Step2a(env, context) {
                        break 'lab2;
                    }
                    break 'lab3;
                }
                break 'lab1;
            }
            env.cursor = env.limit - v_3;
            'lab11: loop {
                if !context.b_is_noun {
                    break 'lab11;
                }
                let v_8 = env.limit - env.cursor;
                'lab12: loop {
                    'lab13: loop {
                        let v_9 = env.limit - env.cursor;
                        'lab14: loop {
                            if !r_Suffix_Noun_Step2c2(env, context) {
                                break 'lab14;
                            }
                            break 'lab13;
                        }
                        env.cursor = env.limit - v_9;
                        'lab15: loop {
                            if context.b_is_defined {
                                break 'lab15;
                            }
                            if !r_Suffix_Noun_Step1a(env, context) {
                                break 'lab15;
                            }
                            'lab16: loop {
                                let v_10 = env.limit - env.cursor;
                                'lab17: loop {
                                    if !r_Suffix_Noun_Step2a(env, context) {
                                        break 'lab17;
                                    }
                                    break 'lab16;
                                }
                                env.cursor = env.limit - v_10;
                                'lab18: loop {
                                    if !r_Suffix_Noun_Step2b(env, context) {
                                        break 'lab18;
                                    }
                                    break 'lab16;
                                }
                                env.cursor = env.limit - v_10;
                                'lab19: loop {
                                    if !r_Suffix_Noun_Step2c1(env, context) {
                                        break 'lab19;
                                    }
                                    break 'lab16;
                                }
                                env.cursor = env.limit - v_10;
                                if env.cursor <= env.limit_backward {
                                    break 'lab15;
                                }
                                env.previous_char();
                                break 'lab16;
                            }
                            break 'lab13;
                        }
                        env.cursor = env.limit - v_9;
                        'lab20: loop {
                            if !r_Suffix_Noun_Step1b(env, context) {
                                break 'lab20;
                            }
                            'lab21: loop {
                                let v_11 = env.limit - env.cursor;
                                'lab22: loop {
                                    if !r_Suffix_Noun_Step2a(env, context) {
                                        break 'lab22;
                                    }
                                    break 'lab21;
                                }
                                env.cursor = env.limit - v_11;
                                'lab23: loop {
                                    if !r_Suffix_Noun_Step2b(env, context) {
                                        break 'lab23;
                                    }
                                    break 'lab21;
                                }
                                env.cursor = env.limit - v_11;
                                if !r_Suffix_Noun_Step2c1(env, context) {
                                    break 'lab20;
                                }
                                break 'lab21;
                            }
                            break 'lab13;
                        }
                        env.cursor = env.limit - v_9;
                        'lab24: loop {
                            if context.b_is_defined {
                                break 'lab24;
                            }
                            if !r_Suffix_Noun_Step2a(env, context) {
                                break 'lab24;
                            }
                            break 'lab13;
                        }
                        env.cursor = env.limit - v_9;
                        if !r_Suffix_Noun_Step2b(env, context) {
                            env.cursor = env.limit - v_8;
                            break 'lab12;
                        }
                        break 'lab13;
                    }
                    break 'lab12;
                }
                if !r_Suffix_Noun_Step3(env, context) {
                    break 'lab11;
                }
                break 'lab1;
            }
            env.cursor = env.limit - v_3;
            if !r_Suffix_All_alef_maqsura(env, context) {
                break 'lab0;
            }
            break 'lab1;
        }
        break 'lab0;
    }
    env.cursor = env.limit - v_2;
    env.cursor = env.limit_backward;
    let v_12 = env.cursor;
    'lab25: loop {
        let v_13 = env.cursor;
        'lab26: loop {
            if !r_Prefix_Step1(env, context) {
                env.cursor = v_13;
                break 'lab26;
            }
            break 'lab26;
        }
        let v_14 = env.cursor;
        'lab27: loop {
            if !r_Prefix_Step2(env, context) {
                env.cursor = v_14;
                break 'lab27;
            }
            break 'lab27;
        }
        'lab28: loop {
            let v_15 = env.cursor;
            'lab29: loop {
                if !r_Prefix_Step3a_Noun(env, context) {
                    break 'lab29;
                }
                break 'lab28;
            }
            env.cursor = v_15;
            'lab30: loop {
                if !context.b_is_noun {
                    break 'lab30;
                }
                if !r_Prefix_Step3b_Noun(env, context) {
                    break 'lab30;
                }
                break 'lab28;
            }
            env.cursor = v_15;
            if !context.b_is_verb {
                break 'lab25;
            }
            let v_16 = env.cursor;
            'lab31: loop {
                if !r_Prefix_Step3_Verb(env, context) {
                    env.cursor = v_16;
                    break 'lab31;
                }
                break 'lab31;
            }
            if !r_Prefix_Step4_Verb(env, context) {
                break 'lab25;
            }
            break 'lab28;
        }
        break 'lab25;
    }
    env.cursor = v_12;
    r_Normalize_post(env, context);
    return true
}
