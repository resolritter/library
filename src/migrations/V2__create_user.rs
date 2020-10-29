pub fn migration() -> String {
    (r###"
     CREATE TABLE public."user" (
         id serial NOT NULL,
         username varchar(32) NOT NULL,
         CONSTRAINT user_pk PRIMARY KEY (id)
     );
     "###)
        .to_string()
}
