//! Generated from portuguese.sbl by Snowball 3.0.0 - https://snowballstem.org/

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

static A_0: &'static [Among<Context>; 3] = &[
    Among("", -1, 3, None),
    Among("ã", 0, 1, None),
    Among("õ", 0, 2, None),
];

static A_1: &'static [Among<Context>; 3] = &[
    Among("", -1, 3, None),
    Among("a~", 0, 1, None),
    Among("o~", 0, 2, None),
];

static A_2: &'static [Among<Context>; 4] = &[
    Among("ic", -1, -1, None),
    Among("ad", -1, -1, None),
    Among("os", -1, -1, None),
    Among("iv", -1, 1, None),
];

static A_3: &'static [Among<Context>; 3] = &[
    Among("ante", -1, 1, None),
    Among("avel", -1, 1, None),
    Among("ível", -1, 1, None),
];

static A_4: &'static [Among<Context>; 3] = &[
    Among("ic", -1, 1, None),
    Among("abil", -1, 1, None),
    Among("iv", -1, 1, None),
];

static A_5: &'static [Among<Context>; 45] = &[
    Among("ica", -1, 1, None),
    Among("ância", -1, 1, None),
    Among("ência", -1, 4, None),
    Among("logia", -1, 2, None),
    Among("ira", -1, 9, None),
    Among("adora", -1, 1, None),
    Among("osa", -1, 1, None),
    Among("ista", -1, 1, None),
    Among("iva", -1, 8, None),
    Among("eza", -1, 1, None),
    Among("idade", -1, 7, None),
    Among("ante", -1, 1, None),
    Among("mente", -1, 6, None),
    Among("amente", 12, 5, None),
    Among("ável", -1, 1, None),
    Among("ível", -1, 1, None),
    Among("ico", -1, 1, None),
    Among("ismo", -1, 1, None),
    Among("oso", -1, 1, None),
    Among("amento", -1, 1, None),
    Among("imento", -1, 1, None),
    Among("ivo", -1, 8, None),
    Among("aça~o", -1, 1, None),
    Among("uça~o", -1, 3, None),
    Among("ador", -1, 1, None),
    Among("icas", -1, 1, None),
    Among("ências", -1, 4, None),
    Among("logias", -1, 2, None),
    Among("iras", -1, 9, None),
    Among("adoras", -1, 1, None),
    Among("osas", -1, 1, None),
    Among("istas", -1, 1, None),
    Among("ivas", -1, 8, None),
    Among("ezas", -1, 1, None),
    Among("idades", -1, 7, None),
    Among("adores", -1, 1, None),
    Among("antes", -1, 1, None),
    Among("aço~es", -1, 1, None),
    Among("uço~es", -1, 3, None),
    Among("icos", -1, 1, None),
    Among("ismos", -1, 1, None),
    Among("osos", -1, 1, None),
    Among("amentos", -1, 1, None),
    Among("imentos", -1, 1, None),
    Among("ivos", -1, 8, None),
];

