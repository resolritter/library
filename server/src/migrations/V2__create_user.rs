pub fn migration() -> String {
    (r###"
     CREATE TABLE public."user" (
         email varchar(256) NOT NULL,
         access_level int4 NULL DEFAULT 0,
         CONSTRAINT user_pk PRIMARY KEY (email)
     );
     "###)
        .to_string()
}
