//! Generated from russian.sbl by Snowball 3.0.0 - https://snowballstem.org/

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
    i_pV: i32,
}

static A_0: &'static [Among<Context>; 9] = &[
    Among("вшись", -1, 1, None),
    Among("ывшись", 0, 2, None),
    Among("ившись", 0, 2, None),
    Among("в", -1, 1, None),
    Among("ыв", 3, 2, None),
    Among("ив", 3, 2, None),
    Among("вши", -1, 1, None),
    Among("ывши", 6, 2, None),
    Among("ивши", 6, 2, None),
];

static A_1: &'static [Among<Context>; 26] = &[
    Among("ему", -1, 1, None),
    Among("ому", -1, 1, None),
    Among("ых", -1, 1, None),
    Among("их", -1, 1, None),
    Among("ую", -1, 1, None),
    Among("юю", -1, 1, None),
    Among("ею", -1, 1, None),
    Among("ою", -1, 1, None),
    Among("яя", -1, 1, None),
    Among("ая", -1, 1, None),
    Among("ые", -1, 1, None),
    Among("ее", -1, 1, None),
    Among("ие", -1, 1, None),
    Among("ое", -1, 1, None),
    Among("ыми", -1, 1, None),
    Among("ими", -1, 1, None),
    Among("ый", -1, 1, None),
    Among("ей", -1, 1, None),
    Among("ий", -1, 1, None),
    Among("ой", -1, 1, None),
    Among("ым", -1, 1, None),
    Among("ем", -1, 1, None),
    Among("им", -1, 1, None),
    Among("ом", -1, 1, None),
    Among("его", -1, 1, None),
    Among("ого", -1, 1, None),
];

static A_2: &'static [Among<Context>; 8] = &[
    Among("вш", -1, 1, None),
    Among("ывш", 0, 2, None),
    Among("ивш", 0, 2, None),
    Among("щ", -1, 1, None),
    Among("ющ", 3, 1, None),
    Among("ующ", 4, 2, None),
    Among("ем", -1, 1, None),
    Among("нн", -1, 1, None),
];

static A_3: &'static [Among<Context>; 2] = &[
    Among("сь", -1, 1, None),
    Among("ся", -1, 1, None),
];

static A_4: &'static [Among<Context>; 46] = &[
    Among("ыт", -1, 2, None),
    Among("ют", -1, 1, None),
    Among("уют", 1, 2, None),
    Among("ят", -1, 2, None),
    Among("ет", -1, 1, None),
    Among("ует", 4, 2, None),
    Among("ит", -1, 2, None),
    Among("ны", -1, 1, None),
    Among("ены", 7, 2, None),
    Among("ть", -1, 1, None),
    Among("ыть", 9, 2, None),
    Among("ить", 9, 2, None),
    Among("ешь", -1, 1, None),
    Among("ишь", -1, 2, None),
    Among("ю", -1, 2, None),
    Among("ую", 14, 2, None),
    Among("ла", -1, 1, None),
    Among("ыла", 16, 2, None),
    Among("ила", 16, 2, None),
    Among("на", -1, 1, None),
    Among("ена", 19, 2, None),
    Among("ете", -1, 1, None),
    Among("ите", -1, 2, None),
    Among("йте", -1, 1, None),
    Among("уйте", 23, 2, None),
    Among("ейте", 23, 2, None),
    Among("ли", -1, 1, None),
    Among("ыли", 26, 2, None),
    Among("или", 26, 2, None),
    Among("й", -1, 1, None),
    Among("уй", 29, 2, None),
    Among("ей", 29, 2, None),
    Among("л", -1, 1, None),
    Among("ыл", 32, 2, None),
    Among("ил", 32, 2, None),
    Among("ым", -1, 2, None),
    Among("ем", -1, 1, None),
    Among("им", -1, 2, None),
    Among("н", -1, 1, None),
    Among("ен", 38, 2, None),
    Among("ло", -1, 1, None),
    Among("ыло", 40, 2, None),
    Among("ило", 40, 2, None),
    Among("но", -1, 1, None),
    Among("ено", 43, 2, None),
    Among("нно", 43, 1, None),
];

