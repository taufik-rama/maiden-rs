#![allow(unused_variables)]

use serde::Deserialize;
use thiserror::Error;

impl From<FixtureError> for crate::MaidenError {
    fn from(c: FixtureError) -> Self {
        crate::MaidenError::Fixture(c)
    }
}

#[derive(Error, Debug)]
pub enum FixtureError {}

#[derive(Deserialize, Debug, Clone)]
pub struct Fixture {}

impl std::ops::Add for Fixture {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        unimplemented!()
    }
}
