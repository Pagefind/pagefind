//! Generated from estonian.sbl by Snowball 3.0.0 - https://snowballstem.org/

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

static A_0: &'static [Among<Context>; 2] = &[
    Among("gi", -1, 1, None),
    Among("ki", -1, 2, None),
];

static A_1: &'static [Among<Context>; 21] = &[
    Among("da", -1, 3, None),
    Among("mata", -1, 1, None),
    Among("b", -1, 3, None),
    Among("ksid", -1, 1, None),
    Among("nuksid", 3, 1, None),
    Among("me", -1, 3, None),
    Among("sime", 5, 1, None),
    Among("ksime", 6, 1, None),
    Among("nuksime", 7, 1, None),
    Among("akse", -1, 2, None),
    Among("dakse", 9, 1, None),
    Among("takse", 9, 1, None),
    Among("site", -1, 1, None),
    Among("ksite", 12, 1, None),
    Among("nuksite", 13, 1, None),
    Among("n", -1, 3, None),
    Among("sin", 15, 1, None),
    Among("ksin", 16, 1, None),
    Among("nuksin", 17, 1, None),
    Among("daks", -1, 1, None),
    Among("taks", -1, 1, None),
];

static A_2: &'static [Among<Context>; 9] = &[
    Among("aa", -1, -1, None),
    Among("ee", -1, -1, None),
    Among("ii", -1, -1, None),
    Among("oo", -1, -1, None),
    Among("uu", -1, -1, None),
    Among("ää", -1, -1, None),
    Among("õõ", -1, -1, None),
    Among("öö", -1, -1, None),
    Among("üü", -1, -1, None),
];

static A_3: &'static [Among<Context>; 12] = &[
    Among("lane", -1, 1, None),
    Among("line", -1, 3, None),
    Among("mine", -1, 2, None),
    Among("lasse", -1, 1, None),
    Among("lisse", -1, 3, None),
    Among("misse", -1, 2, None),
    Among("lasi", -1, 1, None),
    Among("lisi", -1, 3, None),
    Among("misi", -1, 2, None),
    Among("last", -1, 1, None),
    Among("list", -1, 3, None),
    Among("mist", -1, 2, None),
];

static A_4: &'static [Among<Context>; 10] = &[
    Among("ga", -1, 1, None),
    Among("ta", -1, 1, None),
    Among("le", -1, 1, None),
    Among("sse", -1, 1, None),
    Among("l", -1, 1, None),
    Among("s", -1, 1, None),
    Among("ks", 5, 1, None),
    Among("t", -1, 2, None),
    Among("lt", 7, 1, None),
    Among("st", 7, 1, None),
];

static A_5: &'static [Among<Context>; 5] = &[
    Among("", -1, 2, None),
    Among("las", 0, 1, None),
    Among("lis", 0, 1, None),
    Among("mis", 0, 1, None),
    Among("t", 0, -1, None),
];

static A_6: &'static [Among<Context>; 7] = &[
    Among("d", -1, 4, None),
    Among("sid", 0, 2, None),
    Among("de", -1, 4, None),
    Among("ikkude", 2, 1, None),
    Among("ike", -1, 1, None),
    Among("ikke", -1, 1, None),
    Among("te", -1, 3, None),
];

static A_7: &'static [Among<Context>; 4] = &[
    Among("va", -1, -1, None),
    Among("du", -1, -1, None),
    Among("nu", -1, -1, None),
    Among("tu", -1, -1, None),
];

static A_8: &'static [Among<Context>; 3] = &[
    Among("kk", -1, 1, None),
    Among("pp", -1, 2, None),
    Among("tt", -1, 3, None),
];

static A_9: &'static [Among<Context>; 3] = &[
    Among("ma", -1, 2, None),
    Among("mai", -1, 1, None),
    Among("m", -1, 1, None),
];

