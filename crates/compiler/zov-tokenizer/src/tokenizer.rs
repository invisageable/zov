use super::cursor::Cursor;
use super::state::State;

use zov_reporter::Result;
use zov_tokens::token::punctuation::Punctuation;
use zov_tokens::token::word::Word;
use zov_tokens::token::{Token, TokenKind};

pub struct Tokenizer {
  /// A cursor.
  cursor: Cursor,
  /// A tokenizer state.
  state: State,
  /// A flag to check if we have reconsume the current character.
  reconsume: bool,
  /// A current character.
  char_current: char,
}

impl Tokenizer {
  pub fn new(source: &str) -> Self {
    Self {
      cursor: Cursor::new(source),
      state: State::Data,
      reconsume: false,
      char_current: '\0',
    }
  }

  /// Gets the next character.
  fn get_char(&mut self) -> Option<char> {
    if self.reconsume {
      self.reconsume = false;

      Some(self.char_current)
    } else {
      self
        .cursor
        .peek()
        .and_then(|ch| self.get_preprocessed_char(ch))
    }
  }

  /// Gets the next character after preprocessing.
  fn get_preprocessed_char(&mut self, ch: char) -> Option<char> {
    self.char_current = ch;

    Some(ch)
  }

  /// Tokenizes a source code into a stream of tokens.
  pub fn tokenize(&mut self) -> Result<Vec<Token>> {
    let mut tokens = Vec::with_capacity(0usize);

    while self.cursor.pos() < self.cursor.source().len() {
      tokens.push(self.next()?);
    }

    // tokens.push(Token::eof(Span::of(self.cursor.pos(), self.cursor.pos())));

    Ok(tokens)
  }
}

impl Tokenizer {
  /// Consomes the current character.
  pub fn next(&mut self) -> Result<Token> {
    let mut pos = self.cursor.pos();

    while let Some(ch) = self.get_char() {
      match self.state {
        State::Data => {
          pos = self.cursor.pos();

          match ch {
            c if matches!(c, '\0') => return Ok(Token::new(TokenKind::Eof)),
            c if matches!(c, ' ') => self.state = State::Space,
            c if matches!(c, '\n') => self.state = State::Newline,
            // '(' => self.state = State::Group,
            '.' => self.state = State::Punctuation,
            // '\"' => self.state = State::Quote,
            c if c.is_alphabetic() => self.state = State::Word,
            _ => self.state = State::Unknown,
          }
        }
        State::Space => {
          self.cursor.next();

          return self.scan(pos);
        }
        State::Newline => {
          self.cursor.next();

          return self.scan(pos);
        }
        State::Punctuation => {
          self.cursor.next();

          return self.scan(pos);
        }
        State::Quote => {
          self.cursor.next();

          return self.scan(pos);
        }
        State::Word => match ch {
          c if c.is_ascii_alphabetic() => {
            self.cursor.next();
          }
          _ => return self.scan(pos),
        },
        _ => panic!("{:?}", ch),
      }
    }

    self.scan(pos)
  }

  /// Scans a token.
  fn scan(&mut self, pos: usize) -> Result<Token> {
    let source: &str = &self.cursor.source()[pos..self.cursor.pos()];

    let maybe_kind = match self.state {
      State::Space => Some(TokenKind::Space),
      State::Newline => Some(TokenKind::Newline),
      State::Punctuation => {
        Some(TokenKind::Punctuation(Punctuation::from(source)))
      }
      State::Word => Some(TokenKind::Word(Word::new(source.to_string()))),
      _ => None,
    };

    self.state = State::Data;

    if let Some(kind) = maybe_kind {
      return Ok(Token::new(kind));
    }

    panic!()
  }
}

/// Transforms a source code into a stream of tokens.
pub fn tokenize(source: &str) -> Result<Vec<Token>> {
  Tokenizer::new(source).tokenize()
}
