pub fn migration() -> String {
    (r###"
     ALTER TABLE public.book ADD CONSTRAINT book_fk FOREIGN KEY (borrow_id) REFERENCES public."user"(email);
     "###).to_string()
}
