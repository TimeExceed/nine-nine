use rand::{Rng, thread_rng};

fn usage(err: i32) -> ! {
    println!("nine-nine OPTIONS");
    println!("OPTIONS:");
    println!("  -h, --help\tShow this help.");
    println!("  -p, --page\tHow many pages are going to generate.");
    std::process::exit(err)
}

fn parse_args() -> usize {
    let mut args = pico_args::Arguments::from_env();
    if args.contains("--help") || args.contains("-h") {
        usage(0);
    }
    let page = parse_long_short_arg(&mut args, "-p", "--page")
        .map(|s| {
            match s.parse::<usize>() {
                Ok(v) => v,
                Err(err) => {
                    println!("error on parsing -p/--page: {}", err);
                    std::process::exit(1)
                }
            }
        })
        .expect("-p/--page is required");
    page
}

fn parse_long_short_arg(
    args: &mut pico_args::Arguments,
    short_key: &'static str,
    long_key: &'static str,
) -> Option<String> {
    parse_single_arg(args, short_key)
        .or_else(move || {
            parse_single_arg(args, long_key)
        })
}

fn parse_single_arg(
    args: &mut pico_args::Arguments,
    key: &'static str,
) -> Option<String> {
    match args.opt_value_from_str(key) {
        Ok(x) => x,
        Err(err) => {
            println!("error on parsing {}: {}", key, err);
            std::process::exit(1)
        }
    }
}

fn gen_single<R: Rng>(rng: &mut R) -> i64 {
    let a: u8 = rng.gen();
    a as i64 % 20
}

fn gen_pair<R: Rng>(rng: &mut R) -> (i64, i64)  {
    (gen_single(rng), gen_single(rng))
}

fn gen_triple<R: Rng>(rng: &mut R) -> (i64, i64, i64) {
    (gen_single(rng), gen_single(rng), gen_single(rng))
}

fn gen_plus<R: Rng>(rng: &mut R) -> bool {
    let (a, b) = gen_pair(rng);
    if a + b > 20 {
        return false;
    }
    let xs = to_latex(&[a, b, a + b], rng);
    print!("${}+{}$&$={}$", xs[0], xs[1], xs[2]);
    true
}

fn gen_minus<R: Rng>(rng: &mut R) -> bool {
    let (a, b) = gen_pair(rng);
    if a < b {
        return false;
    }
    let xs = to_latex(&[a, b, a - b], rng);
    print!("${}-{}$&$={}$", xs[0], xs[1], xs[2]);
    true
}

fn to_latex<R: Rng>(xs: &[i64], rng: &mut R) -> Vec<String> {
    let n = xs.len();
    let j = rng.gen_range(0, n);
    let xs: Vec<_> = xs.iter()
        .enumerate()
        .map(|(i, x)| {
            if i == j {
                if i + 1 < n {
                    "\\fbox{\\textcolor{white}{00}}".to_string()
                } else {
                    "~".to_string()
                }
            } else {
                format!("{}", x)
            }
        })
        .collect();
    xs
}

fn gen_cmp<R: Rng>(rng: &mut R) -> bool {
    let (a, b) = gen_pair(rng);
    print!("{}&\\quad{}", a, b);
    true
}

fn gen_two_plus<R: Rng>(rng: &mut R) -> bool {
    let (a, b, c) = gen_triple(rng);
    if a + b + c > 20 {
        return false
    }
    print!("${}+{}+{}$&=", a, b, c);
    true
}

fn gen_two_minus<R: Rng>(rng: &mut R) -> bool {
    let (a, b, c) = gen_triple(rng);
    if a - b < 0 || a - b - c < 0 {
        return false;
    }
    print!("${}-{}-{}$&=", a, b, c);
    true
}

fn gen_equation<R: Rng>(rng: &mut R) {
    let eq_funcs = vec![
        Box::<fn(_: &mut R)->bool>::new(gen_plus::<R>),
        Box::new(gen_minus::<R>),
        Box::new(gen_two_plus::<R>),
        Box::new(gen_two_minus::<R>),
        // Box::new(gen_cmp::<R>),
    ];
    loop {
        let idx: usize = rng.gen_range(0, eq_funcs.len());
        let f = &eq_funcs[idx];
        let res = f(rng);
        if res {
            break
        }
    }
}

fn one_page<R: Rng>(rng: &mut R) {
    println!("\\begin{{tabular}}{{rl@{{\\qquad\\quad}}rl@{{\\qquad\\quad}}rl@{{\\qquad\\quad}}rl@{{\\qquad\\quad}}rl}}");
    for _ in 0..19 {
        for i in 0..5 {
            if i > 0 {
                print!("&");
            }
            gen_equation(rng);
        }
        println!("\\\\");
    }
    println!("\\end{{tabular}}");
    println!("\\clearpage");
}

fn main() {
    let mut rng = thread_rng();
    let page = parse_args();

    println!("\\documentclass[a4paper,landscape,notitlepage]{{article}}");
    println!("\\usepackage[a4paper,landscape,noheadfoot,left=1cm,top=1cm,right=1cm,bottom=1cm]{{geometry}}");
    println!("\\usepackage{{tabu}}");
    println!("\\usepackage{{amsmath}}");
    println!("\\usepackage{{color}}");
    println!("\\renewcommand{{\\baselinestretch}}{{1.5}}");
    println!("\\pagestyle{{empty}}");
    println!("\\begin{{document}}");
    println!("\\Large");
    for _ in 0..page {
        one_page(&mut rng);
    }
    println!("\\end{{document}}");
}
