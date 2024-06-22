use std::fmt;
use std::fmt::Debug;
use std::fmt::Formatter;

use crate::util::LengthShort;
use crate::util::Location;
use crate::util::Span;
use crate::util::Spanned;

#[derive(Debug)]
pub struct SimpleMessage<'a> {
  pub parts: Vec<MessagePart<'a>>,
}

impl Spanned for SimpleMessage<'_> {
  fn span(&self) -> Span {
    match (self.parts.first(), self.parts.last()) {
      (Some(first), Some(last)) => {
        Span::new(first.span().start..last.span().end)
      }
      _ => Span::new(Location::dummy()..Location::dummy()),
    }
  }
}

pub enum MessagePart<'a> {
  Text(Text<'a>),
  Escape(Escape),
  Expression(Expression<'a>),
  Markup(Markup<'a>),
}

impl fmt::Debug for MessagePart<'_> {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self {
      MessagePart::Text(text) => Debug::fmt(text, f),
      MessagePart::Escape(escape) => Debug::fmt(escape, f),
      MessagePart::Expression(expression) => Debug::fmt(expression, f),
      MessagePart::Markup(markup) => Debug::fmt(markup, f),
    }
  }
}

impl Spanned for MessagePart<'_> {
  fn span(&self) -> Span {
    match self {
      MessagePart::Text(text) => text.span(),
      MessagePart::Escape(escape) => escape.span(),
      MessagePart::Expression(expression) => expression.span(),
      MessagePart::Markup(markup) => markup.span(),
    }
  }
}

#[derive(Debug)]
pub struct Text<'a> {
  pub start: Location,
  pub content: &'a str,
}

impl Spanned for Text<'_> {
  fn span(&self) -> Span {
    Span::new(self.start..self.start + self.content)
  }
}

#[derive(Debug)]
pub struct Escape {
  pub start: Location,
  pub escaped_char: char,
}

impl Spanned for Escape {
  fn span(&self) -> Span {
    Span::new(self.start..self.start + '\\' + self.escaped_char)
  }
}

pub enum Expression<'a> {
  LiteralExpression(LiteralExpression<'a>),
  VariableExpression(VariableExpression<'a>),
  AnnotationExpression(AnnotationExpression<'a>),
}

impl fmt::Debug for Expression<'_> {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self {
      Expression::LiteralExpression(literal_expression) => {
        Debug::fmt(literal_expression, f)
      }
      Expression::VariableExpression(variable_expression) => {
        Debug::fmt(variable_expression, f)
      }
      Expression::AnnotationExpression(annotation_expression) => {
        Debug::fmt(annotation_expression, f)
      }
    }
  }
}

impl Spanned for Expression<'_> {
  fn span(&self) -> Span {
    match self {
      Expression::LiteralExpression(literal_expression) => {
        literal_expression.span()
      }
      Expression::VariableExpression(variable_expression) => {
        variable_expression.span()
      }
      Expression::AnnotationExpression(annotation_expression) => {
        annotation_expression.span()
      }
    }
  }
}

#[derive(Debug)]
pub struct LiteralExpression<'a> {
  pub open: Location,
  pub close: Location,
  pub literal: Literal<'a>,
  pub annotation: Option<Annotation<'a>>,
  pub attributes: Vec<Attribute<'a>>,
}

impl Spanned for LiteralExpression<'_> {
  fn span(&self) -> Span {
    let start = self.open;
    let end = self.close + '}';
    Span::new(start..end)
  }
}

#[derive(Debug)]
pub struct VariableExpression<'a> {
  pub open: Location,
  pub close: Location,
  pub variable: Variable<'a>,
  pub annotation: Option<Annotation<'a>>,
  pub attributes: Vec<Attribute<'a>>,
}

impl Spanned for VariableExpression<'_> {
  fn span(&self) -> Span {
    let start = self.open;
    let end = self.close + '}';
    Span::new(start..end)
  }
}

#[derive(Debug)]
pub struct Variable<'a> {
  pub start: Location,
  pub name: &'a str,
}

impl Spanned for Variable<'_> {
  fn span(&self) -> Span {
    Span::new(self.start..self.start + '$' + self.name)
  }
}

#[derive(Debug)]
pub struct AnnotationExpression<'a> {
  pub open: Location,
  pub close: Location,
  pub annotation: Annotation<'a>,
  pub attributes: Vec<Attribute<'a>>,
}

impl Spanned for AnnotationExpression<'_> {
  fn span(&self) -> Span {
    let start = self.open;
    let end = self.close + '}';
    Span::new(start..end)
  }
}