static A_6: &'static [Among<Context>; 120] = &[
    Among("ada", -1, 1, None),
    Among("ida", -1, 1, None),
    Among("ia", -1, 1, None),
    Among("aria", 2, 1, None),
    Among("eria", 2, 1, None),
    Among("iria", 2, 1, None),
    Among("ara", -1, 1, None),
    Among("era", -1, 1, None),
    Among("ira", -1, 1, None),
    Among("ava", -1, 1, None),
    Among("asse", -1, 1, None),
    Among("esse", -1, 1, None),
    Among("isse", -1, 1, None),
    Among("aste", -1, 1, None),
    Among("este", -1, 1, None),
    Among("iste", -1, 1, None),
    Among("ei", -1, 1, None),
    Among("arei", 16, 1, None),
    Among("erei", 16, 1, None),
    Among("irei", 16, 1, None),
    Among("am", -1, 1, None),
    Among("iam", 20, 1, None),
    Among("ariam", 21, 1, None),
    Among("eriam", 21, 1, None),
    Among("iriam", 21, 1, None),
    Among("aram", 20, 1, None),
    Among("eram", 20, 1, None),
    Among("iram", 20, 1, None),
    Among("avam", 20, 1, None),
    Among("em", -1, 1, None),
    Among("arem", 29, 1, None),
    Among("erem", 29, 1, None),
    Among("irem", 29, 1, None),
    Among("assem", 29, 1, None),
    Among("essem", 29, 1, None),
    Among("issem", 29, 1, None),
    Among("ado", -1, 1, None),
    Among("ido", -1, 1, None),
    Among("ando", -1, 1, None),
    Among("endo", -1, 1, None),
    Among("indo", -1, 1, None),
    Among("ara~o", -1, 1, None),
    Among("era~o", -1, 1, None),
    Among("ira~o", -1, 1, None),
    Among("ar", -1, 1, None),
    Among("er", -1, 1, None),
    Among("ir", -1, 1, None),
    Among("as", -1, 1, None),
    Among("adas", 47, 1, None),
    Among("idas", 47, 1, None),
    Among("ias", 47, 1, None),
    Among("arias", 50, 1, None),
    Among("erias", 50, 1, None),
    Among("irias", 50, 1, None),
    Among("aras", 47, 1, None),
    Among("eras", 47, 1, None),
    Among("iras", 47, 1, None),
    Among("avas", 47, 1, None),
    Among("es", -1, 1, None),
    Among("ardes", 58, 1, None),
    Among("erdes", 58, 1, None),
    Among("irdes", 58, 1, None),
    Among("ares", 58, 1, None),
    Among("eres", 58, 1, None),
    Among("ires", 58, 1, None),
    Among("asses", 58, 1, None),
    Among("esses", 58, 1, None),
    Among("isses", 58, 1, None),
    Among("astes", 58, 1, None),
    Among("estes", 58, 1, None),
    Among("istes", 58, 1, None),
    Among("is", -1, 1, None),
    Among("ais", 71, 1, None),
    Among("eis", 71, 1, None),
    Among("areis", 73, 1, None),
    Among("ereis", 73, 1, None),
    Among("ireis", 73, 1, None),
    Among("áreis", 73, 1, None),
    Among("éreis", 73, 1, None),
    Among("íreis", 73, 1, None),
    Among("ásseis", 73, 1, None),
    Among("ésseis", 73, 1, None),
    Among("ísseis", 73, 1, None),
    Among("áveis", 73, 1, None),
    Among("íeis", 73, 1, None),
    Among("aríeis", 84, 1, None),
    Among("eríeis", 84, 1, None),
    Among("iríeis", 84, 1, None),
    Among("ados", -1, 1, None),
    Among("idos", -1, 1, None),
    Among("amos", -1, 1, None),
    Among("áramos", 90, 1, None),
    Among("éramos", 90, 1, None),
    Among("íramos", 90, 1, None),
    Among("ávamos", 90, 1, None),
    Among("íamos", 90, 1, None),
    Among("aríamos", 95, 1, None),
    Among("eríamos", 95, 1, None),
    Among("iríamos", 95, 1, None),
    Among("emos", -1, 1, None),
    Among("aremos", 99, 1, None),
    Among("eremos", 99, 1, None),
    Among("iremos", 99, 1, None),
    Among("ássemos", 99, 1, None),
    Among("êssemos", 99, 1, None),
    Among("íssemos", 99, 1, None),
    Among("imos", -1, 1, None),
    Among("armos", -1, 1, None),
    Among("ermos", -1, 1, None),
    Among("irmos", -1, 1, None),
    Among("ámos", -1, 1, None),
    Among("arás", -1, 1, None),
    Among("erás", -1, 1, None),
    Among("irás", -1, 1, None),
    Among("eu", -1, 1, None),
    Among("iu", -1, 1, None),
    Among("ou", -1, 1, None),
    Among("ará", -1, 1, None),
    Among("erá", -1, 1, None),
    Among("irá", -1, 1, None),
];

static A_7: &'static [Among<Context>; 7] = &[
    Among("a", -1, 1, None),
    Among("i", -1, 1, None),
    Among("o", -1, 1, None),
    Among("os", -1, 1, None),
    Among("á", -1, 1, None),
    Among("í", -1, 1, None),
    Among("ó", -1, 1, None),
];

static A_8: &'static [Among<Context>; 4] = &[
    Among("e", -1, 1, None),
    Among("ç", -1, 2, None),
    Among("é", -1, 1, None),
    Among("ê", -1, 1, None),
];

static G_v: &'static [u8; 20] = &[17, 65, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 19, 12, 2];

