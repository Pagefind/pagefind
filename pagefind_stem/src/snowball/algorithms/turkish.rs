//! Generated from turkish.sbl by Snowball 3.0.0 - https://snowballstem.org/

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_mut)]
#![allow(unused_parens)]
#![allow(unused_variables)]
use snowball::SnowballEnv;
use snowball::Among;

#[derive(Clone)]
struct Context {
    b_continue_stemming_noun_suffixes: bool,
}

static A_0: &'static [Among<Context>; 10] = &[
    Among("m", -1, -1, None),
    Among("n", -1, -1, None),
    Among("miz", -1, -1, None),
    Among("niz", -1, -1, None),
    Among("muz", -1, -1, None),
    Among("nuz", -1, -1, None),
    Among("mız", -1, -1, None),
    Among("nız", -1, -1, None),
    Among("müz", -1, -1, None),
    Among("nüz", -1, -1, None),
];

static A_1: &'static [Among<Context>; 2] = &[
    Among("leri", -1, -1, None),
    Among("ları", -1, -1, None),
];

static A_2: &'static [Among<Context>; 4] = &[
    Among("ni", -1, -1, None),
    Among("nu", -1, -1, None),
    Among("nı", -1, -1, None),
    Among("nü", -1, -1, None),
];

static A_3: &'static [Among<Context>; 4] = &[
    Among("in", -1, -1, None),
    Among("un", -1, -1, None),
    Among("ın", -1, -1, None),
    Among("ün", -1, -1, None),
];

static A_5: &'static [Among<Context>; 2] = &[
    Among("na", -1, -1, None),
    Among("ne", -1, -1, None),
];

static A_6: &'static [Among<Context>; 4] = &[
    Among("da", -1, -1, None),
    Among("ta", -1, -1, None),
    Among("de", -1, -1, None),
    Among("te", -1, -1, None),
];

static A_7: &'static [Among<Context>; 2] = &[
    Among("nda", -1, -1, None),
    Among("nde", -1, -1, None),
];

static A_8: &'static [Among<Context>; 4] = &[
    Among("dan", -1, -1, None),
    Among("tan", -1, -1, None),
    Among("den", -1, -1, None),
    Among("ten", -1, -1, None),
];

static A_9: &'static [Among<Context>; 2] = &[
    Among("ndan", -1, -1, None),
    Among("nden", -1, -1, None),
];

static A_10: &'static [Among<Context>; 2] = &[
    Among("la", -1, -1, None),
    Among("le", -1, -1, None),
];

static A_11: &'static [Among<Context>; 2] = &[
    Among("ca", -1, -1, None),
    Among("ce", -1, -1, None),
];

static A_12: &'static [Among<Context>; 4] = &[
    Among("im", -1, -1, None),
    Among("um", -1, -1, None),
    Among("ım", -1, -1, None),
    Among("üm", -1, -1, None),
];

static A_13: &'static [Among<Context>; 4] = &[
    Among("sin", -1, -1, None),
    Among("sun", -1, -1, None),
    Among("sın", -1, -1, None),
    Among("sün", -1, -1, None),
];

static A_14: &'static [Among<Context>; 4] = &[
    Among("iz", -1, -1, None),
    Among("uz", -1, -1, None),
    Among("ız", -1, -1, None),
    Among("üz", -1, -1, None),
];

static A_15: &'static [Among<Context>; 4] = &[
    Among("siniz", -1, -1, None),
    Among("sunuz", -1, -1, None),
    Among("sınız", -1, -1, None),
    Among("sünüz", -1, -1, None),
];

static A_16: &'static [Among<Context>; 2] = &[
    Among("lar", -1, -1, None),
    Among("ler", -1, -1, None),
];

static A_17: &'static [Among<Context>; 4] = &[
    Among("niz", -1, -1, None),
    Among("nuz", -1, -1, None),
    Among("nız", -1, -1, None),
    Among("nüz", -1, -1, None),
];

static A_18: &'static [Among<Context>; 8] = &[
    Among("dir", -1, -1, None),
    Among("tir", -1, -1, None),
    Among("dur", -1, -1, None),
    Among("tur", -1, -1, None),
    Among("dır", -1, -1, None),
    Among("tır", -1, -1, None),
    Among("dür", -1, -1, None),
    Among("tür", -1, -1, None),
];

static A_19: &'static [Among<Context>; 2] = &[
    Among("casına", -1, -1, None),
    Among("cesine", -1, -1, None),
];

static A_20: &'static [Among<Context>; 32] = &[
    Among("di", -1, -1, None),
    Among("ti", -1, -1, None),
    Among("dik", -1, -1, None),
    Among("tik", -1, -1, None),
    Among("duk", -1, -1, None),
    Among("tuk", -1, -1, None),
    Among("dık", -1, -1, None),
    Among("tık", -1, -1, None),
    Among("dük", -1, -1, None),
    Among("tük", -1, -1, None),
    Among("dim", -1, -1, None),
    Among("tim", -1, -1, None),
    Among("dum", -1, -1, None),
    Among("tum", -1, -1, None),
    Among("dım", -1, -1, None),
    Among("tım", -1, -1, None),
    Among("düm", -1, -1, None),
    Among("tüm", -1, -1, None),
    Among("din", -1, -1, None),
    Among("tin", -1, -1, None),
    Among("dun", -1, -1, None),
    Among("tun", -1, -1, None),
    Among("dın", -1, -1, None),
    Among("tın", -1, -1, None),
    Among("dün", -1, -1, None),
    Among("tün", -1, -1, None),
    Among("du", -1, -1, None),
    Among("tu", -1, -1, None),
    Among("dı", -1, -1, None),
    Among("tı", -1, -1, None),
    Among("dü", -1, -1, None),
    Among("tü", -1, -1, None),
];

static A_21: &'static [Among<Context>; 8] = &[
    Among("sa", -1, -1, None),
    Among("se", -1, -1, None),
    Among("sak", -1, -1, None),
    Among("sek", -1, -1, None),
    Among("sam", -1, -1, None),
    Among("sem", -1, -1, None),
    Among("san", -1, -1, None),
    Among("sen", -1, -1, None),
];

static A_22: &'static [Among<Context>; 4] = &[
    Among("miş", -1, -1, None),
    Among("muş", -1, -1, None),
    Among("mış", -1, -1, None),
    Among("müş", -1, -1, None),
];

static A_23: &'static [Among<Context>; 4] = &[
    Among("b", -1, 1, None),
    Among("c", -1, 2, None),
    Among("d", -1, 3, None),
    Among("ğ", -1, 4, None),
];

static G_vowel: &'static [u8; 27] = &[17, 65, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 8, 0, 0, 0, 0, 0, 0, 1];

static G_U: &'static [u8; 26] = &[1, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 1];

static G_vowel1: &'static [u8; 27] = &[1, 64, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1];

static G_vowel2: &'static [u8; 19] = &[17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 130];