pub enum Annotation<'a> {
  Function(Function<'a>),
  PrivateUseAnnotation(PrivateUseAnnotation<'a>),
  ReservedAnnotation(ReservedAnnotation<'a>),
}

impl fmt::Debug for Annotation<'_> {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self {
      Annotation::Function(function) => Debug::fmt(function, f),
      Annotation::PrivateUseAnnotation(private_use_annotation) => {
        Debug::fmt(private_use_annotation, f)
      }
      Annotation::ReservedAnnotation(reserved_annotation) => {
        Debug::fmt(reserved_annotation, f)
      }
    }
  }
}

impl Spanned for Annotation<'_> {
  fn span(&self) -> Span {
    match self {
      Annotation::Function(function) => function.span(),
      Annotation::PrivateUseAnnotation(private_use_annotation) => {
        private_use_annotation.span()
      }
      Annotation::ReservedAnnotation(reserved_annotation) => {
        reserved_annotation.span()
      }
    }
  }
}

#[derive(Debug)]
pub struct Identifier<'a> {
  pub start: Location,
  pub namespace: Option<&'a str>,
  pub name: &'a str,
}

impl Spanned for Identifier<'_> {
  fn span(&self) -> Span {
    let mut end = self.start;
    if let Some(namespace) = self.namespace {
      end = end + namespace + ':';
    }
    end = end + self.name;

    Span::new(self.start..end)
  }
}

#[derive(Debug)]
pub struct Function<'a> {
  pub start: Location,
  pub id: Identifier<'a>,
  pub options: Vec<FnOrMarkupOption<'a>>,
}

impl Spanned for Function<'_> {
  fn span(&self) -> Span {
    let start = self.start;
    let end = self
      .options
      .last()
      .map_or(self.id.span().end, |last| last.span().end);
    Span::new(start..end)
  }
}

#[derive(Debug)]
pub struct FnOrMarkupOption<'a> {
  pub key: Identifier<'a>,
  pub value: LiteralOrVariable<'a>,
}

impl Spanned for FnOrMarkupOption<'_> {
  fn span(&self) -> Span {
    let start = self.key.span().start;
    let end = self.value.span().end;
    Span::new(start..end)
  }
}

#[derive(Debug)]
pub struct Attribute<'a> {
  pub start: Location,
  pub key: Identifier<'a>,
  pub value: Option<LiteralOrVariable<'a>>,
}

impl Spanned for Attribute<'_> {
  fn span(&self) -> Span {
    let start = self.start;
    let end = self
      .value
      .as_ref()
      .map_or(self.key.span().end, |value| value.span().end);
    Span::new(start..end)
  }
}

pub enum LiteralOrVariable<'a> {
  Literal(Literal<'a>),
  Variable(Variable<'a>),
}

impl fmt::Debug for LiteralOrVariable<'_> {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self {
      LiteralOrVariable::Literal(literal) => Debug::fmt(literal, f),
      LiteralOrVariable::Variable(variable) => Debug::fmt(variable, f),
    }
  }
}

impl Spanned for LiteralOrVariable<'_> {
  fn span(&self) -> Span {
    match self {
      LiteralOrVariable::Literal(literal) => literal.span(),
      LiteralOrVariable::Variable(variable) => variable.span(),
    }
  }
}

#[derive(Debug)]
pub struct PrivateUseAnnotation<'a> {
  pub start: Location,
  pub sigil: char,
  pub body: Vec<ReservedBodyPart<'a>>,
}

impl Spanned for PrivateUseAnnotation<'_> {
  fn span(&self) -> Span {
    let start = self.start;
    let end = self
      .body
      .last()
      .map_or(start + self.sigil, |last| last.span().end);
    Span::new(start..end)
  }
}

#[derive(Debug)]
pub struct ReservedAnnotation<'a> {
  pub start: Location,
  pub sigil: char,
  pub body: Vec<ReservedBodyPart<'a>>,
}

impl Spanned for ReservedAnnotation<'_> {
  fn span(&self) -> Span {
    let start = self.start;
    let end = self
      .body
      .last()
      .map_or(start + self.sigil, |last| last.span().end);
    Span::new(start..end)
  }
}

pub enum ReservedBodyPart<'a> {
  Text(Text<'a>),
  Escape(Escape),
  Quoted(Quoted<'a>),
}

impl fmt::Debug for ReservedBodyPart<'_> {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self {
      ReservedBodyPart::Text(text) => Debug::fmt(text, f),
      ReservedBodyPart::Escape(escape) => Debug::fmt(escape, f),
      ReservedBodyPart::Quoted(quoted) => Debug::fmt(quoted, f),
    }
  }
}

