pub struct FichaMedica {
    enfermedades: bool,
    operaciones_quirurgicas: bool,
    detalle: String,
}

impl FichaMedica {
    pub fn get_enfermedades(&self) -> bool {
        self.enfermedades
    }
    pub fn get_operaciones_quirurgicas(&self) -> bool {
        self.operaciones_quirurgicas
    }
    pub fn get_detalle(&self) -> &str {
        &self.detalle
    }
}
