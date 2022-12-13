use pest::iterators::Pair;

use super::Answer;

#[derive(Clone, Debug, Eq, Ord)]
enum Packet {
    Number(i32),
    List(Vec<Packet>),
}

impl std::cmp::PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Number(l0), Self::Number(r0)) => l0 == r0,
            (Self::List(l0), Self::List(r0)) => l0 == r0,
            (Self::Number(l0), Self::List(r0)) => &vec![Packet::Number(*l0)] == r0,
            (Self::List(l0), Self::Number(r0)) => &vec![Packet::Number(*r0)] == l0,
        }
    }
}

impl std::cmp::PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Packet::Number(a), Packet::Number(b)) => Some(a.cmp(&b)),
            (Packet::Number(_), Packet::List(_)) => {
                Packet::List(vec![self.clone()]).partial_cmp(other)
            }
            (Packet::List(_), Packet::Number(_)) => {
                self.partial_cmp(&Packet::List(vec![other.clone()]))
            }
            (Packet::List(vec_a), Packet::List(vec_b)) => {
                for (a, b) in vec_a.iter().zip(vec_b.iter()) {
                    match a.partial_cmp(&b).unwrap() {
                        std::cmp::Ordering::Less => return Some(std::cmp::Ordering::Less),
                        std::cmp::Ordering::Equal => {}
                        std::cmp::Ordering::Greater => return Some(std::cmp::Ordering::Greater),
                    }
                }
                vec_a.len().partial_cmp(&vec_b.len())
            }
        }
    }
}

fn parse_rec(pair: Pair<Rule>) -> Packet {
    match pair.as_rule() {
        Rule::file => unimplemented!(),
        Rule::expression => parse_rec(pair.into_inner().next().unwrap()),
        Rule::list => Packet::List(pair.into_inner().map(|x| parse_rec(x)).collect()),
        Rule::number => Packet::Number(pair.as_str().parse().unwrap()),
    }
}

use pest::Parser;

#[derive(Parser)]
#[grammar = "grammars/p13.pest"]
pub struct ListParser;

pub fn solve(input: &str) -> Answer<usize, usize> {
    let file = ListParser::parse(Rule::file, input)
        .expect("unsuccessful parse")
        .next()
        .unwrap();

    let mut lists: Vec<_> = file.into_inner().map(parse_rec).collect();
    let mut in_order_pairs = Vec::new();
    for (i, chunk) in lists.chunks(2).enumerate() {
        let top = &chunk[0];
        let bottom = &chunk[1];
        match top.partial_cmp(bottom).unwrap() {
            std::cmp::Ordering::Less => in_order_pairs.push(i + 1),
            _ => {}
        }
    }
    let s1 = gen_sentinel_packet(2);
    let s2 = gen_sentinel_packet(6);
    lists.push(s1.clone());
    lists.push(s2.clone());
    lists.sort();
    let (idx1, _) = lists
        .iter()
        .enumerate()
        .find(|(_idx, x)| x == &&s1)
        .unwrap();
    let (idx2, _) = lists
        .iter()
        .enumerate()
        .find(|(_idx, x)| x == &&s2)
        .unwrap();
    Answer::new(
        in_order_pairs.into_iter().sum::<usize>(),
        (idx1 + 1) * (idx2 + 1),
    )
}

fn gen_sentinel_packet(inner: i32) -> Packet {
    Packet::List(vec![Packet::List(vec![Packet::Number(inner)])])
}
