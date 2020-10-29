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
#[macro_export]
macro_rules! end_loan_route {
    () => {
        "/end_loan/{}"
    };
}
