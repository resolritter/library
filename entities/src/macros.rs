#[macro_export]
macro_rules! server_root {
    () => {
        "{}"
    };
}
#[macro_export]
macro_rules! user_route {
    () => {
        "/user"
    };
}
#[macro_export]
macro_rules! book_route_root {
    () => {
        "/book"
    };
}
#[macro_export]
macro_rules! book_route {
    () => {
        concat!(book_route_root!(), "/{}")
    };
}
#[macro_export]
macro_rules! books_route_root {
    () => {
        "/books"
    };
}
#[macro_export]
macro_rules! books_route {
    () => {
        concat!(books_route_root!(), "/{}")
    };
}
#[macro_export]
macro_rules! book_borrow_route {
    () => {
        "/borrow"
    };
}
#[macro_export]
macro_rules! session_route {
    () => {
        "/session"
    };
}
