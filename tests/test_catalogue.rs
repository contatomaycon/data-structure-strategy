use megastore_search::prelude::*;

#[test]
fn search_and_recommend() {
    let mut cat = Catalogue::new();
    cat.insert_product(Product{ id:1, name:"Rust Book".into(), brand:"NoStarch".into(), category:"Books".into(), tags:vec!["programming".into()] });
    cat.insert_product(Product{ id:2, name:"Advanced Rust".into(), brand:"NoStarch".into(), category:"Books".into(), tags:vec!["programming".into()] });
    cat.insert_product(Product{ id:3, name:"Mechanical Keyboard".into(), brand:"KeyForge".into(), category:"Electronics".into(), tags:vec!["keyboard".into(),"gaming".into()] });
    cat.insert_product(Product{ id:4, name:"Gaming Mouse".into(), brand:"KeyForge".into(), category:"Electronics".into(), tags:vec!["mouse".into(),"gaming".into()] });
    cat.insert_product(Product{ id:5, name:"Office Chair".into(), brand:"SitWell".into(), category:"Furniture".into(), tags:vec!["chair".into(),"office".into()] });
    cat.build_index();
    cat.build_graph();

    let results = cat.search("rust nostarch");
    assert!(results.iter().any(|p| p.id == 1));
    assert!(results.iter().any(|p| p.id == 2));

    let recs = cat.recommend(3, 1, 2);
    assert!(recs.iter().any(|p| p.id == 4));
    assert!(!recs.iter().any(|p| p.id == 5));
}

#[test]
fn recommendation_depth_expands_candidate_set() {
    let mut cat = Catalogue::new();
    cat.insert_product(Product{ id:1, name:"Phone A".into(), brand:"BrandX".into(), category:"Phones".into(), tags:vec!["android".into()] });
    cat.insert_product(Product{ id:2, name:"Phone B".into(), brand:"BrandY".into(), category:"Phones".into(), tags:vec!["android".into(),"camera".into()] });
    cat.insert_product(Product{ id:3, name:"Camera Lens".into(), brand:"BrandZ".into(), category:"Photo".into(), tags:vec!["camera".into()] });
    cat.build_index();
    cat.build_graph();

    let depth_1 = cat.recommend(1, 1, 10);
    assert!(depth_1.iter().any(|p| p.id == 2));
    assert!(!depth_1.iter().any(|p| p.id == 3));

    let depth_2 = cat.recommend(1, 2, 10);
    assert!(depth_2.iter().any(|p| p.id == 3));
}