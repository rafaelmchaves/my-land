struct Infrastructure {
    id: String,
    name: String,
    infra_type: InfraType,
    amount: f64,
    amount_name: String,
    cost: f64
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