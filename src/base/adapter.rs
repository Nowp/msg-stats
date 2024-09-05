/// Implement this trait to allow conversion between a type [T] and its database model equivalent [U].
pub trait Adapter<T> {
    type Type;
    fn to_database_model(self) -> T;
    fn from_database_model(model: T) -> Self::Type;
}
