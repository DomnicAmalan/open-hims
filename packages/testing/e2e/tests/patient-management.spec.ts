import { test, expect, Page } from '@playwright/test';

// Page Object Model for Patient Management
class PatientManagementPage {
  constructor(private page: Page) {}

  // Navigation
  async goto() {
    await this.page.goto('/patients');
  }

  // Selectors
  get addPatientButton() {
    return this.page.getByTestId('add-patient-button');
  }

  get patientSearchInput() {
    return this.page.getByTestId('patient-search-input');
  }

  get patientTable() {
    return this.page.getByTestId('patients-table');
  }

  get patientRows() {
    return this.page.getByTestId('patient-row');
  }

  // Actions
  async searchPatient(searchTerm: string) {
    await this.patientSearchInput.fill(searchTerm);
    await this.page.keyboard.press('Enter');
    await this.page.waitForLoadState('networkidle');
  }

  async addNewPatient(patientData: {
    firstName: string;
    lastName: string;
    dateOfBirth: string;
    gender: string;
    mrn?: string;
  }) {
    await this.addPatientButton.click();
    
    // Fill patient form
    await this.page.getByTestId('patient-first-name').fill(patientData.firstName);
    await this.page.getByTestId('patient-last-name').fill(patientData.lastName);
    await this.page.getByTestId('patient-dob').fill(patientData.dateOfBirth);
    await this.page.getByTestId('patient-gender').selectOption(patientData.gender);
    
    if (patientData.mrn) {
      await this.page.getByTestId('patient-mrn').fill(patientData.mrn);
    }
    
    await this.page.getByTestId('save-patient-button').click();
    await this.page.waitForLoadState('networkidle');
  }

  async viewPatientDetails(patientId: string) {
    await this.page.getByTestId(`patient-${patientId}-view-button`).click();
    await this.page.waitForLoadState('networkidle');
  }

  async editPatient(patientId: string, updates: Record<string, string>) {
    await this.page.getByTestId(`patient-${patientId}-edit-button`).click();
    
    for (const [field, value] of Object.entries(updates)) {
      await this.page.getByTestId(`patient-${field}`).fill(value);
    }
    
    await this.page.getByTestId('save-patient-button').click();
    await this.page.waitForLoadState('networkidle');
  }

  async deletePatient(patientId: string) {
    await this.page.getByTestId(`patient-${patientId}-delete-button`).click();
    await this.page.getByTestId('confirm-delete-button').click();
    await this.page.waitForLoadState('networkidle');
  }

  // Assertions
  async expectPatientInList(patientName: string) {
    await expect(this.patientTable).toContainText(patientName);
  }

  async expectPatientNotInList(patientName: string) {
    await expect(this.patientTable).not.toContainText(patientName);
  }

  async expectPatientCount(count: number) {
    await expect(this.patientRows).toHaveCount(count);
  }
}

test.describe('Patient Management', () => {
  let patientPage: PatientManagementPage;

  test.beforeEach(async ({ page }) => {
    patientPage = new PatientManagementPage(page);
    
    // Login first
    await page.goto('/login');
    await page.getByTestId('username-input').fill('testuser');
    await page.getByTestId('password-input').fill('testpass');
    await page.getByTestId('login-button').click();
    await page.waitForURL('/dashboard');
    
    // Navigate to patients
    await patientPage.goto();
  });

  test('should display patient list', async () => {
    await expect(patientPage.patientTable).toBeVisible();
    await expect(patientPage.addPatientButton).toBeVisible();
    await expect(patientPage.patientSearchInput).toBeVisible();
  });

  test('should add new patient', async () => {
    const newPatient = {
      firstName: 'John',
      lastName: 'Doe',
      dateOfBirth: '1990-01-15',
      gender: 'male',
      mrn: 'MRN001',
    };

    await patientPage.addNewPatient(newPatient);
    await patientPage.expectPatientInList('John Doe');
  });

  test('should search patients', async () => {
    await patientPage.searchPatient('John');
    await patientPage.expectPatientInList('John Doe');
    
    await patientPage.searchPatient('NonExistent');
    await expect(patientPage.patientTable).toContainText('No patients found');
  });

  test('should edit patient information', async () => {
    // Assuming there's a patient with ID 'patient-1'
    await patientPage.editPatient('patient-1', {
      'first-name': 'Jane',
      'last-name': 'Updated',
    });
    
    await patientPage.expectPatientInList('Jane Updated');
  });

  test('should delete patient', async () => {
    const initialPatientCount = await patientPage.patientRows.count();
    
    await patientPage.deletePatient('patient-1');
    
    await patientPage.expectPatientCount(initialPatientCount - 1);
  });

  test('should handle patient data privacy', async () => {
    // Test PHI masking for different user roles
    await patientPage.viewPatientDetails('patient-1');
    
    // Check that sensitive data is masked for restricted users
    const ssnField = patientPage.page.getByTestId('patient-ssn');
    if (await ssnField.isVisible()) {
      const ssnValue = await ssnField.textContent();
      expect(ssnValue).toMatch(/\*+/); // Should contain asterisks for masking
    }
  });

  test('should validate required fields', async ({ page }) => {
    await patientPage.addPatientButton.click();
    
    // Try to submit without required fields
    await page.getByTestId('save-patient-button').click();
    
    // Should show validation errors
    await expect(page.getByTestId('validation-error')).toBeVisible();
    await expect(page.getByText('First name is required')).toBeVisible();
    await expect(page.getByText('Last name is required')).toBeVisible();
  });

  test('should handle pagination', async () => {
    // Test pagination if there are many patients
    const paginationContainer = patientPage.page.getByTestId('pagination');
    
    if (await paginationContainer.isVisible()) {
      const nextButton = paginationContainer.getByText('Next');
      if (await nextButton.isEnabled()) {
        await nextButton.click();
        await patientPage.page.waitForLoadState('networkidle');
        
        // Should be on page 2
        await expect(paginationContainer.getByText('Page 2')).toBeVisible();
      }
    }
  });

  test('should export patient data', async ({ page }) => {
    const exportButton = page.getByTestId('export-patients-button');
    
    if (await exportButton.isVisible()) {
      // Setup download handling
      const downloadPromise = page.waitForEvent('download');
      
      await exportButton.click();
      
      const download = await downloadPromise;
      expect(download.suggestedFilename()).toMatch(/patients.*\.(csv|xlsx|pdf)$/);
    }
  });

  test('should maintain audit trail', async ({ page }) => {
    // Perform an action that should be audited
    await patientPage.addNewPatient({
      firstName: 'Audit',
      lastName: 'Test',
      dateOfBirth: '1985-06-22',
      gender: 'female',
    });

    // Navigate to audit logs (if accessible)
    await page.goto('/audit');
    
    // Check if the action was logged
    const auditTable = page.getByTestId('audit-logs-table');
    if (await auditTable.isVisible()) {
      await expect(auditTable).toContainText('Patient created');
      await expect(auditTable).toContainText('Audit Test');
    }
  });
});

