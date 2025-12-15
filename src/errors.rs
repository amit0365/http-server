pub enum ServerError {
    Io(String),
    InvalidUtf8(String),
    ParseInt(String),
    ParseFloat(String),
    InvalidCommand(String),
    InvalidRespFormat(String),
    InvalidStreamId(String),
    WrongType(String),
    KeyNotFound(String),
    LockPoisoned(String),
    ChannelSend(String),
    ConnectionClosed,
    TooManyWaiters,
    Base64Decode(String),
    Other(String),
}

pub type ServerResult<T> = Result<T, ServerError>;