static G_vowel3: &'static [u8; 27] = &[1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1];

static G_vowel4: &'static [u8; 1] = &[17];

static G_vowel5: &'static [u8; 1] = &[65];

static G_vowel6: &'static [u8; 1] = &[65];

fn r_check_vowel_harmony(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let v_1 = env.limit - env.cursor;
    if !env.go_out_grouping_b(G_vowel, 97, 305) {
        return false;
    }
    'lab0: loop {
        let v_2 = env.limit - env.cursor;
        'lab1: loop {
            if !env.eq_s_b(&"a") {
                break 'lab1;
            }
            if !env.go_out_grouping_b(G_vowel1, 97, 305) {
                break 'lab1;
            }
            break 'lab0;
        }
        env.cursor = env.limit - v_2;
        'lab2: loop {
            if !env.eq_s_b(&"e") {
                break 'lab2;
            }
            if !env.go_out_grouping_b(G_vowel2, 101, 252) {
                break 'lab2;
            }
            break 'lab0;
        }
        env.cursor = env.limit - v_2;
        'lab3: loop {
            if !env.eq_s_b(&"ı") {
                break 'lab3;
            }
            if !env.go_out_grouping_b(G_vowel3, 97, 305) {
                break 'lab3;
            }
            break 'lab0;
        }
        env.cursor = env.limit - v_2;
        'lab4: loop {
            if !env.eq_s_b(&"i") {
                break 'lab4;
            }
            if !env.go_out_grouping_b(G_vowel4, 101, 105) {
                break 'lab4;
            }
            break 'lab0;
        }
        env.cursor = env.limit - v_2;
        'lab5: loop {
            if !env.eq_s_b(&"o") {
                break 'lab5;
            }
            if !env.go_out_grouping_b(G_vowel5, 111, 117) {
                break 'lab5;
            }
            break 'lab0;
        }
        env.cursor = env.limit - v_2;
        'lab6: loop {
            if !env.eq_s_b(&"ö") {
                break 'lab6;
            }
            if !env.go_out_grouping_b(G_vowel6, 246, 252) {
                break 'lab6;
            }
            break 'lab0;
        }
        env.cursor = env.limit - v_2;
        'lab7: loop {
            if !env.eq_s_b(&"u") {
                break 'lab7;
            }
            if !env.go_out_grouping_b(G_vowel5, 111, 117) {
                break 'lab7;
            }
            break 'lab0;
        }
        env.cursor = env.limit - v_2;
        if !env.eq_s_b(&"ü") {
            return false;
        }
        if !env.go_out_grouping_b(G_vowel6, 246, 252) {
            return false;
        }
        break 'lab0;
    }
    env.cursor = env.limit - v_1;
    return true
}

fn r_mark_suffix_with_optional_n_consonant(env: &mut SnowballEnv, context: &mut Context) -> bool {
    'lab0: loop {
        let v_1 = env.limit - env.cursor;
        'lab1: loop {
            if !env.eq_s_b(&"n") {
                break 'lab1;
            }
            let v_2 = env.limit - env.cursor;
            if !env.in_grouping_b(G_vowel, 97, 305) {
                break 'lab1;
            }
            env.cursor = env.limit - v_2;
            break 'lab0;
        }
        env.cursor = env.limit - v_1;
        let v_3 = env.limit - env.cursor;
        'lab2: loop {
            if !env.eq_s_b(&"n") {
                break 'lab2;
            }
            return false;
        }
        env.cursor = env.limit - v_3;
        let v_4 = env.limit - env.cursor;
        if env.cursor <= env.limit_backward {
            return false;
        }
        env.previous_char();
        if !env.in_grouping_b(G_vowel, 97, 305) {
            return false;
        }
        env.cursor = env.limit - v_4;
        break 'lab0;
    }
    return true
}

fn r_mark_suffix_with_optional_s_consonant(env: &mut SnowballEnv, context: &mut Context) -> bool {
    'lab0: loop {
        let v_1 = env.limit - env.cursor;
        'lab1: loop {
            if !env.eq_s_b(&"s") {
                break 'lab1;
            }
            let v_2 = env.limit - env.cursor;
            if !env.in_grouping_b(G_vowel, 97, 305) {
                break 'lab1;
            }
            env.cursor = env.limit - v_2;
            break 'lab0;
        }
        env.cursor = env.limit - v_1;
        let v_3 = env.limit - env.cursor;
        'lab2: loop {
            if !env.eq_s_b(&"s") {
                break 'lab2;
            }
            return false;
        }
        env.cursor = env.limit - v_3;
        let v_4 = env.limit - env.cursor;
        if env.cursor <= env.limit_backward {
            return false;
        }
        env.previous_char();
        if !env.in_grouping_b(G_vowel, 97, 305) {
            return false;
        }
        env.cursor = env.limit - v_4;
        break 'lab0;
    }
    return true
}

fn r_mark_suffix_with_optional_y_consonant(env: &mut SnowballEnv, context: &mut Context) -> bool {
    'lab0: loop {
        let v_1 = env.limit - env.cursor;
        'lab1: loop {
            if !env.eq_s_b(&"y") {
                break 'lab1;
            }
            let v_2 = env.limit - env.cursor;
            if !env.in_grouping_b(G_vowel, 97, 305) {
                break 'lab1;
            }
            env.cursor = env.limit - v_2;
            break 'lab0;
        }
        env.cursor = env.limit - v_1;
        let v_3 = env.limit - env.cursor;
        'lab2: loop {
            if !env.eq_s_b(&"y") {
                break 'lab2;
            }
            return false;
        }
        env.cursor = env.limit - v_3;
        let v_4 = env.limit - env.cursor;
        if env.cursor <= env.limit_backward {
            return false;
        }
        env.previous_char();
        if !env.in_grouping_b(G_vowel, 97, 305) {
            return false;
        }
        env.cursor = env.limit - v_4;
        break 'lab0;
    }
    return true
}

fn r_mark_suffix_with_optional_U_vowel(env: &mut SnowballEnv, context: &mut Context) -> bool {
    'lab0: loop {
        let v_1 = env.limit - env.cursor;
        'lab1: loop {
            if !env.in_grouping_b(G_U, 105, 305) {
                break 'lab1;
            }
            let v_2 = env.limit - env.cursor;
            if !env.out_grouping_b(G_vowel, 97, 305) {
                break 'lab1;
            }
            env.cursor = env.limit - v_2;
            break 'lab0;
        }
        env.cursor = env.limit - v_1;
        let v_3 = env.limit - env.cursor;
        'lab2: loop {
            if !env.in_grouping_b(G_U, 105, 305) {
                break 'lab2;
            }
            return false;
        }
        env.cursor = env.limit - v_3;
        let v_4 = env.limit - env.cursor;
        if env.cursor <= env.limit_backward {
            return false;
        }
        env.previous_char();
        if !env.out_grouping_b(G_vowel, 97, 305) {
            return false;
        }
        env.cursor = env.limit - v_4;
        break 'lab0;
    }
    return true
}

