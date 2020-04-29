use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Paper {
    pub id: String,
    pub authors: Vec<Author>,
    pub year: Option<usize>,
    pub title: String,
    pub paper_abstract: String,
    pub fields_of_study: Vec<String>,
    pub out_citations: Vec<String>,
    pub in_citations: Vec<String>,
    pub s2_url: String,
    pub doi: String,
    pub doi_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Author {
    pub name: String,
    pub ids: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;
    use std::fs::File;
    use std::io::{self, BufRead};

    #[test]
    fn test_parse_author() {
        let author_json = r#"
        {
            "name": "Christian Mark Cabaluna",
            "ids": [
              "1394532449"
            ]
        }"#;

        let author: Author = serde_json::from_str(author_json).expect("failed to parse author");
        assert_eq!(author.name, "Christian Mark Cabaluna");
        assert_eq!(author.ids[0], "1394532449");
    }

    #[test]
    fn test_parse_paper() {
        let file = File::open("data/sample-S2-records.json").unwrap();
        let papers: Vec<Paper> = io::BufReader::new(file)
            .lines()
            .map(|l| l.unwrap())
            .map(|l| serde_json::from_str(&l).expect("failed to parse paper"))
            .collect();

        assert_eq!(100, papers.len());
        let paper = papers.get(21).unwrap();
        assert_eq!("Usability Evaluation of Google Classroom : Basis for the Adaptation of GSuite E-Learning Platform", paper.title);
        assert_eq!("84d3c1e979cc6dac92684db2b04df0f00d3f114e", paper.id);
        assert_eq!(Some(2017), paper.year);
        assert_eq!(9, paper.out_citations.len());
        assert_eq!(13, paper.in_citations.len());
        assert_eq!(5, paper.authors.len());
    }
}
