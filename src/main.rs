use clap::Parser;
use megastore_search::prelude::*;
use std::fs;

#[derive(Parser)]
#[command(name = "megastore_search")]
struct Args {
    #[arg(long)]
    query: Option<String>,
    #[arg(long)]
    input: Option<String>,
    #[arg(long)]
    id: Option<usize>,
    #[arg(long, value_parser = clap::value_parser!(usize).range(1..), default_value_t = 1)]
    depth: usize,
    #[arg(long, value_parser = clap::value_parser!(usize).range(1..), default_value_t = 5)]
    k: usize,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let mut cat = Catalogue::new();
    if let Some(path) = args.input {
        let data = fs::read_to_string(path)?;
        let products: Vec<Product> = serde_json::from_str(&data)?;
        for p in products { cat.insert_product(p); }
    } else {
        cat.insert_product(Product{ id:1, name:"Rust Book".into(), brand:"NoStarch".into(), category:"Books".into(), tags:vec!["programming".into()] });
        cat.insert_product(Product{ id:2, name:"Cooking for Beginners".into(), brand:"KitchenPress".into(), category:"Books".into(), tags:vec!["cooking".into()] });
        cat.insert_product(Product{ id:3, name:"Mechanical Keyboard".into(), brand:"KeyForge".into(), category:"Electronics".into(), tags:vec!["keyboard".into(),"gaming".into()] });
        cat.insert_product(Product{ id:4, name:"Gaming Mouse".into(), brand:"KeyForge".into(), category:"Electronics".into(), tags:vec!["mouse".into(),"gaming".into()] });
        cat.insert_product(Product{ id:5, name:"Office Chair".into(), brand:"SitWell".into(), category:"Furniture".into(), tags:vec!["chair".into(),"office".into()] });
    }
    cat.build_index();
    cat.build_graph();
    if let Some(q) = args.query {
        let results = cat.search(&q);
        println!("Resultados da busca:");
        for p in results { println!("{} {}", p.id, p.name); }
    }

    if let Some(id) = args.id {
        let recs = cat.recommend(id, args.depth, args.k);
        println!("Recomendacoes:");
        for p in recs { println!("{} {}", p.id, p.name); }
    }

    Ok(())
}