//! This module contains all css theming releated resources.

use std::{fs::File, io::BufReader, io::Read, mem, path::Path, sync::Arc};


use crate::structs::Brush;

use cssparser::{
    self, BasicParseError, CompactCowStr, DeclarationListParser, ParseError, Parser, ParserInput,
    Token,
};

pub use self::cell::CloneCell;
use self::selector::Specificity;
pub use self::selector::{Selector, SelectorRelation};
pub use self::style::Style;
pub use self::update::*;

mod cell;

mod selector;

mod style;
mod update;

use crate::{
    styling::theme::{DEFAULT_THEME_CSS, LIGHT_THEME_EXTENSION_CSS}
};


lazy_static! {
    static ref DEFAULT_THEME: Arc<Theme> = {
        Arc::new(Theme {
            parent: None,
            rules: parse(DEFAULT_THEME_CSS),
        })
    };
}

lazy_static! {
    pub static ref LIGHT_THEME_CSS: String =
        format!("{}{}", LIGHT_THEME_EXTENSION_CSS, DEFAULT_THEME_CSS);
}

#[derive(Debug, Default, Clone)]
pub struct Theme {
    parent: Option<Arc<Theme>>,
    rules: Vec<Rule>,
}

impl Theme {
    pub fn new() -> Self {
        Theme::parse("")
    }

    pub fn parse(s: &str) -> Self {
        Theme {
            parent: Some(DEFAULT_THEME.clone()),
            rules: parse(s),
        }
    }

    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Theme, String> {
        let file = r#try!(File::open(path).map_err(|err| format!("failed to open css: {}", err)));
        let mut reader = BufReader::new(file);
        let mut css = String::new();
        let res = reader
            .read_to_string(&mut css)
            .map_err(|err| format!("failed to read css: {}", err));
        match res {
            Ok(_) => Ok(Theme::parse(&css)),
            Err(err) => Err(err),
        }
    }

    fn all_rules(&self) -> Vec<Rule> {
        if let Some(ref parent) = self.parent {
            self.rules
                .iter()
                .chain(parent.rules.iter())
                .cloned()
                .collect()
        } else {
            self.rules.clone()
        }
    }

    pub fn get(&self, property: &str, query: &Selector) -> Option<Value> {
        let mut matches: Vec<(bool, Specificity, Value)> = Vec::new();

        for rule in self.all_rules().iter().rev() {
            let matching_selectors = rule
                .selectors
                .iter()
                .filter(|x| x.matches(query))
                .collect::<Vec<_>>();

            if !matching_selectors.is_empty() {
                if let Some(decl) = rule
                    .declarations
                    .iter()
                    .find(|decl| decl.property == property)
                {
                    let highest_specifity = matching_selectors
                        .iter()
                        .map(|sel| sel.specificity())
                        .max()
                        .unwrap();
                    matches.push((decl.important, highest_specifity, decl.value.clone()));
                }
            }
        }

        matches.sort_by_key(|x| (x.0, x.1));
        matches.last().map(|x| x.2.clone())
    }

    pub fn brush(&self, property: &str, query: &Selector) -> Brush {
        self.get(property, query)
            .map(|v| v.brush().unwrap_or(Brush::default()))
            .unwrap_or(Brush::default())
    }

    pub fn uint(&self, property: &str, query: &Selector) -> u32 {
        self.get(property, query)
            .map(|v| v.uint().unwrap_or(0))
            .unwrap_or(0)
    }

    pub fn float(&self, property: &str, query: &Selector) -> f32 {
        self.get(property, query)
            .map(|v| v.float().unwrap_or(1.0))
            .unwrap_or(1.0)
    }

    pub fn string(&self, property: &str, query: &Selector) -> String {
        self.get(property, query)
            .map(|v| v.string().unwrap_or(String::default()))
            .unwrap_or(String::default())
    }
}

#[derive(Clone, Debug)]
pub struct Rule {
    pub selectors: Vec<Selector>,
    pub declarations: Vec<Declaration>,
}

impl<T: Into<String>> From<T> for Selector {
    fn from(t: T) -> Self {
        Selector::new().with(t.into())
    }
}

#[derive(Clone, Debug)]
pub struct Declaration {
    pub property: String,
    pub value: Value,
    pub important: bool,
}

#[derive(Clone, Debug)]
pub enum Value {
    UInt(u32),
    Float(f32),
    Brush(Brush),
    Str(String),
}

impl Value {
    pub fn uint(&self) -> Option<u32> {
        match *self {
            Value::UInt(x) => Some(x),
            _ => None,
        }
    }

