//! Generated from spanish.sbl by Snowball 3.0.0 - https://snowballstem.org/

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

static A_0: &'static [Among<Context>; 6] = &[
    Among("", -1, 6, None),
    Among("á", 0, 1, None),
    Among("é", 0, 2, None),
    Among("í", 0, 3, None),
    Among("ó", 0, 4, None),
    Among("ú", 0, 5, None),
];

static A_1: &'static [Among<Context>; 13] = &[
    Among("la", -1, -1, None),
    Among("sela", 0, -1, None),
    Among("le", -1, -1, None),
    Among("me", -1, -1, None),
    Among("se", -1, -1, None),
    Among("lo", -1, -1, None),
    Among("selo", 5, -1, None),
    Among("las", -1, -1, None),
    Among("selas", 7, -1, None),
    Among("les", -1, -1, None),
    Among("los", -1, -1, None),
    Among("selos", 10, -1, None),
    Among("nos", -1, -1, None),
];

static A_2: &'static [Among<Context>; 11] = &[
    Among("ando", -1, 6, None),
    Among("iendo", -1, 6, None),
    Among("yendo", -1, 7, None),
    Among("ándo", -1, 2, None),
    Among("iéndo", -1, 1, None),
    Among("ar", -1, 6, None),
    Among("er", -1, 6, None),
    Among("ir", -1, 6, None),
    Among("ár", -1, 3, None),
    Among("ér", -1, 4, None),
    Among("ír", -1, 5, None),
];

static A_3: &'static [Among<Context>; 4] = &[
    Among("ic", -1, -1, None),
    Among("ad", -1, -1, None),
    Among("os", -1, -1, None),
    Among("iv", -1, 1, None),
];

static A_4: &'static [Among<Context>; 3] = &[
    Among("able", -1, 1, None),
    Among("ible", -1, 1, None),
    Among("ante", -1, 1, None),
];

static A_5: &'static [Among<Context>; 3] = &[
    Among("ic", -1, 1, None),
    Among("abil", -1, 1, None),
    Among("iv", -1, 1, None),
];

static A_6: &'static [Among<Context>; 48] = &[
    Among("ica", -1, 1, None),
    Among("ancia", -1, 2, None),
    Among("encia", -1, 5, None),
    Among("adora", -1, 2, None),
    Among("osa", -1, 1, None),
    Among("ista", -1, 1, None),
    Among("iva", -1, 9, None),
    Among("anza", -1, 1, None),
    Among("logía", -1, 3, None),
    Among("idad", -1, 8, None),
    Among("able", -1, 1, None),
    Among("ible", -1, 1, None),
    Among("ante", -1, 2, None),
    Among("mente", -1, 7, None),
    Among("amente", 13, 6, None),
    Among("acion", -1, 2, None),
    Among("ucion", -1, 4, None),
    Among("ación", -1, 2, None),
    Among("ución", -1, 4, None),
    Among("ico", -1, 1, None),
    Among("ismo", -1, 1, None),
    Among("oso", -1, 1, None),
    Among("amiento", -1, 1, None),
    Among("imiento", -1, 1, None),
    Among("ivo", -1, 9, None),
    Among("ador", -1, 2, None),
    Among("icas", -1, 1, None),
    Among("ancias", -1, 2, None),
    Among("encias", -1, 5, None),
    Among("adoras", -1, 2, None),
    Among("osas", -1, 1, None),
    Among("istas", -1, 1, None),
    Among("ivas", -1, 9, None),
    Among("anzas", -1, 1, None),
    Among("logías", -1, 3, None),
    Among("idades", -1, 8, None),
    Among("ables", -1, 1, None),
    Among("ibles", -1, 1, None),
    Among("aciones", -1, 2, None),
    Among("uciones", -1, 4, None),
    Among("adores", -1, 2, None),
    Among("antes", -1, 2, None),
    Among("icos", -1, 1, None),
    Among("ismos", -1, 1, None),
    Among("osos", -1, 1, None),
    Among("amientos", -1, 1, None),
    Among("imientos", -1, 1, None),
    Among("ivos", -1, 9, None),
];