impl Spanned for ReservedBodyPart<'_> {
  fn span(&self) -> Span {
    match self {
      ReservedBodyPart::Text(text) => text.span(),
      ReservedBodyPart::Escape(escape) => escape.span(),
      ReservedBodyPart::Quoted(quoted) => quoted.span(),
    }
  }
}

pub enum Literal<'a> {
  Quoted(Quoted<'a>),
  Name(Text<'a>),
  Number(Number<'a>),
}

impl fmt::Debug for Literal<'_> {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self {
      Literal::Quoted(quoted) => Debug::fmt(quoted, f),
      Literal::Name(name) => Debug::fmt(name, f),
      Literal::Number(number) => Debug::fmt(number, f),
    }
  }
}

impl Spanned for Literal<'_> {
  fn span(&self) -> Span {
    match self {
      Literal::Quoted(quoted) => quoted.span(),
      Literal::Name(name) => name.span(),
      Literal::Number(number) => number.span(),
    }
  }
}

#[derive(Debug)]
pub struct Quoted<'a> {
  pub open: Location,
  pub parts: Vec<QuotedPart<'a>>,
}

impl Spanned for Quoted<'_> {
  fn span(&self) -> Span {
    let start = self.open;
    let end = self
      .parts
      .last()
      .map_or(start + '|', |last| last.span().end)
      + '|';
    Span::new(start..end)
  }
}

pub enum QuotedPart<'a> {
  Text(Text<'a>),
  Escape(Escape),
}

impl fmt::Debug for QuotedPart<'_> {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self {
      QuotedPart::Text(text) => Debug::fmt(text, f),
      QuotedPart::Escape(escape) => Debug::fmt(escape, f),
    }
  }
}

impl Spanned for QuotedPart<'_> {
  fn span(&self) -> Span {
    match self {
      QuotedPart::Text(text) => text.span(),
      QuotedPart::Escape(escape) => escape.span(),
    }
  }
}

#[derive(Debug, Clone, Copy)]
pub enum ExponentSign {
  Plus,
  Minus,
  None,
}

#[derive(Debug)]
pub struct Number<'a> {
  pub start: Location,
  pub raw: &'a str,
  pub is_negative: bool,
  pub integral_len: LengthShort,
  pub fractional_len: Option<LengthShort>,
  pub exponent_len: Option<(ExponentSign, LengthShort)>,
}

impl Spanned for Number<'_> {
  fn span(&self) -> Span {
    Span::new(self.start..self.start + self.raw)
  }
}

impl<'a> Number<'a> {
  fn integral_start(&self) -> usize {
    if self.is_negative {
      '-'.len_utf8()
    } else {
      0
    }
  }

  fn integral_end(&self) -> usize {
    self.integral_start() + self.integral_len.inner() as usize
  }

  pub fn integral_part(&self) -> &'a str {
    &self.raw[self.integral_start()..self.integral_end()]
  }

  pub fn fractional_part(&self) -> Option<&'a str> {
    self.fractional_len.as_ref().map(|fractional_len| {
      let start = self.integral_end() + '.'.len_utf8();
      let end = start + fractional_len.inner() as usize;
      &self.raw[start..end]
    })
  }

  pub fn exponent_part(&self) -> Option<(ExponentSign, &'a str)> {
    self.exponent_len.as_ref().map(|(sign, exponent_len)| {
      let mut start = self.integral_end();
      if let Some(fractional_len) = &self.fractional_len {
        start += '.'.len_utf8() + fractional_len.inner() as usize;
      }
      start += 'e'.len_utf8();

      if !matches!(sign, ExponentSign::None) {
        start += '-'.len_utf8();
      };

      let end = start + exponent_len.inner() as usize;

      (*sign, &self.raw[start..end])
    })
  }
}

#[derive(Debug)]
pub struct Markup<'a> {
  pub open: Location,
  pub close: Location,
  pub kind: MarkupKind,
  pub id: Identifier<'a>,
  pub options: Vec<FnOrMarkupOption<'a>>,
  pub attributes: Vec<Attribute<'a>>,
}

#[derive(Debug)]
pub enum MarkupKind {
  Open,
  Standalone,
  Close,
}

impl Spanned for Markup<'_> {
  fn span(&self) -> Span {
    let start = self.open;
    let close_token = match self.kind {
      MarkupKind::Open | MarkupKind::Close => "}",
      MarkupKind::Standalone => "/}",
    };
    let end = self.close + close_token;
    Span::new(start..end)
  }
}