    pub fn float(&self) -> Option<f32> {
        match *self {
            Value::Float(x) => Some(x),
            _ => None,
        }
    }

    pub fn brush(&self) -> Option<Brush> {
        match self {
            Value::Brush(x) => Some(x.clone()),
            _ => None,
        }
    }

    pub fn string(&self) -> Option<String> {
        match self {
            Value::Str(x) => Some(x.clone()),
            _ => None,
        }
    }
}

#[derive(Clone, Debug)]
pub enum CustomParseError {
    InvalidColorName(String),
    InvalidColorHex(String),
    InvalidStringName(String),
}

impl<'t> From<CustomParseError> for ParseError<'t, CustomParseError> {
    fn from(e: CustomParseError) -> Self {
        ParseError::Custom(e)
    }
}

struct RuleParser;

impl RuleParser {
    fn new() -> Self {
        RuleParser {}
    }
}

impl<'i> cssparser::QualifiedRuleParser<'i> for RuleParser {
    type Prelude = Vec<Selector>;
    type QualifiedRule = Rule;
    type Error = CustomParseError;

    fn parse_prelude<'t>(
        &mut self,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::Prelude, ParseError<'i, Self::Error>> {
        let res = parse_selectors(input)?;
        Ok(res)
    }

    fn parse_block<'t>(
        &mut self,
        selectors: Self::Prelude,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::QualifiedRule, ParseError<'i, Self::Error>> {
        let decl_parser = DeclarationParser {};

        let decls = DeclarationListParser::new(input, decl_parser).collect::<Vec<_>>();

        for decl in &decls {
            match *decl {
                Ok(_) => {}
                Err(ref e) => {
                    match e.error {
                        ParseError::Basic(ref e) => eprintln!("{:?}", e),
                        ParseError::Custom(ref e) => eprintln!("{:?}", e),
                    }
                    println!("Error occured in `{}`", input.slice(e.span.clone()));
                }
            }
        }

        let decls = decls.into_iter().filter_map(|decl| decl.ok()).collect();

        Ok(Rule {
            selectors: selectors,
            declarations: decls,
        })
    }
}

impl<'i> cssparser::AtRuleParser<'i> for RuleParser {
    type Prelude = ();
    type AtRule = Rule;
    type Error = CustomParseError;
}

fn parse_selectors<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> Result<Vec<Selector>, ParseError<'i, CustomParseError>> {
    let mut selectors = Vec::new();

    let mut selector = Selector::default();

    let mut first_token_in_selector = true;
    while let Ok(t) = input.next() {
        match t {
            // Element
            Token::Ident(ref element_name) => {
                if first_token_in_selector {
                    selector.element = Some(element_name.to_string())
                } else {
                    let mut old_selector = Selector::new().with(element_name.to_string());
                    mem::swap(&mut old_selector, &mut selector);
                    selector.relation = Some(Box::new(SelectorRelation::Ancestor(old_selector)));
                }
            }

            Token::Delim('>') => {
                let mut old_selector = Selector::new().with(input.expect_ident()?.to_string());
                mem::swap(&mut old_selector, &mut selector);
                selector.relation = Some(Box::new(SelectorRelation::Parent(old_selector)));
            }

            // Id
            Token::IDHash(ref id_name) => {
                selector.id = Some(id_name.to_string());
            }

            // Any element
            Token::Delim('*') => {}

            // Class
            Token::Delim('.') => {
                selector.classes.insert(input.expect_ident()?.into_owned());
            }

            // Pseudo-class
            Token::Colon => {
                selector
                    .pseudo_classes
                    .insert(input.expect_ident()?.into_owned());
            }

            // This selector is done, on to the next one
            Token::Comma => {
                selectors.push(selector);
                selector = Selector::default();
                first_token_in_selector = true;
                continue; // need to continue to avoid `first_token_in_selector` being set to false
            }

            t => {
                let basic_error = BasicParseError::UnexpectedToken(t);
                return Err(basic_error.into());
            }
        }

        first_token_in_selector = false;
    }

    selectors.push(selector);

    if selectors.iter().any(|sel| sel.relation.is_some()) {
        eprintln!("WARNING: Complex selector relations not implemented");
    }

    Ok(selectors)
}

struct DeclarationParser;

impl<'i> cssparser::DeclarationParser<'i> for DeclarationParser {
    type Declaration = Declaration;
    type Error = CustomParseError;

