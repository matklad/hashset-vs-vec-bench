use rustc_hash::FxHashSet;

fn main() {
    let args: Vec<String> = std::env::args_os()
        .skip(1)
        .map(|it| it.to_str().unwrap_or_default().to_string())
        .collect();

    let n = args
        .get(1)
        .and_then(|it| it.parse::<usize>().ok())
        .unwrap_or(100_000_000);
    let range = (0..n).rev();

    let res = match args[0].as_str() {
        "vec" => range.collect::<Vec<_>>().len(),
        "hash" => range.collect::<FxHashSet<_>>().len(),
        "svec" => {
            let mut vec = range.collect::<Vec<_>>();
            vec.sort();
            vec.dedup();
            vec.len()
        }

        _ => panic!(),
    };
    assert_eq!(res, n)
}
