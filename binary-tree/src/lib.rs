#![allow(dead_code)]

use anyhow::Result;

#[derive(Debug, PartialEq, Eq)]
struct Tree {
    root: Option<Node>,
}

impl Tree {
    fn insert(&mut self, value: i32) -> Result<()> {
        match &mut self.root {
            None => {
                self.root = Some(Node::new(value));
                Ok(())
            }
            Some(n) => n.insert(value),
        }
    }

    fn search(&self, value: i32) -> Option<i32> {
        if let Some(root) = &self.root {
            if root.value == value {
                return Some(root.value);
            } else {
                return root.search(value);
            }
        }

        None
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(value: i32) -> Self {
        Self {
            left: None,
            right: None,
            value,
        }
    }

    fn search(&self, value: i32) -> Option<i32> {
        if value > self.value {
            self.search_right(value)
        } else {
            self.search_left(value)
        }
    }

    fn search_right(&self, value: i32) -> Option<i32> {
        if let Some(right) = &self.right {
            if right.value == value {
                return Some(right.value);
            } else {
                return right.search(value);
            }
        }

        None
    }

    fn search_left(&self, value: i32) -> Option<i32> {
        if let Some(left) = &self.left {
            if left.value == value {
                return Some(left.value);
            } else {
                return left.search(value);
            }
        }

        None
    }

    fn insert(&mut self, value: i32) -> Result<()> {
        if value > self.value {
            self.insert_right(value)
        } else {
            self.insert_left(value)
        }
    }

    fn insert_right(&mut self, value: i32) -> Result<()> {
        if let Some(right) = &mut self.right {
            right.insert(value)
        } else {
            self.right = Some(Box::new(Node::new(value)));
            Ok(())
        }
    }

    fn insert_left(&mut self, value: i32) -> Result<()> {
        if let Some(left) = &mut self.left {
            left.insert(value)
        } else {
            self.left = Some(Box::new(Node::new(value)));
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initialize_empty_tree() -> Result<()> {
        let mut t = Tree { root: None };
        t.insert(3)?;
        let expected = Tree {
            root: Some(Node::new(3)),
        };
        assert_eq!(expected, t);

        Ok(())
    }

    #[test]
    fn insert_into_existing_partial_tree() -> Result<()> {
        let mut t = Tree {
            root: Some(Node {
                value: 5,
                left: Some(Box::new(Node::new(3))),
                right: None,
            }),
        };

        t.insert(12)?;

        let expected = Tree {
            root: Some(Node {
                value: 5,
                left: Some(Box::new(Node::new(3))),
                right: Some(Box::new(Node::new(12))),
            }),
        };

        assert_eq!(expected, t);

        Ok(())
    }

    #[test]
    fn insert_into_existing_full_tree() -> Result<()> {
        let mut t = Tree {
            root: Some(Node {
                value: 5,
                left: Some(Box::new(Node::new(3))),
                right: Some(Box::new(Node::new(12))),
            }),
        };

        t.insert(15)?;
        t.insert(1)?;

        let expected = Tree {
            root: Some(Node {
                value: 5,
                left: Some(Box::new(Node {
                    value: 3,
                    left: Some(Box::new(Node::new(1))),
                    right: None,
                })),

                right: Some(Box::new(Node {
                    value: 12,
                    left: None,
                    right: Some(Box::new(Node {
                        value: 15,
                        left: None,
                        right: None,
                    })),
                })),
            }),
        };
        assert_eq!(expected, t);

        Ok(())
    }

    #[test]
    fn search_into_empty_tree() {
        let t = Tree { root: None };

        let found = t.search(3);

        assert_eq!(None, found);
    }

    #[test]
    fn search_inexisting_value() {
        let t = Tree {
            root: Some(Node {
                value: 5,
                left: Some(Box::new(Node {
                    value: 3,
                    left: Some(Box::new(Node::new(1))),
                    right: None,
                })),

                right: Some(Box::new(Node {
                    value: 12,
                    left: None,
                    right: Some(Box::new(Node {
                        value: 15,
                        left: None,
                        right: None,
                    })),
                })),
            }),
        };

        let found = t.search(21);
        assert_eq!(None, found);

    }

    #[test]
    fn search_existing_value() {
        let t = Tree {
            root: Some(Node {
                value: 5,
                left: Some(Box::new(Node {
                    value: 3,
                    left: Some(Box::new(Node::new(1))),
                    right: None,
                })),

                right: Some(Box::new(Node {
                    value: 12,
                    left: None,
                    right: Some(Box::new(Node {
                        value: 15,
                        left: None,
                        right: None,
                    })),
                })),
            }),
        };

        let found = t.search(15);
        assert_eq!(Some(15), found);

    }

}
