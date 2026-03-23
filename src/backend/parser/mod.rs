use crate::backend::tokens::{Tokens, Token, Keywords};
use super::errors::Error;
use super::nodes::{Statement, Expression, Block, Identifier, FunctionDecl, LetDecl, WhileDecl};
pub mod handlers;

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}



impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0}
    }

    fn peek(&mut self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn next(&mut self) -> Option<Token> {
        let tok = self.tokens.get(self.pos).cloned();
        if tok.is_some() {
            self.pos += 1;
        }
        tok
    }

    pub fn parse_code(&mut self)  -> Result<Vec<Statement>, Error>{
        let mut statements = Vec::new();
        while let Some(token) = self.peek() {
            if token.kind == Tokens::EOF {
                break;
            }
            statements.push(self.parse_statement()?);
        }
        Ok(statements)
    }

    fn parse_statement(&mut self)  -> Result<Statement, Error> {
            match self.next() {
                Some(Token { kind: Tokens::Keyword(Keywords::Let), .. }) => Ok(self.parse_let()?),
                Some(Token { kind: Tokens::Keyword(Keywords::Fn), .. }) => Ok(self.parse_function()?),
                // Some(Token { kind: Tokens::Secondary(Secondary::Keyword(Keywords::While)), .. }) => self.parse_while(),
                // Some(Token { kind: Tokens::Secondary(Secondary::Keyword(Keywords::If)), .. }) => self.parse_if(),
                Some(Token { kind: Tokens::EOF, .. }) => Err(Error::UnexpectedEOF),
                _ => Err(Error::InvalidSyntax("Invalid Syntax".to_string()))
                // _ => {
                //     // let expr = self.parse_expression(Precedence::Lowest)?;
                //     // // Expect semicolon at end of expression statement
                //     // self.expect(|k| matches!(k, Tokens::NonAtomic(NonAtomic::SemiColon)))?;
                //     // Ok(Statement::Expression(expr))
                //     Ok(self.parse_let()?)
                //     }
            }
        }
}






//     fn parse_function(&mut self) -> Result<Statement, Error> {
//         self.expect(|k| matches!(k, Tokens::Secondary(Secondary::Keyword(Keywords::Fn))))?;

//         let name = match self.next() {
//             Some(Token { kind: Tokens::Secondary(Secondary::Variable { val }), .. }) => Identifier(val.clone()),
//             Some(t) => return Err(Error::UnexpectedToken {
//                 expected: Tokens::Secondary(Secondary::Variable { val: "".into() }),
//                 found: t.kind.clone(),
//             }),
//             None => return Err(Error::UnexpectedEOF),
//         };

//         self.expect(|k| matches!(k, Tokens::NonAtomic(NonAtomic::Paren(SymbolVal::Open))))?;

//         let mut args = Vec::new();
//         while let Some(Token { kind: Tokens::Secondary(Secondary::Variable { val }), .. }) = self.peek() {
//             args.push(Identifier(val.clone()));
//             self.next();
//             if let Some(Token { kind: Tokens::NonAtomic(NonAtomic::Comma), .. }) = self.peek() {
//                 self.next();
//             } else {
//                 break;
//             }
//         }

//         self.expect(|k| matches!(k, Tokens::NonAtomic(NonAtomic::Paren(SymbolVal::Close))))?;
//         let body = self.parse_block()?;

//         Ok(Statement::Function(Box::new(FunctionDecl { name, args, body })))
//     }

//     fn parse_let(&mut self) -> Result<Statement, Error> {
//         self.expect(|k| matches!(k, Tokens::Secondary(Secondary::Keyword(Keywords::Let))))?;

//         let name = match self.next() {
//             Some(Token { kind: Tokens::Secondary(Secondary::Variable { val }), .. }) => Identifier(val.clone()),
//             Some(t) => return Err(Error::UnexpectedToken {
//                 expected: Tokens::Secondary(Secondary::Variable { val: "".into() }),
//                 found: t.kind.clone(),
//             }),
//             None => return Err(Error::UnexpectedEOF),
//         };

//         self.expect(|k| matches!(k, Tokens::NonAtomic(NonAtomic::Assignment)))?;
//         let value = self.parse_expression(Precedence::Lowest)?;
//         self.expect(|k| matches!(k, Tokens::NonAtomic(NonAtomic::SemiColon)))?;

//         Ok(Statement::Let(Box::new(LetDecl { name, value })))
//     }

//     fn parse_while(&mut self) -> Result<Statement, Error> {
//         self.expect(|k| matches!(k, Tokens::Secondary(Secondary::Keyword(Keywords::While))))?;
//         let condition = self.parse_expression(Precedence::Lowest)?;
//         let body = self.parse_block()?;
//         Ok(Statement::While(Box::new(WhileDecl { condition, body })))
//     }

//     fn parse_block(&mut self) -> Result<Block, Error> {
//         self.expect(|k| matches!(k, Tokens::NonAtomic(NonAtomic::CurlyBraces(SymbolVal::Open))))?;
//         let mut statements = Vec::new();

//         while let Some(Token { kind, .. }) = self.peek() {
//             if matches!(kind, Tokens::NonAtomic(NonAtomic::CurlyBraces(SymbolVal::Close))) {
//                 break;
//             }
//             statements.push(self.parse_statement()?);
//         }

//         self.expect(|k| matches!(k, Tokens::NonAtomic(NonAtomic::CurlyBraces(SymbolVal::Close))))?;
//         Ok(Block { statements })
//     }