static A_10: &'static [Among<Context>; 290] = &[
    Among("joob", -1, 1, None),
    Among("jood", -1, 1, None),
    Among("joodakse", 1, 1, None),
    Among("jooma", -1, 1, None),
    Among("joomata", 3, 1, None),
    Among("joome", -1, 1, None),
    Among("joon", -1, 1, None),
    Among("joote", -1, 1, None),
    Among("joovad", -1, 1, None),
    Among("juua", -1, 1, None),
    Among("juuakse", 9, 1, None),
    Among("jäi", -1, 12, None),
    Among("jäid", 11, 12, None),
    Among("jäime", 11, 12, None),
    Among("jäin", 11, 12, None),
    Among("jäite", 11, 12, None),
    Among("jääb", -1, 12, None),
    Among("jääd", -1, 12, None),
    Among("jääda", 17, 12, None),
    Among("jäädakse", 18, 12, None),
    Among("jäädi", 17, 12, None),
    Among("jääks", -1, 12, None),
    Among("jääksid", 21, 12, None),
    Among("jääksime", 21, 12, None),
    Among("jääksin", 21, 12, None),
    Among("jääksite", 21, 12, None),
    Among("jääma", -1, 12, None),
    Among("jäämata", 26, 12, None),
    Among("jääme", -1, 12, None),
    Among("jään", -1, 12, None),
    Among("jääte", -1, 12, None),
    Among("jäävad", -1, 12, None),
    Among("jõi", -1, 1, None),
    Among("jõid", 32, 1, None),
    Among("jõime", 32, 1, None),
    Among("jõin", 32, 1, None),
    Among("jõite", 32, 1, None),
    Among("keeb", -1, 4, None),
    Among("keed", -1, 4, None),
    Among("keedakse", 38, 4, None),
    Among("keeks", -1, 4, None),
    Among("keeksid", 40, 4, None),
    Among("keeksime", 40, 4, None),
    Among("keeksin", 40, 4, None),
    Among("keeksite", 40, 4, None),
    Among("keema", -1, 4, None),
    Among("keemata", 45, 4, None),
    Among("keeme", -1, 4, None),
    Among("keen", -1, 4, None),
    Among("kees", -1, 4, None),
    Among("keeta", -1, 4, None),
    Among("keete", -1, 4, None),
    Among("keevad", -1, 4, None),
    Among("käia", -1, 8, None),
    Among("käiakse", 53, 8, None),
    Among("käib", -1, 8, None),
    Among("käid", -1, 8, None),
    Among("käidi", 56, 8, None),
    Among("käiks", -1, 8, None),
    Among("käiksid", 58, 8, None),
    Among("käiksime", 58, 8, None),
    Among("käiksin", 58, 8, None),
    Among("käiksite", 58, 8, None),
    Among("käima", -1, 8, None),
    Among("käimata", 63, 8, None),
    Among("käime", -1, 8, None),
    Among("käin", -1, 8, None),
    Among("käis", -1, 8, None),
    Among("käite", -1, 8, None),
    Among("käivad", -1, 8, None),
    Among("laob", -1, 16, None),
    Among("laod", -1, 16, None),
    Among("laoks", -1, 16, None),
    Among("laoksid", 72, 16, None),
    Among("laoksime", 72, 16, None),
    Among("laoksin", 72, 16, None),
    Among("laoksite", 72, 16, None),
    Among("laome", -1, 16, None),
    Among("laon", -1, 16, None),
    Among("laote", -1, 16, None),
    Among("laovad", -1, 16, None),
    Among("loeb", -1, 14, None),
    Among("loed", -1, 14, None),
    Among("loeks", -1, 14, None),
    Among("loeksid", 83, 14, None),
    Among("loeksime", 83, 14, None),
    Among("loeksin", 83, 14, None),
    Among("loeksite", 83, 14, None),
    Among("loeme", -1, 14, None),
    Among("loen", -1, 14, None),
    Among("loete", -1, 14, None),
    Among("loevad", -1, 14, None),
    Among("loob", -1, 7, None),
    Among("lood", -1, 7, None),
    Among("loodi", 93, 7, None),
    Among("looks", -1, 7, None),
    Among("looksid", 95, 7, None),
    Among("looksime", 95, 7, None),
    Among("looksin", 95, 7, None),
    Among("looksite", 95, 7, None),
    Among("looma", -1, 7, None),
    Among("loomata", 100, 7, None),
    Among("loome", -1, 7, None),
    Among("loon", -1, 7, None),
    Among("loote", -1, 7, None),
    Among("loovad", -1, 7, None),
    Among("luua", -1, 7, None),
    Among("luuakse", 106, 7, None),
    Among("lõi", -1, 6, None),
    Among("lõid", 108, 6, None),
    Among("lõime", 108, 6, None),
    Among("lõin", 108, 6, None),
    Among("lõite", 108, 6, None),
    Among("lööb", -1, 5, None),
    Among("lööd", -1, 5, None),
    Among("löödakse", 114, 5, None),
    Among("löödi", 114, 5, None),
    Among("lööks", -1, 5, None),
    Among("lööksid", 117, 5, None),
    Among("lööksime", 117, 5, None),
    Among("lööksin", 117, 5, None),
    Among("lööksite", 117, 5, None),
    Among("lööma", -1, 5, None),
    Among("löömata", 122, 5, None),
    Among("lööme", -1, 5, None),
    Among("löön", -1, 5, None),
    Among("lööte", -1, 5, None),
    Among("löövad", -1, 5, None),
    Among("lüüa", -1, 5, None),
    Among("lüüakse", 128, 5, None),
    Among("müüa", -1, 13, None),
    Among("müüakse", 130, 13, None),
    Among("müüb", -1, 13, None),
    Among("müüd", -1, 13, None),
    Among("müüdi", 133, 13, None),
    Among("müüks", -1, 13, None),
    Among("müüksid", 135, 13, None),
    Among("müüksime", 135, 13, None),
    Among("müüksin", 135, 13, None),
    Among("müüksite", 135, 13, None),
    Among("müüma", -1, 13, None),
    Among("müümata", 140, 13, None),
    Among("müüme", -1, 13, None),
    Among("müün", -1, 13, None),
    Among("müüs", -1, 13, None),
    Among("müüte", -1, 13, None),
    Among("müüvad", -1, 13, None),
    Among("näeb", -1, 18, None),
    Among("näed", -1, 18, None),
    Among("näeks", -1, 18, None),
    Among("näeksid", 149, 18, None),
    Among("näeksime", 149, 18, None),
    Among("näeksin", 149, 18, None),
    Among("näeksite", 149, 18, None),
    Among("näeme", -1, 18, None),
    Among("näen", -1, 18, None),
    Among("näete", -1, 18, None),
    Among("näevad", -1, 18, None),
    Among("nägema", -1, 18, None),
    Among("nägemata", 158, 18, None),
    Among("näha", -1, 18, None),
    Among("nähakse", 160, 18, None),
    Among("nähti", -1, 18, None),
    Among("põeb", -1, 15, None),
    Among("põed", -1, 15, None),
    Among("põeks", -1, 15, None),
    Among("põeksid", 165, 15, None),
    Among("põeksime", 165, 15, None),
    Among("põeksin", 165, 15, None),
    Among("põeksite", 165, 15, None),
    Among("põeme", -1, 15, None),
    Among("põen", -1, 15, None),
    Among("põete", -1, 15, None),
    Among("põevad", -1, 15, None),
    Among("saab", -1, 2, None),
    Among("saad", -1, 2, None),
    Among("saada", 175, 2, None),
    Among("saadakse", 176, 2, None),
    Among("saadi", 175, 2, None),
    Among("saaks", -1, 2, None),
    Among("saaksid", 179, 2, None),
    Among("saaksime", 179, 2, None),
    Among("saaksin", 179, 2, None),
    Among("saaksite", 179, 2, None),
    Among("saama", -1, 2, None),
    Among("saamata", 184, 2, None),
    Among("saame", -1, 2, None),
    Among("saan", -1, 2, None),
    Among("saate", -1, 2, None),
    Among("saavad", -1, 2, None),
    Among("sai", -1, 2, None),
    Among("said", 190, 2, None),
    Among("saime", 190, 2, None),
    Among("sain", 190, 2, None),
    Among("saite", 190, 2, None),
    Among("sõi", -1, 9, None),
    Among("sõid", 195, 9, None),
    Among("sõime", 195, 9, None),
    Among("sõin", 195, 9, None),
    Among("sõite", 195, 9, None),
    Among("sööb", -1, 9, None),
    Among("sööd", -1, 9, None),
    Among("söödakse", 201, 9, None),
    Among("söödi", 201, 9, None),
    Among("sööks", -1, 9, None),
    Among("sööksid", 204, 9, None),
    Among("sööksime", 204, 9, None),
    Among("sööksin", 204, 9, None),
    Among("sööksite", 204, 9, None),
    Among("sööma", -1, 9, None),
    Among("söömata", 209, 9, None),
    Among("sööme", -1, 9, None),
    Among("söön", -1, 9, None),
    Among("sööte", -1, 9, None),
    Among("söövad", -1, 9, None),
    Among("süüa", -1, 9, None),
    Among("süüakse", 215, 9, None),
    Among("teeb", -1, 17, None),
    Among("teed", -1, 17, None),
    Among("teeks", -1, 17, None),
    Among("teeksid", 219, 17, None),
    Among("teeksime", 219, 17, None),
    Among("teeksin", 219, 17, None),
    Among("teeksite", 219, 17, None),
    Among("teeme", -1, 17, None),
    Among("teen", -1, 17, None),
    Among("teete", -1, 17, None),
    Among("teevad", -1, 17, None),
    Among("tegema", -1, 17, None),
    Among("tegemata", 228, 17, None),
    Among("teha", -1, 17, None),
    Among("tehakse", 230, 17, None),
    Among("tehti", -1, 17, None),
    Among("toob", -1, 10, None),
    Among("tood", -1, 10, None),
    Among("toodi", 234, 10, None),
    Among("tooks", -1, 10, None),
    Among("tooksid", 236, 10, None),
    Among("tooksime", 236, 10, None),
    Among("tooksin", 236, 10, None),
    Among("tooksite", 236, 10, None),
    Among("tooma", -1, 10, None),
    Among("toomata", 241, 10, None),
    Among("toome", -1, 10, None),
    Among("toon", -1, 10, None),
    Among("toote", -1, 10, None),
    Among("toovad", -1, 10, None),
    Among("tuua", -1, 10, None),
    Among("tuuakse", 247, 10, None),
    Among("tõi", -1, 10, None),
    Among("tõid", 249, 10, None),
    Among("tõime", 249, 10, None),
    Among("tõin", 249, 10, None),
    Among("tõite", 249, 10, None),
    Among("viia", -1, 3, None),
    Among("viiakse", 254, 3, None),
    Among("viib", -1, 3, None),
    Among("viid", -1, 3, None),
    Among("viidi", 257, 3, None),
    Among("viiks", -1, 3, None),
    Among("viiksid", 259, 3, None),
    Among("viiksime", 259, 3, None),
    Among("viiksin", 259, 3, None),
    Among("viiksite", 259, 3, None),
    Among("viima", -1, 3, None),
    Among("viimata", 264, 3, None),
    Among("viime", -1, 3, None),
    Among("viin", -1, 3, None),
    Among("viisime", -1, 3, None),
    Among("viisin", -1, 3, None),
    Among("viisite", -1, 3, None),
    Among("viite", -1, 3, None),
    Among("viivad", -1, 3, None),
    Among("võib", -1, 11, None),
    Among("võid", -1, 11, None),
    Among("võida", 274, 11, None),
    Among("võidakse", 275, 11, None),
    Among("võidi", 274, 11, None),
    Among("võiks", -1, 11, None),
    Among("võiksid", 278, 11, None),
    Among("võiksime", 278, 11, None),
    Among("võiksin", 278, 11, None),
    Among("võiksite", 278, 11, None),
    Among("võima", -1, 11, None),
    Among("võimata", 283, 11, None),
    Among("võime", -1, 11, None),
    Among("võin", -1, 11, None),
    Among("võis", -1, 11, None),
    Among("võite", -1, 11, None),
    Among("võivad", -1, 11, None),
];

