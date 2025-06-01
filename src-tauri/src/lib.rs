pub mod lib {
    pub mod github_api;
    pub mod scoop_check;
}

// Re-export the lib module's content for ease of use
pub use lib::*;
