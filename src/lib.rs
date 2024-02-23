use std::collections::BTreeSet;

use nom::error::VerboseError;
use thiserror::Error;

pub fn parse_number_set(input: &str) -> Result<Vec<usize>, ParseError<'_>> {
    let mut set = BTreeSet::new();

    let (_input, number) =
        nom::character::complete::u64::<_, VerboseError<_>>(input).map_err(ParseError::NomError)?;

    set.insert(number as usize);

    Ok(set.into_iter().collect())
}

#[derive(Debug, Error)]
pub enum ParseError<'a> {
    #[error(transparent)]
    NomError(nom::Err<VerboseError<&'a str>>),
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
}