static G_V1: &'static [u8; 20] = &[17, 65, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 48, 8];

static G_RV: &'static [u8; 3] = &[17, 65, 16];

static G_KI: &'static [u8; 36] = &[117, 66, 6, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 0, 0, 0, 16];

static G_GI: &'static [u8; 20] = &[21, 123, 243, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 48, 8];

fn r_mark_regions(env: &mut SnowballEnv, context: &mut Context) -> bool {
    context.i_p1 = env.limit;
    if !env.go_out_grouping(G_V1, 97, 252) {
        return false;
    }
    env.next_char();
    if !env.go_in_grouping(G_V1, 97, 252) {
        return false;
    }
    env.next_char();
    context.i_p1 = env.cursor;
    return true
}

fn r_emphasis(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    if env.cursor < context.i_p1 {
        return false;
    }
    let v_1 = env.limit_backward;
    env.limit_backward = context.i_p1;
    env.ket = env.cursor;
    if (env.cursor - 1 <= env.limit_backward || env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 105 as u8) {
        env.limit_backward = v_1;
        return false;
    }

    among_var = env.find_among_b(A_0, context);
    if among_var == 0 {
        env.limit_backward = v_1;
        return false;
    }
    env.bra = env.cursor;
    env.limit_backward = v_1;
    let v_2 = env.limit - env.cursor;
    if !env.hop_back(4) {
        return false;
    }
    env.cursor = env.limit - v_2;
    match among_var {
        1 => {
            let v_3 = env.limit - env.cursor;
            if !env.in_grouping_b(G_GI, 97, 252) {
                return false;
            }
            env.cursor = env.limit - v_3;
            let v_4 = env.limit - env.cursor;
            'lab0: loop {
                if !r_LONGV(env, context) {
                    break 'lab0;
                }
                return false;
            }
            env.cursor = env.limit - v_4;
            env.slice_del();
        }
        2 => {
            if !env.in_grouping_b(G_KI, 98, 382) {
                return false;
            }
            env.slice_del();
        }
        _ => ()
    }
    return true
}