fn r_mark_possessives(env: &mut SnowballEnv, context: &mut Context) -> bool {
    if (env.cursor <= env.limit_backward || env.current.as_bytes()[(env.cursor - 1) as usize] as u8 >> 5 != 3 as u8 || ((67133440 as i32 >> (env.current.as_bytes()[(env.cursor - 1) as usize] as u8 & 0x1f)) & 1) == 0) {
        return false;
    }

    if env.find_among_b(A_0, context) == 0 {
        return false;
    }
    return r_mark_suffix_with_optional_U_vowel(env, context);
}

fn r_mark_sU(env: &mut SnowballEnv, context: &mut Context) -> bool {
    if !r_check_vowel_harmony(env, context) {
        return false;
    }
    if !env.in_grouping_b(G_U, 105, 305) {
        return false;
    }
    return r_mark_suffix_with_optional_s_consonant(env, context);
}

fn r_mark_lArI(env: &mut SnowballEnv, context: &mut Context) -> bool {
    if (env.cursor - 3 <= env.limit_backward || (env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 105 as u8 && env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 177 as u8)) {
        return false;
    }

    return env.find_among_b(A_1, context) != 0;
}

fn r_mark_yU(env: &mut SnowballEnv, context: &mut Context) -> bool {
    if !r_check_vowel_harmony(env, context) {
        return false;
    }
    if !env.in_grouping_b(G_U, 105, 305) {
        return false;
    }
    return r_mark_suffix_with_optional_y_consonant(env, context);
}

fn r_mark_nU(env: &mut SnowballEnv, context: &mut Context) -> bool {
    if !r_check_vowel_harmony(env, context) {
        return false;
    }
    return env.find_among_b(A_2, context) != 0;
}

fn r_mark_nUn(env: &mut SnowballEnv, context: &mut Context) -> bool {
    if !r_check_vowel_harmony(env, context) {
        return false;
    }
    if (env.cursor - 1 <= env.limit_backward || env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 110 as u8) {
        return false;
    }

    if env.find_among_b(A_3, context) == 0 {
        return false;
    }
    return r_mark_suffix_with_optional_n_consonant(env, context);
}

fn r_mark_yA(env: &mut SnowballEnv, context: &mut Context) -> bool {
    if !r_check_vowel_harmony(env, context) {
        return false;
    }
    if (env.cursor <= env.limit_backward || (env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 97 as u8 && env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 101 as u8)) {
        return false;
    }

    env.cursor -= 1;
    return r_mark_suffix_with_optional_y_consonant(env, context);
}

fn r_mark_nA(env: &mut SnowballEnv, context: &mut Context) -> bool {
    if !r_check_vowel_harmony(env, context) {
        return false;
    }
    if (env.cursor - 1 <= env.limit_backward || (env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 97 as u8 && env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 101 as u8)) {
        return false;
    }

    return env.find_among_b(A_5, context) != 0;
}

fn r_mark_DA(env: &mut SnowballEnv, context: &mut Context) -> bool {
    if !r_check_vowel_harmony(env, context) {
        return false;
    }
    if (env.cursor - 1 <= env.limit_backward || (env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 97 as u8 && env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 101 as u8)) {
        return false;
    }

    return env.find_among_b(A_6, context) != 0;
}

fn r_mark_ndA(env: &mut SnowballEnv, context: &mut Context) -> bool {
    if !r_check_vowel_harmony(env, context) {
        return false;
    }
    if (env.cursor - 2 <= env.limit_backward || (env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 97 as u8 && env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 101 as u8)) {
        return false;
    }

    return env.find_among_b(A_7, context) != 0;
}

fn r_mark_DAn(env: &mut SnowballEnv, context: &mut Context) -> bool {
    if !r_check_vowel_harmony(env, context) {
        return false;
    }
    if (env.cursor - 2 <= env.limit_backward || env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 110 as u8) {
        return false;
    }

    return env.find_among_b(A_8, context) != 0;
}

fn r_mark_ndAn(env: &mut SnowballEnv, context: &mut Context) -> bool {
    if !r_check_vowel_harmony(env, context) {
        return false;
    }
    if (env.cursor - 3 <= env.limit_backward || env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 110 as u8) {
        return false;
    }

    return env.find_among_b(A_9, context) != 0;
}

fn r_mark_ylA(env: &mut SnowballEnv, context: &mut Context) -> bool {
    if !r_check_vowel_harmony(env, context) {
        return false;
    }
    if (env.cursor - 1 <= env.limit_backward || (env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 97 as u8 && env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 101 as u8)) {
        return false;
    }

    if env.find_among_b(A_10, context) == 0 {
        return false;
    }
    return r_mark_suffix_with_optional_y_consonant(env, context);
}

fn r_mark_ki(env: &mut SnowballEnv, context: &mut Context) -> bool {
    return env.eq_s_b(&"ki");
}

fn r_mark_ncA(env: &mut SnowballEnv, context: &mut Context) -> bool {
    if !r_check_vowel_harmony(env, context) {
        return false;
    }
    if (env.cursor - 1 <= env.limit_backward || (env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 97 as u8 && env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 101 as u8)) {
        return false;
    }

    if env.find_among_b(A_11, context) == 0 {
        return false;
    }
    return r_mark_suffix_with_optional_n_consonant(env, context);
}

fn r_mark_yUm(env: &mut SnowballEnv, context: &mut Context) -> bool {
    if !r_check_vowel_harmony(env, context) {
        return false;
    }
    if (env.cursor - 1 <= env.limit_backward || env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 109 as u8) {
        return false;
    }

    if env.find_among_b(A_12, context) == 0 {
        return false;
    }
    return r_mark_suffix_with_optional_y_consonant(env, context);
}

fn r_mark_sUn(env: &mut SnowballEnv, context: &mut Context) -> bool {
    if !r_check_vowel_harmony(env, context) {
        return false;
    }
    if (env.cursor - 2 <= env.limit_backward || env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 110 as u8) {
        return false;
    }

    return env.find_among_b(A_13, context) != 0;
}

fn r_mark_yUz(env: &mut SnowballEnv, context: &mut Context) -> bool {
    if !r_check_vowel_harmony(env, context) {
        return false;
    }
    if (env.cursor - 1 <= env.limit_backward || env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 122 as u8) {
        return false;
    }

    if env.find_among_b(A_14, context) == 0 {
        return false;
    }
    return r_mark_suffix_with_optional_y_consonant(env, context);
}

fn r_mark_sUnUz(env: &mut SnowballEnv, context: &mut Context) -> bool {
    if (env.cursor - 4 <= env.limit_backward || env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 122 as u8) {
        return false;
    }

    return env.find_among_b(A_15, context) != 0;
}

