#[derive(Clone, Debug)]
pub enum ParsingError<'a> {
    NotASizingUnit(&'a str),
    InvalidConstraintSyntax(&'a str),
    NotEnoughArgumentsToSizing(&'a str),
}

use ParsingError::*;

use crate::{ConstraintUnit, Float, SizingUnit};

pub fn parse_constraint_unit<'a>(input: &'a str) -> Result<ConstraintUnit, ParsingError<'a>> {
    if input == "_" {
        return Ok(ConstraintUnit::None);
    }

    let (input, is_percent) = match input.find('%') {
        Some(b) => {
            if &input[(b + 1)..] != "" {
                Err(InvalidConstraintSyntax(input))?
            }

            (&input[..b], true)
        }
        None => (input, false),
    };

    input
        .parse::<Float>()
        .map(|f| {
            if is_percent {
                ConstraintUnit::Percent(f)
            } else {
                ConstraintUnit::Fixed(f)
            }
        })
        .map_err(|_| InvalidConstraintSyntax(input))
}

pub fn parse_sizing_unit<'a>(input: &'a str) -> Result<SizingUnit, ParsingError<'a>> {
    let delim = input.find(':');

    let sizing = match delim {
        Some(b) => &input[..b],
        None => input,
    };

    match sizing {
        "Percent" => match delim {
            Some(b) => {
                let constraint_string = &input[(b + 1)..];

                let next = constraint_string
                    .find(',')
                    .ok_or(NotEnoughArgumentsToSizing(input))?;
                let size = constraint_string[..next]
                    .parse::<Float>()
                    .map_err(|_| InvalidConstraintSyntax(&constraint_string[..next]))?;

                let constraint_string = &constraint_string[(next + 1)..];
                let next = constraint_string
                    .find(',')
                    .ok_or(NotEnoughArgumentsToSizing(input))?;
                let min_constraint = parse_constraint_unit(&constraint_string[..next])?;

                let constraint_string = &constraint_string[(next + 1)..];
                let max_constraint = parse_constraint_unit(&constraint_string)?;

                Ok(SizingUnit::Percent(size, min_constraint, max_constraint))
            }
            None => Err(NotEnoughArgumentsToSizing(input)),
        },
        "Stretch" => match delim {
            Some(b) => Ok(SizingUnit::Stretch(parse_constraint_unit(
                &input[(b + 1)..],
            )?)),
            None => Ok(SizingUnit::Stretch(ConstraintUnit::None)),
        },
        "Collapse" => match delim {
            Some(b) => Ok(SizingUnit::Collapse(parse_constraint_unit(
                &input[(b + 1)..],
            )?)),
            None => Ok(SizingUnit::Collapse(ConstraintUnit::None)),
        },
        "Fixed" => match delim {
            Some(b) => Ok(SizingUnit::Fixed(
                input[(b + 1)..]
                    .parse::<Float>()
                    .map_err(|_| InvalidConstraintSyntax(&input[(b + 1)..]))?,
            )),
            None => Err(NotEnoughArgumentsToSizing(input)),
        },
        _ => Err(NotASizingUnit(sizing))?,
    }
}
