pub struct Rocket {
    modules: Vec<Module>,
}

impl Rocket {
    pub fn new() -> Rocket {
        Rocket {
            modules: Vec::new(),
        }
    }

    pub fn load(&mut self, module: Module) {
        self.modules.push(module);
    }

    pub fn fuel_requirement(&self) -> i32 {
        let mut total_fuel_req = 0;
        for module in &self.modules {
            total_fuel_req += module.fuel_requirement();
        }
        total_fuel_req
    }
}

pub struct Module {
    weight: i32,
}

impl Module {
    pub fn new(weight: i32) -> Module {
        Module { weight: weight }
    }

    pub fn fuel_requirement(&self) -> i32 {
        ((self.weight as f32 / 3.0).floor() as i32) - 2
    }
}