// Compliance Dashboard Tests
test.describe('Compliance Dashboard', () => {
  test.beforeEach(async ({ page }) => {
    // Login and navigate to compliance dashboard
    await page.goto('/login');
    await page.getByTestId('username-input').fill('testuser');
    await page.getByTestId('password-input').fill('testpass');
    await page.getByTestId('login-button').click();
    await page.waitForURL('/dashboard');
    await page.goto('/compliance');
  });

  test('should display compliance overview', async ({ page }) => {
    await expect(page.getByTestId('compliance-score')).toBeVisible();
    await expect(page.getByTestId('regulations-list')).toBeVisible();
    await expect(page.getByTestId('country-selector')).toBeVisible();
  });

  test('should filter by country/state', async ({ page }) => {
    const countrySelector = page.getByTestId('country-selector');
    await countrySelector.selectOption('US');
    
    await page.waitForLoadState('networkidle');
    
    // Should show US-specific regulations
    await expect(page.getByText('HIPAA')).toBeVisible();
    
    // Select California
    const stateSelector = page.getByTestId('state-selector');
    await stateSelector.selectOption('CA');
    
    await page.waitForLoadState('networkidle');
    
    // Should show California-specific regulations
    await expect(page.getByText('CCPA')).toBeVisible();
  });

  test('should update compliance status', async ({ page }) => {
    const regulationRow = page.getByTestId('regulation-hipaa-privacy');
    const statusButton = regulationRow.getByTestId('update-status-button');
    
    await statusButton.click();
    await page.getByTestId('status-compliant').click();
    
    // Should show updated status
    await expect(regulationRow.getByText('Compliant')).toBeVisible();
  });

  test('should generate compliance report', async ({ page }) => {
    const reportButton = page.getByTestId('generate-report-button');
    
    // Setup download handling
    const downloadPromise = page.waitForEvent('download');
    
    await reportButton.click();
    await page.getByTestId('report-format-pdf').click();
    await page.getByTestId('confirm-generate-button').click();
    
    const download = await downloadPromise;
    expect(download.suggestedFilename()).toMatch(/compliance-report.*\.pdf$/);
  });
});

// Security and Access Control Tests
test.describe('Security and Access Control', () => {
  test('should enforce role-based access', async ({ page }) => {
    // Test with restricted user
    await page.goto('/login');
    await page.getByTestId('username-input').fill('restricteduser');
    await page.getByTestId('password-input').fill('testpass');
    await page.getByTestId('login-button').click();
    
    // Should not have access to admin features
    await page.goto('/admin');
    await expect(page.getByText('Access Denied')).toBeVisible();
  });

  test('should handle session timeout', async ({ page }) => {
    // Login
    await page.goto('/login');
    await page.getByTestId('username-input').fill('testuser');
    await page.getByTestId('password-input').fill('testpass');
    await page.getByTestId('login-button').click();
    
    // Simulate session timeout by clearing storage
    await page.evaluate(() => {
      localStorage.clear();
      sessionStorage.clear();
    });
    
    // Try to access protected route
    await page.goto('/patients');
    
    // Should redirect to login
    await expect(page).toHaveURL(/.*login.*/);
  });

  test('should mask sensitive data based on permissions', async ({ page }) => {
    // Login as viewer role
    await page.goto('/login');
    await page.getByTestId('username-input').fill('viewer');
    await page.getByTestId('password-input').fill('testpass');
    await page.getByTestId('login-button').click();
    
    await page.goto('/patients');
    
    // Check that SSN and other sensitive fields are masked
    const ssnElements = page.getByTestId('patient-ssn');
    const count = await ssnElements.count();
    
    for (let i = 0; i < count; i++) {
      const ssnText = await ssnElements.nth(i).textContent();
      expect(ssnText).toMatch(/\*+/); // Should be masked
    }
  });
});