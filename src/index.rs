use std::collections::HashMap;
use crate::models::Product;

pub struct SearchIndex {
    // Tabela Hash principal: ID -> Produto
    pub products_by_id: HashMap<u32, Product>,
    // Índice Invertido: Categoria -> Lista de IDs de Produtos
    pub index_by_category: HashMap<String, Vec<u32>>,
}

impl SearchIndex {
    pub fn new() -> Self {
        SearchIndex {
            products_by_id: HashMap::new(),
            index_by_category: HashMap::new(),
        }
    }

    // Insere o produto nas Tabelas Hash (Complexidade O(1))
    pub fn insert(&mut self, product: Product) {
        let id = product.id;
        let category = product.category.to_lowercase();

        // Salva no mapa principal
        self.products_by_id.insert(id, product);

        // Atualiza o índice por categoria
        self.index_by_category
            .entry(category)
            .or_insert_with(Vec::new)
            .push(id);
    }

    // Busca direta por ID - O(1)
    pub fn search_by_id(&self, id: u32) -> Option<&Product> {
        self.products_by_id.get(&id)
    }

    // Busca por Categoria - O(1) para achar a lista
    pub fn search_by_category(&self, category: &str) -> Vec<&Product> {
        if let Some(ids) = self.index_by_category.get(&category.to_lowercase()) {
            ids.iter()
                .filter_map(|id| self.products_by_id.get(id))
                .collect()
        } else {
            Vec::new()
        }
    }
}