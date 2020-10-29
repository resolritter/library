pub fn migration() -> String {
    (r###"
     CREATE TABLE public.book (
         id serial NOT NULL,
         title text NOT NULL,
         CONSTRAINT book_pk PRIMARY KEY (id)
     );
     "###)
        .to_string()
}
