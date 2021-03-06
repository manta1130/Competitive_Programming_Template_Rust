//提出時は下と一番下のコメントアウトを外す。
//mod segtree {

///汎用のセグメント木
pub struct SegTreeBasic<T, F> {
    //n: usize,
    v: Vec<T>,
    init: T,
    parent_generator: F,
}

impl<T, F> SegTreeBasic<T, F>
where
    T: PartialOrd + Clone + Copy,
    F: Fn(T, T) -> T,
{
    ///セグメント木を構築する。
    ///
    /// # Examples
    /// ```ignore
    /// let mut segtree = SegTreeBasic::new(100, usize::max_value(), |a, b| std::cmp::min(a, b));
    /// //要素数100のRMQを構築する。
    /// ```
    pub fn new(n: usize, init: T, parent_generator: F) -> SegTreeBasic<T, F> {
        let mut size = 1;
        while size < n {
            size *= 2;
        }
        let v = vec![init; size * 2 - 1];
        SegTreeBasic {
            //n: size,
            v: v,
            init: init,
            parent_generator: parent_generator,
        }
    }

    ///[l,r)の要素をparent_generatorに基づいて計算する。
    pub fn get(&mut self, l: usize, r: usize) -> T {
        let vlen = self.v.len();
        self._get(l, r, 0, 0, (vlen + 1) / 2)
    }

    fn _get(&mut self, l: usize, r: usize, now: usize, a: usize, b: usize) -> T {
        if a >= r || b <= l {
            self.init
        } else if l <= a && b <= r {
            self.v[now]
        } else {
            let bufa = self._get(l, r, now * 2 + 1, a, (a + b) / 2);
            let bufb = self._get(l, r, now * 2 + 2, (a + b) / 2, b);
            let cond = &self.parent_generator;
            cond(bufa, bufb)
        }
    }

    ///任意の要素を書き換える。
    pub fn update(&mut self, index: usize, value: T) {
        let offset = (self.v.len() + 1) / 2 - 1;
        self.v[offset + index] = value;
        let mut parent = index + offset;
        loop {
            parent = (parent - 1) / 2;
            let cond = &self.parent_generator;
            self.v[parent] = cond(self.v[parent * 2 + 1], self.v[parent * 2 + 2]);
            if parent == 0 {
                break;
            }
        }
    }
}

///汎用の遅延セグメント木
pub struct SegTreeLazy<T, F, G, H> {
    //n: usize,
    v: Vec<T>,
    lazy: Vec<T>,
    lazy_flag: Vec<bool>,
    init: T,
    lazy_init: T,
    lazy_convert: F,
    lazy_propagation: G,
    parent_generator: H,
}

impl<T, F, G, H> SegTreeLazy<T, F, G, H>
where
    T: PartialOrd + Clone + Copy,
    F: Fn(T, T) -> T,
    G: Fn(T, T) -> T,
    H: Fn(T, T) -> T,
{
    ///遅延セグメント木を構築する。
    ///
    /// # Examples
    /// ```ignore
    ///　let mut segtree = SegTreeLazy::new(100, 0, 0, |a, b| a + b, |a, b| a + b, |a, b| a + b / 2);
    /// //要素数100の区間加算・区間取得のセグ木を構築する。
    /// ```
    pub fn new(
        n: usize,
        init: T,
        lazy_init: T,
        parent_generator: H,
        lazy_convert: F,
        lazy_propagation: G,
    ) -> SegTreeLazy<T, F, G, H> {
        let mut size = 1;
        while size < n {
            size *= 2;
        }
        let v = vec![init; size * 2 - 1];
        let lazy = vec![lazy_init; size * 2 - 1];
        let lazy_flag = vec![false; size * 2 - 1];
        SegTreeLazy {
            v: v,
            lazy: lazy,
            init: init,
            lazy_init: lazy_init,
            lazy_flag: lazy_flag,
            lazy_convert: lazy_convert,
            lazy_propagation: lazy_propagation,
            parent_generator: parent_generator,
        }
    }

    ///[l,r)の要素をparent_generatorに基づいて計算する。
    pub fn get(&mut self, l: usize, r: usize) -> T {
        let vlen = self.v.len();
        self._get(l, r, 0, 0, (vlen + 1) / 2)
    }

    fn _eval(&mut self, now: usize) {
        if now >= self.lazy_flag.len() {
            return;
        }

        if now * 2 + 2 < self.lazy.len() && self.lazy[now] != self.lazy_init {
            let propagation = &self.lazy_propagation;
            self.lazy[now * 2 + 1] = propagation(self.lazy[now * 2 + 1], self.lazy[now]);
            self.lazy[now * 2 + 2] = propagation(self.lazy[now * 2 + 2], self.lazy[now]);
        }
        if self.lazy[now] != self.lazy_init {
            let convert = &self.lazy_convert;
            self.v[now] = convert(self.v[now], self.lazy[now]);
        }
        self.lazy[now] = self.lazy_init;
        if self.lazy_flag[now] {
            self._eval(now * 2 + 1);
            self._eval(now * 2 + 2);
            let generator = &self.parent_generator;
            if now * 2 + 2 < self.v.len() {
                self.v[now] = generator(self.v[now * 2 + 1], self.v[now * 2 + 2]);
            }
        }
        self.lazy_flag[now] = false;
    }

    fn _get(&mut self, l: usize, r: usize, now: usize, a: usize, b: usize) -> T {
        self._eval(now);
        if a >= r || b <= l {
            self.init
        } else if l <= a && b <= r {
            self.v[now]
        } else {
            let bufa = self._get(l, r, now * 2 + 1, a, (a + b) / 2);
            let bufb = self._get(l, r, now * 2 + 2, (a + b) / 2, b);
            let cond = &self.parent_generator;
            cond(bufa, bufb)
        }
    }

    ///任意の要素を書き換える。
    pub fn update(&mut self, l: usize, r: usize, v: T) {
        let vlen = self.v.len();
        self._update(l, r, 0, 0, (vlen + 1) / 2, v);
    }

    fn _update(&mut self, l: usize, r: usize, now: usize, a: usize, b: usize, v: T) {
        if now < self.v.len() {
            self._eval(now);
        }
        if l <= a && b <= r {
            let propagation = &self.lazy_propagation;
            self.lazy[now] = propagation(self.lazy[now], v);
            self.lazy_flag[now] = true;
        } else if !(a >= r || b <= l) {
            let v_next;
            {
                let propagation = &self.lazy_propagation;
                v_next = propagation(self.lazy_init, v);
            }
            self._update(l, r, now * 2 + 1, a, (a + b) / 2, v_next);
            self._update(l, r, now * 2 + 2, (a + b) / 2, b, v_next);
            self.lazy_flag[now] = true;
        }
    }
}
//}
