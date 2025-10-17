import { 
  UIPatient, 
  CreatePatientRequest, 
  UpdatePatientRequest, 
  PatientFilters,
  PaginatedResponse 
} from '@open-hims/types';

// Mock API for development/testing
export const mockPatientsApi = {
  async fetchPatients(params: {
    page?: number;
    pageSize?: number;
    filters?: PatientFilters;
  }): Promise<PaginatedResponse<UIPatient>> {
    // Simulate API delay
    await new Promise(resolve => setTimeout(resolve, Math.random() * 1000 + 500));

    const mockPatients: UIPatient[] = [
      {
        id: '1',
        mrn: 'MRN001',
        firstName: 'John',
        lastName: 'Doe',
        dateOfBirth: '1990-01-15',
        gender: 'male',
        phone: '+1-555-0123',
        email: 'john.doe@email.com',
        createdAt: new Date().toISOString(),
        updatedAt: new Date().toISOString(),
      },
      {
        id: '2',
        mrn: 'MRN002',
        firstName: 'Jane',
        lastName: 'Smith',
        dateOfBirth: '1985-03-22',
        gender: 'female',
        phone: '+1-555-0456',
        email: 'jane.smith@email.com',
        createdAt: new Date().toISOString(),
        updatedAt: new Date().toISOString(),
      },
      {
        id: '3',
        mrn: 'MRN003',
        firstName: 'Ahmed',
        lastName: 'Hassan',
        dateOfBirth: '1978-07-10',
        gender: 'male',
        phone: '+1-555-0789',
        email: 'ahmed.hassan@email.com',
        createdAt: new Date().toISOString(),
        updatedAt: new Date().toISOString(),
      },
    ];

    const page = params.page || 1;
    const pageSize = params.pageSize || 20;
    const start = (page - 1) * pageSize;
    const end = start + pageSize;

    return {
      data: mockPatients.slice(start, end),
      pagination: {
        page,
        pageSize,
        total: mockPatients.length,
        totalPages: Math.ceil(mockPatients.length / pageSize),
        hasNext: page < Math.ceil(mockPatients.length / pageSize),
        hasPrevious: page > 1,
      },
    };
  },

  async createPatient(request: CreatePatientRequest): Promise<UIPatient> {
    await new Promise(resolve => setTimeout(resolve, Math.random() * 800 + 400));
    
    return {
      id: `temp-${Date.now()}`,
      mrn: `MRN${String(Date.now()).slice(-6)}`,
      firstName: request.patient.firstName || '',
      lastName: request.patient.lastName || '',
      dateOfBirth: request.patient.dateOfBirth || '',
      gender: request.patient.gender || 'unknown',
      phone: request.patient.phone,
      email: request.patient.email,
      createdAt: new Date().toISOString(),
      updatedAt: new Date().toISOString(),
      ...request.patient,
    };
  },

  async updatePatient(request: UpdatePatientRequest): Promise<UIPatient> {
    await new Promise(resolve => setTimeout(resolve, Math.random() * 800 + 400));
    
    return {
      id: request.id,
      mrn: request.patient.mrn || `MRN${request.id}`,
      firstName: request.patient.firstName || '',
      lastName: request.patient.lastName || '',
      dateOfBirth: request.patient.dateOfBirth || '',
      gender: request.patient.gender || 'unknown',
      createdAt: new Date(Date.now() - 86400000).toISOString(), // 1 day ago
      updatedAt: new Date().toISOString(),
      ...request.patient,
    };
  },

  async deletePatient(id: string): Promise<string> {
    await new Promise(resolve => setTimeout(resolve, Math.random() * 500 + 300));
    return id;
  },

  async getPatient(id: string): Promise<UIPatient> {
    await new Promise(resolve => setTimeout(resolve, Math.random() * 600 + 200));
    
    return {
      id,
      mrn: `MRN${id}`,
      firstName: 'Sample',
      lastName: 'Patient',
      dateOfBirth: '1990-01-01',
      gender: 'unknown',
      createdAt: new Date().toISOString(),
      updatedAt: new Date().toISOString(),
    };
  },

  async searchPatients(query: string): Promise<UIPatient[]> {
    await new Promise(resolve => setTimeout(resolve, Math.random() * 400 + 200));
    
    // Mock search results
    return [
      {
        id: 'search-1',
        mrn: 'MRN999',
        firstName: 'Search',
        lastName: 'Result',
        dateOfBirth: '1995-01-01',
        gender: 'unknown',
        createdAt: new Date().toISOString(),
        updatedAt: new Date().toISOString(),
      },
    ];
  },
};