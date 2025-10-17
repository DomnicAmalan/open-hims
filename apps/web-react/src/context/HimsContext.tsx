import React, { createContext, useContext, useState, useEffect, ReactNode } from 'react';

// Mock HIMS SDK interface for web demo
interface HimsConfig {
  apiEndpoint: string;
  authToken?: string;
  enableLogging: boolean;
  countryCode?: string;
  stateCode?: string;
}

interface Patient {
  id: string;
  name: { family: string; given: string[] }[];
  gender: string;
  birthDate: string;
  identifier: { system: string; value: string }[];
  address: { line: string[]; city: string; state: string; postalCode: string }[];
  telecom: { system: string; value: string; use: string }[];
}

interface ComplianceCheck {
  level: string;
  authority: string;
  compliant: boolean;
  requirements_checked: string[];
}

interface HimsContextType {
  config: HimsConfig;
  patients: Patient[];
  isLoading: boolean;
  error: string | null;
  
  // Patient operations
  createPatient: (patient: Omit<Patient, 'id'>) => Promise<Patient>;
  getPatient: (id: string) => Promise<Patient | null>;
  updatePatient: (id: string, patient: Partial<Patient>) => Promise<Patient>;
  deletePatient: (id: string) => Promise<void>;
  searchPatients: (query: string) => Promise<Patient[]>;
  
  // HL7 operations
  parseHL7Message: (message: string) => Promise<any>;
  
  // Compliance operations
  validateCompliance: (countryCode: string, stateCode: string, operation: string) => Promise<boolean>;
  getComplianceRequirements: (countryCode: string, stateCode: string) => Promise<ComplianceCheck[]>;
  
  // Data transfer operations
  exportPatientData: (patientId: string, format: 'fhir' | 'hl7' | 'pdf') => Promise<string>;
  importHL7Data: (hl7Data: string) => Promise<Patient[]>;
}

const HimsContext = createContext<HimsContextType | undefined>(undefined);

// Mock data for demo
const mockPatients: Patient[] = [
  {
    id: '1',
    name: [{ family: 'Doe', given: ['John'] }],
    gender: 'male',
    birthDate: '1985-06-15',
    identifier: [{ system: 'SSN', value: '123-45-6789' }],
    address: [{ line: ['123 Main St'], city: 'San Francisco', state: 'CA', postalCode: '94102' }],
    telecom: [{ system: 'phone', value: '+1-555-123-4567', use: 'home' }],
  },
  {
    id: '2',
    name: [{ family: 'Smith', given: ['Jane'] }],
    gender: 'female',
    birthDate: '1990-03-22',
    identifier: [{ system: 'SSN', value: '987-65-4321' }],
    address: [{ line: ['456 Oak Ave'], city: 'Los Angeles', state: 'CA', postalCode: '90210' }],
    telecom: [{ system: 'email', value: 'jane.smith@email.com', use: 'work' }],
  },
  {
    id: '3',
    name: [{ family: 'Patel', given: ['Raj'] }],
    gender: 'male',
    birthDate: '1978-11-30',
    identifier: [{ system: 'ABHA', value: '12-3456-7890-1234' }],
    address: [{ line: ['789 Tech Park'], city: 'Mumbai', state: 'MH', postalCode: '400001' }],
    telecom: [{ system: 'phone', value: '+91-98765-43210', use: 'mobile' }],
  },
];

interface HimsProviderProps {
  children: ReactNode;
  initialCountry: string;
  initialState: string;
}