fn r_prelude(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    'replab0: loop{
        let v_1 = env.cursor;
        'lab1: for _ in 0..1 {
            env.bra = env.cursor;
            if (env.cursor + 1 >= env.limit || (env.current.as_bytes()[(env.cursor + 1) as usize] as u8 != 163 as u8 && env.current.as_bytes()[(env.cursor + 1) as usize] as u8 != 181 as u8)) {among_var = 3;}
            else {
                among_var = env.find_among(A_0, context);
            }
            env.ket = env.cursor;
            match among_var {
                1 => {
                    env.slice_from("a~");
                }
                2 => {
                    env.slice_from("o~");
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

fn r_mark_regions(env: &mut SnowballEnv, context: &mut Context) -> bool {
    context.i_pV = env.limit;
    context.i_p1 = env.limit;
    context.i_p2 = env.limit;
    let v_1 = env.cursor;
    'lab0: loop {
        'lab1: loop {
            let v_2 = env.cursor;
            'lab2: loop {
                if !env.in_grouping(G_v, 97, 250) {
                    break 'lab2;
                }
                'lab3: loop {
                    let v_3 = env.cursor;
                    'lab4: loop {
                        if !env.out_grouping(G_v, 97, 250) {
                            break 'lab4;
                        }
                        if !env.go_out_grouping(G_v, 97, 250) {
                            break 'lab4;
                        }
                        env.next_char();
                        break 'lab3;
                    }
                    env.cursor = v_3;
                    if !env.in_grouping(G_v, 97, 250) {
                        break 'lab2;
                    }
                    if !env.go_in_grouping(G_v, 97, 250) {
                        break 'lab2;
                    }
                    env.next_char();
                    break 'lab3;
                }
                break 'lab1;
            }
            env.cursor = v_2;
            if !env.out_grouping(G_v, 97, 250) {
                break 'lab0;
            }
            'lab5: loop {
                let v_4 = env.cursor;
                'lab6: loop {
                    if !env.out_grouping(G_v, 97, 250) {
                        break 'lab6;
                    }
                    if !env.go_out_grouping(G_v, 97, 250) {
                        break 'lab6;
                    }
                    env.next_char();
                    break 'lab5;
                }
                env.cursor = v_4;
                if !env.in_grouping(G_v, 97, 250) {
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
        if !env.go_out_grouping(G_v, 97, 250) {
            break 'lab7;
        }
        env.next_char();
        if !env.go_in_grouping(G_v, 97, 250) {
            break 'lab7;
        }
        env.next_char();
        context.i_p1 = env.cursor;
        if !env.go_out_grouping(G_v, 97, 250) {
            break 'lab7;
        }
        env.next_char();
        if !env.go_in_grouping(G_v, 97, 250) {
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
            if (env.cursor + 1 >= env.limit || env.current.as_bytes()[(env.cursor + 1) as usize] as u8 != 126 as u8) {among_var = 3;}
            else {
                among_var = env.find_among(A_1, context);
            }
            env.ket = env.cursor;
            match among_var {
                1 => {
                    env.slice_from("ã");
                }
                2 => {
                    env.slice_from("õ");
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
    if (env.cursor - 2 <= env.limit_backward || env.current.as_bytes()[(env.cursor - 1) as usize] as u8 >> 5 != 3 as u8 || ((823330 as i32 >> (env.current.as_bytes()[(env.cursor - 1) as usize] as u8 & 0x1f)) & 1) == 0) {
        return false;
    }

    among_var = env.find_among_b(A_5, context);
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
            env.slice_from("log");
        }
        3 => {
            if !r_R2(env, context) {
                return false;
            }
            env.slice_from("u");
        }
        4 => {
            if !r_R2(env, context) {
                return false;
            }
            env.slice_from("ente");
        }
        5 => {
            if !r_R1(env, context) {
                return false;
            }
            env.slice_del();
            let v_1 = env.limit - env.cursor;
            'lab0: loop {
                env.ket = env.cursor;
                if (env.cursor - 1 <= env.limit_backward || env.current.as_bytes()[(env.cursor - 1) as usize] as u8 >> 5 != 3 as u8 || ((4718616 as i32 >> (env.current.as_bytes()[(env.cursor - 1) as usize] as u8 & 0x1f)) & 1) == 0) {
                    env.cursor = env.limit - v_1;
                    break 'lab0;
                }

                among_var = env.find_among_b(A_2, context);
                if among_var == 0 {
                    env.cursor = env.limit - v_1;
                    break 'lab0;
                }
                env.bra = env.cursor;
                if !r_R2(env, context) {
                    env.cursor = env.limit - v_1;
                    break 'lab0;
                }
                env.slice_del();
                match among_var {
                    1 => {
                        env.ket = env.cursor;
                        if !env.eq_s_b(&"at") {
                            env.cursor = env.limit - v_1;
                            break 'lab0;
                        }
                        env.bra = env.cursor;
                        if !r_R2(env, context) {
                            env.cursor = env.limit - v_1;
                            break 'lab0;
                        }
                        env.slice_del();
                    }
                    _ => ()
                }
                break 'lab0;
            }
        }
        6 => {
            if !r_R2(env, context) {
                return false;
            }
            env.slice_del();
            let v_2 = env.limit - env.cursor;
            'lab1: loop {
                env.ket = env.cursor;
                if (env.cursor - 3 <= env.limit_backward || (env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 101 as u8 && env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 108 as u8)) {
                    env.cursor = env.limit - v_2;
                    break 'lab1;
                }

                if env.find_among_b(A_3, context) == 0 {
                    env.cursor = env.limit - v_2;
                    break 'lab1;
                }
                env.bra = env.cursor;
                if !r_R2(env, context) {
                    env.cursor = env.limit - v_2;
                    break 'lab1;
                }
                env.slice_del();
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
                if (env.cursor - 1 <= env.limit_backward || env.current.as_bytes()[(env.cursor - 1) as usize] as u8 >> 5 != 3 as u8 || ((4198408 as i32 >> (env.current.as_bytes()[(env.cursor - 1) as usize] as u8 & 0x1f)) & 1) == 0) {
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
                if !env.eq_s_b(&"at") {
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
            if !r_RV(env, context) {
                return false;
            }
            if !env.eq_s_b(&"e") {
                return false;
            }
            env.slice_from("ir");
        }
        _ => ()
    }
    return true
}

fn r_verb_suffix(env: &mut SnowballEnv, context: &mut Context) -> bool {
    if env.cursor < context.i_pV {
        return false;
    }
    let v_1 = env.limit_backward;
    env.limit_backward = context.i_pV;
    env.ket = env.cursor;
    if env.find_among_b(A_6, context) == 0 {
        env.limit_backward = v_1;
        return false;
    }
    env.bra = env.cursor;
    env.slice_del();
    env.limit_backward = v_1;
    return true
}

fn r_residual_suffix(env: &mut SnowballEnv, context: &mut Context) -> bool {
    env.ket = env.cursor;
    if env.find_among_b(A_7, context) == 0 {
        return false;
    }
    env.bra = env.cursor;
    if !r_RV(env, context) {
        return false;
    }
    env.slice_del();
    return true
}

fn r_residual_form(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    env.ket = env.cursor;
    among_var = env.find_among_b(A_8, context);
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
            env.ket = env.cursor;
            'lab0: loop {
                let v_1 = env.limit - env.cursor;
                'lab1: loop {
                    if !env.eq_s_b(&"u") {
                        break 'lab1;
                    }
                    env.bra = env.cursor;
                    let v_2 = env.limit - env.cursor;
                    if !env.eq_s_b(&"g") {
                        break 'lab1;
                    }
                    env.cursor = env.limit - v_2;
                    break 'lab0;
                }
                env.cursor = env.limit - v_1;
                if !env.eq_s_b(&"i") {
                    return false;
                }
                env.bra = env.cursor;
                let v_3 = env.limit - env.cursor;
                if !env.eq_s_b(&"c") {
                    return false;
                }
                env.cursor = env.limit - v_3;
                break 'lab0;
            }
            if !r_RV(env, context) {
                return false;
            }
            env.slice_del();
        }
        2 => {
            env.slice_from("c");
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
    r_prelude(env, context);
    env.cursor = v_1;
    r_mark_regions(env, context);
    env.limit_backward = env.cursor;
    env.cursor = env.limit;
    let v_2 = env.limit - env.cursor;
    'lab0: loop {
        'lab1: loop {
            let v_3 = env.limit - env.cursor;
            'lab2: loop {
                let v_4 = env.limit - env.cursor;
                'lab3: loop {
                    let v_5 = env.limit - env.cursor;
                    'lab4: loop {
                        if !r_standard_suffix(env, context) {
                            break 'lab4;
                        }
                        break 'lab3;
                    }
                    env.cursor = env.limit - v_5;
                    if !r_verb_suffix(env, context) {
                        break 'lab2;
                    }
                    break 'lab3;
                }
                env.cursor = env.limit - v_4;
                let v_6 = env.limit - env.cursor;
                'lab5: loop {
                    env.ket = env.cursor;
                    if !env.eq_s_b(&"i") {
                        break 'lab5;
                    }
                    env.bra = env.cursor;
                    let v_7 = env.limit - env.cursor;
                    if !env.eq_s_b(&"c") {
                        break 'lab5;
                    }
                    env.cursor = env.limit - v_7;
                    if !r_RV(env, context) {
                        break 'lab5;
                    }
                    env.slice_del();
                    break 'lab5;
                }
                env.cursor = env.limit - v_6;
                break 'lab1;
            }
            env.cursor = env.limit - v_3;
            if !r_residual_suffix(env, context) {
                break 'lab0;
            }
            break 'lab1;
        }
        break 'lab0;
    }
    env.cursor = env.limit - v_2;
    let v_8 = env.limit - env.cursor;
    r_residual_form(env, context);
    env.cursor = env.limit - v_8;
    env.cursor = env.limit_backward;
    let v_9 = env.cursor;
    r_postlude(env, context);
    env.cursor = v_9;
    return true
}
