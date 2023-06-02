use honggfuzz::fuzz;
use textdistance::{nstr, str};

type AlgFn = dyn Fn(&str, &str) -> f64;

fn main() {
    let algs: Vec<(&str, Box<AlgFn>)> = vec![
        ("bag", Box::new(nstr::bag)),
        ("cosine", Box::new(nstr::cosine)),
        ("damerau_levenshtein", Box::new(nstr::damerau_levenshtein)),
        (
            "damerau_levenshtein_restricted",
            Box::new(nstr::damerau_levenshtein_restricted),
        ),
        ("entropy_ncd", Box::new(nstr::entropy_ncd)),
        ("hamming", Box::new(nstr::hamming)),
        ("jaccard", Box::new(nstr::jaccard)),
        ("jaro_winkler", Box::new(nstr::jaro_winkler)),
        ("jaro", Box::new(nstr::jaro)),
        ("lcsseq", Box::new(nstr::lcsseq)),
        ("lcsstr", Box::new(nstr::lcsstr)),
        ("length", Box::new(nstr::length)),
        ("levenshtein", Box::new(nstr::levenshtein)),
        ("lig3", Box::new(nstr::lig3)),
        ("mlipns", Box::new(nstr::mlipns)),
        ("overlap", Box::new(nstr::overlap)),
        ("prefix", Box::new(nstr::prefix)),
        ("ratcliff_obershelp", Box::new(nstr::ratcliff_obershelp)),
        ("roberts", Box::new(nstr::roberts)),
        ("sift4_common", Box::new(nstr::sift4_common)),
        ("sift4_simple", Box::new(nstr::sift4_simple)),
        ("smith_waterman", Box::new(nstr::smith_waterman)),
        ("sorensen_dice", Box::new(nstr::sorensen_dice)),
        ("suffix", Box::new(nstr::suffix)),
        ("tversky", Box::new(nstr::tversky)),
        ("yujian_bo", Box::new(nstr::yujian_bo)),
    ];

    loop {
        fuzz!(|data: (&[u8], &[u8])| {
            let (d1, d2) = data;
            let s1 = std::str::from_utf8(d1).unwrap();
            let s2 = std::str::from_utf8(d2).unwrap();
            for (_alg_name, alg_fn) in &algs {
                let a = s1.to_owned().to_string();
                let b = s2.to_owned().to_string();
                alg_fn(&a, &b);
            }
        });
    }
}