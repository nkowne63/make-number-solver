use super::expression::Operator;

// 演算子のリストアップを行うイテレータ
pub(crate) struct OpsIter {
    // 残りの長さ
    length: usize,
    // 現在埋まっている演算子のリスト
    current: Vec<Operator>,
    // 残りのイテレータ
    iters: Vec<OpsIter>,
    // イテレータが終了しているかどうか
    finished: bool,
}

impl Iterator for OpsIter {
    type Item = Vec<Operator>;
    fn next(&mut self) -> Option<Self::Item> {
        // イテレータが終了している場合はNoneを返す
        if self.finished {
            return None;
        }
        // 残りの長さが0の場合は現在のリストを返す
        if self.length == 0 {
            self.finished = true;
            return Some(self.current.clone());
        }
        // イテレータが空の場合はセットする
        if self.iters.is_empty() {
            self.iters = Operator::list()
                .iter()
                .map(|op| OpsIter {
                    length: self.length - 1,
                    current: {
                        let mut current = self.current.clone();
                        current.push(*op);
                        current
                    },
                    iters: vec![],
                    finished: false,
                })
                .collect();
        }
        // 現在のイテレータから値を取り出す
        let current = self.iters[0].next();
        match current {
            // 値がある場合はそれを返す
            Some(current) => Some(current),
            None => {
                // 値がない場合はイテレータを削除する。
                self.iters.remove(0);
                if self.iters.is_empty() {
                    // イテレータが空になった場合はNoneを返す
                    self.finished = true;
                    None
                } else {
                    // イテレータが空になっていない場合は再帰的にnextを呼び出す
                    self.iters[0].next()
                }
            }
        }
    }
}

impl OpsIter {
    pub fn new(length: usize) -> OpsIter {
        OpsIter {
            length,
            current: vec![],
            iters: vec![],
            finished: false,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn listup() {
        let mut ops_iter = OpsIter::new(2);
        assert_eq!(ops_iter.next(), Some(vec![Operator::Add, Operator::Add]));
        assert_eq!(ops_iter.next(), Some(vec![Operator::Add, Operator::Sub]));
        assert_eq!(ops_iter.next(), Some(vec![Operator::Add, Operator::Mul]));
        assert_eq!(ops_iter.next(), Some(vec![Operator::Add, Operator::Div]));
        assert_eq!(ops_iter.next(), Some(vec![Operator::Sub, Operator::Add]));
        assert_eq!(ops_iter.next(), Some(vec![Operator::Sub, Operator::Sub]));
        assert_eq!(ops_iter.next(), Some(vec![Operator::Sub, Operator::Mul]));
        assert_eq!(ops_iter.next(), Some(vec![Operator::Sub, Operator::Div]));
        assert_eq!(ops_iter.next(), Some(vec![Operator::Mul, Operator::Add]));
        assert_eq!(ops_iter.next(), Some(vec![Operator::Mul, Operator::Sub]));
        assert_eq!(ops_iter.next(), Some(vec![Operator::Mul, Operator::Mul]));
        assert_eq!(ops_iter.next(), Some(vec![Operator::Mul, Operator::Div]));
        assert_eq!(ops_iter.next(), Some(vec![Operator::Div, Operator::Add]));
        assert_eq!(ops_iter.next(), Some(vec![Operator::Div, Operator::Sub]));
        assert_eq!(ops_iter.next(), Some(vec![Operator::Div, Operator::Mul]));
        assert_eq!(ops_iter.next(), Some(vec![Operator::Div, Operator::Div]));
        assert_eq!(ops_iter.next(), None);
    }
}
