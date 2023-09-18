enum PolishNode{
    Number(i32),
    Plus(Box<PolishNode>, Box<PolishNode>),
}

impl PolishNode{
    fn calc(&self) -> i32{
        match &self{
            PolishNode::Number(value) => *value,
            PolishNode::Plus(node1, node2) => node1.calc() + node2.calc(),
        }
    }
}

fn main() {
    let e0 = PolishNode::Number(10);
    let e1 = PolishNode::Number(21);
    let e2 = PolishNode::Plus(Box::new(e0), Box::new(e1));
    // assert_eq!(e0.calc(), 10);
    assert_eq!(e2.calc(),31);
}
