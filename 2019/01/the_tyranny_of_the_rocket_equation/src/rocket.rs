use futures::future::join_all;

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

    pub async fn fuel_for_fuel_requirement_await(self) -> i32 {
        let mut future_vec = Vec::new();
        for module in self.modules {
            let f = self::Rocket::calc_fuel(module);
            future_vec.push(f);
        }
        let results = join_all(future_vec).await;
        results.iter().sum() 
    }

    async fn calc_fuel(module: Module) -> i32 {
        module.fuel_for_fuel_requirement()
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