export const HimsProvider: React.FC<HimsProviderProps> = ({ 
  children, 
  initialCountry, 
  initialState 
}) => {
  const [config] = useState<HimsConfig>({
    apiEndpoint: 'https://api.hims-demo.org/fhir',
    authToken: 'demo-token-12345',
    enableLogging: true,
    countryCode: initialCountry,
    stateCode: initialState,
  });

  const [patients, setPatients] = useState<Patient[]>(mockPatients);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  // Mock API delay
  const delay = (ms: number) => new Promise(resolve => setTimeout(resolve, ms));

  const createPatient = async (patientData: Omit<Patient, 'id'>): Promise<Patient> => {
    setIsLoading(true);
    await delay(1000); // Simulate API call
    
    const newPatient: Patient = {
      ...patientData,
      id: Date.now().toString(),
    };
    
    setPatients(prev => [...prev, newPatient]);
    setIsLoading(false);
    return newPatient;
  };

  const getPatient = async (id: string): Promise<Patient | null> => {
    setIsLoading(true);
    await delay(500);
    
    const patient = patients.find(p => p.id === id) || null;
    setIsLoading(false);
    return patient;
  };

  const updatePatient = async (id: string, updates: Partial<Patient>): Promise<Patient> => {
    setIsLoading(true);
    await delay(800);
    
    setPatients(prev => prev.map(p => 
      p.id === id ? { ...p, ...updates } : p
    ));
    
    const updatedPatient = patients.find(p => p.id === id)!;
    setIsLoading(false);
    return updatedPatient;
  };

  const deletePatient = async (id: string): Promise<void> => {
    setIsLoading(true);
    await delay(600);
    
    setPatients(prev => prev.filter(p => p.id !== id));
    setIsLoading(false);
  };

  const searchPatients = async (query: string): Promise<Patient[]> => {
    setIsLoading(true);
    await delay(400);
    
    const results = patients.filter(patient => 
      patient.name[0].family.toLowerCase().includes(query.toLowerCase()) ||
      patient.name[0].given[0].toLowerCase().includes(query.toLowerCase())
    );
    
    setIsLoading(false);
    return results;
  };

  const parseHL7Message = async (message: string): Promise<any> => {
    setIsLoading(true);
    await delay(1200);
    
    // Mock HL7 parsing result
    const result = {
      messageType: 'ADT^A01',
      patientId: 'PID12345',
      patientName: 'DOE^JOHN',
      dateOfBirth: '19850615',
      gender: 'M',
      segments: ['MSH', 'PID', 'PV1'],
      parsed: true,
    };
    
    setIsLoading(false);
    return result;
  };

  const validateCompliance = async (countryCode: string, stateCode: string, operation: string): Promise<boolean> => {
    setIsLoading(true);
    await delay(800);
    
    // Mock compliance validation
    const isCompliant = Math.random() > 0.2; // 80% compliance rate
    setIsLoading(false);
    return isCompliant;
  };

  const getComplianceRequirements = async (countryCode: string, stateCode: string): Promise<ComplianceCheck[]> => {
    setIsLoading(true);
    await delay(1000);
    
    const mockRequirements: ComplianceCheck[] = [
      {
        level: 'Federal',
        authority: countryCode === 'US' ? 'HHS' : 'Ministry of Health',
        compliant: true,
        requirements_checked: countryCode === 'US' ? ['HIPAA', 'HITECH'] : ['DPDP Act', 'ABDM'],
      },
      {
        level: 'State',
        authority: `${stateCode} Health Department`,
        compliant: true,
        requirements_checked: stateCode === 'CA' ? ['CCPA', 'CPRA'] : ['State Privacy Laws'],
      },
    ];
    
    setIsLoading(false);
    return mockRequirements;
  };

  const exportPatientData = async (patientId: string, format: 'fhir' | 'hl7' | 'pdf'): Promise<string> => {
    setIsLoading(true);
    await delay(1500);
    
    const patient = patients.find(p => p.id === patientId);
    if (!patient) throw new Error('Patient not found');
    
    let exportData = '';
    switch (format) {
      case 'fhir':
        exportData = JSON.stringify({
          resourceType: 'Patient',
          id: patient.id,
          name: patient.name,
          gender: patient.gender,
          birthDate: patient.birthDate,
        }, null, 2);
        break;
      case 'hl7':
        exportData = `MSH|^~\\&|HIMS|HOSPITAL|||${new Date().toISOString().replace(/[-:]/g, '').slice(0, 14)}||ADT^A01|${patient.id}|P|2.5\r\n`;
        exportData += `PID|||${patient.identifier[0]?.value}||${patient.name[0].family}^${patient.name[0].given[0]}|||${patient.gender}|${patient.birthDate}\r\n`;
        break;
      case 'pdf':
        exportData = `PDF Export for ${patient.name[0].given[0]} ${patient.name[0].family} would be generated here`;
        break;
    }
    
    setIsLoading(false);
    return exportData;
  };

  const importHL7Data = async (hl7Data: string): Promise<Patient[]> => {
    setIsLoading(true);
    await delay(2000);
    
    // Mock HL7 import - would parse and create patients
    const importedPatients: Patient[] = [
      {
        id: Date.now().toString(),
        name: [{ family: 'Imported', given: ['Patient'] }],
        gender: 'unknown',
        birthDate: '1980-01-01',
        identifier: [{ system: 'HL7Import', value: 'IMP123' }],
        address: [{ line: ['Imported Address'], city: 'Unknown', state: 'XX', postalCode: '00000' }],
        telecom: [{ system: 'phone', value: 'Unknown', use: 'home' }],
      },
    ];
    
    setPatients(prev => [...prev, ...importedPatients]);
    setIsLoading(false);
    return importedPatients;
  };

  const contextValue: HimsContextType = {
    config,
    patients,
    isLoading,
    error,
    createPatient,
    getPatient,
    updatePatient,
    deletePatient,
    searchPatients,
    parseHL7Message,
    validateCompliance,
    getComplianceRequirements,
    exportPatientData,
    importHL7Data,
  };

  return (
    <HimsContext.Provider value={contextValue}>
      {children}
    </HimsContext.Provider>
  );
};

export const useHims = (): HimsContextType => {
  const context = useContext(HimsContext);
  if (context === undefined) {
    throw new Error('useHims must be used within a HimsProvider');
  }
  return context;
};