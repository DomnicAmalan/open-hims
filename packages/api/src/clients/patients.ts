import { 
  UIPatient, 
  CreatePatientRequest, 
  UpdatePatientRequest, 
  PatientFilters,
  ApiResponse,
  PaginatedResponse,
} from '@open-hims/types';
import { HimsApiClient, apiClient } from './base';

// Patients API client that extends the base client
export class PatientsApiClient extends HimsApiClient {
  // Patient API methods
  async fetchPatients(params: {
    page?: number;
    pageSize?: number;
    filters?: PatientFilters;
  }): Promise<PaginatedResponse<UIPatient>> {
    return this.request<PaginatedResponse<UIPatient>>('get', '/patients', undefined, params);
  }

  async createPatient(request: CreatePatientRequest): Promise<ApiResponse<UIPatient>> {
    return this.request<ApiResponse<UIPatient>>('post', '/patients', request);
  }

  async updatePatient(request: UpdatePatientRequest): Promise<ApiResponse<UIPatient>> {
    const { id, ...data } = request;
    return this.request<ApiResponse<UIPatient>>('put', `/patients/${id}`, data);
  }

  async deletePatient(id: string): Promise<ApiResponse<void>> {
    return this.request<ApiResponse<void>>('delete', `/patients/${id}`);
  }

  async getPatient(id: string): Promise<ApiResponse<UIPatient>> {
    return this.request<ApiResponse<UIPatient>>('get', `/patients/${id}`);
  }

  // Search patients
  async searchPatients(query: string): Promise<ApiResponse<UIPatient[]>> {
    return this.request<ApiResponse<UIPatient[]>>('get', '/patients/search', undefined, { q: query });
  }
}

// Default patients API client instance
export const patientsApiClient = new PatientsApiClient();

// Export individual API functions for convenience
export const patientsApi = {
  fetchPatients: (params: Parameters<PatientsApiClient['fetchPatients']>[0]) => 
    patientsApiClient.fetchPatients(params),
  
  createPatient: (request: CreatePatientRequest) => 
    patientsApiClient.createPatient(request),
  
  updatePatient: (request: UpdatePatientRequest) => 
    patientsApiClient.updatePatient(request),
  
  deletePatient: (id: string) => 
    patientsApiClient.deletePatient(id),
  
  getPatient: (id: string) => 
    patientsApiClient.getPatient(id),
  
  searchPatients: (query: string) => 
    patientsApiClient.searchPatients(query),
};