    fn parse_value<'t>(
        &mut self,
        name: CompactCowStr<'i>,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::Declaration, ParseError<'i, Self::Error>> {
        let value = match &*name {
            "color" | "border-color" | "icon-color" => Value::Brush(parse_basic_color(input)?),

            "background" | "foreground" => Value::Brush(parse_basic_color(input)?),

            "font-family" | "icon-font-family" => Value::Str(parse_string(input)?),

            "border-radius" | "border-width" | "border-left" | "border-top" | "border-right" | "border-bottom" | "width" | "height" | "min-width" | "min-height"
            | "max-width" | "max-height" | "padding-top" | "padding-right" | "padding-bottom"
            | "padding-left" | "padding" | "font-size" | "icon-size" | "icon-margin" => {
                match input.next()? {
                    Token::Number {
                        int_value: Some(x),
                        has_sign,
                        ..
                    } if !has_sign && x >= 0 => Value::UInt(x as u32),
                    t => return Err(BasicParseError::UnexpectedToken(t).into()),
                }
            }

            "opacity" => match input.next()? {
                Token::Number { value: x, .. } => Value::Float(x as f32),
                t => return Err(BasicParseError::UnexpectedToken(t).into()),
            },

            _ => return Err(BasicParseError::UnexpectedToken(input.next()?).into()),
        };

        Ok(Declaration {
            property: name.into_owned(),
            value: value,
            important: input.r#try(cssparser::parse_important).is_ok(),
        })
    }
}

impl<'i> cssparser::AtRuleParser<'i> for DeclarationParser {
    type Prelude = ();
    type AtRule = Declaration;
    type Error = CustomParseError;
}

fn css_color(name: &str) -> Option<Brush> {
    Some(match name {
        "transparent" => Brush::from(name),

        "black" => Brush::from("#000000"),
        "silver" => Brush::from("#C0C0C0"),
        "gray" | "grey" => Brush::from("#808080"),
        "white" => Brush::from("#FFFFFF"),
        "maroon" => Brush::from("#800000"),
        "red" => Brush::from("#FF0000"),
        "purple" => Brush::from("#800080"),
        "fuchsia" => Brush::from("#FF00FF"),
        "green" => Brush::from("#008000"),
        "lime" => Brush::from("#00FF00"),
        "olive" => Brush::from("#808000"),
        "yellow" => Brush::from("#FFFF00"),
        "navy" => Brush::from("#000080"),
        "blue" => Brush::from("#0000FF"),
        "teal" => Brush::from("#008080"),
        "aqua" => Brush::from("#00FFFF"),
        _ => return None,
    })
}

fn css_string(name: &str) -> Option<String> {
    Some(String::from(name))
}

fn parse_string<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> Result<String, ParseError<'i, CustomParseError>> {
    Ok(match input.next()? {
        Token::QuotedString(s) => match css_string(&s) {
            Some(string) => string,
            None => return Err(CustomParseError::InvalidStringName(s.into_owned()).into()),
        },

        t => {
            let basic_error = BasicParseError::UnexpectedToken(t);
            return Err(basic_error.into());
        }
    })
}

fn parse_basic_color<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> Result<Brush, ParseError<'i, CustomParseError>> {
    Ok(match input.next()? {
        Token::Ident(s) => match css_color(&s) {
            Some(color) => color,
            None => return Err(CustomParseError::InvalidColorName(s.into_owned()).into()),
        },

        Token::IDHash(hash) | Token::Hash(hash) => Brush::from(hash.into_owned()),

        t => {
            let basic_error = BasicParseError::UnexpectedToken(t);
            return Err(basic_error.into());
        }
    })
}

pub fn parse(s: &str) -> Vec<Rule> {
    let mut input = ParserInput::new(s);
    let mut parser = Parser::new(&mut input);
    let rule_parser = RuleParser::new();

    let rules = {
        let rule_list_parser =
            cssparser::RuleListParser::new_for_stylesheet(&mut parser, rule_parser);
        rule_list_parser.collect::<Vec<_>>()
    };

    for rule in &rules {
        match *rule {
            Ok(_) => {}
            Err(ref e) => {
                match e.error {
                    ParseError::Basic(ref e) => eprintln!("{:?}", e),
                    ParseError::Custom(ref e) => eprintln!("{:?}", e),
                }
                println!("Error occured in `{}`", parser.slice(e.span.clone()));
            }
        }
    }

    rules.into_iter().filter_map(|rule| rule.ok()).collect()
}

// const fn hex(data: u32) -> Color {
//     Color {
//         data: 0xFF_000_000 | data,
//     }
// }