static A_7: &'static [Among<Context>; 12] = &[
    Among("ya", -1, 1, None),
    Among("ye", -1, 1, None),
    Among("yan", -1, 1, None),
    Among("yen", -1, 1, None),
    Among("yeron", -1, 1, None),
    Among("yendo", -1, 1, None),
    Among("yo", -1, 1, None),
    Among("yas", -1, 1, None),
    Among("yes", -1, 1, None),
    Among("yais", -1, 1, None),
    Among("yamos", -1, 1, None),
    Among("yó", -1, 1, None),
];

static A_8: &'static [Among<Context>; 96] = &[
    Among("aba", -1, 2, None),
    Among("ada", -1, 2, None),
    Among("ida", -1, 2, None),
    Among("ara", -1, 2, None),
    Among("iera", -1, 2, None),
    Among("ía", -1, 2, None),
    Among("aría", 5, 2, None),
    Among("ería", 5, 2, None),
    Among("iría", 5, 2, None),
    Among("ad", -1, 2, None),
    Among("ed", -1, 2, None),
    Among("id", -1, 2, None),
    Among("ase", -1, 2, None),
    Among("iese", -1, 2, None),
    Among("aste", -1, 2, None),
    Among("iste", -1, 2, None),
    Among("an", -1, 2, None),
    Among("aban", 16, 2, None),
    Among("aran", 16, 2, None),
    Among("ieran", 16, 2, None),
    Among("ían", 16, 2, None),
    Among("arían", 20, 2, None),
    Among("erían", 20, 2, None),
    Among("irían", 20, 2, None),
    Among("en", -1, 1, None),
    Among("asen", 24, 2, None),
    Among("iesen", 24, 2, None),
    Among("aron", -1, 2, None),
    Among("ieron", -1, 2, None),
    Among("arán", -1, 2, None),
    Among("erán", -1, 2, None),
    Among("irán", -1, 2, None),
    Among("ado", -1, 2, None),
    Among("ido", -1, 2, None),
    Among("ando", -1, 2, None),
    Among("iendo", -1, 2, None),
    Among("ar", -1, 2, None),
    Among("er", -1, 2, None),
    Among("ir", -1, 2, None),
    Among("as", -1, 2, None),
    Among("abas", 39, 2, None),
    Among("adas", 39, 2, None),
    Among("idas", 39, 2, None),
    Among("aras", 39, 2, None),
    Among("ieras", 39, 2, None),
    Among("ías", 39, 2, None),
    Among("arías", 45, 2, None),
    Among("erías", 45, 2, None),
    Among("irías", 45, 2, None),
    Among("es", -1, 1, None),
    Among("ases", 49, 2, None),
    Among("ieses", 49, 2, None),
    Among("abais", -1, 2, None),
    Among("arais", -1, 2, None),
    Among("ierais", -1, 2, None),
    Among("íais", -1, 2, None),
    Among("aríais", 55, 2, None),
    Among("eríais", 55, 2, None),
    Among("iríais", 55, 2, None),
    Among("aseis", -1, 2, None),
    Among("ieseis", -1, 2, None),
    Among("asteis", -1, 2, None),
    Among("isteis", -1, 2, None),
    Among("áis", -1, 2, None),
    Among("éis", -1, 1, None),
    Among("aréis", 64, 2, None),
    Among("eréis", 64, 2, None),
    Among("iréis", 64, 2, None),
    Among("ados", -1, 2, None),
    Among("idos", -1, 2, None),
    Among("amos", -1, 2, None),
    Among("ábamos", 70, 2, None),
    Among("áramos", 70, 2, None),
    Among("iéramos", 70, 2, None),
    Among("íamos", 70, 2, None),
    Among("aríamos", 74, 2, None),
    Among("eríamos", 74, 2, None),
    Among("iríamos", 74, 2, None),
    Among("emos", -1, 1, None),
    Among("aremos", 78, 2, None),
    Among("eremos", 78, 2, None),
    Among("iremos", 78, 2, None),
    Among("ásemos", 78, 2, None),
    Among("iésemos", 78, 2, None),
    Among("imos", -1, 2, None),
    Among("arás", -1, 2, None),
    Among("erás", -1, 2, None),
    Among("irás", -1, 2, None),
    Among("ís", -1, 2, None),
    Among("ará", -1, 2, None),
    Among("erá", -1, 2, None),
    Among("irá", -1, 2, None),
    Among("aré", -1, 2, None),
    Among("eré", -1, 2, None),
    Among("iré", -1, 2, None),
    Among("ió", -1, 2, None),
];

