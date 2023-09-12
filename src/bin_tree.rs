use std::collections::BTreeSet;

// 簡単な2分木
// leafとnodeだけあれば良い
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Node {
    // 末端
    Leaf,
    // そうでないやつ
    Node { left: Box<Node>, right: Box<Node> },
}

impl Node {
    // 末端のleafをnodeに変換するのをleafごとに繰り返してベクトル化する
    fn map_leaf_to_node(&self) -> Box<dyn Iterator<Item = Node> + '_> {
        match self {
            Node::Leaf => Box::new(
                vec![Node::Node {
                    left: Box::new(Node::Leaf),
                    right: Box::new(Node::Leaf),
                }]
                .into_iter(),
            ),
            Node::Node { left, right } => {
                // 左側を末端処理したもの
                let left_leaves = left.map_leaf_to_node();
                let left_leaves = left_leaves.map(|n| Node::Node {
                    left: Box::new(n.clone()),
                    right: right.clone(),
                });
                // 右側を末端処理したもの
                let right_leaves = right.map_leaf_to_node();
                let right_leaves = right_leaves.map(|n| Node::Node {
                    left: left.clone(),
                    right: Box::new(n.clone()),
                });
                Box::new(left_leaves.chain(right_leaves))
            }
        }
    }
    // leafの数がtargetに等しい木構造を全て返す
    pub(crate) fn get_all_tree(target: usize) -> BTreeSet<Node> {
        assert!(target >= 2, "tree size must be greater than 2");
        let node = Node::Leaf;
        let mut result = node.map_leaf_to_node().collect::<BTreeSet<_>>();
        let mut leaf_count = 2;
        while leaf_count < target {
            result = result.iter().flat_map(|n| n.map_leaf_to_node()).collect();
            // 置き換えを行うごとにleafの数は1つ増える
            leaf_count += 1;
        }
        result
    }
    // leafの数をカウントする
    pub(crate) fn count_leaf(&self) -> usize {
        match self {
            Node::Leaf => 1,
            Node::Node { left, right } => left.count_leaf() + right.count_leaf(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_all_tree_test() {
        let trees = Node::get_all_tree(4);
        assert_eq!(
            trees,
            vec![
                Node::Node {
                    left: Box::new(Node::Leaf),
                    right: Box::new(Node::Node {
                        left: Box::new(Node::Leaf),
                        right: Box::new(Node::Node {
                            left: Box::new(Node::Leaf),
                            right: Box::new(Node::Leaf),
                        }),
                    })
                },
                Node::Node {
                    left: Box::new(Node::Leaf),
                    right: Box::new(Node::Node {
                        left: Box::new(Node::Leaf),
                        right: Box::new(Node::Node {
                            left: Box::new(Node::Leaf),
                            right: Box::new(Node::Leaf),
                        }),
                    }),
                },
                Node::Node {
                    left: Box::new(Node::Leaf),
                    right: Box::new(Node::Node {
                        left: Box::new(Node::Node {
                            left: Box::new(Node::Leaf),
                            right: Box::new(Node::Leaf),
                        }),
                        right: Box::new(Node::Leaf),
                    }),
                },
                Node::Node {
                    left: Box::new(Node::Node {
                        left: Box::new(Node::Leaf),
                        right: Box::new(Node::Leaf),
                    }),
                    right: Box::new(Node::Node {
                        left: Box::new(Node::Leaf),
                        right: Box::new(Node::Leaf),
                    }),
                },
                Node::Node {
                    left: Box::new(Node::Node {
                        left: Box::new(Node::Leaf),
                        right: Box::new(Node::Node {
                            left: Box::new(Node::Leaf),
                            right: Box::new(Node::Leaf),
                        }),
                    }),
                    right: Box::new(Node::Leaf),
                },
                Node::Node {
                    left: Box::new(Node::Node {
                        left: Box::new(Node::Node {
                            left: Box::new(Node::Leaf),
                            right: Box::new(Node::Leaf),
                        }),
                        right: Box::new(Node::Leaf),
                    }),
                    right: Box::new(Node::Leaf),
                },
            ]
            .into_iter()
            .collect::<BTreeSet<_>>()
        );
    }
}
