use std::collections::{HashMap, HashSet};

type Input<'a> = HashMap<&'a str, HashSet<&'a str>>;

pub fn parse(input: &str) -> Input {
    let mut adj: HashMap<&str, HashSet<&str>> = HashMap::new();

    for l in input.lines() {
        let mut parts = l.split("-");
        let a = parts.next().unwrap();
        let b = parts.next().unwrap();

        adj.entry(a).or_default().insert(b);
        adj.entry(b).or_default().insert(a);
    }

    adj
}

pub fn part1(input: &Input) -> u64 {
    triplets_with_t(input).len() as u64
}

pub fn part2(input: &Input) -> String {
    let r = input.keys().cloned().collect();
    let cliques = bron_kerbosch_first(input, HashSet::new(), r, HashSet::new());
    let mut clique = cliques
        .iter()
        .max_by(|a, b| a.len().cmp(&b.len()))
        .unwrap()
        .iter()
        .cloned()
        .collect::<Vec<&str>>();
    clique.sort();
    clique.join(",")
}

fn triplets_with_t<'a>(input: &'a Input) -> HashSet<[&'a str; 3]> {
    let mut triplets = HashSet::new();

    for (a, bs) in input {
        if a.starts_with('t') {
            for b1 in bs {
                for b2 in bs {
                    if b1 == b2 {
                        continue;
                    }

                    if let Some(cs) = input.get(b1) {
                        if cs.contains(b2) {
                            let mut xs = [*a, *b1, *b2];
                            xs.sort();

                            triplets.insert(xs);
                        }
                    }
                }
            }
        }
    }

    triplets
}

/// Bron-Kerbosch with short-circuit on first hit.
fn bron_kerbosch_first<'a>(
    adj: &HashMap<&'a str, HashSet<&'a str>>,
    r: HashSet<&'a str>,
    mut p: HashSet<&'a str>,
    mut x: HashSet<&'a str>,
) -> Vec<HashSet<&'a str>> {
    let mut out = Vec::new();

    if p.len() == 0 && x.len() == 0 {
        out.push(r);
        return out;
    }

    let ps = p.iter().cloned().collect::<Vec<_>>();
    for v in ps {
        let mut r = r.clone();
        r.insert(v);

        let n_v = adj.get(v).unwrap();

        out.extend(bron_kerbosch_first(
            adj,
            r,
            p.intersection(n_v).cloned().collect(),
            x.intersection(n_v).cloned().collect(),
        ));

        p.remove(v);
        x.insert(v);
    }

    out
}