fn r_mark_lAr(env: &mut SnowballEnv, context: &mut Context) -> bool {
    if !r_check_vowel_harmony(env, context) {
        return false;
    }
    if (env.cursor - 2 <= env.limit_backward || env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 114 as u8) {
        return false;
    }

    return env.find_among_b(A_16, context) != 0;
}

fn r_mark_nUz(env: &mut SnowballEnv, context: &mut Context) -> bool {
    if !r_check_vowel_harmony(env, context) {
        return false;
    }
    if (env.cursor - 2 <= env.limit_backward || env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 122 as u8) {
        return false;
    }

    return env.find_among_b(A_17, context) != 0;
}

fn r_mark_DUr(env: &mut SnowballEnv, context: &mut Context) -> bool {
    if !r_check_vowel_harmony(env, context) {
        return false;
    }
    if (env.cursor - 2 <= env.limit_backward || env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 114 as u8) {
        return false;
    }

    return env.find_among_b(A_18, context) != 0;
}

fn r_mark_cAsInA(env: &mut SnowballEnv, context: &mut Context) -> bool {
    if (env.cursor - 5 <= env.limit_backward || (env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 97 as u8 && env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 101 as u8)) {
        return false;
    }

    return env.find_among_b(A_19, context) != 0;
}

fn r_mark_yDU(env: &mut SnowballEnv, context: &mut Context) -> bool {
    if !r_check_vowel_harmony(env, context) {
        return false;
    }
    if env.find_among_b(A_20, context) == 0 {
        return false;
    }
    return r_mark_suffix_with_optional_y_consonant(env, context);
}

fn r_mark_ysA(env: &mut SnowballEnv, context: &mut Context) -> bool {
    if (env.cursor - 1 <= env.limit_backward || env.current.as_bytes()[(env.cursor - 1) as usize] as u8 >> 5 != 3 as u8 || ((26658 as i32 >> (env.current.as_bytes()[(env.cursor - 1) as usize] as u8 & 0x1f)) & 1) == 0) {
        return false;
    }

    if env.find_among_b(A_21, context) == 0 {
        return false;
    }
    return r_mark_suffix_with_optional_y_consonant(env, context);
}

fn r_mark_ymUs_(env: &mut SnowballEnv, context: &mut Context) -> bool {
    if !r_check_vowel_harmony(env, context) {
        return false;
    }
    if (env.cursor - 3 <= env.limit_backward || env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 159 as u8) {
        return false;
    }

    if env.find_among_b(A_22, context) == 0 {
        return false;
    }
    return r_mark_suffix_with_optional_y_consonant(env, context);
}

fn r_mark_yken(env: &mut SnowballEnv, context: &mut Context) -> bool {
    if !env.eq_s_b(&"ken") {
        return false;
    }
    return r_mark_suffix_with_optional_y_consonant(env, context);
}

fn r_stem_nominal_verb_suffixes(env: &mut SnowballEnv, context: &mut Context) -> bool {
    env.ket = env.cursor;
    context.b_continue_stemming_noun_suffixes = true;
    'lab0: loop {
        let v_1 = env.limit - env.cursor;
        'lab1: loop {
            'lab2: loop {
                let v_2 = env.limit - env.cursor;
                'lab3: loop {
                    if !r_mark_ymUs_(env, context) {
                        break 'lab3;
                    }
                    break 'lab2;
                }
                env.cursor = env.limit - v_2;
                'lab4: loop {
                    if !r_mark_yDU(env, context) {
                        break 'lab4;
                    }
                    break 'lab2;
                }
                env.cursor = env.limit - v_2;
                'lab5: loop {
                    if !r_mark_ysA(env, context) {
                        break 'lab5;
                    }
                    break 'lab2;
                }
                env.cursor = env.limit - v_2;
                if !r_mark_yken(env, context) {
                    break 'lab1;
                }
                break 'lab2;
            }
            break 'lab0;
        }
        env.cursor = env.limit - v_1;
        'lab6: loop {
            if !r_mark_cAsInA(env, context) {
                break 'lab6;
            }
            'lab7: loop {
                let v_3 = env.limit - env.cursor;
                'lab8: loop {
                    if !r_mark_sUnUz(env, context) {
                        break 'lab8;
                    }
                    break 'lab7;
                }
                env.cursor = env.limit - v_3;
                'lab9: loop {
                    if !r_mark_lAr(env, context) {
                        break 'lab9;
                    }
                    break 'lab7;
                }
                env.cursor = env.limit - v_3;
                'lab10: loop {
                    if !r_mark_yUm(env, context) {
                        break 'lab10;
                    }
                    break 'lab7;
                }
                env.cursor = env.limit - v_3;
                'lab11: loop {
                    if !r_mark_sUn(env, context) {
                        break 'lab11;
                    }
                    break 'lab7;
                }
                env.cursor = env.limit - v_3;
                'lab12: loop {
                    if !r_mark_yUz(env, context) {
                        break 'lab12;
                    }
                    break 'lab7;
                }
                env.cursor = env.limit - v_3;
                break 'lab7;
            }
            if !r_mark_ymUs_(env, context) {
                break 'lab6;
            }
            break 'lab0;
        }
        env.cursor = env.limit - v_1;
        'lab13: loop {
            if !r_mark_lAr(env, context) {
                break 'lab13;
            }
            env.bra = env.cursor;
            env.slice_del();
            let v_4 = env.limit - env.cursor;
            'lab14: loop {
                env.ket = env.cursor;
                'lab15: loop {
                    let v_5 = env.limit - env.cursor;
                    'lab16: loop {
                        if !r_mark_DUr(env, context) {
                            break 'lab16;
                        }
                        break 'lab15;
                    }
                    env.cursor = env.limit - v_5;
                    'lab17: loop {
                        if !r_mark_yDU(env, context) {
                            break 'lab17;
                        }
                        break 'lab15;
                    }
                    env.cursor = env.limit - v_5;
                    'lab18: loop {
                        if !r_mark_ysA(env, context) {
                            break 'lab18;
                        }
                        break 'lab15;
                    }
                    env.cursor = env.limit - v_5;
                    if !r_mark_ymUs_(env, context) {
                        env.cursor = env.limit - v_4;
                        break 'lab14;
                    }
                    break 'lab15;
                }
                break 'lab14;
            }
            context.b_continue_stemming_noun_suffixes = false;
            break 'lab0;
        }
        env.cursor = env.limit - v_1;
        'lab19: loop {
            if !r_mark_nUz(env, context) {
                break 'lab19;
            }
            'lab20: loop {
                let v_6 = env.limit - env.cursor;
                'lab21: loop {
                    if !r_mark_yDU(env, context) {
                        break 'lab21;
                    }
                    break 'lab20;
                }
                env.cursor = env.limit - v_6;
                if !r_mark_ysA(env, context) {
                    break 'lab19;
                }
                break 'lab20;
            }
            break 'lab0;
        }
        env.cursor = env.limit - v_1;
        'lab22: loop {
            'lab23: loop {
                let v_7 = env.limit - env.cursor;
                'lab24: loop {
                    if !r_mark_sUnUz(env, context) {
                        break 'lab24;
                    }
                    break 'lab23;
                }
                env.cursor = env.limit - v_7;
                'lab25: loop {
                    if !r_mark_yUz(env, context) {
                        break 'lab25;
                    }
                    break 'lab23;
                }
                env.cursor = env.limit - v_7;
                'lab26: loop {
                    if !r_mark_sUn(env, context) {
                        break 'lab26;
                    }
                    break 'lab23;
                }
                env.cursor = env.limit - v_7;
                if !r_mark_yUm(env, context) {
                    break 'lab22;
                }
                break 'lab23;
            }
            env.bra = env.cursor;
            env.slice_del();
            let v_8 = env.limit - env.cursor;
            'lab27: loop {
                env.ket = env.cursor;
                if !r_mark_ymUs_(env, context) {
                    env.cursor = env.limit - v_8;
                    break 'lab27;
                }
                break 'lab27;
            }
            break 'lab0;
        }
        env.cursor = env.limit - v_1;
        if !r_mark_DUr(env, context) {
            return false;
        }
        env.bra = env.cursor;
        env.slice_del();
        let v_9 = env.limit - env.cursor;
        'lab28: loop {
            env.ket = env.cursor;
            'lab29: loop {
                let v_10 = env.limit - env.cursor;
                'lab30: loop {
                    if !r_mark_sUnUz(env, context) {
                        break 'lab30;
                    }
                    break 'lab29;
                }
                env.cursor = env.limit - v_10;
                'lab31: loop {
                    if !r_mark_lAr(env, context) {
                        break 'lab31;
                    }
                    break 'lab29;
                }
                env.cursor = env.limit - v_10;
                'lab32: loop {
                    if !r_mark_yUm(env, context) {
                        break 'lab32;
                    }
                    break 'lab29;
                }
                env.cursor = env.limit - v_10;
                'lab33: loop {
                    if !r_mark_sUn(env, context) {
                        break 'lab33;
                    }
                    break 'lab29;
                }
                env.cursor = env.limit - v_10;
                'lab34: loop {
                    if !r_mark_yUz(env, context) {
                        break 'lab34;
                    }
                    break 'lab29;
                }
                env.cursor = env.limit - v_10;
                break 'lab29;
            }
            if !r_mark_ymUs_(env, context) {
                env.cursor = env.limit - v_9;
                break 'lab28;
            }
            break 'lab28;
        }
        break 'lab0;
    }
    env.bra = env.cursor;
    env.slice_del();
    return true
}

