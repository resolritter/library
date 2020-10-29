pub fn migration() -> String {
    (r###"
     CREATE TABLE public.book (
         id serial NOT NULL,
         title text NOT NULL,
         lease_id varchar(256) NULL,
         lease_until bigint NULL,
         CONSTRAINT book_pk PRIMARY KEY (id)
     );
     "###)
        .to_string()
}