static A_9: &'static [Among<Context>; 8] = &[
    Among("a", -1, 1, None),
    Among("e", -1, 2, None),
    Among("o", -1, 1, None),
    Among("os", -1, 1, None),
    Among("á", -1, 1, None),
    Among("é", -1, 2, None),
    Among("í", -1, 1, None),
    Among("ó", -1, 1, None),
];

static G_v: &'static [u8; 20] = &[17, 65, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 17, 4, 10];

fn r_mark_regions(env: &mut SnowballEnv, context: &mut Context) -> bool {
    context.i_pV = env.limit;
    context.i_p1 = env.limit;
    context.i_p2 = env.limit;
    let v_1 = env.cursor;
    'lab0: loop {
        'lab1: loop {
            let v_2 = env.cursor;
            'lab2: loop {
                if !env.in_grouping(G_v, 97, 252) {
                    break 'lab2;
                }
                'lab3: loop {
                    let v_3 = env.cursor;
                    'lab4: loop {
                        if !env.out_grouping(G_v, 97, 252) {
                            break 'lab4;
                        }
                        if !env.go_out_grouping(G_v, 97, 252) {
                            break 'lab4;
                        }
                        env.next_char();
                        break 'lab3;
                    }
                    env.cursor = v_3;
                    if !env.in_grouping(G_v, 97, 252) {
                        break 'lab2;
                    }
                    if !env.go_in_grouping(G_v, 97, 252) {
                        break 'lab2;
                    }
                    env.next_char();
                    break 'lab3;
                }
                break 'lab1;
            }
            env.cursor = v_2;
            if !env.out_grouping(G_v, 97, 252) {
                break 'lab0;
            }
            'lab5: loop {
                let v_4 = env.cursor;
                'lab6: loop {
                    if !env.out_grouping(G_v, 97, 252) {
                        break 'lab6;
                    }
                    if !env.go_out_grouping(G_v, 97, 252) {
                        break 'lab6;
                    }
                    env.next_char();
                    break 'lab5;
                }
                env.cursor = v_4;
                if !env.in_grouping(G_v, 97, 252) {
                    break 'lab0;
                }
                if env.cursor >= env.limit {
                    break 'lab0;
                }
                env.next_char();
                break 'lab5;
            }
            break 'lab1;
        }
        context.i_pV = env.cursor;
        break 'lab0;
    }
    env.cursor = v_1;
    let v_5 = env.cursor;
    'lab7: loop {
        if !env.go_out_grouping(G_v, 97, 252) {
            break 'lab7;
        }
        env.next_char();
        if !env.go_in_grouping(G_v, 97, 252) {
            break 'lab7;
        }
        env.next_char();
        context.i_p1 = env.cursor;
        if !env.go_out_grouping(G_v, 97, 252) {
            break 'lab7;
        }
        env.next_char();
        if !env.go_in_grouping(G_v, 97, 252) {
            break 'lab7;
        }
        env.next_char();
        context.i_p2 = env.cursor;
        break 'lab7;
    }
    env.cursor = v_5;
    return true
}

