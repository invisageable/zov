use zov_ast::ast::{Ast, Node, NodeKind, Punctuation, Word};
use zov_reporter::Result;
use zov_tokens::token::{Token, TokenKind};

/// The representation of Parser.
#[derive(Debug)]
pub struct Parser<'tokens> {
  /// A current token.
  token_current: Token,
  /// A next token.
  token_next: Token,
  /// The index of the token slice.
  idx: usize,
  /// A group of tokens — see also [`Token`] for more information.
  tokens: &'tokens [Token],
}

impl<'tokens> Parser<'tokens> {
  /// Creates a new [`Parser`] instance.
  pub fn new(tokens: &'tokens [Token]) -> Self {
    Self {
      token_current: Token::EOF,
      token_next: Token::EOF,
      idx: 0usize,
      tokens,
    }
  }

  /// Checks end of file.
  #[inline(always)]
  fn at_eof(&self) -> bool {
    self.idx < self.tokens.len()
  }

  /// Checks if the current token is a specific kind.
  #[inline]
  pub(crate) fn ensure(&self, kind: TokenKind) -> bool {
    self.token_current.is(kind)
  }

  /// Peeks ahead in the token stram to look at a token based of the index.
  #[inline(always)]
  fn peek(&self) -> Option<&'tokens Token> {
    self.tokens.get(self.idx)
  }
}

impl<'tokens> Parser<'tokens> {
  /// Creates a new [`Parser`] instance.
  pub fn parse(&mut self) -> Result<Ast> {
    let mut ast = Ast::new(Vec::with_capacity(0usize));

    self.next();
    self.next();

    while self.at_eof() {
      match self.parse_node() {
        Ok(node) => ast.add_node(node),
        Err(error) => panic!("{error:?}"),
      }

      self.next();
    }

    Ok(ast)
  }

  /// ...
  fn parse_node(&mut self) -> Result<Node> {
    match &self.token_current.kind {
      TokenKind::Space => self.parse_node_space(),
      TokenKind::Newline => self.parse_node_newline(),
      TokenKind::Punctuation(_) => self.parse_node_punctuation(),
      TokenKind::Word(_) => self.parse_node_word(),
      _ => panic!(),
    }
  }

  /// ...
  fn parse_node_space(&mut self) -> Result<Node> {
    Ok(Node {
      kind: NodeKind::Whitespace,
    })
  }

  /// ...
  fn parse_node_newline(&mut self) -> Result<Node> {
    Ok(Node {
      kind: NodeKind::LineBreak,
    })
  }

  /// ...
  fn parse_node_punctuation(&mut self) -> Result<Node> {
    if let TokenKind::Punctuation(punctuation) = &self.token_current.kind {
      return Ok(Node {
        kind: NodeKind::Punctuation(Punctuation::from(punctuation)),
      });
    };

    panic!()
  }

  /// ...
  fn parse_node_word(&mut self) -> Result<Node> {
    if let TokenKind::Word(word) = &self.token_current.kind {
      return Ok(Node {
        kind: NodeKind::Word(Word::from(word)),
      });
    }

    panic!()
  }
}

impl<'tokens> Iterator for Parser<'tokens> {
  type Item = &'tokens Token;

  /// Moves to the next token.
  fn next(&mut self) -> Option<Self::Item> {
    std::mem::swap(&mut self.token_current, &mut self.token_next);

    self.peek().inspect(|token| {
      self.idx += 1;
      self.token_next = token.clone().clone(); // note(ivs) — this is bad.
    })
  }
}

/// ...
pub fn parse(tokens: Vec<Token>) -> Result<Ast> {
  Parser::new(&tokens).parse()
}
