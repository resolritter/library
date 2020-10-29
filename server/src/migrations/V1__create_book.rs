pub fn migration() -> String {
    (r###"
     CREATE TABLE public.book (
         id serial NOT NULL,
         title text NOT NULL,
         borrow_id varchar(256) NULL,
         borrow_until bigint NULL,
         CONSTRAINT book_pk PRIMARY KEY (id)
     );
     "###)
        .to_string()
}
