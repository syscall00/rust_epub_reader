// create tests for the library
#[cfg(test)]
mod tests {


    // test the function that, starting from an html string, returns the dom to be rendered
    #[test]
    fn test_html_to_dom() {
        use super::html_to_dom;
        use super::dom::Dom;

        
        let html = "<html><head></head><body><p>test</p></body></html>";

        let dom = html_to_dom(html);

        assert_eq!(dom, Dom::new("html", vec![Dom::new("head", vec![]), Dom::new("body", vec![Dom::new("p", vec![])])]));
    }

    
}

