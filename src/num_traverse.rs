/*
数字の列が分かれば計算の木の構造が分かる
-> 数字列のイテレータを用いて走査する

3*(8-3)+4
 2  3  1

3*5+4*2
 2 1 3
*/
// 数字の並び替え
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct NumberReplacer {
    // 数字列の並び替え
    pub(crate) numbers: Vec<u32>,
}

impl NumberReplacer {
    pub(crate) fn apply<T>(&self, values: Vec<T>) -> Vec<T>
    where
        T: Clone,
    {
        self.numbers
            .iter()
            .map(|n| values[*n as usize - 1].clone())
            .collect()
    }
}

// 数字の並び替え
pub(crate) struct NumberReplacerIter {
    // 入れることのできる数字の列
    numbers: Vec<u32>,
    // 確定した数値の列
    current: Vec<u32>,
    // 現在入っているイテレータ
    iters: Vec<NumberReplacerIter>,
    // イテレータが終了しているか
    finished: bool,
}

impl Iterator for NumberReplacerIter {
    type Item = NumberReplacer;
    fn next(&mut self) -> Option<Self::Item> {
        // イテレータが終了している場合はNoneを返す
        if self.finished {
            return None;
        }
        // 入れることのできる数字の列が空になっていたら確定しているものを返す
        if self.numbers.is_empty() {
            self.finished = true;
            return Some(NumberReplacer {
                numbers: self.current.clone(),
            });
        }
        // 現在入っているイテレータが空の場合はセットする
        if self.iters.is_empty() {
            self.iters = self
                .numbers
                .iter()
                .map(|n| NumberReplacerIter {
                    // numbersからnを取り除き、currentにnを追加する
                    numbers: {
                        let mut numbers = self.numbers.clone();
                        numbers.retain(|x| x != n);
                        numbers
                    },
                    current: {
                        let mut current = self.current.clone();
                        current.push(*n);
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

impl NumberReplacerIter {
    pub(crate) fn new(numbers: Vec<u32>) -> Self {
        NumberReplacerIter {
            numbers,
            current: vec![],
            iters: vec![],
            finished: false,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    impl NumberReplacer {
        pub(crate) fn new(numbers: Vec<u32>) -> Self {
            NumberReplacer { numbers }
        }
    }

    #[test]
    fn sample_replace() {
        let mut replacer = NumberReplacerIter::new(vec![1, 2, 3]);
        assert_eq!(replacer.next(), Some(NumberReplacer::new(vec![1, 2, 3])));
        assert_eq!(replacer.next(), Some(NumberReplacer::new(vec![1, 3, 2])));
        assert_eq!(replacer.next(), Some(NumberReplacer::new(vec![2, 1, 3])));
        assert_eq!(replacer.next(), Some(NumberReplacer::new(vec![2, 3, 1])));
        assert_eq!(replacer.next(), Some(NumberReplacer::new(vec![3, 1, 2])));
        assert_eq!(replacer.next(), Some(NumberReplacer::new(vec![3, 2, 1])));
        assert_eq!(replacer.next(), None);
    }
}
