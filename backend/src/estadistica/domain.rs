pub struct ClaseMasConcurrida {
    id_clase: String,
    descripcion: String,
    cantidad: i64,
}

impl ClaseMasConcurrida {
    pub fn new(id_clase: String, descripcion: String, cantidad: i64) -> Self {
        Self {
            id_clase,
            descripcion,
            cantidad,
        }
    }

    pub fn id_clase(&self) -> &str {
        &self.id_clase
    }

    pub fn descripcion(&self) -> &str {
        &self.descripcion
    }

    pub fn cantidad(&self) -> i64 {
        self.cantidad
    }
}

pub struct ClaseMasCancelada {
    id_clase: String,
    descripcion: String,
    cantidad: i64,
}

impl ClaseMasCancelada {
    pub fn new(id_clase: String, descripcion: String, cantidad: i64) -> Self {
        Self {
            id_clase,
            descripcion,
            cantidad,
        }
    }

    pub fn id_clase(&self) -> &str {
        &self.id_clase
    }

    pub fn descripcion(&self) -> &str {
        &self.descripcion
    }

    pub fn cantidad(&self) -> i64 {
        self.cantidad
    }
}

pub struct Recaudacion {
    total: f64,
}

impl Recaudacion {
    pub fn new(total: f64) -> Self {
        Self { total }
    }

    pub fn total(&self) -> f64 {
        self.total
    }
}