fn r_stem_suffix_chain_before_ki(env: &mut SnowballEnv, context: &mut Context) -> bool {
    env.ket = env.cursor;
    if !r_mark_ki(env, context) {
        return false;
    }
    'lab0: loop {
        let v_1 = env.limit - env.cursor;
        'lab1: loop {
            if !r_mark_DA(env, context) {
                break 'lab1;
            }
            env.bra = env.cursor;
            env.slice_del();
            let v_2 = env.limit - env.cursor;
            'lab2: loop {
                env.ket = env.cursor;
                'lab3: loop {
                    let v_3 = env.limit - env.cursor;
                    'lab4: loop {
                        if !r_mark_lAr(env, context) {
                            break 'lab4;
                        }
                        env.bra = env.cursor;
                        env.slice_del();
                        let v_4 = env.limit - env.cursor;
                        'lab5: loop {
                            if !r_stem_suffix_chain_before_ki(env, context) {
                                env.cursor = env.limit - v_4;
                                break 'lab5;
                            }
                            break 'lab5;
                        }
                        break 'lab3;
                    }
                    env.cursor = env.limit - v_3;
                    if !r_mark_possessives(env, context) {
                        env.cursor = env.limit - v_2;
                        break 'lab2;
                    }
                    env.bra = env.cursor;
                    env.slice_del();
                    let v_5 = env.limit - env.cursor;
                    'lab6: loop {
                        env.ket = env.cursor;
                        if !r_mark_lAr(env, context) {
                            env.cursor = env.limit - v_5;
                            break 'lab6;
                        }
                        env.bra = env.cursor;
                        env.slice_del();
                        if !r_stem_suffix_chain_before_ki(env, context) {
                            env.cursor = env.limit - v_5;
                            break 'lab6;
                        }
                        break 'lab6;
                    }
                    break 'lab3;
                }
                break 'lab2;
            }
            break 'lab0;
        }
        env.cursor = env.limit - v_1;
        'lab7: loop {
            if !r_mark_nUn(env, context) {
                break 'lab7;
            }
            env.bra = env.cursor;
            env.slice_del();
            let v_6 = env.limit - env.cursor;
            'lab8: loop {
                env.ket = env.cursor;
                'lab9: loop {
                    let v_7 = env.limit - env.cursor;
                    'lab10: loop {
                        if !r_mark_lArI(env, context) {
                            break 'lab10;
                        }
                        env.bra = env.cursor;
                        env.slice_del();
                        break 'lab9;
                    }
                    env.cursor = env.limit - v_7;
                    'lab11: loop {
                        env.ket = env.cursor;
                        'lab12: loop {
                            let v_8 = env.limit - env.cursor;
                            'lab13: loop {
                                if !r_mark_possessives(env, context) {
                                    break 'lab13;
                                }
                                break 'lab12;
                            }
                            env.cursor = env.limit - v_8;
                            if !r_mark_sU(env, context) {
                                break 'lab11;
                            }
                            break 'lab12;
                        }
                        env.bra = env.cursor;
                        env.slice_del();
                        let v_9 = env.limit - env.cursor;
                        'lab14: loop {
                            env.ket = env.cursor;
                            if !r_mark_lAr(env, context) {
                                env.cursor = env.limit - v_9;
                                break 'lab14;
                            }
                            env.bra = env.cursor;
                            env.slice_del();
                            if !r_stem_suffix_chain_before_ki(env, context) {
                                env.cursor = env.limit - v_9;
                                break 'lab14;
                            }
                            break 'lab14;
                        }
                        break 'lab9;
                    }
                    env.cursor = env.limit - v_7;
                    if !r_stem_suffix_chain_before_ki(env, context) {
                        env.cursor = env.limit - v_6;
                        break 'lab8;
                    }
                    break 'lab9;
                }
                break 'lab8;
            }
            break 'lab0;
        }
        env.cursor = env.limit - v_1;
        if !r_mark_ndA(env, context) {
            return false;
        }
        'lab15: loop {
            let v_10 = env.limit - env.cursor;
            'lab16: loop {
                if !r_mark_lArI(env, context) {
                    break 'lab16;
                }
                env.bra = env.cursor;
                env.slice_del();
                break 'lab15;
            }
            env.cursor = env.limit - v_10;
            'lab17: loop {
                if !r_mark_sU(env, context) {
                    break 'lab17;
                }
                env.bra = env.cursor;
                env.slice_del();
                let v_11 = env.limit - env.cursor;
                'lab18: loop {
                    env.ket = env.cursor;
                    if !r_mark_lAr(env, context) {
                        env.cursor = env.limit - v_11;
                        break 'lab18;
                    }
                    env.bra = env.cursor;
                    env.slice_del();
                    if !r_stem_suffix_chain_before_ki(env, context) {
                        env.cursor = env.limit - v_11;
                        break 'lab18;
                    }
                    break 'lab18;
                }
                break 'lab15;
            }
            env.cursor = env.limit - v_10;
            if !r_stem_suffix_chain_before_ki(env, context) {
                return false;
            }
            break 'lab15;
        }
        break 'lab0;
    }
    return true
}

