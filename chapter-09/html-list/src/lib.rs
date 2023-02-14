#[macro_export]
macro_rules! html_list {
    ([$($x:expr),*]) => (html_list!($($x),*));
    ($($x:expr),*) => ({
        let mut list_items = "<ul>".to_string();
        $(
            list_items.push_str(&format!("<li>{}</li>", $x));
            
        )*
        list_items.push_str("</ul>");
        list_items
    }); 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn html_test() {
        let result = "<ul><li>1</li><li>2</li></ul>".to_string();
        let list1 = html_list!([1, 2]);
        let list2 = html_list!(1, 2);
        assert_eq!(list1, result);
        assert_eq!(list2, result);
    }
}
