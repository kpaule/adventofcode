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

    pub fn fuel_for_fuel_requirement(&self) -> i32 {
        let mut total_fuel_req = 0;
        for module in &self.modules {
            total_fuel_req += module.fuel_for_fuel_requirement();
        }
        total_fuel_req
    }

    pub fn fuel_for_fuel_requirement_async(&self) -> i32 {
        let result = crossbeam::scope(|scope| {
            let mut handles = Vec::new();
            for module in &self.modules {
                let handle = scope.spawn(move |_| module.fuel_for_fuel_requirement());
                handles.push(handle);
            }

            let mut result = 0;
            for handle in handles {
                result += handle.join().unwrap();
            }
            result
        })
        .unwrap();
        result
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

    pub fn fuel_for_fuel_requirement(&self) -> i32 {
        let mut total_fuel = 0;
        let mut fuel = self.fuel_requirement();

        while fuel > 0 {
            total_fuel += fuel;
            fuel = ((fuel as f32 / 3.0).floor() as i32) - 2;
        }

        total_fuel
    }
}
