////////////////////////////////////////////////////////////////////////////////
// Tephra parser library
////////////////////////////////////////////////////////////////////////////////
// Copyright 2020 Skylor R. Schermer
// This code is dual licenced using the MIT or Apache 2 license.
// See licence-mit.md and licence-apache.md for details.
////////////////////////////////////////////////////////////////////////////////
//! Parser combinators for repeating.
////////////////////////////////////////////////////////////////////////////////

// Local imports.
use crate::combinator::discard;
use crate::combinator::empty;
use crate::combinator::right;
use crate::lexer::Lexer;
use crate::lexer::Scanner;
use crate::result::ParseResult;
use crate::result::ParseResultExt as _;
use crate::result::Success;
use crate::position::ColumnMetrics;

////////////////////////////////////////////////////////////////////////////////
// Repetition combinators.
////////////////////////////////////////////////////////////////////////////////

/// Returns a parser which repeats the given number of times, interspersed by
/// parse attempts from a secondary parser. Each parsed value is collected into
/// a `Vec`.
pub fn intersperse_collect<'text, Sc, Cm, F, G, V, U>(
    low: usize,
    high: Option<usize>,
    mut parser: F,
    mut inter_parser: G)
    -> impl FnMut(Lexer<'text, Sc, Cm>) -> ParseResult<'text, Sc, Cm, Vec<V>>
    where
        Sc: Scanner,
        Cm: ColumnMetrics,
        F: FnMut(Lexer<'text, Sc, Cm>) -> ParseResult<'text, Sc, Cm, V>,
        G: FnMut(Lexer<'text, Sc, Cm>) -> ParseResult<'text, Sc, Cm, U>,
{
    move |lexer| {
        log::debug!("intersperse_collect: Begin (low = {:?}, high = {:?})",
            low,
            high);

        if let Some(h) = high {
            if h < low { panic!("intersperse_collect with high < low") }
            if h == 0 {
                log::trace!("intersperse_collect: End (0 repetitions)");
                return Ok(Success {
                    lexer,
                    value: Vec::new(),
                });
            }
        }

        let mut vals = Vec::with_capacity(4);
        // println!("vals len: {}", vals.len());

        let (val, mut succ) = match (&mut parser)
            (lexer)
        {
            Ok(succ) => succ.take_value(),
            Err(fail) => return if low == 0 {
                log::trace!("intersperse_collect: Ok (0 repetitions)");
                Ok(Success { lexer: fail.lexer, value: vals })
            } else {
                log::trace!("intersperse_collect: Fail (0 repetitions)");
                Err(fail)
            },
        };

        

        vals.push(val);
        log::trace!("intersperse_collect: (1 repetition)");

        while vals.len() < low {
            let (val, next) = right(&mut inter_parser, &mut parser)
                (succ.lexer)?
                .take_value();
            vals.push(val);
            log::trace!("intersperse_collect: ({:?} repetitions)", vals.len());
            succ = next;

        }

        log::trace!("intersperse_collect: minimum reps satisfied");

        while high.map_or(true, |h| vals.len() < h) {
            match right(&mut inter_parser, &mut parser)
                (succ.lexer.clone())
            {
                Ok(next) => {
                    let (val, next) = next.take_value();
                    vals.push(val);
                    log::trace!("intersperse_collect: ({:?} repetitions)", vals.len());
                    succ = next;
                }
                Err(_) => break,
            }

            if high.map_or(false, |h| vals.len() >= h) {
                break;
            }
        }

        log::trace!("intersperse_collect: Ok ({} repetitions)", vals.len());
        Ok(succ.map_value(|_| vals))
    }
}

/// Returns a parser which repeats the given number of times or until a stop
/// parser succeeds, interspersed by parse attempts from a secondary parser.
/// Each parsed value is collected into a `Vec`. The stop parse is not included
/// in the result.
pub fn intersperse_collect_until<'text, Sc, Cm, F, G, H, V, U, T>(
    low: usize,
    high: Option<usize>,
    mut stop_parser: F,
    mut parser: G,
    mut inter_parser: H)
    -> impl FnMut(Lexer<'text, Sc, Cm>) -> ParseResult<'text, Sc, Cm, Vec<U>>
    where
        Sc: Scanner,
        Cm: ColumnMetrics,
        F: FnMut(Lexer<'text, Sc, Cm>) -> ParseResult<'text, Sc, Cm, V>,
        G: FnMut(Lexer<'text, Sc, Cm>) -> ParseResult<'text, Sc, Cm, U>,
        H: FnMut(Lexer<'text, Sc, Cm>) -> ParseResult<'text, Sc, Cm, T>,
{
    move |lexer| {
        if let Some(h) = high {
            if h < low { panic!("intersperse_collect with high < low") }
            if h == 0 {
                return Ok(Success {
                    lexer,
                    value: Vec::new(),
                });
            }
        }

        let mut vals = Vec::with_capacity(4);

        if (&mut stop_parser)(lexer.clone()).is_ok() {
            return Ok(Success { lexer, value: vals });
        }

        let (val, mut succ) = (&mut parser)
            (lexer)?
            .take_value();
        vals.push(val);

        while vals.len() < low {
            if (&mut stop_parser)(succ.lexer.clone()).is_ok() {
                return Ok(succ.map_value(|_| vals));
            }
            let (val, next) = right(&mut inter_parser, &mut parser)
                (succ.lexer)?
                .take_value();
            vals.push(val);
            succ = next;
        }

        while high.map_or(true, |h| vals.len() < h) {
            if (&mut stop_parser)(succ.lexer.clone()).is_ok() {
                return Ok(succ.map_value(|_| vals));
            }

            match right(&mut inter_parser, &mut parser)
                (succ.lexer.clone())
            {
                Ok(next) => {
                    let (val, next) = next.take_value();
                    vals.push(val);
                    succ = next;
                }
                Err(_) => break,
            }

            if high.map_or(false, |h| vals.len() >= h) {
                break;
            }
        }

        Ok(succ.map_value(|_| vals))
    }
}

/// Returns a parser which repeats the given number of times, interspersed by
/// parse attempts from a secondary parser. The parsed value is the number of
/// successful parses.
pub fn intersperse<'text, Sc, Cm, F, G, V, U>(
    low: usize,
    high: Option<usize>,
    mut parser: F,
    mut inter_parser: G)
    -> impl FnMut(Lexer<'text, Sc, Cm>) -> ParseResult<'text, Sc, Cm, usize>
    where
        Sc: Scanner,
        Cm: ColumnMetrics,
        F: FnMut(Lexer<'text, Sc, Cm>) -> ParseResult<'text, Sc, Cm, V>,
        G: FnMut(Lexer<'text, Sc, Cm>) -> ParseResult<'text, Sc, Cm, U>,
{
    move |lexer| {
        intersperse_collect(low, high, 
                discard(&mut parser),
                &mut inter_parser)
            (lexer)
            .map_value(|vals| vals.len())
    }
}

/// Returns a parser which repeats the given number of times, interspersed by
/// parse attempts from a secondary parser. The parsed value is the number of
/// successful parses.
pub fn intersperse_until<'text, Sc, Cm, F, G, H, V, U, T>(
    low: usize,
    high: Option<usize>,
    mut stop_parser: F,
    mut parser: G,
    mut inter_parser: H)
    -> impl FnMut(Lexer<'text, Sc, Cm>) -> ParseResult<'text, Sc, Cm, usize>
    where
        Sc: Scanner,
        Cm: ColumnMetrics,
        F: FnMut(Lexer<'text, Sc, Cm>) -> ParseResult<'text, Sc, Cm, V>,
        G: FnMut(Lexer<'text, Sc, Cm>) -> ParseResult<'text, Sc, Cm, U>,
        H: FnMut(Lexer<'text, Sc, Cm>) -> ParseResult<'text, Sc, Cm, T>,
{
    move |lexer| {
        intersperse_collect_until(low, high, 
                &mut stop_parser,
                discard(&mut parser),
                &mut inter_parser)
            (lexer)
            .map_value(|vals| vals.len())
    }
}

/// Returns a parser which repeats the given number of times. Each parsed value
/// is collected into a `Vec`.
pub fn repeat_collect<'text, Sc, Cm, F, V>(
    low: usize,
    high: Option<usize>,
    mut parser: F)
    -> impl FnMut(Lexer<'text, Sc, Cm>) -> ParseResult<'text, Sc, Cm, Vec<V>>
    where
        Sc: Scanner,
        Cm: ColumnMetrics,
        F: FnMut(Lexer<'text, Sc, Cm>) -> ParseResult<'text, Sc, Cm, V>,
{
    move |lexer| {
        intersperse_collect(low, high,
            &mut parser,
            empty)
            (lexer)
    }
}

/// Returns a parser which repeats the given number of times or until a stop
/// parser succeeds, interspersed by parse attempts from a secondary parser.
/// Each parsed value is collected into a `Vec`. The stop parse is not included
/// in the result.
pub fn repeat_collect_until<'text, Sc, Cm, F, G, V, U>(
    low: usize,
    high: Option<usize>,
    mut stop_parser: F,
    mut parser: G)
    -> impl FnMut(Lexer<'text, Sc, Cm>) -> ParseResult<'text, Sc, Cm, Vec<U>>
    where
        Sc: Scanner,
        Cm: ColumnMetrics,
        F: FnMut(Lexer<'text, Sc, Cm>) -> ParseResult<'text, Sc, Cm, V>,
        G: FnMut(Lexer<'text, Sc, Cm>) -> ParseResult<'text, Sc, Cm, U>,
{
    move |lexer| {
        intersperse_collect_until(low, high, &mut stop_parser,
            &mut parser,
            empty)
            (lexer)
    }
}

/// Returns a parser which repeats the given number of times, interspersed by
/// parse attempts from a secondary parser. The parsed value is the number of
/// successful parses.
pub fn repeat<'text, Sc, Cm, F, V>(
    low: usize,
    high: Option<usize>,
    mut parser: F)
    -> impl FnMut(Lexer<'text, Sc, Cm>) -> ParseResult<'text, Sc, Cm, usize>
    where
        Sc: Scanner,
        Cm: ColumnMetrics,
        F: FnMut(Lexer<'text, Sc, Cm>) -> ParseResult<'text, Sc, Cm, V>,
{
    move |lexer| {
        intersperse_collect(low, high, 
                discard(&mut parser),
                empty)
            (lexer)
            .map_value(|vals| vals.len())
    }
}

/// Returns a parser which repeats the given number of times, interspersed by
/// parse attempts from a secondary parser. The parsed value is the number of
/// successful parses.
pub fn repeat_until<'text, Sc, Cm, F, G, V, U>(
    low: usize,
    high: Option<usize>,
    mut stop_parser: F,
    mut parser: G)
    -> impl FnMut(Lexer<'text, Sc, Cm>) -> ParseResult<'text, Sc, Cm, usize>
    where
        Sc: Scanner,
        Cm: ColumnMetrics,
        F: FnMut(Lexer<'text, Sc, Cm>) -> ParseResult<'text, Sc, Cm, V>,
        G: FnMut(Lexer<'text, Sc, Cm>) -> ParseResult<'text, Sc, Cm, U>,
{
    move |lexer| {
        intersperse_collect_until(low, high, 
                &mut stop_parser,
                discard(&mut parser),
                empty)
            (lexer)
            .map_value(|vals| vals.len())
    }
}