static A_5: &'static [Among<Context>; 36] = &[
    Among("у", -1, 1, None),
    Among("ях", -1, 1, None),
    Among("иях", 1, 1, None),
    Among("ах", -1, 1, None),
    Among("ы", -1, 1, None),
    Among("ь", -1, 1, None),
    Among("ю", -1, 1, None),
    Among("ью", 6, 1, None),
    Among("ию", 6, 1, None),
    Among("я", -1, 1, None),
    Among("ья", 9, 1, None),
    Among("ия", 9, 1, None),
    Among("а", -1, 1, None),
    Among("ев", -1, 1, None),
    Among("ов", -1, 1, None),
    Among("е", -1, 1, None),
    Among("ье", 15, 1, None),
    Among("ие", 15, 1, None),
    Among("и", -1, 1, None),
    Among("еи", 18, 1, None),
    Among("ии", 18, 1, None),
    Among("ями", 18, 1, None),
    Among("иями", 21, 1, None),
    Among("ами", 18, 1, None),
    Among("й", -1, 1, None),
    Among("ей", 24, 1, None),
    Among("ией", 25, 1, None),
    Among("ий", 24, 1, None),
    Among("ой", 24, 1, None),
    Among("ям", -1, 1, None),
    Among("иям", 29, 1, None),
    Among("ам", -1, 1, None),
    Among("ем", -1, 1, None),
    Among("ием", 32, 1, None),
    Among("ом", -1, 1, None),
    Among("о", -1, 1, None),
];

static A_6: &'static [Among<Context>; 2] = &[
    Among("ост", -1, 1, None),
    Among("ость", -1, 1, None),
];

static A_7: &'static [Among<Context>; 4] = &[
    Among("ейш", -1, 1, None),
    Among("ь", -1, 3, None),
    Among("ейше", -1, 1, None),
    Among("н", -1, 2, None),
];

static G_v: &'static [u8; 4] = &[33, 65, 8, 232];

fn r_mark_regions(env: &mut SnowballEnv, context: &mut Context) -> bool {
    context.i_pV = env.limit;
    context.i_p2 = env.limit;
    let v_1 = env.cursor;
    'lab0: loop {
        if !env.go_out_grouping(G_v, 1072, 1103) {
            break 'lab0;
        }
        env.next_char();
        context.i_pV = env.cursor;
        if !env.go_in_grouping(G_v, 1072, 1103) {
            break 'lab0;
        }
        env.next_char();
        if !env.go_out_grouping(G_v, 1072, 1103) {
            break 'lab0;
        }
        env.next_char();
        if !env.go_in_grouping(G_v, 1072, 1103) {
            break 'lab0;
        }
        env.next_char();
        context.i_p2 = env.cursor;
        break 'lab0;
    }
    env.cursor = v_1;
    return true
}

fn r_R2(env: &mut SnowballEnv, context: &mut Context) -> bool {
    return context.i_p2 <= env.cursor
}

fn r_perfective_gerund(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    env.ket = env.cursor;
    among_var = env.find_among_b(A_0, context);
    if among_var == 0 {
        return false;
    }
    env.bra = env.cursor;
    match among_var {
        1 => {
            'lab0: loop {
                let v_1 = env.limit - env.cursor;
                'lab1: loop {
                    if !env.eq_s_b(&"а") {
                        break 'lab1;
                    }
                    break 'lab0;
                }
                env.cursor = env.limit - v_1;
                if !env.eq_s_b(&"я") {
                    return false;
                }
                break 'lab0;
            }
            env.slice_del();
        }
        2 => {
            env.slice_del();
        }
        _ => ()
    }
    return true
}

fn r_adjective(env: &mut SnowballEnv, context: &mut Context) -> bool {
    env.ket = env.cursor;
    if env.find_among_b(A_1, context) == 0 {
        return false;
    }
    env.bra = env.cursor;
    env.slice_del();
    return true
}

fn r_adjectival(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    if !r_adjective(env, context) {
        return false;
    }
    let v_1 = env.limit - env.cursor;
    'lab0: loop {
        env.ket = env.cursor;
        among_var = env.find_among_b(A_2, context);
        if among_var == 0 {
            env.cursor = env.limit - v_1;
            break 'lab0;
        }
        env.bra = env.cursor;
        match among_var {
            1 => {
                'lab1: loop {
                    let v_2 = env.limit - env.cursor;
                    'lab2: loop {
                        if !env.eq_s_b(&"а") {
                            break 'lab2;
                        }
                        break 'lab1;
                    }
                    env.cursor = env.limit - v_2;
                    if !env.eq_s_b(&"я") {
                        env.cursor = env.limit - v_1;
                        break 'lab0;
                    }
                    break 'lab1;
                }
                env.slice_del();
            }
            2 => {
                env.slice_del();
            }
            _ => ()
        }
        break 'lab0;
    }
    return true
}

fn r_reflexive(env: &mut SnowballEnv, context: &mut Context) -> bool {
    env.ket = env.cursor;
    if (env.cursor - 3 <= env.limit_backward || (env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 140 as u8 && env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 143 as u8)) {
        return false;
    }

    if env.find_among_b(A_3, context) == 0 {
        return false;
    }
    env.bra = env.cursor;
    env.slice_del();
    return true
}

