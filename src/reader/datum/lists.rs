/*!
One-line description.

More detailed description, with

# Example

YYYYY

*/

use crate::error::{cannot_append_to_improper_pair, pair_missing_car, Error};
use crate::lexer::token::Span;
use crate::reader::datum::Datum;
use crate::syntax::{PAIR_DOT, PAIR_END, PAIR_START};
use std::{
    fmt::{Debug, Display},
    rc::Rc,
};

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, PartialEq)]
pub struct SList(Option<SPair>);

#[derive(Clone, PartialEq)]
pub struct SPair {
    car: Rc<Datum>,
    cdr: Rc<Datum>,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

pub const EMPTY_LIST: Datum = Datum::List(SList(None));

impl Display for SList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            None => write!(f, "{PAIR_START}{PAIR_END}"),
            Some(pair) => write!(f, "{}", pair),
        }
    }
}

impl Debug for SList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            None => write!(f, "{PAIR_START}{PAIR_END}"),
            Some(pair) => write!(f, "{:?}", pair),
        }
    }
}

impl_datum_value!(List, SList);

impl From<SPair> for SList {
    fn from(v: SPair) -> Self {
        Self(Some(v))
    }
}

impl From<Vec<Datum>> for SList {
    fn from(v: Vec<Datum>) -> Self {
        Self::from_iter(v)
    }
}

impl From<Vec<Rc<Datum>>> for SList {
    fn from(v: Vec<Rc<Datum>>) -> Self {
        Self::from_iter(v)
    }
}

impl FromIterator<Datum> for SList {
    fn from_iter<T: IntoIterator<Item = Datum>>(iter: T) -> Self {
        Self::from_iter(iter.into_iter().map(Rc::new))
    }
}

impl FromIterator<Rc<Datum>> for SList {
    fn from_iter<T: IntoIterator<Item = Rc<Datum>>>(iter: T) -> Self {
        let mut root = SList::empty();

        for datum in iter {
            root.append(datum, None).unwrap();
        }

        root
    }
}

impl SList {
    pub fn empty() -> Self {
        Self(None)
    }

    pub fn is_empty(&self) -> bool {
        matches!(self, Self(None))
    }

    pub fn is_improper_list(&self) -> bool {
        match &self.0 {
            None => false,
            Some(pair) => pair.is_improper_list(),
        }
    }

    pub fn is_list(&self) -> bool {
        match &self.0 {
            None => true,
            Some(pair) => pair.is_proper_list(),
        }
    }

    pub fn append(&mut self, datum: Rc<Datum>, span: Option<Span>) -> Result<(), Error> {
        match &mut self.0 {
            None => {
                self.0 = Some(SPair::from(datum));
                Ok(())
            }
            Some(SPair {
                car: _,
                ref mut cdr,
            }) if cdr.is_list() => Rc::make_mut(cdr).as_list_mut().unwrap().append(datum, span),
            _ => cannot_append_to_improper_pair(span.unwrap_or_default()),
        }
    }

    pub fn append_improper(&mut self, datum: Rc<Datum>, span: Option<Span>) -> Result<(), Error> {
        match &mut self.0 {
            None => pair_missing_car(span.unwrap_or_default()),
            Some(SPair {
                car: _,
                ref mut cdr,
            }) if cdr.is_empty_list() => {
                *cdr = datum;
                Ok(())
            }
            Some(SPair {
                car: _,
                ref mut cdr,
            }) if cdr.is_list() => Rc::make_mut(cdr)
                .as_list_mut()
                .unwrap()
                .append_improper(datum, span),
            _ => cannot_append_to_improper_pair(span.unwrap_or_default()),
        }
    }
}

// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------

impl Display for SPair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{PAIR_START}")?;
        self.inner_fmt(f)?;
        write!(f, "{PAIR_END}")
    }
}

impl Debug for SPair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{PAIR_START}{:?} . {:?}{PAIR_END}", self.car, self.cdr)
    }
}

impl From<Datum> for SPair {
    fn from(v: Datum) -> Self {
        Self::cons(Rc::new(v), EMPTY_LIST.into())
    }
}

impl From<Rc<Datum>> for SPair {
    fn from(v: Rc<Datum>) -> Self {
        Self::cons(v, EMPTY_LIST.into())
    }
}

impl From<(Datum, Datum)> for SPair {
    fn from(v: (Datum, Datum)) -> Self {
        Self::cons(Rc::new(v.0), Rc::new(v.1))
    }
}

impl From<(Rc<Datum>, Rc<Datum>)> for SPair {
    fn from(v: (Rc<Datum>, Rc<Datum>)) -> Self {
        Self::cons(v.0, v.1)
    }
}

impl SPair {
    pub fn cons(car: Rc<Datum>, cdr: Rc<Datum>) -> Self {
        Self { car, cdr }
    }

    pub fn is_improper(&self) -> bool {
        !self.is_proper()
    }

    pub fn is_proper(&self) -> bool {
        self.cdr.is_list() // Datum::List(_)
    }

    pub fn is_improper_list(&self) -> bool {
        !self.is_proper_list()
    }

    pub fn is_proper_list(&self) -> bool {
        match self.cdr().as_ref() {
            Datum::List(SList(None)) => true,
            Datum::List(SList(Some(pair))) => pair.is_improper_list(),
            _ => false,
        }
    }

    pub fn car(&self) -> &Rc<Datum> {
        &self.car
    }

    pub fn car_mut(&mut self) -> &mut Rc<Datum> {
        &mut self.car
    }

    pub fn set_car(&mut self, datum: Rc<Datum>) {
        self.car = datum;
    }

    pub fn cdr(&self) -> &Rc<Datum> {
        &self.cdr
    }

    pub fn cdr_mut(&mut self) -> &mut Rc<Datum> {
        &mut self.cdr
    }

    pub fn set_cdr(&mut self, datum: Rc<Datum>) {
        self.cdr = datum
    }

    fn inner_fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.car())?;

        match self.cdr.as_ref() {
            Datum::List(SList(None)) => Ok(()),
            Datum::List(SList(Some(pair))) => {
                write!(f, " ")?;
                pair.inner_fmt(f)
            }
            _ => write!(f, " {PAIR_DOT} {}", self.cdr),
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------
