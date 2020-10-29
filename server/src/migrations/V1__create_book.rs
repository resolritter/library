pub fn migration() -> String {
    (r###"
     CREATE TABLE public.book (
         title text NOT NULL,
         borrow_id varchar(256) NULL,
         borrow_until bigint NULL,
         CONSTRAINT book_pk PRIMARY KEY (title)
     );
     "###)
        .to_string()
}
