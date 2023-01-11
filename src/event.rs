/// BGP の RFC 内 8.1 で定義されている Event を表す enum

#[derive(PartialEq, Eq, Debug, Clone, Hash)]
pub enum Event {
    ManualStart,
    // 正常系しか実装しない本実装では別の Event として扱う意味が無いため、TcpConnectionConfirmed は TcpCrAcked も兼ねている
    TcpConnectionConfirmed,
}