fn r_verb(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    if env.cursor < context.i_p1 {
        return false;
    }
    let v_1 = env.limit_backward;
    env.limit_backward = context.i_p1;
    env.ket = env.cursor;
    if (env.cursor <= env.limit_backward || env.current.as_bytes()[(env.cursor - 1) as usize] as u8 >> 5 != 3 as u8 || ((540726 as i32 >> (env.current.as_bytes()[(env.cursor - 1) as usize] as u8 & 0x1f)) & 1) == 0) {
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
            env.slice_from("a");
        }
        3 => {
            if !env.in_grouping_b(G_V1, 97, 252) {
                return false;
            }
            env.slice_del();
        }
        _ => ()
    }
    return true
}

fn r_LONGV(env: &mut SnowballEnv, context: &mut Context) -> bool {
    return env.find_among_b(A_2, context) != 0;
}

fn r_i_plural(env: &mut SnowballEnv, context: &mut Context) -> bool {
    if env.cursor < context.i_p1 {
        return false;
    }
    let v_1 = env.limit_backward;
    env.limit_backward = context.i_p1;
    env.ket = env.cursor;
    if !env.eq_s_b(&"i") {
        env.limit_backward = v_1;
        return false;
    }
    env.bra = env.cursor;
    env.limit_backward = v_1;
    if !env.in_grouping_b(G_RV, 97, 117) {
        return false;
    }
    env.slice_del();
    return true
}

