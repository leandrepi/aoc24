use std::{
    collections::{HashMap, HashSet},
    fs,
};
const COMP_LEN: usize = 2;
const MAIN_KEY: u8 = b't';
type Vertex = (u8, u8);
type Edge = (Vertex, Vertex);
type SubGraph = Vec<Vertex>;
type Graph = HashMap<Vertex, SubGraph>;

fn parse_input() -> Result<Vec<Edge>, ()> {
    let raw = fs::read_to_string("input.txt")
        .map_err(|e| eprintln!("ERROR: Failed to read file: {e}"))?;

    let mut rules = vec![];
    for line in raw.lines().map(|l| l.trim()).filter(|l| !l.is_empty()) {
        let splits: Vec<String> = line.split('-').map(|s| s.trim().to_owned()).collect();
        if splits.len() != 2 {
            eprintln!("ERROR: invalid rule, should match wx-yz");
            return Err(());
        }
        let fst: Vec<u8> = splits[0].bytes().collect();
        let snd: Vec<u8> = splits[1].bytes().collect();
        if fst.len() != COMP_LEN || snd.len() != COMP_LEN {
            eprintln!("ERROR: invalid rule, should match wx-yz");
            return Err(());
        }
        rules.push(((fst[0], fst[1]), (snd[0], snd[1])));
    }
    Ok(rules)
}

fn vertex_graph(rules: &[Edge]) -> Graph {
    let mut computers = HashMap::new();
    for &(fst, snd) in rules {
        computers
            .entry(fst)
            .and_modify(|v: &mut Vec<_>| v.push(snd))
            .or_insert(vec![snd]);
        computers
            .entry(snd)
            .and_modify(|v| v.push(fst))
            .or_insert(vec![fst]);
    }
    computers
}

fn find_three_cycles(graph: &Graph) -> usize {
    let mut res = 0;
    let mut three_cycles = HashSet::new();
    for (c, deps) in graph {
        let mut cur = vec![c];
        for d in deps {
            let cand = graph.get(d).unwrap();
            for v in cand {
                if v != c && graph.get(v).unwrap().contains(c) {
                    cur.push(d);
                    cur.push(v);
                    cur.sort();
                    three_cycles.insert((cur[0], cur[1], cur[2]));
                    cur = vec![c];
                }
            }
        }
    }

    // only count three-cycle with any vertex starting with 't'
    // we could do this check prior to insertion into the set but whatever,
    // the current solution is fast enough
    for (&(a, _), &(b, _), &(c, _)) in three_cycles {
        if [a, b, c].iter().any(|&c| c == MAIN_KEY) {
            res += 1;
        }
    }
    res
}

fn bron_kerbosch(
    graph: &Graph,
    r: &HashSet<Vertex>,
    p: &mut HashSet<Vertex>,
    x: &mut HashSet<Vertex>,
    cliques: &mut Vec<SubGraph>,
) {
    if p.is_empty() && x.is_empty() {
        cliques.push(r.iter().copied().collect::<SubGraph>());
        return;
    }

    // pick pivot as the vertex from P U X of maximal degree
    let pivot_candidates = p.union(x);
    let pivot = pivot_candidates
        .max_by(|v1, v2| {
            graph
                .get(v1)
                .unwrap()
                .len()
                .cmp(&graph.get(v2).unwrap().len())
        })
        .unwrap();

    // we need a lot of local variables to avoid borrowing issues...
    let pivot_neighbors = graph
        .get(pivot)
        .unwrap()
        .iter()
        .copied()
        .collect::<HashSet<Vertex>>();
    let p_without_pivot_neighbors = p
        .difference(&pivot_neighbors)
        .copied()
        .collect::<Vec<Vertex>>();

    for v in &p_without_pivot_neighbors {
        let v_neigh = graph
            .get(v)
            .unwrap()
            .iter()
            .copied()
            .collect::<HashSet<Vertex>>();
        let mut new_r = r.clone();
        new_r.insert(*v);
        let mut new_p = p.intersection(&v_neigh).copied().collect();
        let mut new_x = x.intersection(&v_neigh).copied().collect();
        bron_kerbosch(graph, &new_r, &mut new_p, &mut new_x, cliques);
        p.remove(v);
        x.insert(*v);
    }
}

fn find_max_clique(graph: &Graph) -> SubGraph {
    let mut cliques = vec![];
    let mut p = graph.iter().map(|(c, _)| *c).collect();
    bron_kerbosch(
        graph,
        &HashSet::new(),
        &mut p,
        &mut HashSet::new(),
        &mut cliques,
    );
    cliques
        .iter()
        .max_by(|x, y| x.len().cmp(&y.len()))
        .unwrap()
        .to_vec()
}

fn to_password(computers: &[Vertex]) -> String {
    let mut names = computers
        .iter()
        .map(|&(a, b)| format!("{}{}", a as char, b as char))
        .collect::<Vec<String>>();
    names.sort();
    names.join(",")
}

fn main() {
    let rules = parse_input().unwrap();
    let graph = vertex_graph(&rules);
    let fst = find_three_cycles(&graph);
    println!("Day 23, part 1: {fst}");
    let snd = find_max_clique(&graph);
    println!("Day 23, part 2: {}", to_password(&snd));
}
