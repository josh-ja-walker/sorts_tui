use std::fmt::{self, Display};

use strum_macros::Display as EnumDisplay;

#[derive(Clone)]
pub struct Count {
    count: usize,
    #[allow(dead_code)] count_type: CountType,
}

impl Count {
    pub fn new(count_type: CountType) -> Count {
        Count {
            count: 0,
            count_type
        }
    } 

    pub fn increment(&mut self) {
        self.count += 1
    }
}

#[derive(Clone, EnumDisplay)]
pub enum CountType {
    Shuffles,
    Comparisons
}

impl Display for Count {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}",
            self.count,
            self.count_type.to_string().to_lowercase()
        )
    }
}