fn r_stem_noun_suffixes(env: &mut SnowballEnv, context: &mut Context) -> bool {
    'lab0: loop {
        let v_1 = env.limit - env.cursor;
        'lab1: loop {
            env.ket = env.cursor;
            if !r_mark_lAr(env, context) {
                break 'lab1;
            }
            env.bra = env.cursor;
            env.slice_del();
            let v_2 = env.limit - env.cursor;
            'lab2: loop {
                if !r_stem_suffix_chain_before_ki(env, context) {
                    env.cursor = env.limit - v_2;
                    break 'lab2;
                }
                break 'lab2;
            }
            break 'lab0;
        }
        env.cursor = env.limit - v_1;
        'lab3: loop {
            env.ket = env.cursor;
            if !r_mark_ncA(env, context) {
                break 'lab3;
            }
            env.bra = env.cursor;
            env.slice_del();
            let v_3 = env.limit - env.cursor;
            'lab4: loop {
                'lab5: loop {
                    let v_4 = env.limit - env.cursor;
                    'lab6: loop {
                        env.ket = env.cursor;
                        if !r_mark_lArI(env, context) {
                            break 'lab6;
                        }
                        env.bra = env.cursor;
                        env.slice_del();
                        break 'lab5;
                    }
                    env.cursor = env.limit - v_4;
                    'lab7: loop {
                        env.ket = env.cursor;
                        'lab8: loop {
                            let v_5 = env.limit - env.cursor;
                            'lab9: loop {
                                if !r_mark_possessives(env, context) {
                                    break 'lab9;
                                }
                                break 'lab8;
                            }
                            env.cursor = env.limit - v_5;
                            if !r_mark_sU(env, context) {
                                break 'lab7;
                            }
                            break 'lab8;
                        }
                        env.bra = env.cursor;
                        env.slice_del();
                        let v_6 = env.limit - env.cursor;
                        'lab10: loop {
                            env.ket = env.cursor;
                            if !r_mark_lAr(env, context) {
                                env.cursor = env.limit - v_6;
                                break 'lab10;
                            }
                            env.bra = env.cursor;
                            env.slice_del();
                            if !r_stem_suffix_chain_before_ki(env, context) {
                                env.cursor = env.limit - v_6;
                                break 'lab10;
                            }
                            break 'lab10;
                        }
                        break 'lab5;
                    }
                    env.cursor = env.limit - v_4;
                    env.ket = env.cursor;
                    if !r_mark_lAr(env, context) {
                        env.cursor = env.limit - v_3;
                        break 'lab4;
                    }
                    env.bra = env.cursor;
                    env.slice_del();
                    if !r_stem_suffix_chain_before_ki(env, context) {
                        env.cursor = env.limit - v_3;
                        break 'lab4;
                    }
                    break 'lab5;
                }
                break 'lab4;
            }
            break 'lab0;
        }
        env.cursor = env.limit - v_1;
        'lab11: loop {
            env.ket = env.cursor;
            'lab12: loop {
                let v_7 = env.limit - env.cursor;
                'lab13: loop {
                    if !r_mark_ndA(env, context) {
                        break 'lab13;
                    }
                    break 'lab12;
                }
                env.cursor = env.limit - v_7;
                if !r_mark_nA(env, context) {
                    break 'lab11;
                }
                break 'lab12;
            }
            'lab14: loop {
                let v_8 = env.limit - env.cursor;
                'lab15: loop {
                    if !r_mark_lArI(env, context) {
                        break 'lab15;
                    }
                    env.bra = env.cursor;
                    env.slice_del();
                    break 'lab14;
                }
                env.cursor = env.limit - v_8;
                'lab16: loop {
                    if !r_mark_sU(env, context) {
                        break 'lab16;
                    }
                    env.bra = env.cursor;
                    env.slice_del();
                    let v_9 = env.limit - env.cursor;
                    'lab17: loop {
                        env.ket = env.cursor;
                        if !r_mark_lAr(env, context) {
                            env.cursor = env.limit - v_9;
                            break 'lab17;
                        }
                        env.bra = env.cursor;
                        env.slice_del();
                        if !r_stem_suffix_chain_before_ki(env, context) {
                            env.cursor = env.limit - v_9;
                            break 'lab17;
                        }
                        break 'lab17;
                    }
                    break 'lab14;
                }
                env.cursor = env.limit - v_8;
                if !r_stem_suffix_chain_before_ki(env, context) {
                    break 'lab11;
                }
                break 'lab14;
            }
            break 'lab0;
        }
        env.cursor = env.limit - v_1;
        'lab18: loop {
            env.ket = env.cursor;
            'lab19: loop {
                let v_10 = env.limit - env.cursor;
                'lab20: loop {
                    if !r_mark_ndAn(env, context) {
                        break 'lab20;
                    }
                    break 'lab19;
                }
                env.cursor = env.limit - v_10;
                if !r_mark_nU(env, context) {
                    break 'lab18;
                }
                break 'lab19;
            }
            'lab21: loop {
                let v_11 = env.limit - env.cursor;
                'lab22: loop {
                    if !r_mark_sU(env, context) {
                        break 'lab22;
                    }
                    env.bra = env.cursor;
                    env.slice_del();
                    let v_12 = env.limit - env.cursor;
                    'lab23: loop {
                        env.ket = env.cursor;
                        if !r_mark_lAr(env, context) {
                            env.cursor = env.limit - v_12;
                            break 'lab23;
                        }
                        env.bra = env.cursor;
                        env.slice_del();
                        if !r_stem_suffix_chain_before_ki(env, context) {
                            env.cursor = env.limit - v_12;
                            break 'lab23;
                        }
                        break 'lab23;
                    }
                    break 'lab21;
                }
                env.cursor = env.limit - v_11;
                if !r_mark_lArI(env, context) {
                    break 'lab18;
                }
                break 'lab21;
            }
            break 'lab0;
        }
        env.cursor = env.limit - v_1;
        'lab24: loop {
            env.ket = env.cursor;
            if !r_mark_DAn(env, context) {
                break 'lab24;
            }
            env.bra = env.cursor;
            env.slice_del();
            let v_13 = env.limit - env.cursor;
            'lab25: loop {
                env.ket = env.cursor;
                'lab26: loop {
                    let v_14 = env.limit - env.cursor;
                    'lab27: loop {
                        if !r_mark_possessives(env, context) {
                            break 'lab27;
                        }
                        env.bra = env.cursor;
                        env.slice_del();
                        let v_15 = env.limit - env.cursor;
                        'lab28: loop {
                            env.ket = env.cursor;
                            if !r_mark_lAr(env, context) {
                                env.cursor = env.limit - v_15;
                                break 'lab28;
                            }
                            env.bra = env.cursor;
                            env.slice_del();
                            if !r_stem_suffix_chain_before_ki(env, context) {
                                env.cursor = env.limit - v_15;
                                break 'lab28;
                            }
                            break 'lab28;
                        }
                        break 'lab26;
                    }
                    env.cursor = env.limit - v_14;
                    'lab29: loop {
                        if !r_mark_lAr(env, context) {
                            break 'lab29;
                        }
                        env.bra = env.cursor;
                        env.slice_del();
                        let v_16 = env.limit - env.cursor;
                        'lab30: loop {
                            if !r_stem_suffix_chain_before_ki(env, context) {
                                env.cursor = env.limit - v_16;
                                break 'lab30;
                            }
                            break 'lab30;
                        }
                        break 'lab26;
                    }
                    env.cursor = env.limit - v_14;
                    if !r_stem_suffix_chain_before_ki(env, context) {
                        env.cursor = env.limit - v_13;
                        break 'lab25;
                    }
                    break 'lab26;
                }
                break 'lab25;
            }
            break 'lab0;
        }
        env.cursor = env.limit - v_1;
        'lab31: loop {
            env.ket = env.cursor;
            'lab32: loop {
                let v_17 = env.limit - env.cursor;
                'lab33: loop {
                    if !r_mark_nUn(env, context) {
                        break 'lab33;
                    }
                    break 'lab32;
                }
                env.cursor = env.limit - v_17;
                if !r_mark_ylA(env, context) {
                    break 'lab31;
                }
                break 'lab32;
            }
            env.bra = env.cursor;
            env.slice_del();
            let v_18 = env.limit - env.cursor;
            'lab34: loop {
                'lab35: loop {
                    let v_19 = env.limit - env.cursor;
                    'lab36: loop {
                        env.ket = env.cursor;
                        if !r_mark_lAr(env, context) {
                            break 'lab36;
                        }
                        env.bra = env.cursor;
                        env.slice_del();
                        if !r_stem_suffix_chain_before_ki(env, context) {
                            break 'lab36;
                        }
                        break 'lab35;
                    }
                    env.cursor = env.limit - v_19;
                    'lab37: loop {
                        env.ket = env.cursor;
                        'lab38: loop {
                            let v_20 = env.limit - env.cursor;
                            'lab39: loop {
                                if !r_mark_possessives(env, context) {
                                    break 'lab39;
                                }
                                break 'lab38;
                            }
                            env.cursor = env.limit - v_20;
                            if !r_mark_sU(env, context) {
                                break 'lab37;
                            }
                            break 'lab38;
                        }
                        env.bra = env.cursor;
                        env.slice_del();
                        let v_21 = env.limit - env.cursor;
                        'lab40: loop {
                            env.ket = env.cursor;
                            if !r_mark_lAr(env, context) {
                                env.cursor = env.limit - v_21;
                                break 'lab40;
                            }
                            env.bra = env.cursor;
                            env.slice_del();
                            if !r_stem_suffix_chain_before_ki(env, context) {
                                env.cursor = env.limit - v_21;
                                break 'lab40;
                            }
                            break 'lab40;
                        }
                        break 'lab35;
                    }
                    env.cursor = env.limit - v_19;
                    if !r_stem_suffix_chain_before_ki(env, context) {
                        env.cursor = env.limit - v_18;
                        break 'lab34;
                    }
                    break 'lab35;
                }
                break 'lab34;
            }
            break 'lab0;
        }
        env.cursor = env.limit - v_1;
        'lab41: loop {
            env.ket = env.cursor;
            if !r_mark_lArI(env, context) {
                break 'lab41;
            }
            env.bra = env.cursor;
            env.slice_del();
            break 'lab0;
        }
        env.cursor = env.limit - v_1;
        'lab42: loop {
            if !r_stem_suffix_chain_before_ki(env, context) {
                break 'lab42;
            }
            break 'lab0;
        }
        env.cursor = env.limit - v_1;
        'lab43: loop {
            env.ket = env.cursor;
            'lab44: loop {
                let v_22 = env.limit - env.cursor;
                'lab45: loop {
                    if !r_mark_DA(env, context) {
                        break 'lab45;
                    }
                    break 'lab44;
                }
                env.cursor = env.limit - v_22;
                'lab46: loop {
                    if !r_mark_yU(env, context) {
                        break 'lab46;
                    }
                    break 'lab44;
                }
                env.cursor = env.limit - v_22;
                if !r_mark_yA(env, context) {
                    break 'lab43;
                }
                break 'lab44;
            }
            env.bra = env.cursor;
            env.slice_del();
            let v_23 = env.limit - env.cursor;
            'lab47: loop {
                env.ket = env.cursor;
                'lab48: loop {
                    let v_24 = env.limit - env.cursor;
                    'lab49: loop {
                        if !r_mark_possessives(env, context) {
                            break 'lab49;
                        }
                        env.bra = env.cursor;
                        env.slice_del();
                        let v_25 = env.limit - env.cursor;
                        'lab50: loop {
                            env.ket = env.cursor;
                            if !r_mark_lAr(env, context) {
                                env.cursor = env.limit - v_25;
                                break 'lab50;
                            }
                            break 'lab50;
                        }
                        break 'lab48;
                    }
                    env.cursor = env.limit - v_24;
                    if !r_mark_lAr(env, context) {
                        env.cursor = env.limit - v_23;
                        break 'lab47;
                    }
                    break 'lab48;
                }
                env.bra = env.cursor;
                env.slice_del();
                env.ket = env.cursor;
                if !r_stem_suffix_chain_before_ki(env, context) {
                    env.cursor = env.limit - v_23;
                    break 'lab47;
                }
                break 'lab47;
            }
            break 'lab0;
        }
        env.cursor = env.limit - v_1;
        env.ket = env.cursor;
        'lab51: loop {
            let v_26 = env.limit - env.cursor;
            'lab52: loop {
                if !r_mark_possessives(env, context) {
                    break 'lab52;
                }
                break 'lab51;
            }
            env.cursor = env.limit - v_26;
            if !r_mark_sU(env, context) {
                return false;
            }
            break 'lab51;
        }
        env.bra = env.cursor;
        env.slice_del();
        let v_27 = env.limit - env.cursor;
        'lab53: loop {
            env.ket = env.cursor;
            if !r_mark_lAr(env, context) {
                env.cursor = env.limit - v_27;
                break 'lab53;
            }
            env.bra = env.cursor;
            env.slice_del();
            if !r_stem_suffix_chain_before_ki(env, context) {
                env.cursor = env.limit - v_27;
                break 'lab53;
            }
            break 'lab53;
        }
        break 'lab0;
    }
    return true
}

