// Importamos o pacote principal (substitua pelo nome do seu projeto no Cargo.toml se mudou)
use megastore_search::models::Product;
use megastore_search::index::SearchIndex;
use megastore_search::recommendation::RecommendationGraph;

// Função auxiliar para criar um cenário de teste limpo
fn setup_mock_catalog() -> (SearchIndex, RecommendationGraph) {
    let mut catalog = SearchIndex::new();
    let mut recommender = RecommendationGraph::new();

    let p1 = Product::new(1, "Notebook Gamer", "Dell", "Eletrônicos", 4500.00);
    let p2 = Product::new(2, "Mouse Sem Fio", "Logitech", "Eletrônicos", 150.00);
    let p3 = Product::new(3, "Camisa Polo", "Lacoste", "Vestuário", 299.00);

    catalog.insert(p1);
    catalog.insert(p2);
    catalog.insert(p3);

    // Conecta Notebook (1) ao Mouse (2) no grafo de recomendação
    recommender.add_relation(1, 2);

    (catalog, recommender)
}

#[test]
fn test_search_by_id_item_exists() {
    let (catalog, _) = setup_mock_catalog();
    
    let result = catalog.search_by_id(1);
    
    assert!(result.is_some());
    let product = result.unwrap();
    assert_eq!(product.name, "Notebook Gamer");
    assert_eq!(product.brand, "Dell");
}

#[test]
fn test_search_by_id_item_does_not_exist() {
    let (catalog, _) = setup_mock_catalog();
    
    let result = catalog.search_by_id(999); // ID inexistente
    
    assert!(result.is_none());
}

#[test]
fn test_search_by_category() {
    let (catalog, _) = setup_mock_catalog();
    
    // Testando busca case-insensitive (eletrônicos vs Eletrônicos)
    let eletronicos = catalog.search_by_category("eletrônicos");
    
    assert_eq!(eletronicos.len(), 2);
    let names: Vec<&str> = eletronicos.iter().map(|p| p.name.as_str()).collect();
    assert!(names.contains(&"Notebook Gamer"));
    assert!(names.contains(&"Mouse Sem Fio"));
}

#[test]
fn test_graph_recommendations() {
    let (_, recommender) = setup_mock_catalog();
    
    // Quem olha o Notebook (1) deve receber a recomendação do Mouse (2)
    let recommendations = recommender.get_recommendations(1);
    
    assert_eq!(recommendations.len(), 1);
    assert_eq!(recommendations[0], 2);
}