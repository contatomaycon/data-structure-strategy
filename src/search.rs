use std::collections::HashMap;
use crate::indexer::SearchIndex;
use crate::models::ProductId;
use crate::normalizer::tokenize;

pub struct SearchEngine<'a> {
    pub index: &'a SearchIndex
}

impl<'a> SearchEngine<'a> {
    pub fn search(&self, query: &str) -> Vec<ProductId> {
        let mut scores: HashMap<ProductId, usize> = HashMap::new();
        for token in tokenize(query) {
            if let Some(ids) = self.index.by_name.get(&token) {
                for id in ids { *scores.entry(*id).or_insert(0) += 4; }
            }
            if let Some(ids) = self.index.by_brand.get(&token) {
                for id in ids { *scores.entry(*id).or_insert(0) += 3; }
            }
            if let Some(ids) = self.index.by_category.get(&token) {
                for id in ids { *scores.entry(*id).or_insert(0) += 2; }
            }
            if let Some(ids) = self.index.by_tag.get(&token) {
                for id in ids { *scores.entry(*id).or_insert(0) += 1; }
            }
        }
        let mut v: Vec<(ProductId, usize)> = scores.into_iter().collect();
        v.sort_by(|a,b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
        v.into_iter().map(|(id,_)| id).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::indexer::SearchIndex;
    use crate::models::Product;

    #[test]
    fn prefer_name_matches_over_tag_only_matches() {
        let p1 = Product {
            id: 1,
            name: "Gaming Keyboard".into(),
            brand: "Acme".into(),
            category: "Electronics".into(),
            tags: vec!["keyboard".into(), "pc".into()],
        };
        let p2 = Product {
            id: 2,
            name: "Office Setup".into(),
            brand: "WorkPro".into(),
            category: "Furniture".into(),
            tags: vec!["gaming".into()],
        };

        let mut index = SearchIndex::new();
        index.add_product(&p1);
        index.add_product(&p2);
        index.finalize();

        let engine = SearchEngine { index: &index };
        let results = engine.search("gaming");

        assert_eq!(results.first().copied(), Some(1));
    }
}