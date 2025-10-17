import React from 'react';
import { Stack, Text, Loader, Center } from '@mantine/core';
import { PatientCard } from './PatientCard';
import type { UIPatient } from '@open-hims/types';

interface PatientListProps {
  patients: UIPatient[];
  loading?: boolean;
  error?: string | null;
  onPatientSelect?: (patient: UIPatient) => void;
}

export function PatientList({ patients, loading, error, onPatientSelect }: PatientListProps) {
  if (loading) {
    return (
      <Center py="xl">
        <Loader size="lg" />
      </Center>
    );
  }

  if (error) {
    return (
      <Center py="xl">
        <Text c="red">Error loading patients: {error}</Text>
      </Center>
    );
  }

  if (patients.length === 0) {
    return (
      <Center py="xl">
        <Text c="dimmed">No patients found</Text>
      </Center>
    );
  }

  return (
    <Stack gap="md">
      {patients.map((patient) => (
        <PatientCard
          key={patient.id}
          patient={patient}
          onClick={onPatientSelect ? () => onPatientSelect(patient) : undefined}
        />
      ))}
    </Stack>
  );
}