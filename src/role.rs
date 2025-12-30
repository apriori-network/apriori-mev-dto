use serde::{Deserialize, Serialize};

/// User role enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(i32)]
pub enum Role {
    /// Searcher: can submit and simulate bundles (unary methods only)
    Searcher = 0,
    /// Builder: same permissions as Searcher (unary methods only)
    Builder = 1,
    /// Relayer: can stream bundles to leaders (for relayer nodes, internal use)
    Relayer = 2,
    /// Fullnode: can stream bundles to leaders (for full nodes, internal use)
    Fullnode = 3,
    /// Invalid: invalid or unauthenticated role
    Invalid = 255,
}

impl Role {
    /// Parse role from i32
    pub fn from_i32(value: i32) -> Option<Self> {
        match value {
            0 => Some(Role::Searcher),
            1 => Some(Role::Builder),
            2 => Some(Role::Relayer),
            3 => Some(Role::Fullnode),
            255 => Some(Role::Invalid),
            _ => None,
        }
    }

    /// Get role name
    pub fn name(&self) -> &'static str {
        match self {
            Role::Searcher => "Searcher",
            Role::Builder => "Builder",
            Role::Relayer => "Relayer",
            Role::Fullnode => "Fullnode",
            Role::Invalid => "Invalid",
        }
    }
}

