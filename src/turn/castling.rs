#[derive(Debug, PartialEq, Clone, Copy)]
/// The type of castling
pub enum CastlingType {
    /// Queenside castling
    Long,
    /// Kingside castling
    Short,
}