fn r_special_noun_endings(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    if env.cursor < context.i_p1 {
        return false;
    }
    let v_1 = env.limit_backward;
    env.limit_backward = context.i_p1;
    env.ket = env.cursor;
    if (env.cursor - 3 <= env.limit_backward || env.current.as_bytes()[(env.cursor - 1) as usize] as u8 >> 5 != 3 as u8 || ((1049120 as i32 >> (env.current.as_bytes()[(env.cursor - 1) as usize] as u8 & 0x1f)) & 1) == 0) {
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
            env.slice_from("lase");
        }
        2 => {
            env.slice_from("mise");
        }
        3 => {
            env.slice_from("lise");
        }
        _ => ()
    }
    return true
}

fn r_case_ending(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    if env.cursor < context.i_p1 {
        return false;
    }
    let v_1 = env.limit_backward;
    env.limit_backward = context.i_p1;
    env.ket = env.cursor;
    if (env.cursor <= env.limit_backward || env.current.as_bytes()[(env.cursor - 1) as usize] as u8 >> 5 != 3 as u8 || ((1576994 as i32 >> (env.current.as_bytes()[(env.cursor - 1) as usize] as u8 & 0x1f)) & 1) == 0) {
        env.limit_backward = v_1;
        return false;
    }

    among_var = env.find_among_b(A_4, context);
    if among_var == 0 {
        env.limit_backward = v_1;
        return false;
    }
    env.bra = env.cursor;
    env.limit_backward = v_1;
    match among_var {
        1 => {
            'lab0: loop {
                let v_2 = env.limit - env.cursor;
                'lab1: loop {
                    if !env.in_grouping_b(G_RV, 97, 117) {
                        break 'lab1;
                    }
                    break 'lab0;
                }
                env.cursor = env.limit - v_2;
                if !r_LONGV(env, context) {
                    return false;
                }
                break 'lab0;
            }
        }
        2 => {
            let v_3 = env.limit - env.cursor;
            if !env.hop_back(4) {
                return false;
            }
            env.cursor = env.limit - v_3;
        }
        _ => ()
    }
    env.slice_del();
    return true
}