fn r_postlude(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    'replab0: loop{
        let v_1 = env.cursor;
        'lab1: for _ in 0..1 {
            env.bra = env.cursor;
            if (env.cursor + 1 >= env.limit || env.current.as_bytes()[(env.cursor + 1) as usize] as u8 >> 5 != 5 as u8 || ((67641858 as i32 >> (env.current.as_bytes()[(env.cursor + 1) as usize] as u8 & 0x1f)) & 1) == 0) {among_var = 6;}
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

fn r_attached_pronoun(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    env.ket = env.cursor;
    if (env.cursor - 1 <= env.limit_backward || env.current.as_bytes()[(env.cursor - 1) as usize] as u8 >> 5 != 3 as u8 || ((557090 as i32 >> (env.current.as_bytes()[(env.cursor - 1) as usize] as u8 & 0x1f)) & 1) == 0) {
        return false;
    }

    if env.find_among_b(A_1, context) == 0 {
        return false;
    }
    env.bra = env.cursor;
    if (env.cursor - 1 <= env.limit_backward || (env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 111 as u8 && env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 114 as u8)) {
        return false;
    }

    among_var = env.find_among_b(A_2, context);
    if among_var == 0 {
        return false;
    }
    if !r_RV(env, context) {
        return false;
    }
    match among_var {
        1 => {
            env.bra = env.cursor;
            env.slice_from("iendo");
        }
        2 => {
            env.bra = env.cursor;
            env.slice_from("ando");
        }
        3 => {
            env.bra = env.cursor;
            env.slice_from("ar");
        }
        4 => {
            env.bra = env.cursor;
            env.slice_from("er");
        }
        5 => {
            env.bra = env.cursor;
            env.slice_from("ir");
        }
        6 => {
            env.slice_del();
        }
        7 => {
            if !env.eq_s_b(&"u") {
                return false;
            }
            env.slice_del();
        }
        _ => ()
    }
    return true
}

fn r_standard_suffix(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    env.ket = env.cursor;
    if (env.cursor - 2 <= env.limit_backward || env.current.as_bytes()[(env.cursor - 1) as usize] as u8 >> 5 != 3 as u8 || ((835634 as i32 >> (env.current.as_bytes()[(env.cursor - 1) as usize] as u8 & 0x1f)) & 1) == 0) {
        return false;
    }

    among_var = env.find_among_b(A_6, context);
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
                if !r_R2(env, context) {
                    env.cursor = env.limit - v_1;
                    break 'lab0;
                }
                env.slice_del();
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
            env.slice_from("ente");
        }
        6 => {
            if !r_R1(env, context) {
                return false;
            }
            env.slice_del();
            let v_2 = env.limit - env.cursor;
            'lab1: loop {
                env.ket = env.cursor;
                if (env.cursor - 1 <= env.limit_backward || env.current.as_bytes()[(env.cursor - 1) as usize] as u8 >> 5 != 3 as u8 || ((4718616 as i32 >> (env.current.as_bytes()[(env.cursor - 1) as usize] as u8 & 0x1f)) & 1) == 0) {
                    env.cursor = env.limit - v_2;
                    break 'lab1;
                }

                among_var = env.find_among_b(A_3, context);
                if among_var == 0 {
                    env.cursor = env.limit - v_2;
                    break 'lab1;
                }
                env.bra = env.cursor;
                if !r_R2(env, context) {
                    env.cursor = env.limit - v_2;
                    break 'lab1;
                }
                env.slice_del();
                match among_var {
                    1 => {
                        env.ket = env.cursor;
                        if !env.eq_s_b(&"at") {
                            env.cursor = env.limit - v_2;
                            break 'lab1;
                        }
                        env.bra = env.cursor;
                        if !r_R2(env, context) {
                            env.cursor = env.limit - v_2;
                            break 'lab1;
                        }
                        env.slice_del();
                    }
                    _ => ()
                }
                break 'lab1;
            }
        }
        7 => {
            if !r_R2(env, context) {
                return false;
            }
            env.slice_del();
            let v_3 = env.limit - env.cursor;
            'lab2: loop {
                env.ket = env.cursor;
                if (env.cursor - 3 <= env.limit_backward || env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 101 as u8) {
                    env.cursor = env.limit - v_3;
                    break 'lab2;
                }

                if env.find_among_b(A_4, context) == 0 {
                    env.cursor = env.limit - v_3;
                    break 'lab2;
                }
                env.bra = env.cursor;
                if !r_R2(env, context) {
                    env.cursor = env.limit - v_3;
                    break 'lab2;
                }
                env.slice_del();
                break 'lab2;
            }
        }
        8 => {
            if !r_R2(env, context) {
                return false;
            }
            env.slice_del();
            let v_4 = env.limit - env.cursor;
            'lab3: loop {
                env.ket = env.cursor;
                if (env.cursor - 1 <= env.limit_backward || env.current.as_bytes()[(env.cursor - 1) as usize] as u8 >> 5 != 3 as u8 || ((4198408 as i32 >> (env.current.as_bytes()[(env.cursor - 1) as usize] as u8 & 0x1f)) & 1) == 0) {
                    env.cursor = env.limit - v_4;
                    break 'lab3;
                }

                if env.find_among_b(A_5, context) == 0 {
                    env.cursor = env.limit - v_4;
                    break 'lab3;
                }
                env.bra = env.cursor;
                if !r_R2(env, context) {
                    env.cursor = env.limit - v_4;
                    break 'lab3;
                }
                env.slice_del();
                break 'lab3;
            }
        }
        9 => {
            if !r_R2(env, context) {
                return false;
            }
            env.slice_del();
            let v_5 = env.limit - env.cursor;
            'lab4: loop {
                env.ket = env.cursor;
                if !env.eq_s_b(&"at") {
                    env.cursor = env.limit - v_5;
                    break 'lab4;
                }
                env.bra = env.cursor;
                if !r_R2(env, context) {
                    env.cursor = env.limit - v_5;
                    break 'lab4;
                }
                env.slice_del();
                break 'lab4;
            }
        }
        _ => ()
    }
    return true
}