//     fn parse_if(&mut self) -> Result<Statement, Error> {
//         self.expect(|k| matches!(k, Tokens::Secondary(Secondary::Keyword(Keywords::If))))?;
//         self.expect(|k| matches!(k, Tokens::NonAtomic(NonAtomic::Paren(SymbolVal::Open))))?;
//         let condition = self.parse_expression(Precedence::Lowest)?;
//         self.expect(|k| matches!(k, Tokens::NonAtomic(NonAtomic::Paren(SymbolVal::Close))))?;
//         let body = self.parse_block()?;

//         let else_body = if let Some(Token { kind: Tokens::Secondary(Secondary::Keyword(Keywords::Else)), .. }) = self.peek() {
//             self.next();
//             Some(self.parse_block()?)
//         } else {
//             None
//         };

//         Ok(Statement::If { condition, body, else_body })
//     }

//     // ===== Pratt parser =====

//     fn parse_expression(&mut self, precedence: Precedence) -> Result<Expression, Error> {
//         let mut left = self.parse_prefix()?;

//         while let Some(tok) = self.peek() {
//             match tok.kind {
//                 Tokens::NonAtomic(NonAtomic::SemiColon)
//                 | Tokens::NonAtomic(NonAtomic::Comma)
//                 | Tokens::NonAtomic(NonAtomic::Paren(SymbolVal::Close))
//                 | Tokens::NonAtomic(NonAtomic::CurlyBraces(SymbolVal::Close)) => break,
//                 _ => {}
//             }

//             let tok_prec = self.get_precedence(&tok.kind);
//             if tok_prec <= precedence {
//                 break;
//             }

//             match &tok.kind {
//                 Tokens::Atomic(_) => left = self.parse_infix(left)?,
//                 Tokens::NonAtomic(NonAtomic::Paren(SymbolVal::Open)) => left = self.parse_call_expression(left)?,
//                 _ => break,
//             }
//         }

//         Ok(left)
//     }

//     fn parse_prefix(&mut self) -> Result<Expression, Error> {
//         let tok = self.next().ok_or(Error::UnexpectedEOF)?;
//         match &tok.kind {
//             Tokens::Primary(p) => Ok(Expression::Literal(p.clone())),
//             Tokens::Secondary(Secondary::Variable { val }) => Ok(Expression::Variable(Identifier(val.clone()))),
//             Tokens::Atomic(op) => Ok(Expression::Unary { op: op.clone(), expr: Box::new(self.parse_prefix()?) }),
//             Tokens::NonAtomic(NonAtomic::Paren(SymbolVal::Open)) => {
//                 let expr = self.parse_expression(Precedence::Lowest)?;
//                 self.expect(|k| matches!(k, Tokens::NonAtomic(NonAtomic::Paren(SymbolVal::Close))))?;
//                 Ok(expr)
//             }
//             _ => Err(Error::UnexpectedToken { expected: tok.kind.clone(), found: tok.kind.clone() }),
//         }
//     }

//     fn parse_infix(&mut self, left: Expression) -> Result<Expression, Error> {
//         let op_tok = self.next().ok_or(Error::UnexpectedEOF)?.clone();
//         let precedence = self.get_precedence(&op_tok.kind);
//         let right = self.parse_expression(precedence)?;
//         Ok(Expression::Binary {
//             left: Box::new(left),
//             op: match &op_tok.kind {
//                 Tokens::Atomic(o) => o.clone(),
//                 _ => return Err(Error::UnexpectedToken { expected: op_tok.kind.clone(), found: op_tok.kind.clone() }),
//             },
//             right: Box::new(right),
//         })
//     }

//     fn parse_call_expression(&mut self, callee: Expression) -> Result<Expression, Error> {
//         self.expect(|k| matches!(k, Tokens::NonAtomic(NonAtomic::Paren(SymbolVal::Open))))?;
//         let mut args = Vec::new();

//         if let Some(Token { kind: Tokens::NonAtomic(NonAtomic::Paren(SymbolVal::Close)), .. }) = self.peek() {
//             self.next(); // empty args
//         } else {
//             loop {
//                 args.push(self.parse_expression(Precedence::Lowest)?);
//                 if let Some(Token { kind: Tokens::NonAtomic(NonAtomic::Comma), .. }) = self.peek() {
//                     self.next();
//                 } else {
//                     break;
//                 }
//             }
//             self.expect(|k| matches!(k, Tokens::NonAtomic(NonAtomic::Paren(SymbolVal::Close))))?;
//         }

//         Ok(Expression::Call {
//             callee: match callee {
//                 Expression::Variable(id) => id,
//                 _ => return Err(Error::InvalidCall),
//             },
//             args,
//         })
//     }

//     fn get_precedence(&self, tok: &Tokens) -> Precedence {
//         match tok {
//             Tokens::Atomic(Operations::Logical(op)) => match op {
//                 crate::backend::tokens::LogicalOp::Or => Precedence::LogicalOr,
//                 crate::backend::tokens::LogicalOp::And => Precedence::LogicalAnd,
//                 _ => Precedence::Lowest,
//             },
//             Tokens::Atomic(Operations::Comparison(_)) => Precedence::Comparison,
//             Tokens::Atomic(Operations::Arithmetic(op)) => match op {
//                 crate::backend::tokens::ArithmeticOp::Add | crate::backend::tokens::ArithmeticOp::Sub => Precedence::Sum,
//                 crate::backend::tokens::ArithmeticOp::Mul | crate::backend::tokens::ArithmeticOp::Div => Precedence::Product,
//             },
//             _ => Precedence::Lowest,
//         }
//     }
