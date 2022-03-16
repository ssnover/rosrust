#![recursion_limit = "1024"]

mod rosmsg_include;
use proc_macro::TokenStream;

#[proc_macro]
pub fn rosmsg_include(input: TokenStream) -> TokenStream {
    let mut messages = Vec::new();
    let mut next_item = String::new();
    for item in input {
        match item.to_string().as_str() {
            "," => {
                messages.push(next_item);
                next_item = String::new();
            }
            s => next_item += s,
        }
    }
    let is_internal = next_item == "INTERNAL";
    let ignore_bad = next_item == "IGNORE_BAD";
    if !is_internal && !ignore_bad && !next_item.is_empty() {
        messages.push(next_item);
    }
    let message_refs = messages.iter().map(String::as_str).collect::<Vec<&str>>();
    rosmsg_include::depend_on_messages(&message_refs, is_internal, ignore_bad)
}
