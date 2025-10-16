use crate::core::HimsError;
use std::collections::HashMap;

/// HL7v2 Message structure
#[derive(Debug, Clone)]
pub struct Hl7Message {
    pub message_type: String,
    pub segments: Vec<Hl7Segment>,
}

#[derive(Debug, Clone)]
pub struct Hl7Segment {
    pub segment_type: String,
    pub fields: Vec<String>,
}

/// HL7v2 Parser for common message types
pub struct Hl7Parser;

impl Hl7Parser {
    pub fn new() -> Self {
        Self
    }

    /// Parse an HL7v2 message from string
    pub fn parse_message(&self, message: &str) -> Result<Hl7Message, HimsError> {
        let lines: Vec<&str> = message.trim().split('\n').collect();
        
        if lines.is_empty() {
            return Err(HimsError::ValidationError {
                message: "Empty HL7 message".to_string(),
            });
        }

        let mut segments = Vec::new();
        let mut message_type = String::new();

        for line in lines {
            let segment = self.parse_segment(line)?;
            
            // Extract message type from MSH segment
            if segment.segment_type == "MSH" && segment.fields.len() > 8 {
                message_type = segment.fields[8].clone();
            }
            
            segments.push(segment);
        }

        Ok(Hl7Message {
            message_type,
            segments,
        })
    }

    /// Parse a single HL7 segment
    fn parse_segment(&self, segment_line: &str) -> Result<Hl7Segment, HimsError> {
        if segment_line.len() < 3 {
            return Err(HimsError::ValidationError {
                message: "Invalid segment format".to_string(),
            });
        }

        let segment_type = segment_line[0..3].to_string();
        
        // Handle MSH segment specially (uses different field separator)
        let fields: Vec<String> = if segment_type == "MSH" {
            let field_separator = segment_line.chars().nth(3).unwrap_or('|');
            segment_line[3..].split(field_separator).map(|s| s.to_string()).collect()
        } else {
            segment_line[4..].split('|').map(|s| s.to_string()).collect()
        };

        Ok(Hl7Segment {
            segment_type,
            fields,
        })
    }

    /// Parse ADT (Admit, Discharge, Transfer) message
    pub fn parse_adt_message(&self, message: &str) -> Result<AdtMessage, HimsError> {
        let hl7_msg = self.parse_message(message)?;
        
        let mut adt = AdtMessage::default();
        
        for segment in &hl7_msg.segments {
            match segment.segment_type.as_str() {
                "MSH" => {
                    if segment.fields.len() > 8 {
                        adt.message_type = segment.fields[8].clone();
                    }
                }
                "PID" => {
                    adt.patient_info = self.parse_pid_segment(segment)?;
                }
                "PV1" => {
                    adt.visit_info = Some(self.parse_pv1_segment(segment)?);
                }
                _ => {}
            }
        }
        
        Ok(adt)
    }

    /// Parse PID (Patient Identification) segment
    fn parse_pid_segment(&self, segment: &Hl7Segment) -> Result<PatientInfo, HimsError> {
        if segment.fields.len() < 5 {
            return Err(HimsError::ValidationError {
                message: "Invalid PID segment".to_string(),
            });
        }

        Ok(PatientInfo {
            patient_id: segment.fields.get(2).cloned().unwrap_or_default(),
            patient_name: segment.fields.get(4).cloned().unwrap_or_default(),
            date_of_birth: segment.fields.get(6).cloned(),
            gender: segment.fields.get(7).cloned(),
            address: segment.fields.get(10).cloned(),
        })
    }

    /// Parse PV1 (Patient Visit) segment
    fn parse_pv1_segment(&self, segment: &Hl7Segment) -> Result<VisitInfo, HimsError> {
        Ok(VisitInfo {
            patient_class: segment.fields.get(1).cloned().unwrap_or_default(),
            assigned_location: segment.fields.get(2).cloned(),
            admission_type: segment.fields.get(3).cloned(),
            attending_doctor: segment.fields.get(6).cloned(),
            visit_number: segment.fields.get(18).cloned(),
        })
    }
}

/// ADT Message structure
#[derive(Debug, Clone, Default)]
pub struct AdtMessage {
    pub message_type: String,
    pub patient_info: PatientInfo,
    pub visit_info: Option<VisitInfo>,
}

#[derive(Debug, Clone, Default)]
pub struct PatientInfo {
    pub patient_id: String,
    pub patient_name: String,
    pub date_of_birth: Option<String>,
    pub gender: Option<String>,
    pub address: Option<String>,
}

#[derive(Debug, Clone)]
pub struct VisitInfo {
    pub patient_class: String,
    pub assigned_location: Option<String>,
    pub admission_type: Option<String>,
    pub attending_doctor: Option<String>,
    pub visit_number: Option<String>,
}