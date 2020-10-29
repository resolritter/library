pub fn migration() -> String {
    (r###"
     CREATE TABLE public."user" (
         email varchar(256) NOT NULL,
         access_mask int4 NULL DEFAULT 0,
         access_token varchar(512) NULL,
         CONSTRAINT user_pk PRIMARY KEY (email)
     );
     ALTER TABLE public.book ADD CONSTRAINT book_fk FOREIGN KEY (borrow_id) REFERENCES public."user"(email);
     "###)
        .to_string()
}
