use std::collections::HashMap;

struct SistemaDeBusca {
    indice: HashMap<String, String>,
}

impl SistemaDeBusca {
    fn new() -> Self {
        SistemaDeBusca {
            indice: HashMap::new(),
        }
    }

    fn adicionar_produto(&mut self, id: String, nome: &str) {
        for palavra in nome.split_whitespace() {
            self.indice.insert(palavra.to_lowercase(), id.clone());
        }
    }

    fn buscar(&self, termo: &str) -> Option<&String> {
        self.indice.get(&termo.to_lowercase())
    }
}

fn main() {
    let mut sistema = SistemaDeBusca::new();
    sistema.adicionar_produto("1".to_string(), "Smartphone Samsung Galaxy");

    let termo = "Samsung";
    match sistema.buscar(termo) {
        Some(id) => println!("Produto encontrado: ID {}", id),
        None => println!("Produto não encontrado"),
    }
}

// Seção de testes
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_busca_existente() {
        let mut sistema = SistemaDeBusca::new();
        sistema.adicionar_produto("1".to_string(), "Smartphone Samsung Galaxy");

        assert_eq!(sistema.buscar("Samsung"), Some(&"1".to_string()));
    }

    #[test]
    fn test_busca_inexistente() {
        let mut sistema = SistemaDeBusca::new();
        sistema.adicionar_produto("1".to_string(), "Smartphone Samsung Galaxy");

        assert_eq!(sistema.buscar("iPhone"), None);
    }
}
