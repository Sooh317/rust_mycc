use crate::tokenizer::Token;

#[derive(PartialEq, Eq, Debug)]
pub enum NodeKind {
    NDAdd, 
    NDSub, 
    NDMul, 
    NDDiv, 
    NDNum(i32),
}

#[derive(Debug)]
pub struct Node {
    pub kind : NodeKind,
    pub left_index : usize,
    pub right_index : usize,
}

impl Node {
    pub fn new(kind: NodeKind, left_index: usize, right_index: usize) -> Node {
        Node {
            kind, 
            left_index, 
            right_index,
        }
    }
    pub fn new_num(val : i32) -> Node {
        Node {
            kind : NodeKind::NDNum(val),
            left_index : std::usize::MAX,
            right_index : std::usize::MAX,
        }
    }

    fn expr(s : &str, tokens : &Vec<Token>, index : &mut usize, tree : &mut Vec<Node>) -> usize {
        let mut left_index = Node::mul(s, tokens, index, tree);
        loop {
            if Token::consume(s, &tokens[*index], index, &'+'.to_string()) {
                let right_index = Node::mul(s, tokens, index, tree);
                tree.push(Node::new(NodeKind::NDAdd, left_index, right_index));
            }   
            else if Token::consume(s, &tokens[*index], index, &'-'.to_string()) {
                let right_index = Node::mul(s, tokens, index, tree);
                tree.push(Node::new(NodeKind::NDSub, left_index, right_index));
            }   
            else {
                return tree.len() - 1;
            }
            left_index = tree.len() - 1;
        }
    }

    fn mul(s : &str, tokens : &Vec<Token>, index : &mut usize, tree : &mut Vec<Node>) -> usize {
        let mut left_index = Node::primary(s, tokens, index, tree);
        loop {
            if Token::consume(s, &tokens[*index], index, &'*'.to_string()) {
                let right_index = Node::primary(s, tokens, index, tree);
                tree.push(Node::new(NodeKind::NDMul, left_index, right_index));
            }   
            else if Token::consume(s, &tokens[*index], index, &'/'.to_string()) {
                let right_index = Node::primary(s, tokens, index, tree);
                tree.push(Node::new(NodeKind::NDDiv, left_index, right_index));
            }   
            else {
                return tree.len() - 1;
            }
            left_index = tree.len() - 1;
        }
    }

    fn primary(s : &str, tokens : &Vec<Token>, index : &mut usize, tree : &mut Vec<Node>) -> usize {
        let token = &tokens[*index];
        if Token::consume(s, token, index, &'('.to_string()) {
            let id = Node::expr(s, tokens, index, tree);

            let token = &tokens[*index];
            Token::expect(s, token, index, &')'.to_string());
            
            return id;
        }
        tree.push(Node::new_num(Token::expect_number(s, token, index)));
        return tree.len() - 1;
    }

    pub fn parse(s : &str, tokens : &Vec<Token>) -> Vec<Node> {
        let mut tree : Vec<Node> = Vec::new();
        let mut index = 0;
        Node::expr(s, tokens, &mut index, &mut tree);
        tree
    }
}