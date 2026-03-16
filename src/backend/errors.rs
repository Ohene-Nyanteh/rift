pub struct Error {
    type: ErrorType,
    value: String
}



pub enum ErrorType {
    DeclarationError,
    InitiationError
}
