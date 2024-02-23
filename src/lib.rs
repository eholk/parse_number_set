use std::collections::BTreeSet;

use nom::{branch::alt, bytes::complete::tag, character::complete::space1, multi::separated_list1};
use thiserror::Error;

pub fn parse_number_set(input: &str) -> Result<Vec<usize>, ParseError<'_>> {
    let mut set = BTreeSet::new();

    let (_, entries) =
        separated_list1(alt((tag(","), space1)), alt((single_range, single_num)))(input)
            .map_err(ParseError::NomError)?;
    for e in entries {
        match e {
            Entry::Number(n) => {
                set.insert(n);
            }
            Entry::Range(start, end) => {
                for n in start..=end {
                    set.insert(n);
                }
            }
        }
    }

    Ok(set.into_iter().collect())
}

enum Entry {
    Number(usize),
    Range(usize, usize),
}

fn single_num(input: &str) -> nom::IResult<&str, Entry> {
    let (input, number) = nom::character::complete::u64(input)?;
    Ok((input, Entry::Number(number as usize)))
}

fn single_range(input: &str) -> nom::IResult<&str, Entry> {
    let (input, start) = nom::character::complete::u64(input)?;
    let (input, _) = alt((tag("-"), tag("..")))(input)?;
    let (input, end) = nom::character::complete::u64(input)?;
    Ok((input, Entry::Range(start as usize, end as usize)))
}

#[derive(Debug, Error)]
pub enum ParseError<'a> {
    #[error(transparent)]
    NomError(nom::Err<nom::error::Error<&'a str>>),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_number() -> anyhow::Result<()> {
        let result = parse_number_set("42")?;
        assert_eq!(result, vec![42]);
        Ok(())
    }

    #[test]
    fn one_range() -> anyhow::Result<()> {
        let result = parse_number_set("1..3")?;
        assert_eq!(result, vec![1, 2, 3]);
        Ok(())
    }

    #[test]
    fn numbers_and_ranges() -> anyhow::Result<()> {
        let result = parse_number_set("1,3,5..7,9,12..14")?;
        assert_eq!(result, vec![1, 3, 5, 6, 7, 9, 12, 13, 14]);
        Ok(())
    }

    #[test]
    fn space_separated() -> anyhow::Result<()> {
        let result = parse_number_set("1 3 5..7 9 12..14")?;
        assert_eq!(result, vec![1, 3, 5, 6, 7, 9, 12, 13, 14]);
        Ok(())
    }
}
