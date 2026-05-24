#[derive(Debug, Clone)]
pub struct FichaMedica {
    id_ficha: String,
    enfermedades: bool,
    operaciones_quirurgicas: bool,
    detalles: String,
}

use crate::dtos::CreateFichaMedicaRequest;

impl FichaMedica {
    pub fn get_id_ficha(&self) -> String {
        self.id_ficha.clone()
    }
    pub fn get_enfermedades(&self) -> bool {
        self.enfermedades
    }
    pub fn get_operaciones_quirurgicas(&self) -> bool {
        self.operaciones_quirurgicas
    }
    pub fn get_detalles(&self) -> String {
        self.detalles.clone()
    }
}

impl From<CreateFichaMedicaRequest> for FichaMedica {
    fn from(request: CreateFichaMedicaRequest) -> Self {
        Self {
            id_ficha: uuid::Uuid::new_v4().to_string(),
            enfermedades: request.enfermedades,
            operaciones_quirurgicas: request.operaciones_quirurgicas,
            detalles: request.detalle,
        }
    }
}
