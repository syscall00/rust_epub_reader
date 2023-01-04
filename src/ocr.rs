use druid::im::Vector;

use crate::appstate::PagePosition;



fn text_preparation(text: &str) -> String {
    text.chars()
        .filter(|c| c.is_alphanumeric() || c.is_whitespace())
        .collect::<String>()
}





pub fn search_with_ocr_input(full_text : Vector<Vector<String>>, image_path: &str) -> PagePosition {
        let mut lt = leptess::LepTess::new(None, "eng").unwrap();
        lt.set_image(image_path).unwrap();

        let mut schema_builder = tantivy::schema::Schema::builder();

        let title = schema_builder.add_text_field("title", tantivy::schema::TEXT | tantivy::schema::STORED);
        let chapter = schema_builder.add_u64_field("chapter", tantivy::schema::FAST | tantivy::schema::STORED);
        let position = schema_builder.add_u64_field("start_pos", tantivy::schema::FAST | tantivy::schema::STORED);

        let schema = schema_builder.build();

        let index = tantivy::Index::create_in_ram(schema);
        let mut index_writer = index.writer(50_000_000).unwrap();


        let recognized_text = text_preparation(&lt.get_utf8_text().unwrap());

        for i in 0..full_text.len() {
            let mut texta = String::new();
            // For each richtext in the chapter, when size is greater than 1000, create a new document
            for (j, richtext) in full_text[i].iter().enumerate() {
                // texta should contain recognized_text.len() character, starting from j richtext and ending at most at j + 1 richtext
                let rich_text = richtext.as_str();
                if rich_text.len() >= recognized_text.len() {
                    texta = utf8_slice::slice(rich_text, 0, recognized_text.len()).to_string();
                    let mut doc = tantivy::Document::default();
                    doc.add_text(title, &texta);
                    doc.add_u64(chapter, i as u64);
                    doc.add_u64(position, j as u64);
                    index_writer.add_document(doc).unwrap();
                    texta.clear();
                } else if rich_text.len() + texta.len() >= recognized_text.len() {
                    texta = utf8_slice::slice(rich_text, 0, recognized_text.len()).to_string();
                    let mut doc = tantivy::Document::default();
                    doc.add_text(title, &texta);
                    doc.add_u64(chapter, i as u64);
                    doc.add_u64(position, j as u64);
                    index_writer.add_document(doc).unwrap();
                    texta.clear();
                } else {
                    texta.push_str(rich_text);
                }


            }
            if texta.len() > 0 {
                let mut doc = tantivy::Document::default();
                doc.add_text(title, &texta);
                doc.add_u64(chapter, i as u64);
                doc.add_u64(position, 0 as u64);
                index_writer.add_document(doc).unwrap();
                texta.clear();
            }
        }


        index_writer.commit().unwrap();
        let reader = index.reader().unwrap();
        let searcher = reader.searcher();
        let query_parser = tantivy::query::QueryParser::for_index(
            &index,
            vec![tantivy::schema::Field::from_field_id(0)],
        );

        let query = query_parser.parse_query(&recognized_text).unwrap();

        let top_docs = searcher
            .search(&query, &tantivy::collector::TopDocs::with_limit(3))
            .unwrap();


        if top_docs.len() > 0 {
            let (_score, doc_address) = top_docs[0];
            let retrieved_doc = searcher.doc(doc_address).unwrap();
            let chapter = retrieved_doc.get_first(chapter).unwrap();
            let position = retrieved_doc.get_first(position).unwrap();
            //println!(
            //    "Document: {:?} in chapter {:?} position {:?} with score {}",
            //    retrieved_doc, chapter, position, score
            //);

            return PagePosition::new(
                chapter.as_u64().unwrap() as usize,
                position.as_u64().unwrap() as usize,
            );
        }

        PagePosition::new(0, 0)
}


pub fn reverse_search_with_ocr_input(full_text : Vector<Vector<String>>, image_1: &str, image_2: &str, current_position: &PagePosition) -> PagePosition {

    let image_1_rec = search_with_ocr_input(full_text.clone(), image_1);
    let image_2_rec = search_with_ocr_input(full_text.clone(), image_2);
    // extract image 1 text with leptonica
    //let mut lt = leptess::LepTess::new(None, "ita").unwrap();
    //lt.set_image(image_1).unwrap();
    
    //println!("image 1 text: {:?}", &lt.get_utf8_text().unwrap());
    //lt.set_image(image_2).unwrap()//;

    //println!("\n\n\n\n\n\n\n\n\n\n\n\n");
    //println!("image 1 text: {:?}",&lt.get_utf8_text().unwrap());

    println!("image 1 rec: {:?}", image_1_rec);
    println!("image 2 rec: {:?}", image_2_rec);

    let image_1_page = 25;
    let image_2_page = 18;

    // given that image 1 is on page 25 and image 2 is on page 18 AND image 1 is on the right of image 2 AND image 1 is at 
    // chapter 8 and position 0 while image 2 is at chapter 7 and position 0, we can deduce the current_position page through this calculation:

    let current_page = (current_position.chapter() as isize * 25) + current_position.richtext_number() as isize - (image_1_page * 8) - (image_2_page * 7);
    println!("current page: {:?}", current_page);
    PagePosition::new(0, 0)
}
