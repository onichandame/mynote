pub trait Private {
    type User;
    fn private(self, user: &Self::User) -> Self;
}
