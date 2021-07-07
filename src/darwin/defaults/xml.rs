use scraper::{ElementRef, Html, Selector};

pub fn parse_xml(v: &str) -> Vec<(String, String, String)> {
    let document = Html::parse_fragment(v);
    let selector = Selector::parse("html > plist > dict > *").unwrap();
    let root = document.select(&selector);

    return parse_dict("", root.collect());
}

fn parse_dict(
    parent_key: &str,
    itr: Vec<ElementRef>,
) -> Vec<(String, String, String)> {
    let mut ktv: Vec<(String, String, String)> = Vec::new();

    for n in (0..itr.len()).step_by(2) {
        let elem = itr.iter().nth(n).unwrap();
        let key = parent_key.to_string() + ":" + elem.text().next().unwrap();

        let elem = match itr.iter().nth(n + 1) {
            Some(v) => v,
            None => {
                continue;
            }
        };

        ktv.extend_from_slice(parse_value(key.as_str(), elem).as_slice());
    }

    return ktv;
}

fn parse_value(
    parent_key: &str,
    elem: &ElementRef,
) -> Vec<(String, String, String)> {
    let typ = elem.value().name().to_string();

    match typ.as_str() {
        "date" | "integer" | "string" => {
            let value = elem.text().next().unwrap_or("").to_string();

            return vec![(parent_key.to_string(), typ, value)];
        }
        "true" | "false" => {
            return vec![(parent_key.to_string(), "bool".to_string(), typ)];
        }
        // "data" => {
        //     let value = elem
        //         .text()
        //         .next()
        //         .unwrap_or("")
        //         .split("\n")
        //         .map(|v| {
        //             String::from_utf8_lossy(
        //                 base64::decode(v.trim()).unwrap().as_slice(),
        //             )
        //             .to_string()
        //         })
        //         .collect::<Vec<_>>()
        //         .join("\n");

        //     return vec![(parent_key.to_string(), typ, value)];
        // }
        "dict" => {
            let f = Html::parse_fragment(elem.html().as_str());
            let selector = Selector::parse("html > dict > *").unwrap();

            return parse_dict(parent_key, f.select(&selector).collect());
        }
        "array" => {
            let f = Html::parse_fragment(elem.html().as_str());
            let selector = Selector::parse("html > array > *").unwrap();
            let mut ktv: Vec<(String, String, String)> = Vec::new();

            for (i, e) in
                f.select(&selector).collect::<Vec<_>>().iter().enumerate()
            {
                let key_with_offset =
                    parent_key.to_string() + ":" + i.to_string().as_str();

                ktv.extend_from_slice(
                    parse_value(key_with_offset.as_str(), e).as_slice(),
                );
            }

            return ktv;
        }
        _ => vec![],
    }
}
