#[derive(Debug)]
pub struct Node {
    type: NodeType,
    value: Vec<Node>
}



#[derive(Debug)]
pub enum NodeType {
    Terminal(TerminalNodes),
    NonTerminal(NonTerminalNodes)
}


#[derive(Debug)]
pub enum TerminalNodes {

}

#[derive(Debug)]
pub enum NonTerminalNodes {

}
