
pub type ScopeLoc = usize;

/// Enum type that captures the lifetime of a variable.
///
/// The lifetime can be either: Dynamic, Static, or Scoped.
///
/// # Dynamic Lifetimes
/// Dynamic lifetimes are lifetimes of heap allocations. These are not analysed by Ferrum.
///
/// # Static Lifetimes
/// Static lifetimes are lifetimes that are static within the context of the programs runtime. This
/// applies mostly to constants.
///
/// # Scoped Lifetimes
/// Scoped lifetimes are the lifetimes of stack variables. Since the data is invalidated once the
/// variable goes out of scope, these lifetimes can be analysed during compile time.
pub enum LifeTime {
    Dynamic,
    Static,
    Scoped(ScopeLoc, ScopeLoc),
}