fn r_plural_three_first_cases(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    if env.cursor < context.i_p1 {
        return false;
    }
    let v_1 = env.limit_backward;
    env.limit_backward = context.i_p1;
    env.ket = env.cursor;
    if (env.cursor <= env.limit_backward || (env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 100 as u8 && env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 101 as u8)) {
        env.limit_backward = v_1;
        return false;
    }

    among_var = env.find_among_b(A_6, context);
    if among_var == 0 {
        env.limit_backward = v_1;
        return false;
    }
    env.bra = env.cursor;
    env.limit_backward = v_1;
    match among_var {
        1 => {
            env.slice_from("iku");
        }
        2 => {
            let v_2 = env.limit - env.cursor;
            'lab0: loop {
                if !r_LONGV(env, context) {
                    break 'lab0;
                }
                return false;
            }
            env.cursor = env.limit - v_2;
            env.slice_del();
        }
        3 => {
            'lab1: loop {
                let v_3 = env.limit - env.cursor;
                'lab2: loop {
                    let v_4 = env.limit - env.cursor;
                    if !env.hop_back(4) {
                        break 'lab2;
                    }
                    env.cursor = env.limit - v_4;
                    if (env.cursor <= env.limit_backward || (env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 115 as u8 && env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 116 as u8)) {among_var = 2;}
                    else {
                        among_var = env.find_among_b(A_5, context);
                    }
                    match among_var {
                        1 => {
                            env.slice_from("e");
                        }
                        2 => {
                            env.slice_del();
                        }
                        _ => ()
                    }
                    break 'lab1;
                }
                env.cursor = env.limit - v_3;
                env.slice_from("t");
                break 'lab1;
            }
        }
        4 => {
            'lab3: loop {
                let v_5 = env.limit - env.cursor;
                'lab4: loop {
                    if !env.in_grouping_b(G_RV, 97, 117) {
                        break 'lab4;
                    }
                    break 'lab3;
                }
                env.cursor = env.limit - v_5;
                if !r_LONGV(env, context) {
                    return false;
                }
                break 'lab3;
            }
            env.slice_del();
        }
        _ => ()
    }
    return true
}

fn r_nu(env: &mut SnowballEnv, context: &mut Context) -> bool {
    if env.cursor < context.i_p1 {
        return false;
    }
    let v_1 = env.limit_backward;
    env.limit_backward = context.i_p1;
    env.ket = env.cursor;
    if (env.cursor - 1 <= env.limit_backward || (env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 97 as u8 && env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 117 as u8)) {
        env.limit_backward = v_1;
        return false;
    }

    if env.find_among_b(A_7, context) == 0 {
        env.limit_backward = v_1;
        return false;
    }
    env.bra = env.cursor;
    env.limit_backward = v_1;
    env.slice_del();
    return true
}

fn r_undouble_kpt(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    if !env.in_grouping_b(G_V1, 97, 252) {
        return false;
    }
    if context.i_p1 > env.cursor{
        return false;
    }
    env.ket = env.cursor;
    if (env.cursor - 1 <= env.limit_backward || env.current.as_bytes()[(env.cursor - 1) as usize] as u8 >> 5 != 3 as u8 || ((1116160 as i32 >> (env.current.as_bytes()[(env.cursor - 1) as usize] as u8 & 0x1f)) & 1) == 0) {
        return false;
    }

    among_var = env.find_among_b(A_8, context);
    if among_var == 0 {
        return false;
    }
    env.bra = env.cursor;
    match among_var {
        1 => {
            env.slice_from("k");
        }
        2 => {
            env.slice_from("p");
        }
        3 => {
            env.slice_from("t");
        }
        _ => ()
    }
    return true
}

