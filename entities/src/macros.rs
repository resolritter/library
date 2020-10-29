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
macro_rules! lease_route {
    () => {
        "/lease/{}"
    };
}
