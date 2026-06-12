use megastore_search::models::Product;
use megastore_search::index::SearchIndex;
use megastore_search::recommendation::RecommendationGraph;



fn main() {
    let mut catalog = SearchIndex::new();
    let mut recommender = RecommendationGraph::new();

    // 1. Populando o sistema com dados (Simulando Milhões de Itens)
    let p1 = Product::new(101, "Smartphone Galaxy S24", "Samsung", "Eletrônicos", 5999.00);
    let p2 = Product::new(102, "Fone de Ouvido Bluetooth", "Sony", "Eletrônicos", 1200.00);
    let p3 = Product::new(103, "Camiseta Esportiva", "Nike", "Vestuário", 149.90);

    catalog.insert(p1);
    catalog.insert(p2);
    catalog.insert(p3);

    // 2. Criando conexões no Grafo (Quem compra o Smartphone, compra o Fone)
    recommender.add_relation(101, 102);

    // --- SIMULAÇÃO DE CASOS DE USO ---

    println!("--- MÓDULO DE BUSCA OTIMIZADA ---");
    // Caso de uso 1: Busca rápida por ID
    if let Some(product) = catalog.search_by_id(101) {
        println!("Produto encontrado por ID: {} - R${:.2}", product.name, product.price);
    }

    // Caso de uso 2: Busca por Categoria
    println!("\nBusca por Categoria 'Eletrônicos':");
    let eletronicos = catalog.search_by_category("Eletrônicos");
    for prod in eletronicos {
        println!("- {} ({})", prod.name, prod.brand);
    }

    println!("\n--- MÓDULO DE RECOMENDAÇÃO (GRAFOS) ---");
    // Caso de uso 3: Sistema de recomendação baseado no produto atual
    let current_product_id = 101;
    let rec_ids = recommender.get_recommendations(current_product_id);
    
    if let Some(current_prod) = catalog.search_by_id(current_product_id) {
        println!("Quem comprou [{}] também comprou:", current_prod.name);
        for id in rec_ids {
            if let Some(rec_prod) = catalog.search_by_id(id) {
                println!("  -> {}", rec_prod.name);
            }
        }
    }
}