use std::collections::HashMap;

pub struct RecommendationGraph {
    // Grafo: ID do Produto -> Lista de IDs de produtos relacionados (vizinhos)
    adj_list: HashMap<u32, Vec<u32>>,
}

impl RecommendationGraph {
    pub fn new() -> Self {
        RecommendationGraph {
            adj_list: HashMap::new(),
        }
    }

    // Cria uma aresta (conexão) entre dois produtos (comprados juntos)
    pub fn add_relation(&mut self, prod_a: u32, prod_b: u32) {
        self.adj_list.entry(prod_a).or_insert_with(Vec::new).push(prod_b);
        self.adj_list.entry(prod_b).or_insert_with(Vec::new).push(prod_a);
    }

    // Retorna os produtos recomendados (vizinhos no grafo)
    pub fn get_recommendations(&self, product_id: u32) -> Vec<u32> {
        self.adj_list.get(&product_id).cloned().unwrap_or_default()
    }
}