fn r_post_process_last_consonants(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    env.ket = env.cursor;
    among_var = env.find_among_b(A_23, context);
    if among_var == 0 {
        return false;
    }
    env.bra = env.cursor;
    match among_var {
        1 => {
            env.slice_from("p");
        }
        2 => {
            env.slice_from("ç");
        }
        3 => {
            env.slice_from("t");
        }
        4 => {
            env.slice_from("k");
        }
        _ => ()
    }
    return true
}

fn r_append_U_to_stems_ending_with_d_or_g(env: &mut SnowballEnv, context: &mut Context) -> bool {
    env.ket = env.cursor;
    env.bra = env.cursor;
    'lab0: loop {
        let v_1 = env.limit - env.cursor;
        'lab1: loop {
            if !env.eq_s_b(&"d") {
                break 'lab1;
            }
            break 'lab0;
        }
        env.cursor = env.limit - v_1;
        if !env.eq_s_b(&"g") {
            return false;
        }
        break 'lab0;
    }
    if !env.go_out_grouping_b(G_vowel, 97, 305) {
        return false;
    }
    'lab2: loop {
        let v_2 = env.limit - env.cursor;
        'lab3: loop {
            'lab4: loop {
                let v_3 = env.limit - env.cursor;
                'lab5: loop {
                    if !env.eq_s_b(&"a") {
                        break 'lab5;
                    }
                    break 'lab4;
                }
                env.cursor = env.limit - v_3;
                if !env.eq_s_b(&"ı") {
                    break 'lab3;
                }
                break 'lab4;
            }
            env.slice_from("ı");
            break 'lab2;
        }
        env.cursor = env.limit - v_2;
        'lab6: loop {
            'lab7: loop {
                let v_4 = env.limit - env.cursor;
                'lab8: loop {
                    if !env.eq_s_b(&"e") {
                        break 'lab8;
                    }
                    break 'lab7;
                }
                env.cursor = env.limit - v_4;
                if !env.eq_s_b(&"i") {
                    break 'lab6;
                }
                break 'lab7;
            }
            env.slice_from("i");
            break 'lab2;
        }
        env.cursor = env.limit - v_2;
        'lab9: loop {
            'lab10: loop {
                let v_5 = env.limit - env.cursor;
                'lab11: loop {
                    if !env.eq_s_b(&"o") {
                        break 'lab11;
                    }
                    break 'lab10;
                }
                env.cursor = env.limit - v_5;
                if !env.eq_s_b(&"u") {
                    break 'lab9;
                }
                break 'lab10;
            }
            env.slice_from("u");
            break 'lab2;
        }
        env.cursor = env.limit - v_2;
        'lab12: loop {
            let v_6 = env.limit - env.cursor;
            'lab13: loop {
                if !env.eq_s_b(&"ö") {
                    break 'lab13;
                }
                break 'lab12;
            }
            env.cursor = env.limit - v_6;
            if !env.eq_s_b(&"ü") {
                return false;
            }
            break 'lab12;
        }
        env.slice_from("ü");
        break 'lab2;
    }
    return true
}

