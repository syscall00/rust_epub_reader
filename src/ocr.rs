use crate::data::PagePosition;
/**
 * OCR is a module that contains functions to search for text from a page image using OCR.
 * It uses Tantivy module as a search engine and Leptess module to perform OCR.
 *
 */

/**
 * Utility function to remove all non-alphanumeric characters from a string.
 *
 * @param text: The text to be cleaned.
 *  
 * @return String: The cleaned text.
 */
fn text_preparation(text: &str) -> String {
    text.chars()
        .filter(|c| c.is_alphanumeric() || c.is_whitespace())
        .collect::<String>()
}


/**
 * Search for text in a page image using OCR.
 *
 * @param full_text: Vector of Vector of String. Each Vector of String is a chapter.
 * @param image_path: Path to the page image.
 *
 * @return PagePosition: The position of the text in the page.
 */
pub fn search_with_ocr_input(full_text: Vec<Vec<String>>, image_path: &str) -> PagePosition {
    
    let mut lt = leptess::LepTess::new(None, "eng").unwrap();
    let set_image = lt.set_image(image_path);

    // check if image exists and is image
    if !std::path::Path::new(image_path).exists() || set_image.is_err() {
        return PagePosition::default();
    }
    let mut schema_builder = tantivy::schema::Schema::builder();

    let title =
        schema_builder.add_text_field("title", tantivy::schema::TEXT | tantivy::schema::STORED);
    let chapter =
        schema_builder.add_u64_field("chapter", tantivy::schema::FAST | tantivy::schema::STORED);
    let position =
        schema_builder.add_u64_field("start_pos", tantivy::schema::FAST | tantivy::schema::STORED);
    
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

    PagePosition::default()
}

/**
 *
 * metrics:
 * - posizione inizio pagina in epub;
 * - num medio di caratteri su libro fisico per pagina ( num char tra le due pagine / num di pagine fisiche (assumption) )
 * - posizione target page in epub;
 *
 * distanza inizio pagina e target page in num char
 *
 *
 * num_target_pagina_libro_fisico = posizione_prima_pagina_fotografata
 *
 * range_epub = num di pagine digitali tra le due pagine fisiche (media) in epub
 *                                         
 * target_physical_page =  mean_physical_page_character(v) / mean_epub_page_character * (range_epub)
 *              = 18 + 1082 / 858 *   (81159 / 858 = 87.2)
 */
pub fn reverse_search_with_ocr_input(
    full_text: Vec<Vec<String>>,
    image_1: &str,
    image_2: &str,
    current_position: &PagePosition,
) -> PagePosition {
    let image_1_rec = search_with_ocr_input(full_text.clone(), image_1);
    let image_2_rec = search_with_ocr_input(full_text.clone(), image_2);
    let mut lt = leptess::LepTess::new(None, "eng").unwrap();
    lt.set_image(image_1).unwrap();
    let image_1_text = text_preparation(&lt.get_utf8_text().unwrap());
    lt.set_image(image_2).unwrap();
    let image_2_text = text_preparation(&lt.get_utf8_text().unwrap());
    // count all character in book
    let mut full_count = 0;
    for chapter in &full_text {
        for line in chapter {
            full_count += line.len();
        }
    }
    let mean_book_page_character = (image_2_text.len() + image_1_text.len()) / 2;
    println!("Mean Book page character: {}", mean_book_page_character);
    
    let distance_from_character = if current_position < &image_1_rec {
        get_distance_in_character(&full_text, &current_position, &image_1_rec)
    }
    else {
        get_distance_in_character(&full_text, &image_1_rec, &current_position)
    };

    let page_1_distance_from_0 =
        get_distance_in_character(&full_text, &PagePosition::new(0, 0), &image_1_rec);

    let page_1 = (page_1_distance_from_0 as f64 / mean_book_page_character as f64).round() as usize;
    let percentage_of_page1 = (page_1_distance_from_0 as f64 / full_count as f64) * 100.0;
    let page_number_distance_1 = (distance_from_character as f64 / mean_book_page_character as f64).round() as usize;

    println!("---------- Page 1 stats ----------");
    println!("Distance from 0: {}", page_1_distance_from_0);
    println!("Distance from current: {}", distance_from_character);
    println!("Percentage of page 1 in epub: {}", percentage_of_page1);
    println!("Mean page 1: {}", page_1); 
    println!("Page number distance from current {}", page_number_distance_1); 
    println!("Should be {} ", if current_position < &image_1_rec {page_1-page_number_distance_1} else {page_1+page_number_distance_1});
    println!("----------------------------------\n");

    let distance_from_character2 =
        get_distance_in_character(&full_text, &current_position, &image_2_rec);
    
    let page_2_distance_from_0 =
        get_distance_in_character(&full_text, &PagePosition::new(0, 0), &image_2_rec);

    let page_2 = (page_2_distance_from_0 as f64 / mean_book_page_character as f64).round() as usize;
    let percentage_of_page2 = (page_2_distance_from_0 as f64 / full_count as f64) * 100.0;
    let page_number_distance_2 = (distance_from_character2 as f64 / mean_book_page_character as f64).round() as usize;

    println!("---------- Page 2 stats ----------");
    println!("Distance from 0: {}", page_2_distance_from_0);
    println!("Distance from current: {}", distance_from_character2);
    println!("Percentage of page 2 in epub: {}", percentage_of_page2);
    println!("Mean page 2: {}", page_2); // 198
    println!("Page number distance from current {}", page_number_distance_2); 
    println!("Should be {} ", if current_position < &image_2_rec {page_2-page_number_distance_2} else {page_2+page_number_distance_2});
    println!("----------------------------------\n");

    let char_read_until_now =
        get_distance_in_character(&full_text, &PagePosition::new(0, 0), &current_position);

    println!("Should be, using current pos: {}", char_read_until_now / mean_book_page_character);

    println!("Read until now {}\n", char_read_until_now);
    PagePosition::default()
}

