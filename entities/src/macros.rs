#[macro_export]
macro_rules! server_root {
    () => {
        "{}"
    };
}
#[macro_export]
macro_rules! book_route {
    () => {
        "/book/{}"
    };
}
#[macro_export]
macro_rules! books_route {
    () => {
        "/books/{}"
    };
}
#[macro_export]
macro_rules! books_route_root {
    () => {
        "/books"
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