fn r_verb(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    env.ket = env.cursor;
    among_var = env.find_among_b(A_4, context);
    if among_var == 0 {
        return false;
    }
    env.bra = env.cursor;
    match among_var {
        1 => {
            'lab0: loop {
                let v_1 = env.limit - env.cursor;
                'lab1: loop {
                    if !env.eq_s_b(&"а") {
                        break 'lab1;
                    }
                    break 'lab0;
                }
                env.cursor = env.limit - v_1;
                if !env.eq_s_b(&"я") {
                    return false;
                }
                break 'lab0;
            }
            env.slice_del();
        }
        2 => {
            env.slice_del();
        }
        _ => ()
    }
    return true
}

fn r_noun(env: &mut SnowballEnv, context: &mut Context) -> bool {
    env.ket = env.cursor;
    if env.find_among_b(A_5, context) == 0 {
        return false;
    }
    env.bra = env.cursor;
    env.slice_del();
    return true
}

fn r_derivational(env: &mut SnowballEnv, context: &mut Context) -> bool {
    env.ket = env.cursor;
    if (env.cursor - 5 <= env.limit_backward || (env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 130 as u8 && env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 140 as u8)) {
        return false;
    }

    if env.find_among_b(A_6, context) == 0 {
        return false;
    }
    env.bra = env.cursor;
    if !r_R2(env, context) {
        return false;
    }
    env.slice_del();
    return true
}

fn r_tidy_up(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    env.ket = env.cursor;
    among_var = env.find_among_b(A_7, context);
    if among_var == 0 {
        return false;
    }
    env.bra = env.cursor;
    match among_var {
        1 => {
            env.slice_del();
            env.ket = env.cursor;
            if !env.eq_s_b(&"н") {
                return false;
            }
            env.bra = env.cursor;
            if !env.eq_s_b(&"н") {
                return false;
            }
            env.slice_del();
        }
        2 => {
            if !env.eq_s_b(&"н") {
                return false;
            }
            env.slice_del();
        }
        3 => {
            env.slice_del();
        }
        _ => ()
    }
    return true
}

pub fn stem(env: &mut SnowballEnv) -> bool {
    let mut context = &mut Context {
        i_p2: 0,
        i_pV: 0,
    };
    let v_1 = env.cursor;
    'lab0: loop {
        'replab1: loop{
            let v_2 = env.cursor;
            'lab2: for _ in 0..1 {
                'golab3: loop {
                    let v_3 = env.cursor;
                    'lab4: loop {
                        env.bra = env.cursor;
                        if !env.eq_s(&"ё") {
                            break 'lab4;
                        }
                        env.ket = env.cursor;
                        env.cursor = v_3;
                        break 'golab3;
                    }
                    env.cursor = v_3;
                    if env.cursor >= env.limit {
                        break 'lab2;
                    }
                    env.next_char();
                }
                env.slice_from("е");
                continue 'replab1;
            }
            env.cursor = v_2;
            break 'replab1;
        }
        break 'lab0;
    }
    env.cursor = v_1;
    r_mark_regions(env, context);
    env.limit_backward = env.cursor;
    env.cursor = env.limit;
    if env.cursor < context.i_pV {
        return false;
    }
    let v_4 = env.limit_backward;
    env.limit_backward = context.i_pV;
    let v_5 = env.limit - env.cursor;
    'lab5: loop {
        'lab6: loop {
            let v_6 = env.limit - env.cursor;
            'lab7: loop {
                if !r_perfective_gerund(env, context) {
                    break 'lab7;
                }
                break 'lab6;
            }
            env.cursor = env.limit - v_6;
            let v_7 = env.limit - env.cursor;
            'lab8: loop {
                if !r_reflexive(env, context) {
                    env.cursor = env.limit - v_7;
                    break 'lab8;
                }
                break 'lab8;
            }
            'lab9: loop {
                let v_8 = env.limit - env.cursor;
                'lab10: loop {
                    if !r_adjectival(env, context) {
                        break 'lab10;
                    }
                    break 'lab9;
                }
                env.cursor = env.limit - v_8;
                'lab11: loop {
                    if !r_verb(env, context) {
                        break 'lab11;
                    }
                    break 'lab9;
                }
                env.cursor = env.limit - v_8;
                if !r_noun(env, context) {
                    break 'lab5;
                }
                break 'lab9;
            }
            break 'lab6;
        }
        break 'lab5;
    }
    env.cursor = env.limit - v_5;
    let v_9 = env.limit - env.cursor;
    'lab12: loop {
        env.ket = env.cursor;
        if !env.eq_s_b(&"и") {
            env.cursor = env.limit - v_9;
            break 'lab12;
        }
        env.bra = env.cursor;
        env.slice_del();
        break 'lab12;
    }
    let v_10 = env.limit - env.cursor;
    r_derivational(env, context);
    env.cursor = env.limit - v_10;
    let v_11 = env.limit - env.cursor;
    r_tidy_up(env, context);
    env.cursor = env.limit - v_11;
    env.limit_backward = v_4;
    env.cursor = env.limit_backward;
    return true
}
