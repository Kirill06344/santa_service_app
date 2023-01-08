#[derive(Debug)]
pub enum Errors {
    CantFindGroupByName,
    AccessDenied,
    NotUpdated,
    DbConnectionError,
    NotUniqueGroupName,
    CantFindUserName,
    AloneAdmin,
    GroupClosed,
    NotEnoughParticipants
}