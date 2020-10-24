////////////////////////////////////////////////////////////////////////////////
// Tephra parser library
////////////////////////////////////////////////////////////////////////////////
// Copyright 2020 Skylor R. Schermer
// This code is dual licenced using the MIT or Apache 2 license.
// See licence-mit.md and licence-apache.md for details.
////////////////////////////////////////////////////////////////////////////////
//! Parser combinators for joining and bracketting.
////////////////////////////////////////////////////////////////////////////////

// Local imports.
use crate::lexer::Lexer;
use crate::lexer::Scanner;
use crate::position::ColumnMetrics;
use crate::result::ParseResult;
use crate::result::ParseResultExt as _;


////////////////////////////////////////////////////////////////////////////////
// Parse result selection combinators.
////////////////////////////////////////////////////////////////////////////////

/// Returns a parser which sequences two parsers wich must both succeed,
/// returning the value of the first one.
pub fn left<'text, Sc, Cm, L, R, X, Y>(mut left: L, mut right: R)
    -> impl FnMut(Lexer<'text, Sc, Cm>) -> ParseResult<'text, Sc, Cm, X>
    where
        Sc: Scanner,
        Cm: ColumnMetrics,
        L: FnMut(Lexer<'text, Sc, Cm>) -> ParseResult<'text, Sc, Cm, X>,
        R: FnMut(Lexer<'text, Sc, Cm>) -> ParseResult<'text, Sc, Cm, Y>,
{
    move |lexer| {
        let (l, succ) = (left)
            (lexer)?
            .take_value();

        (right)
            (succ.lexer)
            .map_value(|_| l)
    }
}

/// Returns a parser which sequences two parsers wich must both succeed,
/// returning the value of the second one.
pub fn right<'text, Sc, Cm, L, R, X, Y>(mut left: L, mut right: R)
    -> impl FnMut(Lexer<'text, Sc, Cm>) -> ParseResult<'text, Sc, Cm, Y>
    where
        Sc: Scanner,
        Cm: ColumnMetrics,
        L: FnMut(Lexer<'text, Sc, Cm>) -> ParseResult<'text, Sc, Cm, X>,
        R: FnMut(Lexer<'text, Sc, Cm>) -> ParseResult<'text, Sc, Cm, Y>,
{
    move |lexer| {
        let succ = (left)
            (lexer)?;

        (right)
            (succ.lexer)
    }
}

/// Returns a parser which sequences two parsers wich must both succeed,
/// returning their values in a tuple.
pub fn both<'text, Sc, Cm, L, R, X, Y>(mut left: L, mut right: R)
    -> impl FnMut(Lexer<'text, Sc, Cm>) -> ParseResult<'text, Sc, Cm, (X, Y)>
    where
        Sc: Scanner,
        Cm: ColumnMetrics,
        L: FnMut(Lexer<'text, Sc, Cm>) -> ParseResult<'text, Sc, Cm, X>,
        R: FnMut(Lexer<'text, Sc, Cm>) -> ParseResult<'text, Sc, Cm, Y>,
{
    move |lexer| {
        let (l, succ) = (left)
            (lexer)?
            .take_value();

        (right)
            (succ.lexer)
            .map_value(|r| (l, r))
    }
}

/// Returns a parser which sequences three parsers which must all succeed,
/// returning the value of the center parser.
pub fn bracket<'text, Sc, Cm, L, C, R, X, Y, Z>(
    mut left: L,
    mut center: C,
    mut right: R)
    -> impl FnMut(Lexer<'text, Sc, Cm>) -> ParseResult<'text, Sc, Cm, Y>
    where
        Sc: Scanner,
        Cm: ColumnMetrics,
        L: FnMut(Lexer<'text, Sc, Cm>) -> ParseResult<'text, Sc, Cm, X>,
        C: FnMut(Lexer<'text, Sc, Cm>) -> ParseResult<'text, Sc, Cm, Y>,
        R: FnMut(Lexer<'text, Sc, Cm>) -> ParseResult<'text, Sc, Cm, Z>,
{
    move |lexer| {
        log::debug!("bracket: left");
        let succ = (left)
            (lexer)?;

        log::debug!("bracket: center");
        let (c, succ) = (center)
            (succ.lexer)?
            .take_value();

        log::debug!("bracket: right");
        (right)
            (succ.lexer)
            .map_value(|_| c)
    }
}

/// Returns a parser which calls a bracketting parser before and after a center
/// parser.
pub fn bracket_symmetric<'text, Sc, Cm, C, B, X, Y>(
    mut bracket: B,
    mut center: C)
    -> impl FnMut(Lexer<'text, Sc, Cm>) -> ParseResult<'text, Sc, Cm, Y>
    where
        Sc: Scanner,
        Cm: ColumnMetrics,
        B: FnMut(Lexer<'text, Sc, Cm>) -> ParseResult<'text, Sc, Cm, X>,
        C: FnMut(Lexer<'text, Sc, Cm>) -> ParseResult<'text, Sc, Cm, Y>,
{
    move |lexer| {
        let succ = (&mut bracket)
            (lexer)?;

        let (c, succ) = (center)
            (succ.lexer)?
            .take_value();

        (&mut bracket)
            (succ.lexer)
            .map_value(|_| c)
    }
}

/// Returns a parser which sequences three parsers which must all succeed,
/// returning the value of the center parser. The right parser will receive the
/// output of the left parser as an argument.
pub fn bracket_dynamic<'text, Sc, Cm, L, C, R, X, Y, Z>(
    mut left: L,
    mut center: C,
    mut right: R)
    -> impl FnMut(Lexer<'text, Sc, Cm>) -> ParseResult<'text, Sc, Cm, Y>
    where
        Sc: Scanner,
        Cm: ColumnMetrics,
        L: FnMut(Lexer<'text, Sc, Cm>) -> ParseResult<'text, Sc, Cm, X>,
        C: FnMut(Lexer<'text, Sc, Cm>) -> ParseResult<'text, Sc, Cm, Y>,
        R: FnMut(Lexer<'text, Sc, Cm>, X) -> ParseResult<'text, Sc, Cm, Z>,
{
    move |lexer| {
        let (l, succ) = (left)
            (lexer)?
            .take_value();

        let (c, succ) = (center)
            (succ.lexer)?
            .take_value();

        (right)
            (succ.lexer, l)
            .map_value(|_| c)
    }
}