fn r_y_verb_suffix(env: &mut SnowballEnv, context: &mut Context) -> bool {
    if env.cursor < context.i_pV {
        return false;
    }
    let v_1 = env.limit_backward;
    env.limit_backward = context.i_pV;
    env.ket = env.cursor;
    if env.find_among_b(A_7, context) == 0 {
        env.limit_backward = v_1;
        return false;
    }
    env.bra = env.cursor;
    env.limit_backward = v_1;
    if !env.eq_s_b(&"u") {
        return false;
    }
    env.slice_del();
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
    among_var = env.find_among_b(A_8, context);
    if among_var == 0 {
        env.limit_backward = v_1;
        return false;
    }
    env.bra = env.cursor;
    env.limit_backward = v_1;
    match among_var {
        1 => {
            let v_2 = env.limit - env.cursor;
            'lab0: loop {
                if !env.eq_s_b(&"u") {
                    env.cursor = env.limit - v_2;
                    break 'lab0;
                }
                let v_3 = env.limit - env.cursor;
                if !env.eq_s_b(&"g") {
                    env.cursor = env.limit - v_2;
                    break 'lab0;
                }
                env.cursor = env.limit - v_3;
                break 'lab0;
            }
            env.bra = env.cursor;
            env.slice_del();
        }
        2 => {
            env.slice_del();
        }
        _ => ()
    }
    return true
}

fn r_residual_suffix(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    env.ket = env.cursor;
    among_var = env.find_among_b(A_9, context);
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
            if !r_RV(env, context) {
                return false;
            }
            env.slice_del();
            let v_1 = env.limit - env.cursor;
            'lab0: loop {
                env.ket = env.cursor;
                if !env.eq_s_b(&"u") {
                    env.cursor = env.limit - v_1;
                    break 'lab0;
                }
                env.bra = env.cursor;
                let v_2 = env.limit - env.cursor;
                if !env.eq_s_b(&"g") {
                    env.cursor = env.limit - v_1;
                    break 'lab0;
                }
                env.cursor = env.limit - v_2;
                if !r_RV(env, context) {
                    env.cursor = env.limit - v_1;
                    break 'lab0;
                }
                env.slice_del();
                break 'lab0;
            }
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
    r_mark_regions(env, context);
    env.limit_backward = env.cursor;
    env.cursor = env.limit;
    let v_1 = env.limit - env.cursor;
    r_attached_pronoun(env, context);
    env.cursor = env.limit - v_1;
    let v_2 = env.limit - env.cursor;
    'lab0: loop {
        'lab1: loop {
            let v_3 = env.limit - env.cursor;
            'lab2: loop {
                if !r_standard_suffix(env, context) {
                    break 'lab2;
                }
                break 'lab1;
            }
            env.cursor = env.limit - v_3;
            'lab3: loop {
                if !r_y_verb_suffix(env, context) {
                    break 'lab3;
                }
                break 'lab1;
            }
            env.cursor = env.limit - v_3;
            if !r_verb_suffix(env, context) {
                break 'lab0;
            }
            break 'lab1;
        }
        break 'lab0;
    }
    env.cursor = env.limit - v_2;
    let v_4 = env.limit - env.cursor;
    r_residual_suffix(env, context);
    env.cursor = env.limit - v_4;
    env.cursor = env.limit_backward;
    let v_5 = env.cursor;
    r_postlude(env, context);
    env.cursor = v_5;
    return true
}