fn r_is_reserved_word(env: &mut SnowballEnv, context: &mut Context) -> bool {
    if !env.eq_s_b(&"ad") {
        return false;
    }
    let v_1 = env.limit - env.cursor;
    'lab0: loop {
        if !env.eq_s_b(&"soy") {
            env.cursor = env.limit - v_1;
            break 'lab0;
        }
        break 'lab0;
    }
    if env.cursor > env.limit_backward {
        return false;
    }
    return true
}

fn r_remove_proper_noun_suffix(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let v_1 = env.cursor;
    'lab0: loop {
        env.bra = env.cursor;
        'golab1: loop {
            let v_2 = env.cursor;
            'lab2: loop {
                let v_3 = env.cursor;
                'lab3: loop {
                    if !env.eq_s(&"'") {
                        break 'lab3;
                    }
                    break 'lab2;
                }
                env.cursor = v_3;
                env.cursor = v_2;
                break 'golab1;
            }
            env.cursor = v_2;
            if env.cursor >= env.limit {
                break 'lab0;
            }
            env.next_char();
        }
        env.ket = env.cursor;
        env.slice_del();
        break 'lab0;
    }
    env.cursor = v_1;
    let v_4 = env.cursor;
    'lab4: loop {
        if !env.hop(2) {
            break 'lab4;
        }
        'golab5: loop {
            let v_5 = env.cursor;
            'lab6: loop {
                if !env.eq_s(&"'") {
                    break 'lab6;
                }
                env.cursor = v_5;
                break 'golab5;
            }
            env.cursor = v_5;
            if env.cursor >= env.limit {
                break 'lab4;
            }
            env.next_char();
        }
        env.bra = env.cursor;
        env.cursor = env.limit;
        env.ket = env.cursor;
        env.slice_del();
        break 'lab4;
    }
    env.cursor = v_4;
    return true
}

fn r_more_than_one_syllable_word(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let v_1 = env.cursor;
    for _ in 0..2 {
        if !env.go_out_grouping(G_vowel, 97, 305) {
            return false;
        }
        env.next_char();
    }
    env.cursor = v_1;
    return true
}

fn r_postlude(env: &mut SnowballEnv, context: &mut Context) -> bool {
    env.limit_backward = env.cursor;
    env.cursor = env.limit;
    let v_1 = env.limit - env.cursor;
    'lab0: loop {
        if !r_is_reserved_word(env, context) {
            break 'lab0;
        }
        return false;
    }
    env.cursor = env.limit - v_1;
    let v_2 = env.limit - env.cursor;
    r_append_U_to_stems_ending_with_d_or_g(env, context);
    env.cursor = env.limit - v_2;
    let v_3 = env.limit - env.cursor;
    r_post_process_last_consonants(env, context);
    env.cursor = env.limit - v_3;
    env.cursor = env.limit_backward;
    return true
}

pub fn stem(env: &mut SnowballEnv) -> bool {
    let mut context = &mut Context {
        b_continue_stemming_noun_suffixes: false,
    };
    r_remove_proper_noun_suffix(env, context);
    if !r_more_than_one_syllable_word(env, context) {
        return false;
    }
    env.limit_backward = env.cursor;
    env.cursor = env.limit;
    let v_1 = env.limit - env.cursor;
    r_stem_nominal_verb_suffixes(env, context);
    env.cursor = env.limit - v_1;
    if !context.b_continue_stemming_noun_suffixes {
        return false;
    }
    let v_2 = env.limit - env.cursor;
    r_stem_noun_suffixes(env, context);
    env.cursor = env.limit - v_2;
    env.cursor = env.limit_backward;
    return r_postlude(env, context);
}
