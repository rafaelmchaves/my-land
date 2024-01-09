pub struct Infrastructure {
    id: String,
    name: String,
    infra_type: InfraType,
    amount: f64,
    amount_name: String,
    cost: f64,
    building_time: i8,
    effect_time: i8,
    ranking_effects: Vec<RankingEffect>
}

pub enum InfraType {
    Road,
    Street,
    Park,
    Hospital,
    School,
    College,
    University,
    Cemitery,
    FireDepartment,
    PoliceStation,
    BurocracyDepartment,
    Airport,
    Port,
    Toll
}

struct RankingEffect {
    index: String,
    score: f64
}