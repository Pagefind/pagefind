use super::{PageWord, SearchIndex, WordData, WordVariant};
use crate::util::*;
use minicbor::{decode, Decoder};

/*
{} = fixed length array
{
    [
        {
            String,             // word
            [                   // pages
                {
                    u32,        // page number
                    [
                        u32,    // page location
                        ...
                    ]
                },
                ...
            ],
            [                   // additional_variants
                {
                    String,     // variant form
                    [           // variant pages
                        {
                            u32,    // page number
                            [u32]   // locations
                        },
                        ...
                    ]
                },
                ...
            ]
        },
        ...
    ]
}
*/

impl SearchIndex {
    fn decode_pages(decoder: &mut Decoder) -> Result<Vec<PageWord>, decode::Error> {
        let pages_count = consume_arr_len!(decoder);
        let mut page_arr = Vec::with_capacity(pages_count as usize);
        let mut cumulative_page = 0;

        for _ in 0..pages_count {
            consume_fixed_arr!(decoder);
            let page_delta = consume_num!(decoder);
            cumulative_page += page_delta;

            let mut page = PageWord {
                page: cumulative_page,
                locs: vec![],
            };

            let word_locations = consume_arr_len!(decoder);
            let mut weight = 25;
            let mut last_position = 0;

            for _ in 0..word_locations {
                let loc = consume_inum!(decoder);
                // Negative numbers represent a change in the weighting of subsequent words.
                if loc.is_negative() {
                    let abs_weight = (loc + 1) * -1;
                    weight = if abs_weight > 255 {
                        255
                    } else {
                        abs_weight as u8
                    };
                    // First position after weight change is absolute
                    last_position = 0;
                    debug!({
                        format!("Encountered weight marker {loc:#?}, weighting subsequent words as {weight:#?}")
                    });
                } else {
                    // Delta-encoded position
                    last_position += loc as u32;
                    page.locs.push((weight, last_position));
                }
            }

            page_arr.push(page);
        }
        Ok(page_arr)
    }

    pub fn decode_index_chunk(&mut self, index_bytes: &[u8]) -> Result<(), decode::Error> {
        debug!({ format!("Decoding {:#?} index bytes", index_bytes.len()) });
        let mut decoder = Decoder::new(index_bytes);

        consume_fixed_arr!(decoder);

        debug!({ "Reading words array" });
        let words = consume_arr_len!(decoder);
        debug!({ format!("Reading {:#?} words", words) });
        for _ in 0..words {
            let word_arr_len = match decoder.array()? {
                Some(n) => n,
                None => return Err(decode::Error::message("Word array length not specified")),
            };

            let key = consume_string!(decoder);
            let pages = Self::decode_pages(&mut decoder)?;

            // We generally don't strive for backwards compat in our indexes,
            // but where possible we include some just in case.
            // Here, we handle parsing older indexes that do not include the
            // additional_variants field within each word.
            // In a future release we can remove this compat.
            let additional_variants = if word_arr_len >= 3 {
                let variants_count = consume_arr_len!(decoder);
                let mut variants = Vec::with_capacity(variants_count as usize);
                for _ in 0..variants_count {
                    consume_fixed_arr!(decoder);
                    let form = consume_string!(decoder);
                    let variant_pages = Self::decode_pages(&mut decoder)?;
                    variants.push(WordVariant {
                        form,
                        pages: variant_pages,
                    });
                }
                variants
            } else {
                Vec::new()
            };

            self.words.insert(
                key,
                WordData {
                    pages,
                    additional_variants,
                },
            );
        }
        debug!({ "Finished reading words" });

        Ok(())
    }
}
