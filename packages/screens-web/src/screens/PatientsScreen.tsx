import React from 'react';
import { Container, Title, Button, Group, Text } from '@mantine/core';
import { useSelector, useDispatch } from 'react-redux';
import { PatientList } from '../components/PatientList';
import { fetchPatients } from '@open-hims/store';
import type { RootState, AppDispatch } from '@open-hims/store';

export function PatientsScreen() {
  const dispatch = useDispatch<AppDispatch>();
  const { patients, loading, error, pagination } = useSelector((state: RootState) => state.patients);
  
  const handleRefresh = () => {
    dispatch(fetchPatients({ 
      page: pagination.page, 
      pageSize: pagination.pageSize 
    }));
  };

  return (
    <Container size="xl" py="md">
      <Group justify="space-between" mb="md">
        <Title order={2}>Patient Management</Title>
        <Group>
          <Button onClick={handleRefresh} loading={loading.fetchPatients}>
            Refresh
          </Button>
          <Button variant="filled">
            Add Patient
          </Button>
        </Group>
      </Group>
      
      {error && (
        <Text c="red" mb="md">
          Error: {error}
        </Text>
      )}
      
      <PatientList patients={patients} loading={loading.fetchPatients} />
    </Container>
  );
}