pub fn migration() -> String {
    (r###"
     ALTER TABLE public.book ADD CONSTRAINT book_fk FOREIGN KEY (lease_id) REFERENCES public."user"(id);
     "###).to_string()
}
