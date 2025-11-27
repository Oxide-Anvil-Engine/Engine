use crate::graphics::render::pass::RenderPass;
use std::collections::{HashMap, HashSet};

pub struct FrameGraph {
    passes: Vec<RenderPass>,
    order: Vec<usize>,
}

impl FrameGraph {
    pub fn new() -> Self {
        Self {
            passes: Vec::new(),
            order: Vec::new(),
        }
    }

    pub fn add_pass(&mut self, pass: RenderPass) {
        self.passes.push(pass);
    }

    pub fn build(&mut self) {
        // Topological sort simples baseado em nomes
        let name_to_index: HashMap<&'static str, usize> = self
            .passes
            .iter()
            .enumerate()
            .map(|(i, p)| (p.desc.name, i))
            .collect();
        let mut visited = HashSet::new();
        let mut temp = HashSet::new();
        self.order.clear();

        fn visit(
            i: usize,
            passes: &Vec<RenderPass>,
            name_to_index: &HashMap<&'static str, usize>,
            visited: &mut HashSet<usize>,
            temp: &mut HashSet<usize>,
            out: &mut Vec<usize>,
        ) {
            if visited.contains(&i) {
                return;
            }
            if temp.contains(&i) {
                panic!("Ciclo em dependências de render pass");
            }
            temp.insert(i);
            for dep in &passes[i].desc.depends_on {
                if let Some(&didx) = name_to_index.get(dep) {
                    visit(didx, passes, name_to_index, visited, temp, out);
                }
            }
            temp.remove(&i);
            visited.insert(i);
            out.push(i);
        }

        for i in 0..self.passes.len() {
            visit(
                i,
                &self.passes,
                &name_to_index,
                &mut visited,
                &mut temp,
                &mut self.order,
            );
        }
    }

    pub fn ordered(&self) -> impl Iterator<Item = &RenderPass> {
        self.order.iter().map(|&i| &self.passes[i])
    }

    pub fn passes_mut(&mut self) -> &mut [RenderPass] {
        &mut self.passes
    }
}