fn r_degrees(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    if env.cursor < context.i_p1 {
        return false;
    }
    let v_1 = env.limit_backward;
    env.limit_backward = context.i_p1;
    env.ket = env.cursor;
    if (env.cursor <= env.limit_backward || env.current.as_bytes()[(env.cursor - 1) as usize] as u8 >> 5 != 3 as u8 || ((8706 as i32 >> (env.current.as_bytes()[(env.cursor - 1) as usize] as u8 & 0x1f)) & 1) == 0) {
        env.limit_backward = v_1;
        return false;
    }

    among_var = env.find_among_b(A_9, context);
    if among_var == 0 {
        env.limit_backward = v_1;
        return false;
    }
    env.bra = env.cursor;
    env.limit_backward = v_1;
    match among_var {
        1 => {
            if !env.in_grouping_b(G_RV, 97, 117) {
                return false;
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

fn r_substantive(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let v_1 = env.limit - env.cursor;
    r_special_noun_endings(env, context);
    env.cursor = env.limit - v_1;
    let v_2 = env.limit - env.cursor;
    r_case_ending(env, context);
    env.cursor = env.limit - v_2;
    let v_3 = env.limit - env.cursor;
    r_plural_three_first_cases(env, context);
    env.cursor = env.limit - v_3;
    let v_4 = env.limit - env.cursor;
    r_degrees(env, context);
    env.cursor = env.limit - v_4;
    let v_5 = env.limit - env.cursor;
    r_i_plural(env, context);
    env.cursor = env.limit - v_5;
    let v_6 = env.limit - env.cursor;
    r_nu(env, context);
    env.cursor = env.limit - v_6;
    return true
}

fn r_verb_exceptions(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    env.bra = env.cursor;
    among_var = env.find_among(A_10, context);
    if among_var == 0 {
        return false;
    }
    env.ket = env.cursor;
    if env.cursor < env.limit {
        return false;
    }
    match among_var {
        1 => {
            env.slice_from("joo");
        }
        2 => {
            env.slice_from("saa");
        }
        3 => {
            env.slice_from("viima");
        }
        4 => {
            env.slice_from("keesi");
        }
        5 => {
            env.slice_from("löö");
        }
        6 => {
            env.slice_from("lõi");
        }
        7 => {
            env.slice_from("loo");
        }
        8 => {
            env.slice_from("käisi");
        }
        9 => {
            env.slice_from("söö");
        }
        10 => {
            env.slice_from("too");
        }
        11 => {
            env.slice_from("võisi");
        }
        12 => {
            env.slice_from("jääma");
        }
        13 => {
            env.slice_from("müüsi");
        }
        14 => {
            env.slice_from("luge");
        }
        15 => {
            env.slice_from("põde");
        }
        16 => {
            env.slice_from("ladu");
        }
        17 => {
            env.slice_from("tegi");
        }
        18 => {
            env.slice_from("nägi");
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
    'lab0: loop {
        if !r_verb_exceptions(env, context) {
            break 'lab0;
        }
        return false;
    }
    env.cursor = v_1;
    let v_2 = env.cursor;
    r_mark_regions(env, context);
    env.cursor = v_2;
    env.limit_backward = env.cursor;
    env.cursor = env.limit;
    let v_3 = env.limit - env.cursor;
    r_emphasis(env, context);
    env.cursor = env.limit - v_3;
    let v_4 = env.limit - env.cursor;
    'lab1: loop {
        'lab2: loop {
            let v_5 = env.limit - env.cursor;
            'lab3: loop {
                if !r_verb(env, context) {
                    break 'lab3;
                }
                break 'lab2;
            }
            env.cursor = env.limit - v_5;
            r_substantive(env, context);
            break 'lab2;
        }
        break 'lab1;
    }
    env.cursor = env.limit - v_4;
    let v_6 = env.limit - env.cursor;
    r_undouble_kpt(env, context);
    env.cursor = env.limit - v_6;
    env.cursor = env.limit_backward;
    return true
}
