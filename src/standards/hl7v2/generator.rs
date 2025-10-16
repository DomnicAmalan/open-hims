use crate::core::HimsError;

pub struct Hl7Generator;

impl Hl7Generator {
    pub fn new() -> Self {
        Self
    }
    
    pub fn generate_ack_message(original_control_id: &str, ack_code: &str) -> Result<String, HimsError> {
        let timestamp = chrono::Utc::now().format("%Y%m%d%H%M%S");
        
        let ack = format!(
            "MSH|^~\\&|HIMS|HOSPITAL|||{}||ACK|{}|P|2.5\r\nMSA|{}|{}\r\n",
            timestamp,
            original_control_id,
            ack_code,
            original_control_id
        );
        
        Ok(ack)
    }
}

impl Default for Hl7Generator {
    fn default() -> Self {
        Self::new()
    }
}