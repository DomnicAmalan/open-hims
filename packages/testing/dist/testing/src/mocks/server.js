import { setupServer } from 'msw/node';
import { http, HttpResponse } from 'msw';
// Mock FHIR server responses
const mockPatients = [
    {
        resourceType: 'Patient',
        id: 'test-patient-1',
        name: [{ family: 'Doe', given: ['John'] }],
        gender: 'male',
        birthDate: '1990-01-15',
        active: true,
    },
    {
        resourceType: 'Patient',
        id: 'test-patient-2',
        name: [{ family: 'Smith', given: ['Jane'] }],
        gender: 'female',
        birthDate: '1985-06-22',
        active: true,
    },
];
export const handlers = [
    // FHIR Patient endpoints
    http.get('/fhir/Patient/:id', ({ params }) => {
        const patient = mockPatients.find(p => p.id === params.id);
        if (patient) {
            return HttpResponse.json(patient);
        }
        return new HttpResponse(null, { status: 404 });
    }),
    http.get('/fhir/Patient', ({ request }) => {
        const url = new URL(request.url);
        const family = url.searchParams.get('family');
        let filteredPatients = mockPatients;
        if (family) {
            filteredPatients = mockPatients.filter(p => p.name?.[0]?.family?.toLowerCase().includes(family.toLowerCase()));
        }
        const bundle = {
            resourceType: 'Bundle',
            type: 'searchset',
            total: filteredPatients.length,
            entry: filteredPatients.map(patient => ({
                resource: patient,
                fullUrl: `/fhir/Patient/${patient.id}`,
            })),
        };
        return HttpResponse.json(bundle);
    }),
    http.post('/fhir/Patient', async ({ request }) => {
        const patient = await request.json();
        const newPatient = {
            ...patient,
            id: `patient-${Date.now()}`,
            meta: {
                lastUpdated: new Date().toISOString(),
            },
        };
        mockPatients.push(newPatient);
        return HttpResponse.json(newPatient, { status: 201 });
    }),
    http.put('/fhir/Patient/:id', async ({ params, request }) => {
        const patientIndex = mockPatients.findIndex(p => p.id === params.id);
        if (patientIndex === -1) {
            return new HttpResponse(null, { status: 404 });
        }
        const updatedPatient = await request.json();
        mockPatients[patientIndex] = {
            ...updatedPatient,
            id: params.id,
            meta: {
                ...updatedPatient.meta,
                lastUpdated: new Date().toISOString(),
            },
        };
        return HttpResponse.json(mockPatients[patientIndex]);
    }),
    http.delete('/fhir/Patient/:id', ({ params }) => {
        const patientIndex = mockPatients.findIndex(p => p.id === params.id);
        if (patientIndex === -1) {
            return new HttpResponse(null, { status: 404 });
        }
        mockPatients.splice(patientIndex, 1);
        return new HttpResponse(null, { status: 204 });
    }),
    // Authentication endpoints
    http.post('/auth/login', async ({ request }) => {
        const credentials = await request.json();
        if (credentials.username === 'testuser' && credentials.password === 'testpass') {
            return HttpResponse.json({
                accessToken: 'mock-access-token',
                refreshToken: 'mock-refresh-token',
                expiresIn: 3600,
                user: {
                    id: 'user-123',
                    username: 'testuser',
                    email: 'test@example.com',
                    role: 'doctor',
                },
            });
        }
        return HttpResponse.json({ error: 'Invalid credentials' }, { status: 401 });
    }),
    // Compliance/Audit endpoints
    http.get('/api/compliance/regulations', () => {
        return HttpResponse.json([
            {
                id: 'hipaa-us',
                name: 'HIPAA',
                country: 'US',
                category: 'privacy',
                mandatory: true,
                status: 'compliant',
            },
            {
                id: 'gdpr-eu',
                name: 'GDPR',
                country: 'EU',
                category: 'data-protection',
                mandatory: true,
                status: 'warning',
            },
        ]);
    }),
    http.get('/api/audit/logs', ({ request }) => {
        const url = new URL(request.url);
        const page = parseInt(url.searchParams.get('page') || '1');
        const pageSize = parseInt(url.searchParams.get('pageSize') || '10');
        const mockLogs = Array.from({ length: 50 }, (_, i) => ({
            id: `log-${i + 1}`,
            timestamp: new Date(Date.now() - i * 3600000).toISOString(),
            userId: `user-${(i % 5) + 1}`,
            action: ['read', 'create', 'update', 'delete'][i % 4],
            resourceType: 'Patient',
            resourceId: `patient-${(i % 10) + 1}`,
            outcome: 'success',
        }));
        const startIndex = (page - 1) * pageSize;
        const endIndex = startIndex + pageSize;
        const paginatedLogs = mockLogs.slice(startIndex, endIndex);
        return HttpResponse.json({
            data: paginatedLogs,
            pagination: {
                page,
                pageSize,
                total: mockLogs.length,
                totalPages: Math.ceil(mockLogs.length / pageSize),
                hasNext: endIndex < mockLogs.length,
                hasPrevious: page > 1,
            },
        });
    }),
];
export const server = setupServer(...handlers);