fn get_distance_in_character(
    full_text: &Vec<Vec<String>>,
    start_position: &PagePosition,
    end_position: &PagePosition,
) -> usize {
    let mut count = 0;
    
    for (i, chapter) in full_text.iter().enumerate() {
        if i < start_position.chapter() {
            continue;
        }
        if i > end_position.chapter() {
            break;
        }
        if i == end_position.chapter() {
            count += end_position.richtext_number();
            break;
        }

        // sum all the characters of the chapter
        for line in chapter {
            count += line.len();
        }
    }

    return count;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn generate_sample_vector() -> Vec<Vec<String>> {
        vec![
            vec![
                "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed euismod, nunc ut aliquam".to_owned(),
                "tincidunt, nunc nisl aliquet nisl, ut aliquet nunc nisl eget nisl. Donec auctor, nunc".to_owned(),
            ],
            vec![
                "MR Bennet was among the earliest of those who waited on Mr Bingley. He had 
                always intended to visit him, toigh to the last always assuring his wife
                that he should not go; and till the evening after the visit was paid, she had no 
                knowledge of it. It was then disclosed in the following manner. Observing his second daughter
                employed in trimming a hat, he suddendly addressed her with,".to_owned(),
                "tincidunt, nunc nisl aliquet nisl, ut aliquet nunc nisl eget nisl. Donec auctor, nunc".to_owned(),
            ],
            vec![
                "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed euismod, nunc ut aliquam".to_owned(),
                "tincidunt, nunc nisl aliquet nisl, ut aliquet nunc nisl eget nisl. Donec auctor, nunc".to_owned(),
            ]]
    }

    #[test]
    fn test_search_with_ocr_input() {
        let full_text = generate_sample_vector();

        let image_1 = "examples/assets/ocr_pride_and_prejudice.jpg";
        let result = search_with_ocr_input(full_text, image_1);

        assert_eq!(result, PagePosition::new(1, 0));
    }

    #[test]
    fn test_search_with_ocr_input_with_wrong_result() {
        let full_text = generate_sample_vector();

        let image_1 = "examples/assets/ocr_pride_and_prejudice.jpg";
        let result = search_with_ocr_input(full_text, image_1);

        assert_ne!(result, PagePosition::new(0, 0));
    }

    #[test]
    fn test_search_with_ocr_input_with_wrong_image_or_non_existing_image() {
        let full_text = generate_sample_vector();

        let image_1 = "examples/assets/image_not_existing.jpg";
        let image_2 = "examples/assets/not_an_image.jpg";

        let result = search_with_ocr_input(full_text.clone(), image_1);

        assert_eq!(result, PagePosition::new(usize::MAX, usize::MAX));
        let result = search_with_ocr_input(full_text, image_2);
        assert_eq!(result, PagePosition::new(usize::MAX, usize::MAX));

        
    }

    #[test]
    fn test_search_with_ocr_input_with_empty_text() {
        let full_text = vec![];

        let image_1 = "examples/assets/ocr_pride_and_prejudice.jpg";
        let result = search_with_ocr_input(full_text, image_1);

        assert_eq!(result, PagePosition::new(usize::MAX, usize::MAX));
    }


}