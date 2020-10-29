pub fn migration() -> String {
    (r###"
     ALTER TABLE public."user" ADD access_token varchar(512) NULL;
     "###)
        .to_string()
}
