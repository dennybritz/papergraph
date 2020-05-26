enum RDFObject<'a> {
    Literal(&'a str),
    UID(&'a str),
}

use RDFObject::Literal;
use RDFObject::UID;

impl<'a> RDFObject<'a> {
    pub fn string(&self) -> String {
        match self {
            RDFObject::Literal(s) => {
                let escaped = s
                    .replace("\\", "\\\\")
                    .replace(r###"""###, r###"\""###)
                    .replace("\n", "\\n")
                    .replace("\r", "\\r");
                format!("\"{}\"", escaped)
            }
            RDFObject::UID(s) => format!("{}", s),
        }
    }
}

struct Triple<'a> {
    pub subject: RDFObject<'a>,
    pub predicate: &'a str,
    pub object: RDFObject<'a>,
}

impl<'a> Triple<'a> {
    pub fn new(subject: RDFObject<'a>, predicate: &'a str, object: RDFObject<'a>) -> Self {
        Triple {
            subject,
            predicate,
            object,
        }
    }

    pub fn str(&self) -> String {
        format!(
            "{} {} {} .",
            &self.subject.string(),
            self.predicate,
            self.object.string()
        )
    }
}

pub fn s2_record_to_rdf_triples<'a>(record: &'a crate::io::Paper) -> Vec<String> {
    let mut res = vec![];
    let id = &record.id;
    let blank = format!("_:{}", id);
    let blank: &str = blank.as_ref();

    res.push(Triple::new(UID(blank), "<dgraph.type>", Literal("Paper")).str());
    res.push(Triple::new(UID(blank), "<paper_id>", Literal(&record.id)).str());
    res.push(Triple::new(UID(blank), "<title>", Literal(&record.title)).str());
    res.push(Triple::new(UID(blank), "<abstract>", Literal(&record.paper_abstract)).str());
    if let Some(year) = record.year {
        let year = format!("{}", year);
        res.push(Triple::new(UID(blank), "<year>", Literal(&year)).str());
    }
    res.push(Triple::new(UID(blank), "<s2_url>", Literal(&record.s2_url)).str());
    res.push(Triple::new(UID(blank), "<doi>", Literal(&record.doi)).str());
    res.push(Triple::new(UID(blank), "<doi_url>", Literal(&record.doi_url)).str());

    for pdf_url in record.pdf_urls.iter() {
        res.push(Triple::new(UID(blank), "<pdf_urls>", Literal(pdf_url)).str());
    }
    for fos in record.fields_of_study.iter() {
        res.push(Triple::new(UID(blank), "<fields_of_study>", Literal(fos)).str());
    }
    for entity in record.entities.iter() {
        res.push(Triple::new(UID(blank), "<entities>", Literal(entity)).str());
    }

    for author in record.authors.iter() {
        for id in author.ids.first() {
            let author_blank = format!("_:{}", id);
            res.push(Triple::new(UID(&author_blank), "<dgraph.type>", Literal("Author")).str());
            res.push(Triple::new(UID(&author_blank), "<name>", Literal(&author.name)).str());
            res.push(Triple::new(UID(&author_blank), "<author_id>", Literal(id)).str());
            res.push(Triple::new(UID(blank), "<authors>", UID(&author_blank)).str());
        }
    }

    for citation in record.out_citations.iter() {
        let cit_blank = format!("_:{}", citation);
        res.push(Triple::new(UID(&cit_blank), "<dgraph.type>", Literal("Paper")).str());
        res.push(Triple::new(UID(blank), "<cites>", UID(&cit_blank)).str());
    }

    for citation in record.in_citations.iter() {
        let cit_blank = format!("_:{}", citation);
        res.push(Triple::new(UID(&cit_blank), "<dgraph.type>", Literal("Paper")).str());
        res.push(Triple::new(UID(&cit_blank), "<cites>", UID(blank)).str());
    }

    res
}
