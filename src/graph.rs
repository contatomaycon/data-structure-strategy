use petgraph::graph::{NodeIndex, UnGraph};
use std::collections::{HashMap, HashSet, VecDeque};

use crate::models::{Product, ProductId};
use crate::normalizer::normalize_token;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum GraphNode {
    Product(ProductId),
    Feature(String),
}

pub struct ProductGraph {
    g: UnGraph<GraphNode, ()>,
    idx_by_product_id: HashMap<ProductId, NodeIndex>,
    idx_by_feature: HashMap<String, NodeIndex>,
}

impl ProductGraph {
    pub fn new() -> Self {
        Self {
            g: UnGraph::new_undirected(),
            idx_by_product_id: HashMap::new(),
            idx_by_feature: HashMap::new(),
        }
    }

    pub fn build(&mut self, products: &[Product]) {
        self.g = UnGraph::new_undirected();
        self.idx_by_product_id.clear();
        self.idx_by_feature.clear();

        for p in products {
            let idx = self.g.add_node(GraphNode::Product(p.id));
            self.idx_by_product_id.insert(p.id, idx);
        }

        for p in products {
            let product_idx = self.idx_by_product_id[&p.id];
            let mut features: HashSet<String> = HashSet::new();

            let category = normalize_token(&p.category);
            if !category.is_empty() {
                features.insert(format!("cat:{category}"));
            }

            let brand = normalize_token(&p.brand);
            if !brand.is_empty() {
                features.insert(format!("brand:{brand}"));
            }

            for tag in &p.tags {
                let normalized = normalize_token(tag);
                if !normalized.is_empty() {
                    features.insert(format!("tag:{normalized}"));
                }
            }

            for feature_key in features {
                self.connect_product_feature(product_idx, feature_key);
            }
        }
    }

    pub fn recommend(&self, id: ProductId, depth: usize, k: usize) -> Vec<ProductId> {
        if depth == 0 || k == 0 {
            return Vec::new();
        }

        let Some(&start_idx) = self.idx_by_product_id.get(&id) else {
            return Vec::new();
        };

        let max_hops = depth.saturating_mul(2);
        let direct_overlap = self.direct_feature_overlap(start_idx, id);

        let mut queue: VecDeque<(NodeIndex, usize)> = VecDeque::new();
        let mut visited_hops: HashMap<NodeIndex, usize> = HashMap::new();
        let mut product_distance: HashMap<ProductId, usize> = HashMap::new();

        queue.push_back((start_idx, 0));
        visited_hops.insert(start_idx, 0);

        while let Some((node_idx, hops)) = queue.pop_front() {
            if hops >= max_hops {
                continue;
            }

            for neighbor in self.g.neighbors(node_idx) {
                let next_hops = hops + 1;
                let should_visit = match visited_hops.get(&neighbor) {
                    Some(prev_hops) => next_hops < *prev_hops,
                    None => true,
                };

                if should_visit {
                    visited_hops.insert(neighbor, next_hops);
                    queue.push_back((neighbor, next_hops));
                }

                if next_hops % 2 == 0 {
                    if let Some(GraphNode::Product(pid)) = self.g.node_weight(neighbor) {
                        if *pid != id {
                            let d = next_hops / 2;
                            product_distance
                                .entry(*pid)
                                .and_modify(|best| {
                                    if d < *best {
                                        *best = d;
                                    }
                                })
                                .or_insert(d);
                        }
                    }
                }
            }
        }

        let mut ranked: Vec<(ProductId, usize, usize)> = product_distance
            .into_iter()
            .map(|(pid, distance)| {
                let overlap = *direct_overlap.get(&pid).unwrap_or(&0);
                (pid, distance, overlap)
            })
            .collect();

        ranked.sort_by(|a, b| a.1.cmp(&b.1).then_with(|| b.2.cmp(&a.2)).then_with(|| a.0.cmp(&b.0)));

        ranked.into_iter().take(k).map(|(pid, _, _)| pid).collect()
    }

    fn connect_product_feature(&mut self, product_idx: NodeIndex, feature_key: String) {
        let feature_idx = if let Some(&idx) = self.idx_by_feature.get(&feature_key) {
            idx
        } else {
            let idx = self.g.add_node(GraphNode::Feature(feature_key.clone()));
            self.idx_by_feature.insert(feature_key, idx);
            idx
        };

        self.g.add_edge(product_idx, feature_idx, ());
    }

    fn direct_feature_overlap(&self, start_idx: NodeIndex, source_id: ProductId) -> HashMap<ProductId, usize> {
        let mut overlap: HashMap<ProductId, usize> = HashMap::new();

        for feature_idx in self.g.neighbors(start_idx) {
            for product_idx in self.g.neighbors(feature_idx) {
                if let Some(GraphNode::Product(pid)) = self.g.node_weight(product_idx) {
                    if *pid != source_id {
                        *overlap.entry(*pid).or_insert(0) += 1;
                    }
                }
            }
        }

        overlap
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn p(id: usize, name: &str, brand: &str, category: &str, tags: &[&str]) -> Product {
        Product {
            id,
            name: name.into(),
            brand: brand.into(),
            category: category.into(),
            tags: tags.iter().map(|t| (*t).to_string()).collect(),
        }
    }

    #[test]
    fn recommend_respects_depth() {
        let products = vec![
            p(1, "Alpha", "A", "Cat1", &["x"]),
            p(2, "Beta", "B", "Cat2", &["x", "y"]),
            p(3, "Gamma", "C", "Cat3", &["y"]),
        ];

        let mut graph = ProductGraph::new();
        graph.build(&products);

        let d1 = graph.recommend(1, 1, 10);
        assert!(d1.contains(&2));
        assert!(!d1.contains(&3));

        let d2 = graph.recommend(1, 2, 10);
        assert!(d2.contains(&3));
    }
}
