pub struct Model {
    pub tabela: &'static str,
    pub campos: Vec<&'static str>,
}

impl Model {
    pub fn insert(&self, valores: Vec<&str>) {
        let campos_str = self.campos.join(", ");
        let valores_str = valores.join(", ");
        
        let query = format!(
            "INSERT INTO {} ({}) VALUES ({})",
            self.tabela, campos_str, valores_str
        );
        
        println!("{}", query);
    }

    pub fn funcionando(&self){
        println!("Funcionando perfeitamente pq eu sou genio")
    }
}

