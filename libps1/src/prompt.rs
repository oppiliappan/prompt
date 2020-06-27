use crate::module::Module;
use std::fmt;

pub struct Prompt {
    modules: Vec<Module>,
}

impl Prompt {
    pub fn with_modules(modules: Vec<Module>) -> Self {
        return Prompt { modules };
    }
}

impl fmt::Display for Prompt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for m in self.modules.iter() {
            write!(f, "{}", m)?
        }
        Ok(())
    }
}

#[macro_export]
macro_rules! prompt {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            Prompt::with_modules(temp_vec)
        }
    };
}
