/// Parsed and validated representation of ELF objects.
/// Suitable for creating and manipulating objects.
pub mod parsed;

/// Raw representation of ELF objects, intended to map directly to the ELF format.
/// Suitable for zero-copy reading and loading of objects.
pub mod raw;
