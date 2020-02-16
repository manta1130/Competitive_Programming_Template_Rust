//提出時は下と一番下のコメントアウトを外す。
//mod flow {
use std;
type CapValue = u64;
const INF: CapValue = 999999999;

#[derive(Clone, Copy)]
struct Edge {
    to: usize,
    cap: CapValue,
    rev: usize,
}

///最大流を計算(Ford-Fulkersonのアルゴリズム)
pub struct FordFulkerson {
    n: usize,
    g: Vec<Vec<Edge>>,
    used: Vec<bool>,
}

impl FordFulkerson {
    ///要素数nのグラフを構築する。
    pub fn new(n: usize) -> FordFulkerson {
        let res = FordFulkerson {
            n: n,
            g: vec![vec![]; n],
            used: vec![],
        };
        res
    }
    fn dfs(&mut self, k: usize, t: usize, f: CapValue) -> CapValue {
        if k == t {
            return f;
        }
        self.used[k] = true;
        for i in 0..self.g[k].len() {
            let e = self.g[k][i];
            if !self.used[e.to] && e.cap > 0 {
                let r = self.dfs(e.to, t, std::cmp::min(f, e.cap));
                if r > 0 {
                    self.g[k][i].cap -= r;
                    self.g[e.to][e.rev].cap += r;
                    return r;
                }
            }
        }

        0
    }
    ///最大フローを求める。
    pub fn get_maxflow(&mut self, s: usize, t: usize) -> CapValue {
        let mut res = 0;
        loop {
            self.used = vec![false; self.n];
            let f = self.dfs(s, t, INF);
            if f == 0 {
                return res;
            }
            res += f;
        }
    }
    ///辺の追加
    ///
    /// f->tにvalueの容量を持つ辺を追加する。
    pub fn add_edge(&mut self, f: usize, t: usize, value: CapValue) {
        let idx_t = self.g[t].len();
        let idx_f = self.g[f].len();
        self.g[f].push(Edge {
            to: t,
            cap: value,
            rev: idx_t,
        });
        self.g[t].push(Edge {
            to: f,
            cap: 0,
            rev: idx_f,
        });
    }
}
//}
